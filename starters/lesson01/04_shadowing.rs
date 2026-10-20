// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 1-4 · Shadow of the Scroll                       100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     let x = 5;
//     let x = x * 2;          // a NEW `x` that "shadows" the old one → 10
//
//     let spaces = "   ";     // a string...
//     let spaces = spaces.len(); // ...shadowed by a number (3)!
//
//     let n = "42".parse::<i32>().unwrap();   // text → number, way #1
//     let n: i32 = "42".parse().unwrap();     // text → number, way #2
//
//   • Shadowing (`let` again with the same name) creates a brand-new
//     variable. It can even have a DIFFERENT TYPE than the old one.
//   • `mut` is different: it changes the SAME variable, and the type
//     can never change.
//   • A shadow inside a `{ }` block only lasts until that block's `}`.
//     After it, the outer variable is visible again.
//   • `.parse()` can turn text into many types (u8, i32, f64, ...), so
//     Rust needs you to say which one you want.
//   • `.trim()` removes surrounding whitespace, `.expect("msg")` crashes
//     with "msg" if parsing failed. (Proper error handling comes later!)
//
// 🎯 YOUR TASK
//
//   Make the program print:
//
//       Level up! You are now level 8

fn main() {
    // The messenger's scroll: your level, as TEXT.
    let level = " 7 ";

    // Shadow it with a NUMBER.
    let level = level.trim().parse().expect("the scroll is unreadable");

    // Shadow it again: level up!
    let level = level + 1;

    println!("Level up! You are now level {level}");
}
