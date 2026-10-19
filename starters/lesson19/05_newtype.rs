// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 19-5 · The Orphan Rule                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   You may implement a trait for a type only if the TRAIT or the TYPE
//   is defined in your crate. That's the ORPHAN RULE. It stops two crates
//   from both implementing, say, Display for Vec<String> and clashing.
//
//     impl fmt::Display for Vec<String> { ... }
//     // ✘ E0117: only traits defined in the current crate can be
//     //   implemented for types defined outside of the crate
//
//   The fix: wrap it in a NEWTYPE, a one-field struct of your own:
//
//     struct Roster(Vec<String>);                  // your type now
//
//     impl fmt::Display for Roster {               // ✔ allowed
//         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             write!(f, "{}", self.0.join(", "))
//         }
//     }
//
//   To keep using the Vec's methods, implement `Deref` (lesson 11):
//
//     impl Deref for Roster {
//         type Target = Vec<String>;
//         fn deref(&self) -> &Vec<String> { &self.0 }
//     }
//     roster.len();                                // Vec's method, via Deref
//
//   A newtype costs nothing at runtime: same memory as the inner value.
//
// 🎯 YOUR TASK
//
//   The code below breaks the orphan rule. Introduce a newtype
//   `Inventory(Vec<Item>)`, implement Display and Deref for it, and make
//   the tests pass.

use std::fmt;
use std::ops::Deref;

struct Item {
    name: String,
    qty: u32,
}

impl fmt::Display for Vec<Item> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts: Vec<String> = self.iter().map(|i| format!("{} x{}", i.name, i.qty)).collect();
        write!(f, "[{}]", parts.join(", "))
    }
}

fn main() {
    let inv = vec![Item { name: "rope".into(), qty: 2 }];
    println!("{inv}");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn inv() -> Inventory {
        Inventory(vec![
            Item { name: "rope".into(), qty: 2 },
            Item { name: "torch".into(), qty: 1 },
        ])
    }

    #[test]
    fn display() {
        assert_eq!(inv().to_string(), "[rope x2, torch x1]");
        assert_eq!(Inventory(vec![]).to_string(), "[]");
    }

    #[test]
    fn deref() {
        let i = inv();
        assert_eq!(i.len(), 2);
        assert_eq!(i[1].name, "torch");
        assert_eq!(i.iter().map(|x| x.qty).sum::<u32>(), 3);
    }
}
