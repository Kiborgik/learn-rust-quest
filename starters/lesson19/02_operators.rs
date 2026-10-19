// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 19-2 · Coin Arithmetic                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Operator traits have a type parameter for the RIGHT-hand side, with a
//   DEFAULT of Self:
//
//     trait Add<Rhs = Self> {        // Rhs defaults to Self
//         type Output;
//         fn add(self, rhs: Rhs) -> Self::Output;
//     }
//
//   So one type can support several right-hand sides:
//
//     impl Add for Gold { ... }           // Gold + Gold     (Rhs = Self)
//     impl Add<u32> for Gold { ... }      // Gold + 5
//     impl Mul<u32> for Gold { ... }      // Gold * 3
//
//   Compound assignment is a separate trait: `AddAssign` gives `+=`.
//
//     impl AddAssign<u32> for Gold {
//         fn add_assign(&mut self, rhs: u32) { self.0 += rhs; }
//     }
//
//   `Output` is an associated type, because `Gold + u32` has exactly one
//   result type.
//
// 🎯 YOUR TASK
//
//   `Gold + Gold` works already. Make these work too:
//     Gold + u32,   Gold * u32,   gold += u32

use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Gold(u32);

impl Add for Gold {
    type Output = Gold;
    fn add(self, rhs: Gold) -> Gold {
        Gold(self.0 + rhs.0)
    }
}

fn main() {
    let mut purse = Gold(10) + Gold(5);
    purse += 3;
    println!("{:?}", purse * 2);
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic() {
        assert_eq!(Gold(10) + Gold(5), Gold(15));
        assert_eq!(Gold(10) + 5, Gold(15));
        assert_eq!(Gold(7) * 3, Gold(21));
        let mut g = Gold(1);
        g += 9;
        assert_eq!(g, Gold(10));
    }
}
