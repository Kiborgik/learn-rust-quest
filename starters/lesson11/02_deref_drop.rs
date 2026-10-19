// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 11-2 · Gears That Turn                           150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Two traits make a type behave like a smart pointer:
//
//   DEREF: "treat me like the thing I wrap"
//
//     use std::ops::Deref;
//     struct Wrapper(String);
//     impl Deref for Wrapper {
//         type Target = String;
//         fn deref(&self) -> &String { &self.0 }
//     }
//     let w = Wrapper(String::from("hi"));
//     w.len();                     // String's method, through Deref
//     fn shout(s: &str) {}
//     shout(&w);                   // &Wrapper → &String → &str (deref coercion)
//
//   DROP: "run this code when I'm destroyed" (a destructor / RAII):
//
//     impl Drop for Wrapper {
//         fn drop(&mut self) { println!("bye {}", self.0); }
//     }
//
//   Drop order: locals are dropped in REVERSE order of declaration, at
//   the end of their scope. `drop(x)` (from the prelude) drops early.
//   You can never call `x.drop()` yourself.
//
// 🎯 YOUR TASK
//
//   Expected output:
//
//       Gear Alpha starts turning
//       Gear Bronze starts turning
//       Gear Copper starts turning
//       Gear Bronze is 6 letters long
//       Gear Bronze stops
//       Main loop done
//       Gear Copper stops
//       Gear Alpha stops
//
//   1. Implement `Drop` for `Gear`: print "Gear {name} stops".
//   2. Implement `Deref` for `Gear` with Target = String (the name), so
//      `print_len("Bronze", &b)` works.
//   3. In `main`, stop gear Bronze early with `drop`.
//
//   (Keep the `_a` / `_c` names: a variable named just `_` would be
//   dropped IMMEDIATELY, which is a classic gotcha!)

use std::ops::Deref;

struct Gear {
    name: String,
}

impl Gear {
    fn new(name: &str) -> Gear {
        println!("Gear {name} starts turning");
        Gear { name: name.to_string() }
    }
}

fn print_len(label: &str, s: &str) {
    println!("Gear {label} is {} letters long", s.len());
}

fn main() {
    let _a = Gear::new("Alpha");
    let b = Gear::new("Bronze");
    let _c = Gear::new("Copper");
    print_len("Bronze", &b);
    println!("Main loop done");
}
