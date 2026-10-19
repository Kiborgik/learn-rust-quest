// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 4-3 · Skills & Abilities                         100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Methods live in an `impl` block. The first parameter decides how
//   the method gets the instance. It's the ownership rules again:
//
//     struct Torch { fuel: u32 }
//
//     impl Torch {
//         fn is_lit(&self) -> bool {          // &self     → READ
//             self.fuel > 0
//         }
//         fn burn(&mut self) {                // &mut self → MODIFY
//             self.fuel -= 1;
//         }
//         fn into_ash(self) -> String {       // self      → CONSUME
//             String::from("ash")             // (torch is gone after this)
//         }
//     }
//
//     let mut t = Torch { fuel: 2 };
//     t.burn();                 // Rust auto-borrows: (&mut t).burn()
//     t.is_lit();               // (&t).is_lit()
//     let a = t.into_ash();     // t is moved; can't use it anymore
//
//   No `this` keyword: you name `self` explicitly. There's also no
//   inheritance (traits replace it in lesson 8).
//
// 🎯 YOUR TASK
//
//   1. `is_empty` and `drink` have the wrong kind of `self`. Fix them.
//   2. Add a method `label` that returns e.g. "Healing Potion [2]".

struct Potion {
    name: String,
    doses: u32,
}

impl Potion {
    fn is_empty(self) -> bool {
        self.doses == 0
    }

    /// Drink one dose. Returns false if there was nothing left.
    fn drink(&self) -> bool {
        if self.doses == 0 {
            return false;
        }
        self.doses -= 1;
        true
    }

    // TODO: fn label(...) -> String

    fn pour_out(self) -> String {
        self.name
    }
}

fn main() {
    let mut p = Potion { name: String::from("Healing Potion"), doses: 2 };
    p.drink();
    println!("{}", p.label());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potion_lifecycle() {
        let mut p = Potion { name: String::from("Healing Potion"), doses: 2 };
        assert!(!p.is_empty());
        assert_eq!(p.label(), "Healing Potion [2]");
        assert!(p.drink());
        assert_eq!(p.label(), "Healing Potion [1]");
        assert!(p.drink());
        assert!(p.is_empty());
        assert!(!p.drink());
        assert_eq!(p.pour_out(), "Healing Potion");
    }
}
