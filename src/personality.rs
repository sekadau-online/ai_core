use crate::memory::Memory;
use crate::pattern::PatternRecognizer;
use serde::{Deserialize, Serialize};

/// AI personality traits that evolve over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub curiosity: f32,  // 0.0 - 1.0, how curious the AI is
    pub happiness: f32,  // 0.0 - 1.0, mood level
    pub caution: f32,    // 0.0 - 1.0, carefulness level
}

impl Default for Personality {
    fn default() -> Self {
        Self::new()
    }
}

impl Personality {
    /// Create a new personality with balanced traits
    pub fn new() -> Self {
        Self {
            curiosity: 0.5,
            happiness: 0.5,
            caution: 0.5,
        }
    }

    /// Update personality based on new input
    pub fn update(&mut self, input: &str, _mem: &Memory, _patterns: &PatternRecognizer) {
        let input_lower = input.to_lowercase();
        
        // Increase happiness on positive words
        if input_lower.contains("halo") || input_lower.contains("hello") || input_lower.contains("terima kasih") {
            self.happiness += 0.1;
        }
        
        // Increase curiosity on questions
        if input_lower.contains("apa") || input_lower.contains("mengapa") || input_lower.contains("bagaimana") {
            self.curiosity += 0.1;
        }
        
        // Increase caution on negative/warning words
        if input_lower.contains("bahaya") || input_lower.contains("error") || input_lower.contains("warning") {
            self.caution += 0.2;
        }

        // Clamp values between 0.0 and 1.0
        self.happiness = self.happiness.clamp(0.0, 1.0);
        self.curiosity = self.curiosity.clamp(0.0, 1.0);
        self.caution = self.caution.clamp(0.0, 1.0);
    }

    /// Influence response based on current personality state
    pub fn influence_response(&self, reply: &str) -> String {
        if self.happiness > 0.7 {
            format!("😊 {}", reply)
        } else if self.curiosity > 0.7 {
            format!("🤔 {}", reply)
        } else if self.caution > 0.7 {
            format!("⚠️ {}", reply)
        } else {
            reply.to_string()
        }
    }

    /// Get the dominant trait
    pub fn dominant_trait(&self) -> &str {
        if self.happiness >= self.curiosity && self.happiness >= self.caution {
            "happy"
        } else if self.curiosity >= self.caution {
            "curious"
        } else {
            "cautious"
        }
    }
}
