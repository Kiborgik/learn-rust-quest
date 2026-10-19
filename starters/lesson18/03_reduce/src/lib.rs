// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 18-3 · Many Hands, One Answer                    175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Combining results across threads needs care: each thread works on its
//   own piece, then the pieces are merged in some tree shape.
//
//   `reduce` takes an IDENTITY (a fresh starting value) and a merge:
//
//     let longest = words.par_iter()
//         .map(|w| w.len())
//         .reduce(|| 0, |a, b| a.max(b));
//
//   The merge must be ASSOCIATIVE: (a·b)·c == a·(b·c). max, +, min,
//   merging maps: fine. Subtraction: wrong answers, randomly.
//
//   `fold` + `reduce`: each thread builds its own accumulator, then they
//   are merged. The standard trick for parallel counting:
//
//     let counts: HashMap<char, usize> = words.par_iter()
//         .fold(HashMap::new, |mut map, w| {          // per thread
//             *map.entry(first_char(w)).or_insert(0) += 1;
//             map
//         })
//         .reduce(HashMap::new, |mut a, b| {         // merge threads
//             for (k, v) in b { *a.entry(k).or_insert(0) += v; }
//             a
//         });
//
//   No locks needed: no two threads ever touch the same map.
//
// 🎯 YOUR TASK
//
//   Implement both functions with `reduce` / `fold`.

use rayon::prelude::*;
use std::collections::HashMap;

/// The highest damage, or 0 for no hits.
pub fn max_hit(hits: &[u32]) -> u32 {
    todo!()
}

/// How many monsters of each kind.
pub fn census(monsters: &[&str]) -> HashMap<String, usize> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maxes() {
        assert_eq!(max_hit(&[3, 99, 12]), 99);
        assert_eq!(max_hit(&[]), 0);
        let many: Vec<u32> = (0..100_000).collect();
        assert_eq!(max_hit(&many), 99_999);
    }

    #[test]
    fn counting() {
        let kinds = ["slime", "bat", "slime", "orc"];
        let monsters: Vec<&str> = kinds.iter().copied().cycle().take(40_000).collect();
        let c = census(&monsters);
        assert_eq!(c["slime"], 20_000);
        assert_eq!(c["bat"], 10_000);
        assert_eq!(c.len(), 3);
    }
}
