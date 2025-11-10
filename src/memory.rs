use crate::experience::Experience;

#[derive(Debug, Default)]
pub struct Memory {
    experiences: Vec<Experience>,
}

impl Memory {
    pub fn new() -> Self {
        Self { experiences: vec![] }
    }

    pub fn remember(&mut self, exp: Experience) {
        println!("🪶 Mengingat: \"{}\"", exp.content);
        self.experiences.push(exp);
    }

    pub fn reflect(&self) {
        println!("\n📜 Refleksi:");
        for e in &self.experiences {
            println!("- [{}] {} → {}", e.timestamp, e.source, e.content);
        }
    }
}
