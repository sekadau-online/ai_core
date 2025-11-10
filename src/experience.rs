use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single experience or memory entry
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experience {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub content: String,
    #[serde(default)]
    pub metadata: Option<String>,
}

impl Experience {
    /// Create a new experience with automatic timestamp and ID
    pub fn new(content: &str, source: &str) -> Self {
        let now = Utc::now();
        Self {
            id: format!("{}-{}", now.timestamp_millis(), source),
            timestamp: now,
            source: source.to_string(),
            content: content.to_string(),
            metadata: None,
        }
    }

    /// Create experience with custom metadata
    pub fn with_metadata(content: &str, source: &str, metadata: String) -> Self {
        let mut exp = Self::new(content, source);
        exp.metadata = Some(metadata);
        exp
    }
}
