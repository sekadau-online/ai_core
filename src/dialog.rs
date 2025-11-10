use crate::{memory::Memory, pattern::PatternRecognizer};

/// Interact with memory and update patterns
pub fn interact(mem: &Memory, patterns: &mut PatternRecognizer) {
    println!("\n💬 Interaction Summary:");
    println!("   Total experiences: {}", mem.experiences_len());

    if mem.is_empty() {
        println!("   No experiences to analyze yet.");
        return;
    }

    println!("   Analyzing patterns...");
    for exp in mem.get_experiences() {
        patterns.analyze(exp);
    }

    patterns.show_patterns();
}
