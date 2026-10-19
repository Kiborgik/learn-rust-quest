// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 27-5 · Echoes                                    175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A macro can call itself. Each call peels off a piece of the input,
//   until a rule for "nothing left" ends it:
//
//     macro_rules! count {
//         () => { 0usize };                                       // the end
//         ($head:tt $($tail:tt)*) => { 1usize + count!($($tail)*) };
//     }
//     count!(a b c)   →   1 + 1 + 1 + 0   →   3
//
//   The result is plain code, so it even works in a `const`.
//
//   Internal rules: a private "helper" rule starts with a marker like
//   `@step` that callers won't type:
//
//     (@step $acc:expr;) => { $acc };
//     (@step $acc:expr; $x:expr $(, $rest:expr)*) => { name!(@step $acc + $x; $($rest),*) };
//
//   There's a recursion limit (128 by default), so this is for short
//   inputs. For bigger jobs, use repetition instead.
//
// 🎯 YOUR TASK
//
//   1. `count!` is off by one. Fix its end rule.
//   2. Write `sum!`: sum!() is 0, sum!(1, 2, 3) is 6. Recursion, please.

macro_rules! count {
    () => {
        1usize
    };
    ($head:tt $($tail:tt)*) => {
        1usize + count!($($tail)*)
    };
}

macro_rules! sum {
    ($($t:tt)*) => {
        0 // TODO
    };
}

const PARTY_SIZE: usize = count!(Ferris Lumi Corro);

fn main() {
    println!("party of {PARTY_SIZE}, loot {}", sum!(1, 2, 3));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting() {
        assert_eq!(count!(), 0);
        assert_eq!(count!(x), 1);
        assert_eq!(PARTY_SIZE, 3);
    }

    #[test]
    fn summing() {
        let empty: i32 = sum!();
        assert_eq!(empty, 0);
        assert_eq!(sum!(5), 5);
        assert_eq!(sum!(1, 2, 3), 6);
        assert_eq!(sum!(10, -4, 2 * 3), 12);
    }
}
