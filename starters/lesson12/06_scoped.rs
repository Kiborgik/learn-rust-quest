// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 12-6 · Scoped Swarms                             150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `thread::spawn` needs `'static` data because the thread might
//   outlive everything. SCOPED threads are guaranteed to finish before
//   the scope ends, so they CAN borrow local data, with no Arc and no
//   cloning:
//
//     let numbers = vec![1, 2, 3, 4];
//     let total = thread::scope(|s| {
//         let left = s.spawn(|| numbers[..2].iter().sum::<i32>());
//         let right = s.spawn(|| numbers[2..].iter().sum::<i32>());
//         left.join().unwrap() + right.join().unwrap()
//     });                               // all threads are joined here
//
//   Splitting work into pieces:
//
//     for chunk in data.chunks(100) { ... }   // slices of up to 100 items
//
//   Atomics: lock-free counters for simple numbers:
//
//     use std::sync::atomic::{AtomicU32, Ordering};
//     let hits = AtomicU32::new(0);
//     hits.fetch_add(1, Ordering::Relaxed);   // works through &, from any thread
//     hits.load(Ordering::Relaxed);
//
// 🎯 YOUR TASK
//
//   1. `parallel_sum`: fix it to use `thread::scope` so the threads can
//      borrow `data`.
//   2. `count_big`: count the values > 50 using several scoped threads
//      (one per chunk of `chunk_size`), all incrementing one shared
//      `AtomicU32`.

use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

/// Sum `data` by splitting it into two halves, one per thread.
fn parallel_sum(data: &[u64]) -> u64 {
    let mid = data.len() / 2;
    let left = thread::spawn(|| data[..mid].iter().sum::<u64>());
    let right = thread::spawn(|| data[mid..].iter().sum::<u64>());
    left.join().unwrap() + right.join().unwrap()
}

fn count_big(data: &[u32], chunk_size: usize) -> u32 {
    let counter = AtomicU32::new(0);
    // TODO: thread::scope + data.chunks(chunk_size) + counter.fetch_add(...)
    counter.load(Ordering::Relaxed)
}

fn main() {
    let data: Vec<u64> = (1..=100).collect();
    println!("{}", parallel_sum(&data));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summing() {
        let data: Vec<u64> = (1..=100).collect();
        assert_eq!(parallel_sum(&data), 5050);
        assert_eq!(parallel_sum(&[]), 0);
    }

    #[test]
    fn big_ones() {
        let data: Vec<u32> = (1..=100).collect();
        assert_eq!(count_big(&data, 7), 50);
        assert_eq!(count_big(&[99, 1, 51], 1), 2);
    }
}
