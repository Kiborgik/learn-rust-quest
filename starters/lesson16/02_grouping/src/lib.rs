// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 16-2 · Sorting the Spoils                        175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     use itertools::Itertools;
//
//     ["a", "b", "a"].iter().counts()          // HashMap {"a": 2, "b": 1}
//
//     // chunk_by groups RUNS of equal keys (only neighbours!):
//     let rolls = [6, 6, 2, 6];
//     for (value, run) in &rolls.iter().chunk_by(|r| **r) {
//         println!("{value} x{}", run.count());  // 6 x2, 2 x1, 6 x1
//     }
//
//     // tuple_windows: overlapping pairs (or triples...)
//     [1, 4, 9].iter().tuple_windows::<(_, _)>()   // (1, 4), (4, 9)
//
//     // tuples: non-overlapping chunks
//     [1, 2, 3, 4].iter().tuples::<(_, _)>()       // (1, 2), (3, 4)
//
//   Note the `&` in `for ... in &rolls.iter().chunk_by(..)`: chunk_by
//   returns a value you iterate by reference.
//
// 🎯 YOUR TASK
//
//   Implement the three functions with itertools.

use itertools::Itertools;
use std::collections::HashMap;

/// How many times each monster was slain.
pub fn kill_counts<'a>(log: &[&'a str]) -> HashMap<&'a str, usize> {
    todo!()
}

/// The longest run of the same roll in a row (0 for no rolls).
pub fn longest_streak(rolls: &[u32]) -> usize {
    todo!()
}

/// The biggest increase between two consecutive levels (0 if none).
/// [3, 5, 4, 9] → 5 (4 → 9)
pub fn biggest_jump(levels: &[i32]) -> i32 {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting() {
        let c = kill_counts(&["slime", "bat", "slime", "slime"]);
        assert_eq!(c["slime"], 3);
        assert_eq!(c["bat"], 1);
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn streaks() {
        assert_eq!(longest_streak(&[6, 6, 2, 6, 6, 6, 1]), 3);
        assert_eq!(longest_streak(&[4]), 1);
        assert_eq!(longest_streak(&[]), 0);
    }

    #[test]
    fn jumps() {
        assert_eq!(biggest_jump(&[3, 5, 4, 9]), 5);
        assert_eq!(biggest_jump(&[9, 8, 7]), 0);
        assert_eq!(biggest_jump(&[1]), 0);
    }
}
