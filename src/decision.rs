use crate::memory::Memory;
use crate::pattern::PatternRecognizer;
use serde::{Deserialize, Serialize};

/// Decision result with reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub action: String,
    pub confidence: f32,
    pub reasoning: String,
    pub based_on_experiences: usize,
}

/// Decision maker that uses memory and patterns
pub struct DecisionMaker;

impl DecisionMaker {
    /// Make a decision based on memory and patterns
    pub fn make_decision(mem: &Memory, patterns: &PatternRecognizer) -> Decision {
        let exp_count = mem.experiences_len();

        if exp_count == 0 {
            return Decision {
                action: "default".to_string(),
                confidence: 0.5,
                reasoning: "No previous experiences available. Using default behavior."
                    .to_string(),
                based_on_experiences: 0,
            };
        }

        // Analyze patterns to make decision
        let top_patterns = patterns.get_top_patterns(5);
        let total_patterns = patterns.get_patterns().len();

        let confidence = if exp_count > 10 && total_patterns > 20 {
            0.9
        } else if exp_count > 5 {
            0.7
        } else {
            0.6
        };

        let reasoning = if !top_patterns.is_empty() {
            format!(
                "Based on {} experiences and {} recognized patterns. Top pattern: '{}'",
                exp_count,
                total_patterns,
                top_patterns[0].keyword
            )
        } else {
            format!("Based on {} experiences with limited pattern recognition", exp_count)
        };

        Decision {
            action: "continue_learning".to_string(),
            confidence,
            reasoning,
            based_on_experiences: exp_count,
        }
    }

    /// Make decision with custom query
    pub fn make_decision_for_query(mem: &Memory, query: &str) -> Decision {
        let relevant = mem.search(query);
        let count = relevant.len();

        if count == 0 {
            Decision {
                action: "ask_for_clarification".to_string(),
                confidence: 0.3,
                reasoning: format!("No relevant experiences found for query: '{}'", query),
                based_on_experiences: 0,
            }
        } else {
            Decision {
                action: "provide_response".to_string(),
                confidence: (count as f32 / 10.0).min(0.95),
                reasoning: format!("Found {} relevant experiences for query: '{}'", count, query),
                based_on_experiences: count,
            }
        }
    }
}
