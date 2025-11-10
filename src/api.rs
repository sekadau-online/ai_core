use crate::{
    decision::DecisionMaker,
    dialog,
    experience::Experience,
    memory::SharedMemory,
    pattern::PatternRecognizer,
    personality::Personality,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

// ============ Request/Response DTOs ============

#[derive(Debug, Deserialize)]
pub struct CreateExperienceRequest {
    pub content: String,
    pub source: String,
    #[serde(default)]
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total_experiences: usize,
    pub total_patterns: usize,
    pub top_patterns: Vec<PatternInfo>,
}

#[derive(Debug, Serialize)]
pub struct PatternInfo {
    pub keyword: String,
    pub frequency: usize,
    pub experience_count: usize,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

// ============ Handlers ============

/// Health check endpoint
pub async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("AI Core is running".to_string()),
        message: "OK".to_string(),
    })
}

/// Get all experiences
pub async fn get_experiences(
    State(memory): State<SharedMemory>,
) -> Result<Json<ApiResponse<Vec<Experience>>>, StatusCode> {
    let mem = memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if mem.is_empty() {
        return Ok(Json(ApiResponse {
            success: true,
            data: Some(vec![]),
            message: "No experiences found. Memory is empty.".to_string(),
        }));
    }
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(mem.get_experiences().to_vec()),
        message: format!("Retrieved {} experiences", mem.experiences_len()),
    }))
}

/// Get experience by ID
pub async fn get_experience_by_id(
    State(memory): State<SharedMemory>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Experience>>, StatusCode> {
    let mem = memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match mem.get_by_id(&id) {
        Some(exp) => Ok(Json(ApiResponse {
            success: true,
            data: Some(exp.clone()),
            message: "Experience found".to_string(),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Create new experience
pub async fn create_experience(
    State(memory): State<SharedMemory>,
    Json(payload): Json<CreateExperienceRequest>,
) -> Result<Json<ApiResponse<Experience>>, StatusCode> {
    let exp = if let Some(metadata) = payload.metadata {
        Experience::with_metadata(&payload.content, &payload.source, metadata)
    } else {
        Experience::new(&payload.content, &payload.source)
    };

    let mut mem = memory.write().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    mem.remember(exp.clone());

    Ok(Json(ApiResponse {
        success: true,
        data: Some(exp),
        message: "Experience created successfully".to_string(),
    }))
}

/// Search experiences
pub async fn search_experiences(
    State(memory): State<SharedMemory>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<ApiResponse<Vec<Experience>>>, StatusCode> {
    let mem = memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let results: Vec<Experience> = mem.search(&params.q).into_iter().cloned().collect();
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(results.clone()),
        message: format!("Found {} matching experiences", results.len()),
    }))
}

/// Get statistics and patterns
pub async fn get_stats(
    State(memory): State<SharedMemory>,
) -> Result<Json<ApiResponse<StatsResponse>>, StatusCode> {
    let mem = memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut patterns = PatternRecognizer::new();
    for exp in mem.get_experiences() {
        patterns.analyze(exp);
    }

    // Show patterns in logs (uses show_patterns method)
    tracing::debug!("Pattern analysis:");
    patterns.show_patterns();

    let top_patterns: Vec<PatternInfo> = patterns
        .get_top_patterns(10)
        .into_iter()
        .map(|p| PatternInfo {
            keyword: p.keyword.clone(),
            frequency: p.frequency,
            experience_count: p.experience_ids.len(),
        })
        .collect();

    let stats = StatsResponse {
        total_experiences: mem.experiences_len(),
        total_patterns: patterns.get_patterns().len(),
        top_patterns,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(stats),
        message: "Statistics retrieved".to_string(),
    }))
}

/// Make a decision
pub async fn make_decision(
    State(memory): State<SharedMemory>,
) -> Result<Json<ApiResponse<crate::decision::Decision>>, StatusCode> {
    let mem = memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut patterns = PatternRecognizer::new();
    for exp in mem.get_experiences() {
        patterns.analyze(exp);
    }

    let decision = DecisionMaker::make_decision(&mem, &patterns);

    Ok(Json(ApiResponse {
        success: true,
        data: Some(decision),
        message: "Decision made".to_string(),
    }))
}

/// Make decision for query
pub async fn make_decision_for_query(
    State(memory): State<SharedMemory>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<ApiResponse<crate::decision::Decision>>, StatusCode> {
    let mem = memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let decision = DecisionMaker::make_decision_for_query(&mem, &params.q);

    Ok(Json(ApiResponse {
        success: true,
        data: Some(decision),
        message: format!("Decision made for query: '{}'", params.q),
    }))
}

/// Clear all experiences
pub async fn clear_memory(
    State(memory): State<SharedMemory>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let mut mem = memory.write().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    mem.clear();

    Ok(Json(ApiResponse {
        success: true,
        data: Some("Memory cleared".to_string()),
        message: "All experiences have been deleted".to_string(),
    }))
}

#[derive(Debug, Serialize)]
pub struct InteractResponse {
    pub analysis: String,
    pub experience_count: usize,
    pub pattern_summary: Vec<String>,
}

/// Interact with AI - analyze experiences with patterns
pub async fn interact_with_ai(
    State(memory): State<SharedMemory>,
) -> Result<Json<ApiResponse<InteractResponse>>, StatusCode> {
    let mem = memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut patterns = PatternRecognizer::new();
    
    // Use dialog::interact for logging and analysis
    dialog::interact(&mem, &mut patterns);
    
    // Get top patterns as summary
    let pattern_summary: Vec<String> = patterns
        .get_top_patterns(5)
        .into_iter()
        .map(|p| format!("{}: {} occurrences", p.keyword, p.frequency))
        .collect();
    
    let response = InteractResponse {
        analysis: format!("Analyzed {} experiences", mem.experiences_len()),
        experience_count: mem.experiences_len(),
        pattern_summary,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: "Interaction completed".to_string(),
    }))
}

#[derive(Debug, Serialize)]
pub struct PersonalityResponse {
    pub curiosity: f32,
    pub happiness: f32,
    pub caution: f32,
    pub dominant_trait: String,
    pub influenced_response: String,
}

#[derive(Debug, Deserialize)]
pub struct PersonalityRequest {
    pub input: String,
    pub response: String,
}

/// Update personality based on input
pub async fn update_personality(
    State(memory): State<SharedMemory>,
    Json(payload): Json<PersonalityRequest>,
) -> Result<Json<ApiResponse<PersonalityResponse>>, StatusCode> {
    let mem = memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut patterns = PatternRecognizer::new();
    for exp in mem.get_experiences() {
        patterns.analyze(exp);
    }
    
    let mut personality = Personality::new();
    personality.update(&payload.input, &mem, &patterns);
    
    let influenced = personality.influence_response(&payload.response);
    let dominant = personality.dominant_trait().to_string();
    
    let response = PersonalityResponse {
        curiosity: personality.curiosity,
        happiness: personality.happiness,
        caution: personality.caution,
        dominant_trait: dominant,
        influenced_response: influenced,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: "Personality updated".to_string(),
    }))
}

#[derive(Debug, Serialize)]
pub struct ReflectionResponse {
    pub total_experiences: usize,
    pub experiences: Vec<ReflectionItem>,
}

#[derive(Debug, Serialize)]
pub struct ReflectionItem {
    pub id: String,
    pub timestamp: String,
    pub source: String,
    pub content: String,
}

/// Get reflection view of all experiences
pub async fn reflect_memory(
    State(memory): State<SharedMemory>,
) -> Result<Json<ApiResponse<ReflectionResponse>>, StatusCode> {
    let mem = memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Use reflect method for logging
    tracing::info!("Memory reflection requested:");
    mem.reflect();
    
    let experiences: Vec<ReflectionItem> = mem
        .get_experiences()
        .iter()
        .map(|e| ReflectionItem {
            id: e.id.clone(),
            timestamp: e.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            source: e.source.clone(),
            content: e.content.clone(),
        })
        .collect();
    
    let response = ReflectionResponse {
        total_experiences: mem.experiences_len(),
        experiences,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: format!("Reflected on {} experiences", mem.experiences_len()),
    }))
}

#[derive(Debug, Serialize)]
pub struct PatternDetailResponse {
    pub keyword: String,
    pub frequency: usize,
    pub experience_ids: Vec<String>,
    pub related_experiences: Vec<String>,
}

/// Get specific pattern by keyword
pub async fn get_pattern_detail(
    State(memory): State<SharedMemory>,
    Path(keyword): Path<String>,
) -> Result<Json<ApiResponse<PatternDetailResponse>>, StatusCode> {
    let mem = memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut patterns = PatternRecognizer::new();
    for exp in mem.get_experiences() {
        patterns.analyze(exp);
    }

    match patterns.get_pattern(&keyword) {
        Some(pattern) => {
            let related_experiences: Vec<String> = pattern
                .experience_ids
                .iter()
                .filter_map(|id| mem.get_by_id(id))
                .map(|exp| exp.content.clone())
                .collect();

            let response = PatternDetailResponse {
                keyword: pattern.keyword.clone(),
                frequency: pattern.frequency,
                experience_ids: pattern.experience_ids.clone(),
                related_experiences,
            };

            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                message: format!("Found pattern for keyword: {}", keyword),
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Clear all patterns (rebuild from scratch)
pub async fn clear_patterns(
    State(memory): State<SharedMemory>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let mem = memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut patterns = PatternRecognizer::new();
    patterns.clear(); // Clear existing patterns
    
    // Rebuild patterns from memory
    for exp in mem.get_experiences() {
        patterns.analyze(exp);
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(format!("Patterns rebuilt. Found {} unique patterns", patterns.get_patterns().len())),
        message: "Pattern cache cleared and rebuilt".to_string(),
    }))
}
