// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 18-6 · Hire Your Own Crew                        175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   By default rayon uses one global pool with one thread per CPU core.
//   Sometimes you want your own: fewer threads (leave cores for others),
//   or a separate pool for one kind of job.
//
//     let pool = rayon::ThreadPoolBuilder::new()
//         .num_threads(2)
//         .build()
//         .unwrap();
//
//     let total = pool.install(|| {
//         // every par_iter in here runs on `pool`
//         data.par_iter().sum::<u64>()
//     });
//
//     rayon::current_num_threads()   // threads of the pool you're in
//
//   `install` returns the closure's value, so it fits right into normal
//   code. Pools are cheap to keep and costly to create: build once,
//   reuse.
//
// 🎯 YOUR TASK
//
//   Implement `with_crew`: run `job` on a fresh pool of `threads` threads
//   and return (the job's result, how many threads that pool has).

use rayon::prelude::*;

pub fn with_crew<T: Send>(threads: usize, job: impl FnOnce() -> T + Send) -> (T, usize) {
    todo!()
}

pub fn total_weight(items: &[u64]) -> u64 {
    items.par_iter().sum()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crews() {
        let items: Vec<u64> = (1..=100).collect();
        let (total, threads) = with_crew(2, || total_weight(&items));
        assert_eq!(total, 5050);
        assert_eq!(threads, 2);
        let (_, threads) = with_crew(3, || ());
        assert_eq!(threads, 3);
    }
}
