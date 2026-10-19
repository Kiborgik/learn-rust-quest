// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Archmage's Exam                            300 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Archmage's final exam: build a spell system from scratch using
//   everything from this lesson.
//
//   Given: the `Target` struct below. Write the rest.
//
//   ⚔️  HEAD 1: trait `Spell`:
//        fn name(&self) -> String;
//        fn mana_cost(&self) -> u32;
//        fn cast(&self, target: &mut Target);
//        fn describe(&self) -> String   ← DEFAULT: "{name} ({cost} mana)"
//
//   ⚔️  HEAD 2: three spells:
//        Fireball { power: u32 }   name "Fireball", cost power / 2,
//                                  cast: hp -= power (never below 0)
//        Heal { amount: u32 }      name "Heal", cost 10,
//                                  cast: hp += amount (max target.max_hp)
//        Freeze                    name "Freeze", cost 25,
//                                  cast: frozen = true,
//                                  and OVERRIDES describe: "Freeze (25 mana, stuns)"
//
//   ⚔️  HEAD 3: generic `fn cheapest<'a, S: Spell>(spells: &'a [S]) -> Option<&'a S>`
//        Returns the spell with the lowest mana cost (first on a tie).
//        (The `'a` is a lifetime, lesson 10. Copy the signature as is.)
//
//   ⚔️  HEAD 4: `struct Spellbook { spells: Vec<Box<dyn Spell>> }` with
//        `new()`, `add(&mut self, spell: Box<dyn Spell>)`, `total_cost(&self) -> u32`,
//        `cast_all(&self, target: &mut Target)` (casts in order)
//
//   ⚔️  HEAD 5: `impl fmt::Display for Spellbook`:
//        one spell `describe()` per line, joined with "\n"
//
//   Your code goes here ↓

use std::fmt;

#[derive(Debug, PartialEq)]
struct Target {
    hp: u32,
    max_hp: u32,
    frozen: bool,
}

impl Target {
    fn new(hp: u32) -> Self {
        Target { hp, max_hp: hp, frozen: false }
    }
}



fn main() {
    println!("The Archmage waits behind a desk of floating books. Run the tests!");
}

// ─── The Archmage's tests: don't change these! ───────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_1_describe() {
        assert_eq!(Fireball { power: 40 }.describe(), "Fireball (20 mana)");
        assert_eq!(Heal { amount: 5 }.describe(), "Heal (10 mana)");
    }

    #[test]
    fn head_2_casting() {
        let mut t = Target::new(100);
        Fireball { power: 30 }.cast(&mut t);
        assert_eq!(t.hp, 70);
        Heal { amount: 50 }.cast(&mut t);
        assert_eq!(t.hp, 100);
        Fireball { power: 500 }.cast(&mut t);
        assert_eq!(t.hp, 0);
        Freeze.cast(&mut t);
        assert!(t.frozen);
        assert_eq!(Freeze.describe(), "Freeze (25 mana, stuns)");
    }

    #[test]
    fn head_3_cheapest() {
        let balls = [Fireball { power: 40 }, Fireball { power: 10 }, Fireball { power: 11 }];
        assert_eq!(cheapest(&balls).unwrap().mana_cost(), 5);
        let none: [Heal; 0] = [];
        assert!(cheapest(&none).is_none());
    }

    #[test]
    fn head_4_spellbook() {
        let mut book = Spellbook::new();
        book.add(Box::new(Fireball { power: 40 }));
        book.add(Box::new(Freeze));
        book.add(Box::new(Heal { amount: 15 }));
        assert_eq!(book.total_cost(), 55);
        let mut t = Target::new(100);
        book.cast_all(&mut t);
        assert_eq!(t, Target { hp: 75, max_hp: 100, frozen: true });
    }

    #[test]
    fn head_5_display() {
        let mut book = Spellbook::new();
        book.add(Box::new(Heal { amount: 1 }));
        book.add(Box::new(Freeze));
        assert_eq!(book.to_string(), "Heal (10 mana)\nFreeze (25 mana, stuns)");
    }
}
