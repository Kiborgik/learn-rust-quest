// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 5-2 · Shapes with Substance                      125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Each variant can carry its OWN data, in tuple or struct form:
//
//     enum Message {
//         Quit,                        // no data
//         Move { x: i32, y: i32 },     // struct-like
//         Write(String),               // tuple-like
//         Color(u8, u8, u8),
//     }
//
//     let m = Message::Move { x: 3, y: 4 };
//     let w = Message::Write(String::from("hi"));
//
//     impl Message {                   // enums can have methods too
//         fn weight(&self) -> u32 {
//             match self {
//                 Message::Quit => 0,
//                 Message::Move { x, y } => (x.abs() + y.abs()) as u32,
//                 Message::Write(text) => text.len() as u32,
//                 Message::Color(..) => 3,       // `..` ignores all fields
//             }
//         }
//     }
//
//   These are "sum types" (tagged unions). Where Java or C# would use a
//   class hierarchy with `instanceof` checks, Rust uses one enum plus
//   an exhaustive `match`.
//
//   `as u32` converts between number types explicitly.
//
// 🎯 YOUR TASK
//
//   1. Fix `starter_spells` (one variant is built the wrong way).
//   2. Implement `mana_cost`:
//        Fireball(damage)   → damage / 2
//        Heal { amount }    → amount + 5
//        Shield             → 10
//        Teleport { x, y }  → |x| + |y|

enum Spell {
    Fireball(u32),
    Heal { amount: u32 },
    Shield,
    Teleport { x: i32, y: i32 },
}

impl Spell {
    fn mana_cost(&self) -> u32 {
        todo!()
    }
}

fn starter_spells() -> [Spell; 3] {
    [Spell::Fireball(40), Spell::Heal(20), Spell::Shield]
}

fn main() {
    let mut total = 0;
    for s in starter_spells() {
        total += s.mana_cost();
    }
    println!("Starter spells cost {total} mana");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn costs() {
        assert_eq!(Spell::Fireball(40).mana_cost(), 20);
        assert_eq!(Spell::Heal { amount: 20 }.mana_cost(), 25);
        assert_eq!(Spell::Shield.mana_cost(), 10);
        assert_eq!(Spell::Teleport { x: 3, y: -4 }.mana_cost(), 7);
    }

    #[test]
    fn starters() {
        let mut total = 0;
        for s in starter_spells() {
            total += s.mana_cost();
        }
        assert_eq!(total, 55);
    }
}
