// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 8-5 · Royal Protocols                            125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The standard library is built on traits. Implement them, and your
//   types plug straight into the language:
//
//     use std::fmt;
//     impl fmt::Display for Gold {                    // {} printing
//         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             write!(f, "{}g", self.0)
//         }
//     }
//
//     impl Default for Config {                       // Config::default()
//         fn default() -> Self { Config { volume: 5 } }
//     }
//
//     impl From<u32> for Gold {                       // Gold::from(5) / 5.into()
//         fn from(n: u32) -> Self { Gold(n) }
//     }
//
//     use std::ops::Add;
//     impl Add for Gold {                             // gold + gold
//         type Output = Gold;
//         fn add(self, other: Gold) -> Gold { Gold(self.0 + other.0) }
//     }
//
//   Ordering: `#[derive(PartialEq, Eq, PartialOrd, Ord)]` compares
//   fields IN DECLARATION ORDER (enum variants by their position), which
//   is what `.sort()` uses.
//
// 🎯 YOUR TASK
//
//   1. `Gold` prints as "15g".
//   2. `Gold::from(7)` and `let g: Gold = 7u32.into();` both work.
//   3. `Gold + Gold` works.
//   4. `Settings::default()` gives difficulty "normal" and volume 5.
//   5. `Rank` sorts Bronze < Silver < Gold. Add the right derives.

use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Gold(u32);

#[derive(Debug, PartialEq)]
struct Settings {
    difficulty: String,
    volume: u8,
}

#[derive(Debug)]
enum Rank {
    Bronze,
    Silver,
    Gold,
}

// Your impls go here ↓


fn main() {
    let total = Gold(10) + Gold::from(5);
    println!("Total: {total}");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Gold(15).to_string(), "15g");
        assert_eq!(format!("[{}]", Gold(0)), "[0g]");
    }

    #[test]
    fn conversions() {
        assert_eq!(Gold::from(7), Gold(7));
        let g: Gold = 9u32.into();
        assert_eq!(g, Gold(9));
    }

    #[test]
    fn adding() {
        assert_eq!(Gold(10) + Gold(5), Gold(15));
    }

    #[test]
    fn defaults() {
        let s = Settings::default();
        assert_eq!(s, Settings { difficulty: "normal".into(), volume: 5 });
    }

    #[test]
    fn ranks() {
        let mut ranks = vec![Rank::Gold, Rank::Bronze, Rank::Silver, Rank::Bronze];
        ranks.sort();
        assert_eq!(ranks, vec![Rank::Bronze, Rank::Bronze, Rank::Silver, Rank::Gold]);
        assert!(Rank::Silver > Rank::Bronze);
    }
}
