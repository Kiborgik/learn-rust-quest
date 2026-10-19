// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 1-2 · Name Your Hero                             100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     let lives = 3;                 // `let` creates a variable
//     let name = "Ferris";           // type is inferred: &str (text)
//     let speed: i32 = 7;            // ...or you can write it explicitly
//
//     println!("{name} has {lives} lives");   // variables can go straight
//                                             // inside the {} placeholder!
//
//     let gems;                      // declared, but no value yet...
//     gems = 12;                     // ...assigned later (exactly once)
//     println!("{gems}");            // ✔ fine, it has a value now
//
//   • Every variable is introduced with `let`. There is no `var` or `int x`.
//   • Rust refuses to let you read a variable that has no value yet.
//     No "undefined", no garbage memory. Ever.
//
// 🎯 YOUR TASK
//
//   Make the program print:
//
//       Ferris enters the dungeon with 100 HP

fn main() {
    hero = "Ferris";
    let health;

    println!("{hero} enters the dungeon with {health} HP");
}
