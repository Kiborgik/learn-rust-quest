// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 1-3 · The Ever-Changing Purse                    100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     let a = 5;
//     a = 6;              // ✘ ERROR: cannot assign twice to immutable variable
//
//     let mut b = 5;      // `mut` = "this value is allowed to change"
//     b = 6;              // ✔ fine
//     b += 1;             // ✔ shorthand for b = b + 1   (also -=, *=, /=)
//
//   • Variables in Rust are IMMUTABLE by default.
//   • Why? When you read code, you instantly know which values can change
//     (the ones marked `mut`) and which can't. Fewer surprises, fewer bugs.
//
// 🎯 YOUR TASK
//
//   Make the program print:
//
//       Starting gold: 10
//       Gold now: 30
//
//   Don't change the numbers. Let the purse change instead!

fn main() {
    let gold = 10;
    println!("Starting gold: {gold}");

    gold = gold + 25; // found a treasure chest!
    gold -= 5; // paid the bridge troll

    println!("Gold now: {gold}");
}
