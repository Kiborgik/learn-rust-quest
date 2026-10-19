// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 3-1 · Hand-Me-Down Sword                         100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 THE THREE OWNERSHIP RULES (the heart of Rust)
//
//   1. Every value has exactly ONE owner (a variable).
//   2. Ownership can be MOVED to another variable or into a function.
//   3. When the owner goes out of scope, the value is dropped (freed).
//
//   No garbage collector and no manual `free`: the compiler inserts the
//   cleanup for you, at the exact point where the owner's scope ends.
//
// 📖 LEARN BY EXAMPLE
//
//     let a = String::from("Excalibur");  // a String lives on the HEAP
//     let b = a;                          // ownership MOVES from a to b
//     println!("{a}");                    // ✘ ERROR: borrow of moved value `a`
//     println!("{b}");                    // ✔
//
//     fn take(s: String) { }              // the parameter becomes the owner...
//     take(b);                            // ...so `b` is moved INTO the function
//     println!("{b}");                    // ✘ ERROR: `b` was moved
//
//     fn take_and_return(s: String) -> String { s }
//     let c = String::from("Axe");
//     let c = take_and_return(c);         // ownership goes in and comes back ✔
//
//   In Java/C#/Python, `b = a` makes two references to one object.
//   In C++, it would copy. In Rust it MOVES: `a` is no longer usable.
//   That's how Rust knows exactly who frees the memory, exactly once.
//
// 🎯 YOUR TASK
//
//   Make the program print:
//
//       Father's sword: Rusty Blade
//       The smith inspects Rusty Blade.
//       Ferris now wields Rusty Blade.
//
//   Don't use `.clone()` (that's the next quest). Use the variable
//   that actually owns the sword at each point.

fn inspect(sword: String) -> String {
    println!("The smith inspects {sword}.");
    sword // gives ownership back to the caller
}

fn main() {
    let sword = String::from("Rusty Blade");
    let heirloom = sword; // the sword is handed down to Ferris

    println!("Father's sword: {sword}");

    inspect(heirloom);

    println!("Ferris now wields {heirloom}.");
}
