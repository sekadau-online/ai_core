use crate::{
    chat::{ChatExporter, ChatMessage, ChatProcessor, ChatSession, DocumentProcessor},
    decision::DecisionMaker,
    dialog,
    experience::Experience,
    memory::SharedMemory,
    ollama::OllamaClient,
    pattern::PatternRecognizer,
    personality::Personality,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

// ============ Application State ============

#[derive(Clone)]
pub struct AppState {
    pub memory: SharedMemory,
    pub ollama: Arc<OllamaClient>,
}

impl AppState {
    pub fn new(memory: SharedMemory, ollama: Arc<OllamaClient>) -> Self {
        Self { memory, ollama }
    }
}

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
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<Experience>>>, StatusCode> {
    let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Experience>>, StatusCode> {
    let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    State(state): State<AppState>,
    Json(payload): Json<CreateExperienceRequest>,
) -> Result<Json<ApiResponse<Experience>>, StatusCode> {
    let exp = if let Some(metadata) = payload.metadata {
        Experience::with_metadata(&payload.content, &payload.source, metadata)
    } else {
        Experience::new(&payload.content, &payload.source)
    };

    let mut mem = state.memory.write().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    mem.remember(exp.clone());

    Ok(Json(ApiResponse {
        success: true,
        data: Some(exp),
        message: "Experience created successfully".to_string(),
    }))
}

/// Search experiences
pub async fn search_experiences(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<ApiResponse<Vec<Experience>>>, StatusCode> {
    let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let results: Vec<Experience> = mem.search(&params.q).into_iter().cloned().collect();
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(results.clone()),
        message: format!("Found {} matching experiences", results.len()),
    }))
}

/// Get statistics and patterns
pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<StatsResponse>>, StatusCode> {
    let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<crate::decision::Decision>>, StatusCode> {
    let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<ApiResponse<crate::decision::Decision>>, StatusCode> {
    let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let decision = DecisionMaker::make_decision_for_query(&mem, &params.q);

    Ok(Json(ApiResponse {
        success: true,
        data: Some(decision),
        message: format!("Decision made for query: '{}'", params.q),
    }))
}

/// Clear all experiences
pub async fn clear_memory(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let mut mem = state.memory.write().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<InteractResponse>>, StatusCode> {
    let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    State(state): State<AppState>,
    Json(payload): Json<PersonalityRequest>,
) -> Result<Json<ApiResponse<PersonalityResponse>>, StatusCode> {
    let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ReflectionResponse>>, StatusCode> {
    let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    State(state): State<AppState>,
    Path(keyword): Path<String>,
) -> Result<Json<ApiResponse<PatternDetailResponse>>, StatusCode> {
    let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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

// ============ Chat-related DTOs and State ============

#[derive(Debug, Deserialize)]
pub struct ChatMessageRequest {
    pub content: String,
    #[serde(default)]
    pub session_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatMessageResponse {
    pub session_id: String,
    pub message: ChatMessage,
    pub context_count: usize,
}

#[derive(Debug, Deserialize)]
pub struct DocumentUploadRequest {
    pub filename: String,
    pub content: String,
    pub filetype: String, // "txt", "json", "csv"
}

#[derive(Debug, Serialize)]
pub struct DocumentUploadResponse {
    pub processed: bool,
    pub text: String,
    pub added_to_memory: bool,
}

#[derive(Debug, Deserialize)]
pub struct ExportRequest {
    pub session_id: String,
    pub format: String, // "json", "txt", "markdown", "html"
}

#[derive(Debug, Deserialize)]
pub struct HttpRequest {
    pub method: String, // GET, POST, PUT, DELETE
    pub url: String,
    pub body: Option<String>,
    pub headers: Option<Vec<(String, String)>>,
    pub save_to_memory: Option<bool>, // Save response to learning records
}

#[derive(Debug, Serialize)]
pub struct HttpRequestResponse {
    pub success: bool,
    pub status: u16,
    pub body: String,
    pub learning_record_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLearningRecordRequest {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
}

// Global chat sessions storage (in a real app, use a database)
lazy_static::lazy_static! {
    static ref CHAT_SESSIONS: Arc<RwLock<std::collections::HashMap<String, ChatSession>>> = 
        Arc::new(RwLock::new(std::collections::HashMap::new()));
    
    static ref API_LEARNING_RECORDS: Arc<RwLock<std::collections::HashMap<String, crate::chat::ApiLearningRecord>>> = 
        Arc::new(RwLock::new(std::collections::HashMap::new()));
}

// ============ Chat Handlers ============

/// Send a chat message and get AI response
pub async fn send_chat_message(
    State(state): State<AppState>,
    Json(payload): Json<ChatMessageRequest>,
) -> Result<Json<ApiResponse<ChatMessageResponse>>, StatusCode> {
    // Get or create session
    let session_id = payload.session_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    
    // Add user message to session
    {
        let mut sessions = CHAT_SESSIONS.write().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let session = sessions.entry(session_id.clone()).or_insert_with(|| ChatSession::new(&session_id));
        session.add_message(ChatMessage::user(&payload.content));
    }
    
    // Collect data for processing (outside of locks)
    let (experiences, user_content, ollama_client) = {
        let mem = state.memory.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let experiences: Vec<Experience> = mem.get_experiences().to_vec();
        let user_content = payload.content.clone();
        let ollama_client = state.ollama.clone();
        (experiences, user_content, ollama_client)
    };
    
    // Process with AI (no locks held during await)
    let ai_message = {
        use crate::memory::Memory;
        let temp_memory = Memory::from_experiences(experiences);
        let mut patterns = PatternRecognizer::new();
        for exp in temp_memory.get_experiences() {
            patterns.analyze(exp);
        }
        
        let processor = ChatProcessor::with_ollama(ollama_client);
        processor.process_message(&user_content, &temp_memory, &mut patterns).await
    };
    
    // Add AI response to session
    let context_count = ai_message.context_used.as_ref().map(|c| c.len()).unwrap_or(0);
    {
        let mut sessions = CHAT_SESSIONS.write().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.add_message(ai_message.clone());
        }
    }
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(ChatMessageResponse {
            session_id,
            message: ai_message,
            context_count,
        }),
        message: "Message processed".to_string(),
    }))
}

/// Get chat history for a session
pub async fn get_chat_history(
    Path(session_id): Path<String>,
) -> Result<Json<ApiResponse<ChatSession>>, StatusCode> {
    let sessions = CHAT_SESSIONS.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match sessions.get(&session_id) {
        Some(session) => Ok(Json(ApiResponse {
            success: true,
            data: Some(session.clone()),
            message: format!("Retrieved {} messages", session.messages.len()),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Upload and process a document
pub async fn upload_document(
    State(state): State<AppState>,
    Json(payload): Json<DocumentUploadRequest>,
) -> Result<Json<ApiResponse<DocumentUploadResponse>>, StatusCode> {
    let processor = DocumentProcessor::new();
    
    let text = match processor.process_document(&payload.content, &payload.filetype) {
        Ok(text) => text,
        Err(e) => return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: format!("Failed to process document: {}", e),
        })),
    };
    
    // Add to memory as an experience
    let mut mem = state.memory.write().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let exp = Experience::new(&text, &format!("document:{}", payload.filename));
    mem.remember(exp);
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(DocumentUploadResponse {
            processed: true,
            text: text.to_string(),
            added_to_memory: true,
        }),
        message: format!("Document '{}' processed and added to memory", payload.filename),
    }))
}

/// Export chat session
pub async fn export_chat_session(
    Query(params): Query<ExportRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let sessions = CHAT_SESSIONS.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let session = match sessions.get(&params.session_id) {
        Some(s) => s,
        None => return Err(StatusCode::NOT_FOUND),
    };
    
    let exporter = ChatExporter::new();
    let exported = match params.format.as_str() {
        "json" => exporter.export_json(session),
        "txt" => exporter.export_txt(session),
        "markdown" | "md" => exporter.export_markdown(session),
        "html" => exporter.export_html(session),
        _ => return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: format!("Unsupported format: {}. Use json, txt, markdown, or html", params.format),
        })),
    };
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(exported),
        message: format!("Chat session exported as {}", params.format),
    }))
}

/// Execute HTTP request and optionally save to learning records
pub async fn execute_http_request(
    State(state): State<AppState>,
    Json(payload): Json<HttpRequest>,
) -> Result<Json<ApiResponse<HttpRequestResponse>>, StatusCode> {
    let processor = ChatProcessor::new();
    
    match processor.execute_http_request(
        &payload.method,
        &payload.url,
        payload.body.clone(),
        payload.headers.clone(),
    ) {
        Ok(response) => {
            let mut learning_record_id = None;
            
            // Save to learning records if requested
            if payload.save_to_memory.unwrap_or(true) {
                let record = crate::chat::ApiLearningRecord::new(
                    payload.method.clone(),
                    payload.url.clone(),
                    payload.body.clone(),
                    response.body.clone(),
                    response.status,
                );
                
                learning_record_id = Some(record.id.clone());
                
                // Store in learning records
                let mut records = API_LEARNING_RECORDS.write()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                records.insert(record.id.clone(), record.clone());
                
                // Also add to memory as experience
                let mut mem = state.memory.write()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                let exp = Experience::with_metadata(
                    &format!("API Call: {} {} - Status {}", payload.method, payload.url, response.status),
                    "api_learning",
                    format!("record_id:{}", record.id),
                );
                mem.remember(exp);
            }
            
            Ok(Json(ApiResponse {
                success: true,
                data: Some(HttpRequestResponse {
                    success: response.success,
                    status: response.status,
                    body: response.body,
                    learning_record_id,
                }),
                message: "HTTP request executed".to_string(),
            }))
        }
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: Some(HttpRequestResponse {
                success: false,
                status: 500,
                body: e.to_string(),
                learning_record_id: None,
            }),
            message: "HTTP request failed".to_string(),
        })),
    }
}

/// Get all API learning records
pub async fn get_learning_records() -> Result<Json<ApiResponse<Vec<crate::chat::ApiLearningRecord>>>, StatusCode> {
    let records = API_LEARNING_RECORDS.read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut records_vec: Vec<crate::chat::ApiLearningRecord> = records.values().cloned().collect();
    records_vec.sort_by(|a, b| b.learned_at.cmp(&a.learned_at));
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(records_vec.clone()),
        message: format!("Retrieved {} learning records", records_vec.len()),
    }))
}

/// Get learning record by ID
pub async fn get_learning_record_by_id(
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<crate::chat::ApiLearningRecord>>, StatusCode> {
    let records = API_LEARNING_RECORDS.read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match records.get(&id) {
        Some(record) => Ok(Json(ApiResponse {
            success: true,
            data: Some(record.clone()),
            message: "Learning record found".to_string(),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Update learning record (tags and summary)
pub async fn update_learning_record(
    Path(id): Path<String>,
    Json(payload): Json<UpdateLearningRecordRequest>,
) -> Result<Json<ApiResponse<crate::chat::ApiLearningRecord>>, StatusCode> {
    let mut records = API_LEARNING_RECORDS.write()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match records.get_mut(&id) {
        Some(record) => {
            if let Some(tags) = payload.tags {
                record.tags = tags;
            }
            if let Some(summary) = payload.summary {
                record.summary = summary;
            }
            
            Ok(Json(ApiResponse {
                success: true,
                data: Some(record.clone()),
                message: "Learning record updated".to_string(),
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Delete learning record
pub async fn delete_learning_record(
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let mut records = API_LEARNING_RECORDS.write()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match records.remove(&id) {
        Some(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some("Learning record deleted".to_string()),
            message: format!("Record {} has been deleted", id),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Search learning records by tag or URL
pub async fn search_learning_records(
    Query(params): Query<SearchQuery>,
) -> Result<Json<ApiResponse<Vec<crate::chat::ApiLearningRecord>>>, StatusCode> {
    let records = API_LEARNING_RECORDS.read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let query_lower = params.q.to_lowercase();
    let results: Vec<crate::chat::ApiLearningRecord> = records
        .values()
        .filter(|r| {
            r.url.to_lowercase().contains(&query_lower)
                || r.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
                || r.summary.to_lowercase().contains(&query_lower)
        })
        .cloned()
        .collect();
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(results.clone()),
        message: format!("Found {} matching records", results.len()),
    }))
}

/// Clear all learning records
pub async fn clear_learning_records() -> Result<Json<ApiResponse<String>>, StatusCode> {
    let mut records = API_LEARNING_RECORDS.write()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let count = records.len();
    records.clear();
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(format!("Cleared {} learning records", count)),
        message: "All learning records deleted".to_string(),
    }))
}

/// Get all chat sessions (list)
pub async fn list_chat_sessions(
) -> Result<Json<ApiResponse<Vec<String>>>, StatusCode> {
    let sessions = CHAT_SESSIONS.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let session_ids: Vec<String> = sessions.keys().cloned().collect();
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(session_ids.clone()),
        message: format!("Found {} active chat sessions", session_ids.len()),
    }))
}

/// Clear a specific chat session
pub async fn clear_chat_session(
    Path(session_id): Path<String>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let mut sessions = CHAT_SESSIONS.write().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match sessions.remove(&session_id) {
        Some(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some("Session cleared".to_string()),
            message: format!("Chat session {} has been deleted", session_id),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}
