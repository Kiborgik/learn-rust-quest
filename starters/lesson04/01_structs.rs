// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 4-1 · Guild Registration                         100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     struct Monster {            // define a type with named fields
//         name: String,
//         hp: u32,
//         angry: bool,
//     }
//
//     let m = Monster {           // instantiate: EVERY field is required
//         name: String::from("Slime"),
//         hp: 12,
//         angry: false,
//     };
//     println!("{} has {} hp", m.name, m.hp);   // dot access
//
//     fn spawn(name: String, hp: u32) -> Monster {
//         Monster { name, hp, angry: true }      // field init shorthand:
//     }                                          // `name` = `name: name`
//
//   • There are no constructors, no `null` fields and no default values
//     unless you write them.
//   • Mutability belongs to the BINDING, not to individual fields:
//     `let mut m = ...` makes ALL fields mutable.
//
// 🎯 YOUR TASK
//
//   Make the program print:
//
//       Ferris the Crabsmith (level 3)
//       Active: true

struct Adventurer {
    name: String,
    class: String,
    level: u32,
    active: bool,
}

fn register(name: String, class: String) -> Adventurer {
    Adventurer { name, class, level: 1 }
}

fn main() {
    let hero = register(String::from("Ferris"), String::from("Crabsmith"));
    hero.level += 2; // passed two trials!

    println!("{} the {} (level {})", hero.name, hero.class, hero.level);
    println!("Active: {}", hero.active);
}
