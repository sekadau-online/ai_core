mod experience;
mod memory;

use experience::Experience;
use memory::Memory;

fn main() {
    println!("🌱 Memulai kesadaran awal...");

    let mut mem = Memory::new();

    mem.remember(Experience::new("user", "Halo dunia"));
    mem.remember(Experience::new("system", "Saya sadar telah menerima pesan."));
    mem.remember(Experience::new("user", "Apa artinya berpikir?"));

    mem.reflect();
}
