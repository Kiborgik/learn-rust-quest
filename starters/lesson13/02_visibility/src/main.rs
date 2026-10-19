// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 13-2 · Private Parts                             150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Everything in a module is PRIVATE by default. Code outside the module
//   only sees what is marked `pub`:
//
//     pub struct Gem {
//         pub name: String,     // readable and writable from outside
//         value: u32,           // private: only code in this module
//     }
//
//     impl Gem {
//         pub fn new(name: &str) -> Gem { Gem { name: name.into(), value: 1 } }
//         pub fn value(&self) -> u32 { self.value }   // a public "getter"
//         fn secret(&self) {}                          // private helper
//     }
//
//   • A `pub struct` with a private field can't be built with `Gem { .. }`
//     from outside. Callers must use your constructor. That's how you keep
//     invariants (here: the value can't be set to nonsense).
//   • `pub(crate)` = visible in this whole crate, but not to other crates.
//   • Child modules can see their parent's private items, not the other
//     way round.
//
// 🎯 YOUR TASK
//
//   Open `src/forge.rs`. The tests below (outside the module) can't reach
//   what they need. Make the right items public, and nothing more:
//   `sharpness` must stay a private field.

mod forge;

fn main() {
    let mut s = forge::Sword::new("Edge");
    s.sharpen();
    println!("{} has sharpness {}", s.name, s.sharpness());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use crate::forge::{MAX_SHARPNESS, Sword};

    #[test]
    fn forging() {
        let mut s = Sword::new("Edge");
        assert_eq!(s.name, "Edge");
        assert_eq!(s.sharpness(), 1);
        s.sharpen();
        assert_eq!(s.sharpness(), 2);
    }

    #[test]
    fn capped() {
        let mut s = Sword::new("Edge");
        for _ in 0..100 {
            s.sharpen();
        }
        assert_eq!(s.sharpness(), MAX_SHARPNESS);
    }
}
