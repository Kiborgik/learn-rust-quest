// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 18-5 · Divide and Conquer                        175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `rayon::join(a, b)` runs two closures, possibly in parallel, and
//   returns both results. It's the building block under par_iter:
//
//     let (left, right) = rayon::join(
//         || count(&data[..mid]),
//         || count(&data[mid..]),
//     );
//
//   Recursive divide and conquer, with a cutoff so tiny pieces run
//   sequentially (splitting has a cost):
//
//     fn sum(v: &[u64]) -> u64 {
//         if v.len() <= 1000 {
//             return v.iter().sum();                 // small: just do it
//         }
//         let (a, b) = v.split_at(v.len() / 2);
//         let (x, y) = rayon::join(|| sum(a), || sum(b));
//         x + y
//     }
//
//   join borrows freely: both closures can use local data, like scoped
//   threads, because join only returns when both are done.
//
// 🎯 YOUR TASK
//
//   1. `tree_max` should use `rayon::join` for big slices (it has the
//      cutoff already). Finish the recursive case.
//   2. `count_paths` counts paths through a grid moving only right or
//      down (a classic). Split the two moves with `rayon::join` when
//      the grid is big, plain recursion otherwise.

/// The biggest value, or 0 for an empty slice.
pub fn tree_max(v: &[u32]) -> u32 {
    if v.len() <= 1_000 {
        return v.iter().copied().max().unwrap_or(0);
    }
    todo!()
}

/// Paths from (0, 0) to (w, h), each step right or down.
pub fn count_paths(w: u32, h: u32) -> u64 {
    if w == 0 || h == 0 {
        return 1;
    }
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maxes() {
        let v: Vec<u32> = (0..200_000).map(|i| (i * 7919) % 100_003).collect();
        assert_eq!(tree_max(&v), *v.iter().max().unwrap());
        assert_eq!(tree_max(&[]), 0);
        assert_eq!(tree_max(&[4, 9, 2]), 9);
    }

    #[test]
    fn paths() {
        assert_eq!(count_paths(1, 1), 2);
        assert_eq!(count_paths(2, 2), 6);
        assert_eq!(count_paths(8, 8), 12_870);
    }
}
