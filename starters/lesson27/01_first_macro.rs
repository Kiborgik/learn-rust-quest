// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 27-1 · First Incantation                         175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `macro_rules!` writes code for you. A macro is a list of rules: a
//   PATTERN to match, and the CODE to put in its place.
//
//     macro_rules! greet {
//         () => { println!("hello!") };                       // greet!()
//         ($name:expr) => { println!("hello, {}!", $name) };  // greet!("Ferris")
//     }
//
//   `$name:expr` captures any expression. In the output, `$name` pastes
//   it back in. Macros run at compile time, before type checking, so they
//   can take any number of arguments (println!) and work with any type.
//
//   Two rules that surprise people:
//
//   1. ORDER. A macro_rules! macro exists only BELOW its definition in the
//      file. Define it first, use it after.
//
//   2. PASTING. The argument is pasted as code, not as a value. If it
//      appears twice, it RUNS twice:
//
//        macro_rules! twice { ($x:expr) => { $x + $x } }
//        twice!(roll_dice())     // calls roll_dice() TWICE
//
//      Fix: evaluate once into a local, inside a block:
//        { let v = $x; v + v }   // note the double braces in the rule: {{ ... }}
//
//   (Unlike C macros, an expr stays one unit: twice!(1 + 2) is 6, not 5.)
//
// 🎯 YOUR TASK
//
//   1. It doesn't compile: `square!` is used before it's defined.
//   2. `square!(roll())` must roll the dice only once.

fn main() {
    let n = square!(7);
    println!("7 squared is {n}");
}

macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    #[test]
    fn squares() {
        assert_eq!(square!(7), 49);
        assert_eq!(square!(2 + 3), 25);
        assert_eq!(square!(1.5), 2.25);
    }

    #[test]
    fn rolls_once() {
        let mut rolls = 0;
        let mut roll = || {
            rolls += 1;
            4
        };
        assert_eq!(square!(roll()), 16);
        assert_eq!(rolls, 1, "the dice were rolled {rolls} times");
    }
}
