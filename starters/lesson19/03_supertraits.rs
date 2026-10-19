// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 19-3 · Prerequisites                             175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A SUPERTRAIT is a trait another trait requires. "To be a Hero, you
//   must also be Named":
//
//     trait Named {
//         fn name(&self) -> String;
//     }
//
//     trait Hero: Named {                        // Hero requires Named
//         fn level(&self) -> u32;
//         fn intro(&self) -> String {
//             format!("{}, level {}", self.name(), self.level())   // can use name()!
//         }
//     }
//
//   Implementing Hero for a type that isn't Named is a compile error.
//   Several at once: `trait Hero: Named + fmt::Display + Clone`.
//
//   Not inheritance: there's no parent object, just "anything that
//   implements Hero also implements Named".
//
//   Two traits with the same method name? Say which one you mean:
//
//     Named::name(&x)              // or, fully qualified:
//     <Knight as Named>::name(&x)
//
// 🎯 YOUR TASK
//
//   1. `Champion`'s default method uses `self.name()` and `{self}`. Make
//      it compile by giving Champion the right supertraits.
//   2. `Knight` is missing something Champion needs. Add it.

use std::fmt;

trait Named {
    fn name(&self) -> String;
}

trait Champion {
    fn victories(&self) -> u32;

    fn boast(&self) -> String {
        format!("I am {}, {self}, with {} victories!", self.name(), self.victories())
    }
}

struct Knight {
    name: String,
    wins: u32,
}

impl Named for Knight {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Champion for Knight {
    fn victories(&self) -> u32 {
        self.wins
    }
}

fn main() {
    let k = Knight { name: "Ferris".into(), wins: 12 };
    println!("{}", k.boast());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boasting() {
        let k = Knight { name: "Ferris".into(), wins: 12 };
        assert_eq!(k.to_string(), "Sir Ferris");
        assert_eq!(k.boast(), "I am Ferris, Sir Ferris, with 12 victories!");
    }
}
