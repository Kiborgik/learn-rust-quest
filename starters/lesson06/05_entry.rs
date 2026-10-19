// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 6-5 · Counting Coins                             125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The ENTRY API: "get this key's slot, creating it if it's missing":
//
//     let mut counts: HashMap<&str, u32> = HashMap::new();
//     for word in ["a", "b", "a"] {
//         let c = counts.entry(word).or_insert(0);   // &mut u32
//         *c += 1;
//     }
//     // counts: {"a": 2, "b": 1}
//
//     // or in one line:
//     *counts.entry("c").or_insert(0) += 1;
//
//     // grouping values into lists:
//     let mut groups: HashMap<char, Vec<&str>> = HashMap::new();
//     groups.entry('a').or_insert_with(Vec::new).push("apple");
//     // or_insert_with only builds the default when it's actually needed
//
//   One lookup instead of "contains_key, then get or insert".
//
//   `word.len()` is fine for ASCII words. Text in this quest is plain
//   ASCII.
//
// 🎯 YOUR TASK
//
//   1. Fix `count_coins` so it counts each coin type.
//   2. Implement `word_lengths`: group words by their length.
//      ("a bb cc" → {1: ["a"], 2: ["bb", "cc"]}, in their original order)

use std::collections::HashMap;

fn count_coins(coins: &[&str]) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for coin in coins {
        counts.insert(coin.to_string(), 1);
    }
    counts
}

fn word_lengths(text: &str) -> HashMap<usize, Vec<String>> {
    todo!()
}

fn main() {
    let c = count_coins(&["gold", "silver", "gold"]);
    println!("gold coins: {}", c["gold"]);
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coins() {
        let c = count_coins(&["gold", "silver", "gold", "copper", "gold"]);
        assert_eq!(c["gold"], 3);
        assert_eq!(c["silver"], 1);
        assert_eq!(c["copper"], 1);
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn grouping() {
        let g = word_lengths("a bb cc ddd e");
        assert_eq!(g[&1], vec!["a", "e"]);
        assert_eq!(g[&2], vec!["bb", "cc"]);
        assert_eq!(g[&3], vec!["ddd"]);
        assert_eq!(g.len(), 3);
    }
}
