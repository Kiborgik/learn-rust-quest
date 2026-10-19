// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 16-6 · Every Combination                         175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     use itertools::{Itertools, MinMaxResult};
//
//     ["a", "b"].iter().cartesian_product([1, 2])   // (a,1) (a,2) (b,1) (b,2)
//     [1, 3].iter().interleave(&[2, 4])             // 1, 2, 3, 4
//     (1..=4).combinations(2)                       // [1,2] [1,3] ... (Vec each)
//     [3, 9, 1].iter().minmax()                     // MinMax(1, 9)
//     [5, 0, 5].iter().positions(|x| *x == 5)       // 0, 2 (indexes)
//
//   MinMaxResult has three cases (NoElements, OneElement, MinMax).
//   `.into_option()` turns it into `Option<(min, max)>`.
//
// 🎯 YOUR TASK
//
//   Implement each function with one itertools chain.

use itertools::Itertools;

/// Every outfit "hat + cloak", hats in the outer loop.
pub fn outfits(hats: &[&str], cloaks: &[&str]) -> Vec<String> {
    todo!()
}

/// Every possible pair of party members, as "A & B", in order.
/// ["A", "B", "C"] → ["A & B", "A & C", "B & C"]
pub fn duos(party: &[&str]) -> Vec<String> {
    todo!()
}

/// (lowest, highest) damage, or None when there are no hits.
pub fn damage_range(hits: &[u32]) -> Option<(u32, u32)> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outfit_list() {
        assert_eq!(
            outfits(&["red", "blue"], &["fur", "silk"]),
            vec!["red + fur", "red + silk", "blue + fur", "blue + silk"]
        );
    }

    #[test]
    fn duo_list() {
        assert_eq!(duos(&["A", "B", "C"]), vec!["A & B", "A & C", "B & C"]);
        assert!(duos(&["solo"]).is_empty());
    }

    #[test]
    fn ranges() {
        assert_eq!(damage_range(&[7, 2, 9, 4]), Some((2, 9)));
        assert_eq!(damage_range(&[5]), Some((5, 5)));
        assert_eq!(damage_range(&[]), None);
    }
}
