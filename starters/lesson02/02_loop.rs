// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 2-2 · The Endless Echo                           100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     loop {                       // an infinite loop (like `while (true)`)
//         println!("again!");
//         break;                   // ...until you `break` out of it
//     }
//
//   `loop` is ALSO an expression. Whatever you hand to `break` becomes
//   the value of the whole loop:
//
//     let mut tries = 0;
//     let found = loop {
//         tries += 1;
//         if tries == 4 {
//             break tries * 10;    // the loop evaluates to 40
//         }
//     };
//
//   A plain `break;` gives the value `()` ("unit" = nothing), which can't
//   be printed with `{}`.
//
// 🎯 YOUR TASK
//
//   Ferris shouts into a cave; each echo doubles in volume. Stop as soon
//   as it gets above 100 and make the loop RETURN the final volume.
//
//   Expected output:
//
//       After 7 shouts the echo reached 128.

fn main() {
    let mut echo = 1;
    let mut shouts = 0;

    let loudest = loop {
        echo *= 2;
        shouts += 1;
        if echo > 100 {
            break;
        }
    };

    println!("After {shouts} shouts the echo reached {loudest}.");
}
