// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 2-4 · Counting Crystals                          100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     for i in 0..3 { }            // 0, 1, 2       (end EXCLUSIVE)
//     for i in 0..=3 { }           // 0, 1, 2, 3    (end INCLUSIVE)
//     for i in (1..=5).rev() { }   // 5, 4, 3, 2, 1
//     for i in (0..10).step_by(3) { }  // 0, 3, 6, 9
//
//     let loot = [10, 20, 30];
//     for coin in loot {           // iterate an array directly
//         println!("{coin}");
//     }
//
//   • `for` is how you loop over anything iterable. There's no C-style
//     `for (i = 0; i < n; i++)`. Ranges replace it.
//   • `return` exits the whole function early, even from inside a loop.
//   • `print!` is `println!` without the line break at the end.
//
//   🆕 `todo!()` is a placeholder macro: the code compiles, but panics
//   ("not yet implemented") if it runs. Replace each one with real code.
//
// 🎯 YOUR TASK
//
//   1. Fix the off-by-one bug in `crystals_up_to`.
//   2. Implement `sum_of_evens` using `step_by`.
//   3. Implement `highest_multiple` using `.rev()` and an early `return`.
//   4. Implement `total_weight` by looping over the array.
//
//   This quest is checked by the TESTS at the bottom.

/// 1 + 2 + ... + n
fn crystals_up_to(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..n {
        total += i;
    }
    total
}

/// 2 + 4 + 6 + ... (all even numbers up to and including n)
fn sum_of_evens(n: u32) -> u32 {
    todo!()
}

/// The largest number <= n that is divisible by k. Return 0 if none.
fn highest_multiple(n: u32, k: u32) -> u32 {
    todo!()
}

/// Sum of all gem weights.
fn total_weight(gems: [u32; 4]) -> u32 {
    todo!()
}

fn main() {
    println!("Crystals: {}", crystals_up_to(10));
    println!("Even crystals: {}", sum_of_evens(10));
    println!("Highest multiple of 7 below 50: {}", highest_multiple(50, 7));
    println!("Gem weight: {}", total_weight([3, 1, 4, 1]));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crystals() {
        assert_eq!(crystals_up_to(10), 55);
        assert_eq!(crystals_up_to(1), 1);
    }

    #[test]
    fn evens() {
        assert_eq!(sum_of_evens(10), 30);
        assert_eq!(sum_of_evens(7), 12);
    }

    #[test]
    fn multiples() {
        assert_eq!(highest_multiple(50, 7), 49);
        assert_eq!(highest_multiple(20, 5), 20);
        assert_eq!(highest_multiple(3, 5), 0);
    }

    #[test]
    fn weight() {
        assert_eq!(total_weight([3, 1, 4, 1]), 9);
        assert_eq!(total_weight([0, 0, 0, 0]), 0);
    }
}
