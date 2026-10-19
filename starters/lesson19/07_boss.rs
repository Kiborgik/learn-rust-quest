// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Sphinx of Forms                           400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Sphinx asks riddles in types. Answer with traits.
//
//   The structs are given. Write the traits and impls.
//
//   ⚔️  HEAD 1: trait `Monster: fmt::Display` with
//        • an associated type `Loot`
//        • an associated const `DANGER: u32`
//        • `fn drop_loot(&self) -> Self::Loot`
//
//   ⚔️  HEAD 2: Display + Monster for both monsters:
//        Slime   shows "slime ({hp} hp)", DANGER 1, Loot = u32 (hp / 10 gold)
//        Dragon  shows "{name} the dragon", DANGER 50,
//                Loot = Hoard: Hoard(vec![format!("{name}'s scale"), "gold pile".into()])
//
//   ⚔️  HEAD 3: trait `Threat` with `fn threat(&self) -> String`, and a
//        BLANKET impl for every Monster: "{monster}: danger {DANGER}"
//
//   ⚔️  HEAD 4: generic `fn total_danger<M: Monster>(ms: &[M]) -> u32`
//        (DANGER × number of monsters)
//
//   ⚔️  HEAD 5: `Hoard(pub Vec<String>)` is a newtype:
//        • Display: "hoard of 2: a, b" (or "empty hoard")
//        • `Add`: hoard + hoard joins the item lists

use std::fmt;
use std::ops::Add;

pub struct Slime {
    pub hp: u32,
}

pub struct Dragon {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Hoard(pub Vec<String>);

// Your traits and impls go here ↓



fn main() {
    println!("The Sphinx smiles. Run the tests to answer its riddles.");
}

// ─── The Sphinx's tests: don't change these! ─────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn smaug() -> Dragon {
        Dragon { name: "Smaug".into() }
    }

    #[test]
    fn head_1_and_2_monsters() {
        assert_eq!(Slime { hp: 55 }.to_string(), "slime (55 hp)");
        assert_eq!(Slime { hp: 55 }.drop_loot(), 5);
        assert_eq!(smaug().to_string(), "Smaug the dragon");
        assert_eq!(smaug().drop_loot(), Hoard(vec!["Smaug's scale".into(), "gold pile".into()]));
        assert_eq!(<Dragon as Monster>::DANGER, 50);
        assert_eq!(Slime::DANGER, 1);
    }

    #[test]
    fn head_3_threat() {
        assert_eq!(Slime { hp: 3 }.threat(), "slime (3 hp): danger 1");
        assert_eq!(smaug().threat(), "Smaug the dragon: danger 50");
    }

    #[test]
    fn head_4_total() {
        let slimes = [Slime { hp: 1 }, Slime { hp: 2 }, Slime { hp: 3 }];
        assert_eq!(total_danger(&slimes), 3);
        assert_eq!(total_danger(&[smaug(), smaug()]), 100);
        assert_eq!(total_danger::<Slime>(&[]), 0);
    }

    #[test]
    fn head_5_hoard() {
        let a = Hoard(vec!["gem".into()]);
        let b = Hoard(vec!["crown".into(), "sword".into()]);
        assert_eq!((a + b).to_string(), "hoard of 3: gem, crown, sword");
        assert_eq!(Hoard(vec![]).to_string(), "empty hoard");
    }
}
