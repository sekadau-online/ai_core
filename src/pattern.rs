use crate::experience::Experience;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pattern data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub keyword: String,
    pub frequency: usize,
    pub experience_ids: Vec<String>,
}

/// Recognizes and tracks patterns in experiences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternRecognizer {
    patterns: HashMap<String, Pattern>,
}

impl Default for PatternRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternRecognizer {
    /// Create a new pattern recognizer
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
        }
    }

    /// Analyze an experience and extract patterns
    pub fn analyze(&mut self, exp: &Experience) {
        let words: Vec<String> = exp
            .content
            .split_whitespace()
            .map(|w| w.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|w| w.len() > 2) // Skip very short words
            .collect();

        for word in words {
            self.patterns
                .entry(word.clone())
                .and_modify(|p| {
                    p.frequency += 1;
                    if !p.experience_ids.contains(&exp.id) {
                        p.experience_ids.push(exp.id.clone());
                    }
                })
                .or_insert_with(|| Pattern {
                    keyword: word,
                    frequency: 1,
                    experience_ids: vec![exp.id.clone()],
                });
        }
    }

    /// Get all recognized patterns
    pub fn get_patterns(&self) -> &HashMap<String, Pattern> {
        &self.patterns
    }

    /// Get pattern by keyword
    pub fn get_pattern(&self, keyword: &str) -> Option<&Pattern> {
        self.patterns.get(&keyword.to_lowercase())
    }

    /// Get top N most frequent patterns
    pub fn get_top_patterns(&self, n: usize) -> Vec<&Pattern> {
        let mut patterns: Vec<&Pattern> = self.patterns.values().collect();
        patterns.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        patterns.into_iter().take(n).collect()
    }

    /// Display recognized patterns
    pub fn show_patterns(&self) {
        println!("\n🔍 Recognized patterns ({} keywords):", self.patterns.len());
        let top = self.get_top_patterns(10);
        for pattern in top {
            println!(
                "- '{}': {} occurrences in {} experiences",
                pattern.keyword,
                pattern.frequency,
                pattern.experience_ids.len()
            );
        }
    }

    /// Clear all patterns
    pub fn clear(&mut self) {
        self.patterns.clear();
    }
}
