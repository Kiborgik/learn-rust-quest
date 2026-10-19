// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 9-1 · Pocket Spells                              150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A CLOSURE is an anonymous function that can capture variables from
//   its surroundings (like JS arrow functions or C# lambdas):
//
//     let add_one = |x| x + 1;                 // types inferred
//     let add = |a: i32, b: i32| -> i32 { a + b };  // fully written out
//     add_one(5);                              // 6
//
//     let bonus = 10;
//     let with_bonus = |x| x + bonus;          // CAPTURES `bonus`
//     with_bonus(5);                           // 15
//
//   Taking a closure as a parameter, via a trait bound:
//
//     fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
//         f(f(x))
//     }
//     apply_twice(|x| x * 3, 2);               // 18
//     apply_twice(add_one, 2);                 // 4
//
//   Normal functions can be passed the same way: `apply_twice(double, 2)`
//   where `fn double(x: i32) -> i32`.
//
//   ⚠️ A closure's parameter types are fixed by its FIRST use:
//     let id = |x| x;
//     id(5);  id("five");        // ✘ ERROR: expected integer, found &str
//
// 🎯 YOUR TASK
//
//   1. `apply` should accept any closure taking and returning a u32.
//   2. Fix `power_up`: its closure must capture and use `multiplier`.
//   3. Implement `apply_n`: apply `f` to `x`, `n` times.

fn apply(f, x: u32) -> u32 {
    f(x)
}

fn power_up(base: u32, multiplier: u32) -> u32 {
    let boost = |x| x * 1;
    apply(boost, base)
}

fn apply_n(f: impl Fn(u32) -> u32, x: u32, n: u32) -> u32 {
    todo!()
}

fn triple(x: u32) -> u32 {
    x * 3
}

fn main() {
    println!("{}", apply(|x| x + 1, 41));
    println!("{}", power_up(5, 4));
    println!("{}", apply_n(triple, 1, 3));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applying() {
        assert_eq!(apply(|x| x + 1, 41), 42);
        assert_eq!(apply(triple, 5), 15);
    }

    #[test]
    fn powering() {
        assert_eq!(power_up(5, 4), 20);
        assert_eq!(power_up(7, 0), 0);
    }

    #[test]
    fn repeated() {
        assert_eq!(apply_n(triple, 1, 3), 27);
        assert_eq!(apply_n(|x| x + 2, 0, 5), 10);
        assert_eq!(apply_n(triple, 9, 0), 9);
    }
}
