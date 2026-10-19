// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 18-2 · Order From Chaos                          175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Threads finish in any order, but rayon's `collect` still puts results
//   in the ORIGINAL order. Indexes are tracked for you:
//
//     let doubled: Vec<u32> = v.par_iter().map(|x| x * 2).collect();
//     // same order as v, always
//
//   Splitting into chunks, each processed in parallel:
//
//     let sums: Vec<u32> = v.par_chunks(100)          // &[T] pieces of up to 100
//         .map(|chunk| chunk.iter().sum())
//         .collect();
//
//   Changing a slice in place, in parallel:
//
//     v.par_iter_mut().for_each(|x| *x *= 2);
//
//   But `for_each` with side effects (println!, pushing to a Mutex<Vec>)
//   runs in random order. Use `collect` when order matters.
//
// 🎯 YOUR TASK
//
//   Implement the three functions with rayon. The tests check the order.

use rayon::prelude::*;

/// "Ferris (lvl 3)" for each hero, in the same order.
pub fn labels(heroes: &[(&str, u32)]) -> Vec<String> {
    todo!()
}

/// Sum of each chunk of `size` gems.
pub fn chunk_sums(gems: &[u32], size: usize) -> Vec<u32> {
    todo!()
}

/// Heal every hero by `amount`, capped at 100, in place.
pub fn heal_all(hp: &mut [u32], amount: u32) {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_order() {
        let heroes: Vec<(&str, u32)> = vec![("Ferris", 3), ("Corro", 7), ("Lumi", 1)];
        assert_eq!(labels(&heroes), vec!["Ferris (lvl 3)", "Corro (lvl 7)", "Lumi (lvl 1)"]);
        let many: Vec<(&str, u32)> = (0..10_000).map(|i| ("x", i)).collect();
        let out = labels(&many);
        assert_eq!(out[9_999], "x (lvl 9999)");
        assert_eq!(out[5_000], "x (lvl 5000)");
    }

    #[test]
    fn chunks() {
        assert_eq!(chunk_sums(&[1, 2, 3, 4, 5], 2), vec![3, 7, 5]);
    }

    #[test]
    fn healing() {
        let mut hp = vec![10, 95, 100];
        heal_all(&mut hp, 20);
        assert_eq!(hp, vec![30, 100, 100]);
    }
}
