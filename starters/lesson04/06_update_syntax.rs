// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 4-6 · Copycat Armor                              100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   STRUCT UPDATE SYNTAX: "like that one, but with these changes":
//
//     let base = Boots { name: String::from("Leather"), speed: 1, armor: 2 };
//     let fast = Boots { speed: 5, ..base };   // the other fields come from `base`
//
//   ⚠️ `..base` MOVES (or copies) the remaining fields out of `base`:
//     • Copy fields (numbers, bools) are copied: no problem.
//     • A String field is MOVED, so `base` is now "partially moved"
//       and can't be used as a whole anymore.
//     • `..base` needs an owned value, not a reference. With a
//       `&Boots`, use `..base.clone()`.
//
//   DESTRUCTURING pulls fields out into variables:
//
//     let Boots { name, speed, .. } = fast;      // `..` ignores the rest
//     let Boots { speed: s, .. } = &other;       // rename a field to `s`
//
// 🎯 YOUR TASK
//
//   1. Fix `enchant` (it receives a reference).
//   2. Fix `lighten` so BOTH armors can be returned.
//   3. Implement `summary` using destructuring: "Iron Plate: 10".

#[derive(Debug, Clone, PartialEq)]
struct Armor {
    name: String,
    defense: u32,
    weight: u32,
    enchanted: bool,
}

fn base_armor() -> Armor {
    Armor {
        name: String::from("Iron Plate"),
        defense: 10,
        weight: 20,
        enchanted: false,
    }
}

fn enchant(base: &Armor) -> Armor {
    Armor {
        enchanted: true,
        defense: base.defense + 5,
        ..base
    }
}

fn lighten() -> (Armor, Armor) {
    let heavy = base_armor();
    let light = Armor { weight: 5, ..heavy };
    (heavy, light)
}

fn summary(armor: &Armor) -> String {
    todo!()
}

fn main() {
    let base = base_armor();
    println!("{:?}", enchant(&base));
    println!("{:?}", lighten());
    println!("{}", summary(&base));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enchanting() {
        let base = base_armor();
        let magic = enchant(&base);
        assert_eq!(magic.defense, 15);
        assert!(magic.enchanted);
        assert_eq!(magic.name, "Iron Plate");
        assert_eq!(magic.weight, 20);
        assert_eq!(base, base_armor()); // original untouched
    }

    #[test]
    fn lightening() {
        let (heavy, light) = lighten();
        assert_eq!(heavy.weight, 20);
        assert_eq!(light.weight, 5);
        assert_eq!(heavy.name, light.name);
    }

    #[test]
    fn summarize() {
        assert_eq!(summary(&base_armor()), "Iron Plate: 10");
    }
}
