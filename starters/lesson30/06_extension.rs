// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 30-6 · New Tricks for Old Types                  175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   You can't add methods to u32 or to Iterator directly. But you CAN
//   define your own trait and implement it for them (the orphan rule
//   allows it because the trait is yours). That's an EXTENSION TRAIT:
//
//     pub trait ShoutExt {
//         fn shout(&self) -> String;
//     }
//     impl ShoutExt for str {
//         fn shout(&self) -> String { self.to_uppercase() + "!" }
//     }
//     "hello".shout()   // "HELLO!" (with ShoutExt in scope)
//
//   With a blanket impl, it works for a whole family of types. This is
//   how `itertools` adds methods to every iterator:
//
//     pub trait CountWhere: Iterator + Sized {
//         fn count_where(self, f: impl Fn(&Self::Item) -> bool) -> usize {
//             self.filter(|x| f(x)).count()
//         }
//     }
//     impl<I: Iterator> CountWhere for I {}     // every iterator gets it
//
//   Name them `...Ext` by convention, and callers must `use` the trait to
//   see the methods.
//
//   ⚠️ On a bare number literal, `5.gold()` fails (E0689: ambiguous
//   numeric type): Rust picks the literal's type AFTER method lookup.
//   Write `5u32.gold()`, or call it on a typed variable.
//
// 🎯 YOUR TASK
//
//   1. Implement `GoldExt` for u32, so `5u32.gold()` is `Gold(5)`.
//   2. Give EVERY iterator of Gold a `total_gold()` method, with a
//      default body in the trait and a blanket impl.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Gold(pub u32);

impl fmt::Display for Gold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}g", self.0)
    }
}

pub trait GoldExt {
    fn gold(self) -> Gold;
}

pub trait TotalGold: Iterator<Item = Gold> + Sized {
    // fn total_gold(self) -> Gold { ... }
}

pub struct Monster {
    pub name: &'static str,
    pub loot: Gold,
}

fn main() {
    println!("{}", Gold(5));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_become_gold() {
        assert_eq!(5u32.gold(), Gold(5));
        let price: u32 = 120;
        assert_eq!(price.gold().to_string(), "120g");
    }

    #[test]
    fn iterators_add_up() {
        assert_eq!([1u32.gold(), 2u32.gold(), 3u32.gold()].into_iter().total_gold(), Gold(6));
        assert_eq!(std::iter::empty().total_gold(), Gold(0));
        let monsters = [Monster { name: "slime", loot: Gold(3) }, Monster { name: "dragon", loot: Gold(500) }];
        assert_eq!(monsters.iter().map(|m| m.loot).total_gold(), Gold(503));
        let big: Vec<&str> = monsters.iter().filter(|m| m.loot > 100u32.gold()).map(|m| m.name).collect();
        assert_eq!(big, ["dragon"]);
    }
}
