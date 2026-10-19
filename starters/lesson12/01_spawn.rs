// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 12-1 · Summoning Workers                         150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     use std::thread;
//
//     let handle = thread::spawn(|| {     // runs the closure on a new OS thread
//         2 + 2                           // a thread can RETURN a value
//     });
//
//     let result = handle.join().unwrap();  // wait for it to finish → 4
//
//   • `spawn` returns a `JoinHandle<T>`, where T is the closure's return type.
//   • `join()` blocks until the thread finishes. It returns a `Result`:
//     Err if the thread panicked.
//   • If `main` ends, the whole process exits, even if other threads
//     are still running. Always join the threads whose work you need!
//   • Output from different threads can interleave in any order. Only
//     print from `main` if you need a predictable order.
//
//   Spawning several workers:
//
//     let mut handles = Vec::new();
//     for i in 0..3 {
//         handles.push(thread::spawn(move || i * 10));
//     }
//     for h in handles { println!("{}", h.join().unwrap()); }  // 0, 10, 20
//
//   (`move` copies `i` into each closure; that's the next quest's topic.)
//
// 🎯 YOUR TASK
//
//   1. `parallel_square` should compute `n * n` on a new thread and
//      return the result. Right now it never waits for the thread.
//   2. `sum_on_workers`: spawn one thread per number in 1..=count,
//      each returning its number * 100, then add up all the results.

use std::thread;

fn parallel_square(n: u64) -> u64 {
    let handle = thread::spawn(move || n * n);
    handle
}

fn sum_on_workers(count: u64) -> u64 {
    todo!()
}

fn main() {
    println!("{}", parallel_square(12));
    println!("{}", sum_on_workers(4));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        assert_eq!(parallel_square(12), 144);
    }

    #[test]
    fn workers() {
        assert_eq!(sum_on_workers(4), 1000);
        assert_eq!(sum_on_workers(0), 0);
    }
}
