// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 5-4 · The Exhaustive Oracle                      125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Patterns can reach deep inside nested data:
//
//     enum Metal { Gold, Silver }
//     enum Loot { Coins(Metal, u32), Gem { carats: u32 }, Nothing }
//
//     fn value(l: &Loot) -> u32 {
//         match l {
//             Loot::Coins(Metal::Gold, n) => n * 10,   // nested pattern
//             Loot::Coins(_, n) => *n,                  // any other metal
//             Loot::Gem { carats } => carats * 50,
//             Loot::Nothing => 0,
//         }
//     }
//
//   • Matching on a REFERENCE (`&Loot`) gives you references to the
//     fields (`n: &u32`). Arithmetic works on them directly; use `*n`
//     when you need the plain value.
//   • Arms are tried TOP TO BOTTOM. A general arm placed before a
//     specific one "swallows" it. The compiler warns about this:
//     "unreachable pattern". Read your warnings!
//   • `{:?}` on an enum that derives Debug prints the variant name, e.g.
//     format!("{:?}", Metal::Gold) == "Gold"
//
// 🎯 YOUR TASK
//
//   1. Make `threat` compile (Ghost has threat 0), then fix its bug:
//      Fire elementals are 3× as dangerous, but the tests disagree...
//   2. Implement `name`:
//        Slime → "Slime", Elemental(Fire, _) → "Fire Elemental"
//        (and likewise for Water/Earth), Dragon → its name, Ghost → "???"

#[derive(Debug, PartialEq)]
enum Element {
    Fire,
    Water,
    Earth,
}

#[derive(Debug)]
enum Creature {
    Slime { hp: u32 },
    Elemental(Element, u32),
    Dragon { name: String, heads: u32 },
    Ghost,
}

fn threat(c: &Creature) -> u32 {
    match c {
        Creature::Slime { hp } => hp / 10,
        Creature::Elemental(_, power) => *power,
        Creature::Elemental(Element::Fire, power) => power * 3,
        Creature::Dragon { heads, .. } => heads * 100,
    }
}

fn name(c: &Creature) -> String {
    todo!()
}

fn main() {
    let foes = [
        Creature::Slime { hp: 55 },
        Creature::Elemental(Element::Fire, 7),
        Creature::Dragon { name: String::from("Smaug"), heads: 3 },
        Creature::Ghost,
    ];
    for f in &foes {
        println!("{}: threat {}", name(f), threat(f));
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn smaug() -> Creature {
        Creature::Dragon { name: String::from("Smaug"), heads: 3 }
    }

    #[test]
    fn threats() {
        assert_eq!(threat(&Creature::Slime { hp: 55 }), 5);
        assert_eq!(threat(&Creature::Elemental(Element::Fire, 7)), 21);
        assert_eq!(threat(&Creature::Elemental(Element::Water, 7)), 7);
        assert_eq!(threat(&smaug()), 300);
        assert_eq!(threat(&Creature::Ghost), 0);
    }

    #[test]
    fn names() {
        assert_eq!(name(&Creature::Slime { hp: 1 }), "Slime");
        assert_eq!(name(&Creature::Elemental(Element::Fire, 1)), "Fire Elemental");
        assert_eq!(name(&Creature::Elemental(Element::Earth, 1)), "Earth Elemental");
        assert_eq!(name(&smaug()), "Smaug");
        assert_eq!(name(&Creature::Ghost), "???");
    }
}
