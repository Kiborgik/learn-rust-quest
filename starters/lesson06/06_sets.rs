// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 6-6 · Unique Relics                              125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     use std::collections::{HashSet, BTreeMap, BTreeSet};
//
//     let mut seen = HashSet::new();
//     seen.insert("ring");           // true:  newly added
//     seen.insert("ring");           // false: already present!
//     seen.contains("ring");         // true
//
//   HashMap / HashSet have NO defined order. The B-tree versions keep
//   their keys SORTED (and keys must be comparable, i.e. `Ord`):
//
//     let mut scores = BTreeMap::new();
//     scores.insert("zed", 1);
//     scores.insert("amy", 2);
//     for (name, s) in &scores { }   // amy, then zed. Always.
//
//     let sorted: BTreeSet<u32> = ...;  // unique AND sorted
//
//   | Need                           | Use       |
//   |--------------------------------|-----------|
//   | fast lookup, order irrelevant  | HashMap   |
//   | sorted / deterministic output  | BTreeMap  |
//   | unique values                  | HashSet   |
//   | unique and sorted              | BTreeSet  |
//
//   A set (or map) can be looped over like a Vec. A B-tree set gives
//   its values in sorted order:
//
//     let mut v = Vec::new();
//     for n in sorted { v.push(n); }    // Vec<u32>, sorted
//
//   `join` glues a list of strings together with a separator:
//
//     let parts = vec![String::from("a"), String::from("b")];
//     parts.join(", ");                 // "a, b"
//
// 🎯 YOUR TASK
//
//   1. Fix `first_duplicate`: return the first relic seen TWICE.
//   2. Fix `report`: the output must be in alphabetical order.
//      Expected: "amulet=2, crown=1, ring=3"
//   3. Implement `unique_sorted`: remove duplicates and sort.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

fn first_duplicate(relics: &[&str]) -> Option<String> {
    let mut seen = HashSet::new();
    for r in relics {
        if seen.insert(*r) {
            return Some(r.to_string());
        }
    }
    None
}

fn report(relics: &[&str]) -> String {
    let mut counts = HashMap::new();
    for r in relics {
        *counts.entry(*r).or_insert(0) += 1;
    }
    let mut parts = Vec::new();
    for (name, n) in &counts {
        parts.push(format!("{name}={n}"));
    }
    parts.join(", ")
}

fn unique_sorted(values: &[u32]) -> Vec<u32> {
    todo!()
}

fn main() {
    let relics = ["ring", "amulet", "ring", "crown", "amulet", "ring"];
    println!("{:?}", first_duplicate(&relics));
    println!("{}", report(&relics));
    println!("{:?}", unique_sorted(&[5, 1, 5, 3, 1]));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicates() {
        let relics = ["ring", "amulet", "crown", "amulet", "ring"];
        assert_eq!(first_duplicate(&relics), Some(String::from("amulet")));
        assert_eq!(first_duplicate(&["a", "b"]), None);
    }

    #[test]
    fn reports() {
        let relics = ["ring", "amulet", "ring", "crown", "amulet", "ring"];
        assert_eq!(report(&relics), "amulet=2, crown=1, ring=3");
    }

    #[test]
    fn uniques() {
        assert_eq!(unique_sorted(&[5, 1, 5, 3, 1]), vec![1, 3, 5]);
        assert_eq!(unique_sorted(&[]), Vec::<u32>::new());
    }
}
