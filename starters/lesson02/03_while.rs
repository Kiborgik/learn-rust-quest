// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 2-3 · Torchlight Countdown                       100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     let mut stamina = 3;
//     while stamina > 0 {          // checked BEFORE every iteration
//         println!("Running! ({stamina})");
//         stamina -= 1;
//     }
//     // prints: Running! (3) / Running! (2) / Running! (1)
//
//   • Same rules as `if`: no parentheses needed, condition must be `bool`.
//   • There is no `do { } while` and no `i++` / `i--` in Rust.
//     Use `i += 1` / `i -= 1`.
//   • Off-by-one errors are just as easy in Rust as anywhere else. 😉
//
// 🎯 YOUR TASK
//
//   Your torch has 5 seconds of light left. Make the program print:
//
//       5...
//       4...
//       3...
//       2...
//       1...
//       Darkness!

fn main() {
    let torch = 5;

    while torch >= 0 {
        println!("{torch}...");
        torch -= 1;
    }

    println!("Darkness!");
}
