use crate::experience::Experience;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use std::sync::{Arc, RwLock};

/// Thread-safe memory storage for experiences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    experiences: Vec<Experience>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// Create a new empty memory
    pub fn new() -> Self {
        Self {
            experiences: Vec::new(),
        }
    }

    /// Get all experiences as a slice
    pub fn get_experiences(&self) -> &[Experience] {
        &self.experiences
    }

    /// Get number of stored experiences
    pub fn experiences_len(&self) -> usize {
        self.experiences.len()
    }

    /// Check if memory is empty
    pub fn is_empty(&self) -> bool {
        self.experiences.is_empty()
    }

    /// Add a new experience to memory
    pub fn remember(&mut self, exp: Experience) {
        tracing::info!("🪶 Remembering: \"{}\" from {}", exp.content, exp.source);
        self.experiences.push(exp);
    }

    /// Get experience by ID
    pub fn get_by_id(&self, id: &str) -> Option<&Experience> {
        self.experiences.iter().find(|e| e.id == id)
    }

    /// Search experiences by content
    pub fn search(&self, query: &str) -> Vec<&Experience> {
        let query_lower = query.to_lowercase();
        self.experiences
            .iter()
            .filter(|e| e.content.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Display all experiences (for debugging)
    pub fn reflect(&self) {
        println!("\n📜 Reflection ({} experiences):", self.experiences.len());
        for e in &self.experiences {
            println!(
                "- [{}] {} → {}",
                e.timestamp.format("%Y-%m-%d %H:%M:%S"),
                e.source,
                e.content
            );
        }
    }

    /// Save memory to JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load memory from JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let memory: Memory = serde_json::from_str(&contents)?;
        Ok(memory)
    }

    /// Clear all experiences
    pub fn clear(&mut self) {
        self.experiences.clear();
    }
}

/// Thread-safe shared memory wrapper
pub type SharedMemory = Arc<RwLock<Memory>>;
