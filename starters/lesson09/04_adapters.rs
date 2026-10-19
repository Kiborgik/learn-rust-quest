// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 9-4 · Filtering the Current                      150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   ADAPTERS transform an iterator into another iterator (lazily):
//
//     let v = vec![1, 2, 3, 4, 5, 6];
//
//     v.iter().map(|x| x * 10)               // 10, 20, 30, ...
//     v.iter().filter(|x| **x % 2 == 0)      // 2, 4, 6
//     v.iter().enumerate()                   // (0, &1), (1, &2), ...
//     v.iter().zip(other.iter())             // pairs: (&a, &b)
//     v.iter().chain(other.iter())           // one after the other
//     v.iter().take(2)  .skip(2)  .rev()     // first 2 / skip 2 / backwards
//     v.iter().copied()                      // &u32 → u32 (for Copy types)
//
//   Then COLLECT the result into a collection:
//
//     let evens: Vec<u32> = v.iter().copied().filter(|x| x % 2 == 0).collect();
//     let s: String = vec!['h', 'i'].into_iter().collect();
//     let m: HashMap<&str, u32> = vec![("a", 1)].into_iter().collect();
//     let v2 = v.iter().map(|x| x + 1).collect::<Vec<_>>();  // turbofish
//
//   ⚠️ `filter` receives a REFERENCE to each item. On `.iter()` that's
//   a reference to a reference (`&&u32`), hence the `**x` above. Calling
//   `.copied()` first keeps things simple.
//
// 🎯 YOUR TASK
//
//   Implement each function as ONE iterator chain (no `for` loops, no
//   `push`).

use std::collections::HashMap;

/// Squares of all numbers.
fn squares(nums: &[u32]) -> Vec<u32> {
    todo!()
}

/// Only the strong ones (power > 50), in order.
fn strong(powers: &[u32]) -> Vec<u32> {
    todo!()
}

/// "1. Ferris", "2. Corro", ...  (numbering starts at 1)
fn numbered(names: &[&str]) -> Vec<String> {
    todo!()
}

/// name → score map, pairing names with scores in order.
fn scoreboard(names: &[&str], scores: &[u32]) -> HashMap<String, u32> {
    todo!()
}

/// The last `n` items, in reverse order.
fn last_n_reversed(items: &[u32], n: usize) -> Vec<u32> {
    todo!()
}

/// All words uppercased, joined with "-" (e.g. "go west" → "GO-WEST").
/// Tip: collect into a Vec<String>, then `.join("-")`.
fn shout_words(text: &str) -> String {
    todo!()
}

fn main() {
    println!("{:?}", squares(&[1, 2, 3]));
    println!("{:?}", numbered(&["Ferris", "Corro"]));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squaring() {
        assert_eq!(squares(&[1, 2, 3]), vec![1, 4, 9]);
    }

    #[test]
    fn filtering() {
        assert_eq!(strong(&[10, 60, 51, 50, 99]), vec![60, 51, 99]);
    }

    #[test]
    fn numbering() {
        assert_eq!(numbered(&["Ferris", "Corro"]), vec!["1. Ferris", "2. Corro"]);
    }

    #[test]
    fn scores() {
        let board = scoreboard(&["a", "b", "c"], &[5, 7]);
        assert_eq!(board.len(), 2); // zip stops at the shorter one
        assert_eq!(board["a"], 5);
        assert_eq!(board["b"], 7);
    }

    #[test]
    fn reversing() {
        assert_eq!(last_n_reversed(&[1, 2, 3, 4, 5], 2), vec![5, 4]);
    }

    #[test]
    fn shouting() {
        assert_eq!(shout_words("go  west young crab"), "GO-WEST-YOUNG-CRAB");
    }
}
