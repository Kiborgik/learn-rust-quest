// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 1-6 · Spells & Functions                         100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     fn add(a: i32, b: i32) -> i32 {   // parameters MUST have types,
//         a + b                         // return type goes after `->`
//     }
//
//     fn greet(name: &str) {            // no `->` = returns nothing: ()
//         println!("Hi, {name}!");
//     }
//
//   ⚠️  The most important rule in this lesson:
//
//     The LAST EXPRESSION of a function (with NO semicolon) is its
//     return value. Adding a `;` turns it into a statement that
//     returns nothing: `()`, called "unit".
//
//         fn five() -> i32 { 5 }     // ✔ returns 5
//         fn five() -> i32 { 5; }    // ✘ ERROR: expected i32, found ()
//
//   You can also leave early with `return value;`, but idiomatic Rust
//   prefers the expression style for the final value.
//
//   Any `{ }` block works the same way: its last expression is its value.
//
//         let v = { let a = 2; a * 3 };   // v = 6
//
// 🎯 YOUR TASK
//
//   1. Fix `damage`.
//   2. Fix `heal`.
//   3. Write a new function `gold_after_battle(gold, enemies)`:
//      every defeated enemy drops 15 gold. Use `u32` for everything.
//
//   This quest is checked by TESTS (bottom of the file). Read them: tests
//   are a great way to understand what code is supposed to do!

fn damage(attack: u32, defense: u32) -> u32 {
    attack - defense;
}

fn heal(hp, potion) -> u32 {
    hp + potion
}

// TODO: write `gold_after_battle` here

fn main() {
    println!("You deal {} damage!", damage(30, 12));
    println!("You heal to {} HP!", heal(40, 25));
    println!("Your purse: {} gold", gold_after_battle(10, 3));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
// `#[test]` marks a test function. `assert_eq!(left, right)` fails the test
// if the two values differ.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_spell() {
        assert_eq!(damage(30, 12), 18);
        assert_eq!(damage(10, 10), 0);
    }

    #[test]
    fn healing_spell() {
        assert_eq!(heal(40, 25), 65);
    }

    #[test]
    fn loot() {
        assert_eq!(gold_after_battle(10, 3), 55);
        assert_eq!(gold_after_battle(0, 0), 0);
        assert_eq!(gold_after_battle(100, 1), 115);
    }
}
