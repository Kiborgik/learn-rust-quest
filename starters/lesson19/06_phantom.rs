// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 19-6 · Ghost Types                               175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Sometimes a type parameter exists only to make the compiler check
//   something, with no data of that type at all. Rust insists every
//   parameter is used, so you add a zero-size marker:
//
//     use std::marker::PhantomData;
//
//     struct Gold;                       // "unit" types used as labels
//     struct Silver;
//
//     struct Money<C> {
//         amount: u32,
//         currency: PhantomData<C>,      // takes no memory, satisfies the compiler
//     }
//
//   Now `Money<Gold>` and `Money<Silver>` are DIFFERENT types. Adding them
//   is a compile error, and methods can exist for one but not the other:
//
//     impl Money<Gold> {
//         fn to_silver(self) -> Money<Silver> { ... }   // only on gold
//     }
//
//   This is how libraries model units, IDs (UserId vs OrderId), and
//   "typestate" (a Door<Locked> has no `open` method).
//
//   Without PhantomData:  struct Money<C> { amount: u32 }
//   ✘ E0392: type parameter `C` is never used
//
// 🎯 YOUR TASK
//
//   1. Make `Money<C>` compile.
//   2. Implement `to_silver` (1 gold = 100 silver), only for gold.

use std::marker::PhantomData;
use std::ops::Add;

struct Gold;
struct Silver;

#[derive(Debug, PartialEq)]
struct Money<C> {
    amount: u32,
}

impl<C> Money<C> {
    fn new(amount: u32) -> Self {
        Money { amount }
    }
}

impl<C> Add for Money<C> {
    type Output = Money<C>;
    fn add(self, rhs: Money<C>) -> Money<C> {
        Money::new(self.amount + rhs.amount)
    }
}

// TODO: impl Money<Gold> { fn to_silver(self) -> Money<Silver> }

fn main() {
    let purse: Money<Gold> = Money::new(3) + Money::new(2);
    println!("{} silver", purse.to_silver().amount);
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currencies() {
        let gold: Money<Gold> = Money::new(3) + Money::new(2);
        assert_eq!(gold.amount, 5);
        let silver: Money<Silver> = gold.to_silver() + Money::new(7);
        assert_eq!(silver.amount, 507);
        assert_eq!(std::mem::size_of::<Money<Gold>>(), std::mem::size_of::<u32>());
    }
}
