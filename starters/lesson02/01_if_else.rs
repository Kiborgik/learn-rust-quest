// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 2-1 · Fork in the Tunnel                         100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     let hp = 30;
//     if hp < 50 {
//         println!("Low health!");
//     } else if hp < 80 {
//         println!("Scratched.");
//     } else {
//         println!("Fresh as a crab.");
//     }
//
//   • No parentheses around the condition. Braces are ALWAYS required.
//   • The condition MUST be a `bool`. There is no "truthiness":
//         if 1 { }        // ✘ ERROR: expected `bool`, found integer
//         if hp != 0 { }  // ✔
//
//   Combine conditions with `&&` (and), `||` (or) and `!` (not):
//         if hp > 0 && hp < 20 { }     // both must be true
//         if hp == 0 || hp > 999 { }   // at least one is true
//
//   `if` is an EXPRESSION: it produces a value (like `?:` in C/Java/JS):
//
//     let label = if hp < 50 { "weak" } else { "strong" };
//
//   Both branches must produce the SAME type:
//     let x = if ok { 5 } else { "five" };   // ✘ ERROR: incompatible types
//
// 🎯 YOUR TASK
//
//   Make the program print:
//
//       The tunnel is lit.
//       You take the golden tunnel.
//       Ferris feels nervous.
//
//   (Hint for the 2nd line: the `else` branch could say "the dark tunnel".)

fn main() {
    let torches = 3;
    let gold = 120;
    let danger = 7;

    if torches {
        println!("The tunnel is lit.");
    }

    let path = if gold > 100 { "the golden tunnel" } else { 0 };
    println!("You take {path}.");

    let mood = if danger > 8 {
        "terrified"
    } else if danger > 5 {
        "nervous"
    } else {
        "calm"
    };
    println!("Ferris feels {mood}.");
}
