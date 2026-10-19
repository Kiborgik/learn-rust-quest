// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 16-1 · More Tools for the Belt                   175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `itertools` adds dozens of adapters to every iterator. Import the
//   trait and they appear as methods:
//
//     use itertools::Itertools;
//
//     ["b", "a", "b"].iter().sorted()           // a, b, b   (collects + sorts)
//     ["b", "a", "b"].iter().unique()           // b, a      (first time seen)
//     [1, 1, 2, 1].iter().dedup()               // 1, 2, 1   (adjacent repeats)
//     ["a", "b"].iter().join(", ")              // "a, b"    (no Vec needed!)
//     [3, 1, 2].iter().sorted_by_key(|x| ...)   // sort by a key
//     v.iter().k_smallest(3)                    // the 3 smallest, cheaply
//
//   Compare with plain std:
//
//     let mut v: Vec<_> = items.iter().collect();
//     v.sort();
//     v.dedup();
//     v.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")
//
//   vs.  items.iter().sorted().dedup().join(", ")
//
// 🎯 YOUR TASK
//
//   Implement each function as one itertools chain.

use itertools::Itertools;

/// Sorted, without duplicates, joined by ", ".   ["b","a","b"] → "a, b"
pub fn roster(names: &[&str]) -> String {
    todo!()
}

/// Names in the order they FIRST appear, each once.
pub fn first_seen(names: &[&str]) -> Vec<String> {
    todo!()
}

/// The 3 lowest scores, smallest first.
pub fn podium(scores: &[u32]) -> Vec<u32> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rosters() {
        assert_eq!(roster(&["Lumi", "Ferris", "Corro", "Ferris"]), "Corro, Ferris, Lumi");
        assert_eq!(roster(&[]), "");
    }

    #[test]
    fn firsts() {
        assert_eq!(first_seen(&["b", "a", "b", "c", "a"]), vec!["b", "a", "c"]);
    }

    #[test]
    fn podiums() {
        assert_eq!(podium(&[9, 3, 7, 1, 8]), vec![1, 3, 7]);
        assert_eq!(podium(&[5]), vec![5]);
    }
}
