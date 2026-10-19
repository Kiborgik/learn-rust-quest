// ╔══════════════════════════════════════════════════════════════════╗
// ║  👑 FINAL BOSS · The Hive Queen                           350 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   This is the end of the journey. The Hive Queen commands ten thousand
//   threads, and she has never lost a race condition. Use EVERYTHING
//   you've learned: ownership, closures, iterators, error handling,
//   smart pointers, and fearless concurrency.
//
//   ⚔️  HEAD 1: `parallel_word_count(texts: &[&str]) -> BTreeMap<String, usize>`
//        Count words (split_whitespace, lowercased) across all texts,
//        ONE scoped thread per text, merged into one map.
//        (Each thread can build its own HashMap/BTreeMap and return it,
//        and main merges them; no locking needed!)
//
//   ⚔️  HEAD 2: `pipeline(numbers: Vec<u64>) -> u64`
//        A 2-stage channel pipeline:
//          stage 1 thread: sends each number SQUARED to channel A
//          stage 2 thread: receives from A, sends only the EVEN values to channel B
//          main: sums everything from B
//        (Senders must be dropped for the loops to end!)
//
//   ⚔️  HEAD 3: `struct WorkQueue` with a worker pool:
//        `fn run_jobs(jobs: Vec<u64>, workers: usize) -> Vec<u64>`
//        Put the jobs in an `Arc<Mutex<Vec<u64>>>`. Each worker loops:
//        pop a job (stop when empty), compute job * 2, push the result into
//        a shared `Arc<Mutex<Vec<u64>>>`. Return the results SORTED.
//        ⚠️ Pop in its own statement, so the lock is released before
//        the work starts:  `let job = queue.lock().unwrap().pop();`
//        (You don't need a `struct WorkQueue`; it's just the theme.)
//
//   ⚔️  HEAD 4: `fn try_all(inputs: &[&str]) -> Result<u64, String>`
//        Parse every input on its OWN thread (std::thread::spawn), and
//        sum them. If any parse fails → Err("bad input: {that input}"),
//        reporting the FIRST failing input in input order.
//
//   ⚔️  HEAD 5: `fn race(n: u32) -> u32`
//        `n` threads each increment a shared AtomicU32 1000 times.
//        Return the final value (it must be exactly n * 1000).

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Your code goes here ↓



fn main() {
    println!("The hive hums with ten thousand wings. Run the tests for the final battle!");
}

// ─── The Queen's tests: don't change these! ──────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_1_word_count() {
        let m = parallel_word_count(&["the bee", "The queen  the hive", "bee BEE"]);
        let v: Vec<(&str, usize)> = m.iter().map(|(k, v)| (k.as_str(), *v)).collect();
        assert_eq!(v, vec![("bee", 3), ("hive", 1), ("queen", 1), ("the", 3)]);
    }

    #[test]
    fn head_2_pipeline() {
        // squares: 1 4 9 16 25 36 → evens: 4 16 36
        assert_eq!(pipeline(vec![1, 2, 3, 4, 5, 6]), 56);
        assert_eq!(pipeline(vec![]), 0);
    }

    #[test]
    fn head_3_worker_pool() {
        let jobs: Vec<u64> = (1..=20).collect();
        let results = run_jobs(jobs, 4);
        let expected: Vec<u64> = (1..=20).map(|x| x * 2).collect();
        assert_eq!(results, expected);
        assert_eq!(run_jobs(vec![], 3), Vec::<u64>::new());
    }

    #[test]
    fn head_4_try_all() {
        assert_eq!(try_all(&["1", "2", "39"]), Ok(42));
        assert_eq!(try_all(&["1", "x", "y"]), Err(String::from("bad input: x")));
        assert_eq!(try_all(&[]), Ok(0));
    }

    #[test]
    fn head_5_race() {
        assert_eq!(race(8), 8000);
        assert_eq!(race(1), 1000);
    }
}
