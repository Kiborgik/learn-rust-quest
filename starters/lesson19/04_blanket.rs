// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 19-4 · A Blanket for Everyone                    175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A BLANKET IMPL implements a trait for every type that meets a bound:
//
//     trait Shout {
//         fn shout(&self) -> String;
//     }
//
//     impl<T: fmt::Display> Shout for T {          // every Display type
//         fn shout(&self) -> String {
//             self.to_string().to_uppercase() + "!"
//         }
//     }
//
//     5.shout();        // "5!"
//     "hey".shout();    // "HEY!"
//
//   std is full of these. `ToString` is one:
//     impl<T: Display + ?Sized> ToString for T { ... }
//   That's why implementing Display gives you `.to_string()` for free.
//
//   ⚠️ Coherence: once a blanket impl covers a type, you can't ALSO write
//   a specific impl for that type (E0119: conflicting implementations).
//
// 🎯 YOUR TASK
//
//   `Describe` only works for i32. The tests want it for every type that
//   implements Display. Replace the specific impl with a blanket impl.

use std::fmt;

trait Describe {
    fn describe(&self) -> String;
}

impl Describe for i32 {
    fn describe(&self) -> String {
        format!("<<{self}>>")
    }
}

struct Rune(char);

impl fmt::Display for Rune {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rune {}", self.0)
    }
}

fn main() {
    println!("{}", 7.describe());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn everything_describes() {
        assert_eq!(7.describe(), "<<7>>");
        assert_eq!("crab".describe(), "<<crab>>");
        assert_eq!(2.5.describe(), "<<2.5>>");
        assert_eq!(Rune('ᚱ').describe(), "<<rune ᚱ>>");
    }
}
