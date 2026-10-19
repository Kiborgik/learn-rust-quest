// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 9-2 · The Three Vows                             150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   How a closure uses what it captured decides which trait it gets:
//
//   | trait    | the closure...                  | can be called |
//   |----------|---------------------------------|---------------|
//   | Fn       | only READS captured values      | many times    |
//   | FnMut    | MODIFIES captured values        | many times    |
//   | FnOnce   | CONSUMES (moves out) a capture  | once          |
//
//     let name = String::from("Ferris");
//     let greet = || println!("hi {name}");        // Fn
//
//     let mut count = 0;
//     let mut tick = || count += 1;                // FnMut (binding needs `mut`!)
//     tick(); tick();                              // count == 2
//
//     let gift = String::from("sword");
//     let give = || gift;                          // FnOnce: moves `gift` out
//     give();  // give();  ✘ second call: value already moved
//
//   `move` makes the closure take OWNERSHIP of everything it captures.
//   You need it when the closure outlives the current function:
//
//     fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
//         move |x| x + n         // without `move`, it would borrow a local
//     }
//
//   Accepting closures:  `impl Fn(..)` ⊂ `impl FnMut(..)` ⊂ `impl FnOnce(..)`
//   (an Fn closure can go anywhere an FnMut or FnOnce is expected)
//
// 🎯 YOUR TASK
//
//   1. Fix `count_hits` (the closure modifies a captured variable).
//   2. Fix `make_multiplier`, which returns a closure.
//   3. Fix `run_twice`'s bound so the test compiles.

fn count_hits(rolls: &[u32]) -> u32 {
    let mut hits = 0;
    let record = |roll: u32| {
        if roll >= 10 {
            hits += 1;
        }
    };
    for &r in rolls {
        record(r);
    }
    hits
}

fn make_multiplier(factor: u32) -> impl Fn(u32) -> u32 {
    |x| x * factor
}

fn run_twice(f: impl Fn()) {
    f();
    f();
}

fn main() {
    println!("{}", count_hits(&[3, 12, 15, 9]));
    let triple = make_multiplier(3);
    println!("{}", triple(7));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hits() {
        assert_eq!(count_hits(&[3, 12, 15, 9]), 2);
        assert_eq!(count_hits(&[]), 0);
    }

    #[test]
    fn multipliers() {
        let double = make_multiplier(2);
        let ten = make_multiplier(10);
        assert_eq!(double(21), 42);
        assert_eq!(ten(3), 30);
    }

    #[test]
    fn twice() {
        let mut log = Vec::new();
        run_twice(|| log.push("boom"));
        assert_eq!(log, vec!["boom", "boom"]);
    }
}
