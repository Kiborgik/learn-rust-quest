// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 4-4 · The Constructor's Oath                     100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Functions in an `impl` WITHOUT `self` are "associated functions",
//   like static methods. Call them with `Type::function()`.
//   By convention, the constructor is called `new`:
//
//     struct Chest { coins: u32, locked: bool }
//
//     impl Chest {
//         const MAX_COINS: u32 = 999;          // associated constant
//
//         fn new(coins: u32) -> Self {         // `Self` = "this type"
//             Self { coins, locked: true }
//         }
//
//         fn fill(&mut self) {
//             self.coins = Self::MAX_COINS;    // constants need `Self::`
//         }
//     }
//
//     let c = Chest::new(10);                  // `::` for associated fns
//
//   • `new` isn't special: it's just a naming convention.
//   • A type can have several `impl` blocks.
//
// 🎯 YOUR TASK
//
//   1. Fix `new` (look at the field types!).
//   2. Fix `recruit`, which can't find the constant.
//   3. Implement `founders()`: a guild named "Founders" with 3 members
//      and the usual starting gold. Reuse `Self::new`!

struct Guild {
    name: String,
    members: u32,
    gold: u32,
}

impl Guild {
    const MAX_MEMBERS: u32 = 50;

    /// A new guild has 1 member (its founder) and 100 gold.
    fn new(name: &str) -> Guild {
        Guild { name, members: 1, gold: 100 }
    }

    /// Adds a member unless the guild is full. Returns whether it worked.
    fn recruit(&mut self) -> bool {
        if self.members >= MAX_MEMBERS {
            return false;
        }
        self.members += 1;
        true
    }
}

impl Guild {
    fn founders() -> Self {
        todo!()
    }
}

fn main() {
    let mut g = Guild::new("Crabs");
    g.recruit();
    println!("{} has {} members and {} gold", g.name, g.members, g.gold);
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating() {
        let g = Guild::new("Crabs");
        assert_eq!(g.name, "Crabs");
        assert_eq!(g.members, 1);
        assert_eq!(g.gold, 100);
    }

    #[test]
    fn recruiting_caps_out() {
        let mut g = Guild::new("Crabs");
        assert!(g.recruit());
        for _ in 0..100 {
            g.recruit();
        }
        assert_eq!(g.members, Guild::MAX_MEMBERS);
        assert!(!g.recruit());
    }

    #[test]
    fn the_founders() {
        let f = Guild::founders();
        assert_eq!(f.name, "Founders");
        assert_eq!(f.members, 3);
        assert_eq!(f.gold, 100);
    }
}
