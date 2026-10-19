// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 7-1 · When All Is Lost                           125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Rust splits errors into two kinds:
//
//   • UNRECOVERABLE: a bug or a broken invariant → `panic!`
//     The thread unwinds and the program stops with a message.
//   • RECOVERABLE: an expected failure (bad input, missing file) →
//     `Result` (next quest).
//
//     panic!("the bridge collapsed at {x}");     // crash with a message
//     let v = some_option.unwrap();              // panics on None / Err
//     let v = some_option.expect("key missing"); // same, with YOUR message
//     unreachable!();  todo!();  assert!(cond);  // also panic
//
//   When to panic: tests, examples, prototypes, and "this can never
//   happen unless there's a bug" situations. Never for normal errors
//   like bad user input.
//
//   Testing that something panics:
//
//     #[test]
//     #[should_panic(expected = "collapsed")]   // the message must CONTAIN this
//     fn bridge_breaks() { cross_bridge(999); }
//
// 🎯 YOUR TASK
//
//   1. `cross_bridge` must panic with a message containing
//      "bridge collapsed" when the weight is over 100. Right now it
//      happily lets anyone through.
//   2. `secret_door` should use `.expect(...)` with a message that
//      contains "no secret door" instead of a bare `.unwrap()`.

fn cross_bridge(weight: u32) -> &'static str {
    "crossed"
}

fn find_door(room: u32) -> Option<u32> {
    if room == 3 { Some(42) } else { None }
}

fn secret_door(room: u32) -> u32 {
    find_door(room).unwrap()
}

fn main() {
    println!("{}", cross_bridge(50));
    println!("door code: {}", secret_door(3));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_heroes_cross() {
        assert_eq!(cross_bridge(80), "crossed");
        assert_eq!(cross_bridge(100), "crossed");
    }

    #[test]
    #[should_panic(expected = "bridge collapsed")]
    fn heavy_heroes_fall() {
        cross_bridge(101);
    }

    #[test]
    fn door_found() {
        assert_eq!(secret_door(3), 42);
    }

    #[test]
    #[should_panic(expected = "no secret door")]
    fn no_door() {
        secret_door(1);
    }
}
