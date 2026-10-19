// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 18-1 · One Word Faster                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   In lesson 12 you split work across threads by hand. `rayon` does it
//   for you: change `iter()` to `par_iter()` and the work spreads over
//   every CPU core.
//
//     use rayon::prelude::*;          // brings par_iter() & co into scope
//
//     let total: u64 = nums.par_iter().map(|n| n * n).sum();
//     let big = nums.par_iter().filter(|n| **n > 100).count();
//     let any = nums.par_iter().any(|n| *n == 42);
//
//     (0..1_000_000u64).into_par_iter()   // ranges and owned Vecs
//     v.par_iter_mut().for_each(|x| *x += 1);
//
//   How it works: a pool of worker threads (one per core) and "work
//   stealing". Idle threads take chunks of work from busy ones.
//
//   Safety comes from the types: the closure may only share data that is
//   `Sync`, exactly like scoped threads. Data races still don't compile.
//
//   It isn't always faster: for tiny inputs, the cost of splitting is
//   bigger than the work. Measure.
//
// 🎯 YOUR TASK
//
//   1. `sum_of_squares` uses par_iter but can't find it. Fix that.
//   2. Implement `count_primes` in parallel (`is_prime` is given).

pub fn sum_of_squares(nums: &[u64]) -> u64 {
    nums.par_iter().map(|n| n * n).sum()
}

pub fn is_prime(n: u64) -> bool {
    n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

/// How many primes are in 0..limit.
pub fn count_primes(limit: u64) -> usize {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        assert_eq!(sum_of_squares(&[1, 2, 3]), 14);
        let big: Vec<u64> = (1..=1000).collect();
        assert_eq!(sum_of_squares(&big), 333_833_500);
    }

    #[test]
    fn primes() {
        assert_eq!(count_primes(10), 4);
        assert_eq!(count_primes(100_000), 9592);
    }
}
