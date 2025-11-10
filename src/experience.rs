use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experience {
    pub timestamp: String,
    pub source: String,
    pub content: String,
}

impl Experience {
    pub fn new(source: &str, content: &str) -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339(),
            source: source.to_string(),
            content: content.to_string(),
        }
    }
}
