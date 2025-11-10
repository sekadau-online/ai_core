use crate::{
    experience::Experience,
    pattern::PatternRecognizer,
};
use serde::{Deserialize, Serialize};

/// Chat message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: String, // "user" or "assistant"
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub context_used: Option<Vec<String>>, // IDs of experiences used for context
}

impl ChatMessage {
    pub fn new(role: &str, content: String) -> Self {
        Self {
            id: format!("msg_{}", uuid::Uuid::new_v4()),
            role: role.to_string(),
            content,
            timestamp: chrono::Utc::now(),
            context_used: None,
        }
    }

    pub fn with_context(role: &str, content: String, context: Vec<String>) -> Self {
        Self {
            id: format!("msg_{}", uuid::Uuid::new_v4()),
            role: role.to_string(),
            content,
            timestamp: chrono::Utc::now(),
            context_used: Some(context),
        }
    }

    /// Helper to create a user message
    pub fn user(content: &str) -> Self {
        Self::new("user", content.to_string())
    }

    /// Helper to create an assistant message
    pub fn assistant(content: String) -> Self {
        Self::new("assistant", content)
    }
}

/// Chat session management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub messages: Vec<ChatMessage>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ChatSession {
    pub fn new(session_id: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: session_id.to_string(),
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
        self.updated_at = chrono::Utc::now();
    }

    pub fn get_recent_messages(&self, count: usize) -> Vec<&ChatMessage> {
        self.messages.iter().rev().take(count).rev().collect()
    }
}

/// Chat processor with context-aware responses
pub struct ChatProcessor {
    pub ollama_client: Option<std::sync::Arc<crate::ollama::OllamaClient>>,
}

impl ChatProcessor {
    pub fn new() -> Self {
        Self {
            ollama_client: None,
        }
    }

    pub fn with_ollama(ollama_client: std::sync::Arc<crate::ollama::OllamaClient>) -> Self {
        Self {
            ollama_client: Some(ollama_client),
        }
    }

    /// Generate response based on user input and memory context
    pub async fn process_message(
        &self,
        user_input: &str,
        memory: &crate::memory::Memory,
        patterns: &mut PatternRecognizer,
    ) -> ChatMessage {
        // Analyze patterns from user input
        let keywords = Self::extract_keywords(user_input);
        
        // Search for relevant experiences
        let mut relevant_experiences: Vec<&Experience> = Vec::new();
        for keyword in &keywords {
            let results = memory.search(keyword);
            relevant_experiences.extend(results);
        }

        // Remove duplicates
        relevant_experiences.sort_by_key(|e| &e.id);
        relevant_experiences.dedup_by_key(|e| &e.id);

        // Build context
        let context_ids: Vec<String> = relevant_experiences
            .iter()
            .map(|e| e.id.clone())
            .collect();

        // Generate response
        let response_content = if let Some(ref ollama) = self.ollama_client {
            // Use Ollama for AI-powered responses
            if ollama.is_enabled() {
                let context_texts: Vec<String> = relevant_experiences
                    .iter()
                    .map(|e| format!("- {} (from {})", e.content, e.source))
                    .collect();

                match ollama.generate_with_context(user_input, &context_texts).await {
                    Ok(ai_response) => ai_response,
                    Err(e) => {
                        tracing::warn!("Ollama generation failed: {}. Using fallback.", e);
                        if relevant_experiences.is_empty() {
                            Self::generate_default_response(user_input)
                        } else {
                            Self::generate_context_aware_response(user_input, &relevant_experiences, patterns)
                        }
                    }
                }
            } else {
                // Ollama disabled, use fallback
                if relevant_experiences.is_empty() {
                    Self::generate_default_response(user_input)
                } else {
                    Self::generate_context_aware_response(user_input, &relevant_experiences, patterns)
                }
            }
        } else {
            // No Ollama client, use fallback
            if relevant_experiences.is_empty() {
                Self::generate_default_response(user_input)
            } else {
                Self::generate_context_aware_response(user_input, &relevant_experiences, patterns)
            }
        };

        ChatMessage::with_context("assistant", response_content, context_ids)
    }

    /// Extract keywords from user input
    fn extract_keywords(input: &str) -> Vec<String> {
        input
            .to_lowercase()
            .split_whitespace()
            .filter(|word| word.len() > 2) // Filter short words
            .map(|word| {
                // Remove punctuation
                word.chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .filter(|word| !word.is_empty())
            .collect()
    }

    /// Generate default response when no context found
    fn generate_default_response(user_input: &str) -> String {
        let input_lower = user_input.to_lowercase();
        
        if input_lower.contains("halo") || input_lower.contains("hello") || input_lower.contains("hi") {
            "Halo! Ada yang bisa saya bantu? Saya memiliki akses ke memori dan pengalaman yang tersimpan.".to_string()
        } else if input_lower.contains("apa") || input_lower.contains("what") {
            "Saya adalah AI Core yang dapat membantu Anda mengakses dan menganalisis informasi dari memori. Silakan tanyakan sesuatu yang lebih spesifik.".to_string()
        } else if input_lower.contains("bagaimana") || input_lower.contains("how") {
            "Saya menggunakan pattern recognition dan memory analysis untuk memberikan jawaban. Coba berikan lebih banyak konteks atau kata kunci.".to_string()
        } else if input_lower.contains("terima kasih") || input_lower.contains("thanks") {
            "Sama-sama! Senang bisa membantu. Ada yang lain yang ingin ditanyakan?".to_string()
        } else {
            format!(
                "Saya memahami pertanyaan Anda tentang '{}'. Namun, saat ini saya tidak menemukan informasi relevan dalam memori. \
                Silakan tambahkan lebih banyak pengalaman atau berikan konteks yang lebih spesifik.",
                user_input
            )
        }
    }

    /// Generate response based on relevant experiences
    fn generate_context_aware_response(
        _user_input: &str,
        experiences: &[&Experience],
        patterns: &mut PatternRecognizer,
    ) -> String {
        // Analyze patterns
        for exp in experiences {
            patterns.analyze(exp);
        }

        let top_patterns = patterns.get_top_patterns(5);
        
        let mut response = format!(
            "Berdasarkan {} pengalaman relevan yang saya temukan:\n\n",
            experiences.len()
        );

        // Add most relevant experiences
        let max_experiences = 3.min(experiences.len());
        for (i, exp) in experiences.iter().take(max_experiences).enumerate() {
            response.push_str(&format!("{}. {} (dari {})\n", i + 1, exp.content, exp.source));
        }

        // Add pattern insights
        if !top_patterns.is_empty() {
            response.push_str("\n🔍 Pola yang terdeteksi: ");
            let pattern_names: Vec<String> = top_patterns
                .iter()
                .take(3)
                .map(|p| p.keyword.clone())
                .collect();
            response.push_str(&pattern_names.join(", "));
        }

        response.push_str("\n\nApakah ini menjawab pertanyaan Anda?");
        
        response
    }

    /// Execute HTTP request and learn from response
    pub fn execute_http_request(
        &self,
        method: &str,
        url: &str,
        body: Option<String>,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<HttpResponse, String> {
        // Validate URL
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err("URL must start with http:// or https://".to_string());
        }

        // For now, return simulated response
        // In production, use reqwest to make actual HTTP calls
        Ok(HttpResponse {
            status: 200,
            body: format!(
                "Simulated {} request to {}\nHeaders: {:?}\nBody: {:?}\n\nNote: In production, this will make actual HTTP requests using reqwest.",
                method, url, headers, body
            ),
            success: true,
        })
    }
}

/// HTTP Response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
    pub success: bool,
}

/// API Learning Record - stores learned data from API calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiLearningRecord {
    pub id: String,
    pub method: String,
    pub url: String,
    pub request_body: Option<String>,
    pub response_body: String,
    pub status_code: u16,
    pub learned_at: chrono::DateTime<chrono::Utc>,
    pub tags: Vec<String>,
    pub summary: String,
}

impl ApiLearningRecord {
    pub fn new(
        method: String,
        url: String,
        request_body: Option<String>,
        response_body: String,
        status_code: u16,
    ) -> Self {
        Self {
            id: format!("api_{}", uuid::Uuid::new_v4()),
            method,
            url: url.clone(),
            request_body,
            response_body: response_body.clone(),
            status_code,
            learned_at: chrono::Utc::now(),
            tags: Self::extract_tags(&url, &response_body),
            summary: Self::generate_summary(&url, status_code),
        }
    }

    fn extract_tags(url: &str, _response: &str) -> Vec<String> {
        let mut tags = Vec::new();
        
        // Extract domain as tag
        if let Some(domain) = url.split('/').nth(2) {
            tags.push(domain.to_string());
        }
        
        // Extract path segments as tags
        let path_parts: Vec<&str> = url.split('/').skip(3).collect();
        for part in path_parts.iter().take(2) {
            if !part.is_empty() && !part.contains('?') {
                tags.push(part.to_string());
            }
        }
        
        tags
    }

    fn generate_summary(url: &str, status_code: u16) -> String {
        let status_text = if status_code >= 200 && status_code < 300 {
            "Success"
        } else if status_code >= 400 && status_code < 500 {
            "Client Error"
        } else if status_code >= 500 {
            "Server Error"
        } else {
            "Unknown"
        };
        
        format!("{} - {} ({})", status_text, url, status_code)
    }
}

/// Document processor for file uploads
pub struct DocumentProcessor;

impl DocumentProcessor {
    pub fn new() -> Self {
        Self
    }

    /// Process uploaded document and extract content
    pub fn process_document(&self, content: &str, filetype: &str) -> Result<String, String> {
        match filetype {
            "txt" | "text/plain" => {
                Ok(content.to_string())
            }
            "json" | "application/json" => {
                // Parse JSON and extract readable content
                match serde_json::from_str::<serde_json::Value>(content) {
                    Ok(json) => {
                        let extracted = Self::extract_from_json(&json);
                        Ok(extracted)
                    }
                    Err(e) => Err(format!("Invalid JSON: {}", e)),
                }
            }
            "csv" | "text/csv" => {
                // Parse CSV and convert to readable format
                let mut result = String::new();
                for (i, line) in content.lines().enumerate() {
                    if i == 0 {
                        result.push_str(&format!("CSV Headers: {}\n", line));
                    } else if !line.trim().is_empty() {
                        result.push_str(&format!("Row {}: {}\n", i, line));
                    }
                }
                Ok(result)
            }
            _ => {
                // Treat as plain text
                Ok(content.to_string())
            }
        }
    }

    /// Recursively extract text from JSON
    fn extract_from_json(value: &serde_json::Value) -> String {
        let mut result = String::new();
        
        match value {
            serde_json::Value::String(s) => {
                if !s.trim().is_empty() {
                    result.push_str(s);
                    result.push('\n');
                }
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    result.push_str(&Self::extract_from_json(item));
                }
            }
            serde_json::Value::Object(obj) => {
                for (key, val) in obj {
                    result.push_str(&format!("{}: ", key));
                    result.push_str(&Self::extract_from_json(val));
                }
            }
            serde_json::Value::Number(n) => {
                result.push_str(&n.to_string());
                result.push('\n');
            }
            serde_json::Value::Bool(b) => {
                result.push_str(&b.to_string());
                result.push('\n');
            }
            _ => {}
        }
        
        result
    }
}

/// Export functionality
pub struct ChatExporter;

impl ChatExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export_json(&self, session: &ChatSession) -> String {
        serde_json::to_string_pretty(session).unwrap_or_else(|e| format!("Error: {}", e))
    }

    pub fn export_txt(&self, session: &ChatSession) -> String {
        let mut output = format!("Chat Session: {}\n", session.id);
        output.push_str(&format!("Created: {}\n\n", session.created_at.format("%Y-%m-%d %H:%M:%S")));
        output.push_str(&"=".repeat(50));
        output.push('\n');

        for msg in &session.messages {
            output.push_str(&format!(
                "\n[{}] {}\n{}\n",
                msg.timestamp.format("%H:%M:%S"),
                msg.role.to_uppercase(),
                msg.content
            ));
            output.push_str(&"-".repeat(50));
            output.push('\n');
        }

        output
    }

    pub fn export_markdown(&self, session: &ChatSession) -> String {
        let mut output = format!("# Chat Session: {}\n\n", session.id);
        output.push_str(&format!("**Created:** {}\n\n", session.created_at.format("%Y-%m-%d %H:%M:%S")));
        output.push_str("---\n\n");

        for msg in &session.messages {
            let emoji = if msg.role == "user" { "👤" } else { "🤖" };
            output.push_str(&format!(
                "## {} {} ({})\n\n{}\n\n",
                emoji,
                msg.role.to_uppercase(),
                msg.timestamp.format("%H:%M:%S"),
                msg.content
            ));
        }

        output
    }

    pub fn export_html(&self, session: &ChatSession) -> String {
        let mut output = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Chat Export</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
        .message { margin: 20px 0; padding: 15px; border-radius: 8px; }
        .user { background-color: #e3f2fd; text-align: right; }
        .assistant { background-color: #f5f5f5; }
        .role { font-weight: bold; margin-bottom: 5px; }
        .time { color: #666; font-size: 0.9em; }
    </style>
</head>
<body>
"#,
        );

        output.push_str(&format!("<h1>Chat Session: {}</h1>\n", session.id));
        output.push_str(&format!("<p>Created: {}</p>\n", session.created_at.format("%Y-%m-%d %H:%M:%S")));
        output.push_str("<hr>\n");

        for msg in &session.messages {
            output.push_str(&format!(
                r#"<div class="message {}">
    <div class="role">{}</div>
    <div class="time">{}</div>
    <p>{}</p>
</div>
"#,
                msg.role,
                msg.role.to_uppercase(),
                msg.timestamp.format("%H:%M:%S"),
                msg.content
            ));
        }

        output.push_str("</body>\n</html>");
        output
    }
}
