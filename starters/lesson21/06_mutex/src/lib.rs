// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 21-6 · Don't Hold the Lock                       175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A `std::sync::MutexGuard` held across an `.await` is a trap: the task
//   may pause there for a long time while holding the lock, and the guard
//   isn't `Send`, so tokio::spawn refuses the future:
//
//     let mut g = vault.lock().unwrap();     // std Mutex
//     slow().await;                          // ✘ "future cannot be sent
//     *g += 1;                               //    between threads safely"
//
//   Two fixes:
//
//   1. Don't hold it across the await (best when you can):
//        let x = slow().await;
//        *vault.lock().unwrap() += x;        // lock, update, unlock at once
//
//   2. Use tokio's async Mutex, which can be held across awaits:
//        let vault = Arc::new(tokio::sync::Mutex::new(0));
//        let mut g = vault.lock().await;
//
//   CPU-heavy or blocking code (big loops, std file I/O, blocking libraries)
//   also freezes a worker thread. Move it to the blocking pool:
//
//     let hash = tokio::task::spawn_blocking(move || expensive(data)).await?;
//
// 🎯 YOUR TASK
//
//   1. `deposit_all` doesn't compile. Fix it (either way works).
//   2. `heavy_sum` must run `crunch` with `spawn_blocking`.

use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

async fn count_coins(amount: u32) -> u32 {
    sleep(Duration::from_millis(10 * amount as u64)).await;
    amount
}

pub async fn deposit_all(amounts: Vec<u32>) -> u32 {
    let vault = Arc::new(Mutex::new(0u32));
    let mut handles = Vec::new();
    for a in amounts {
        let vault = vault.clone();
        handles.push(tokio::spawn(async move {
            let mut guard = vault.lock().unwrap();
            let counted = count_coins(a).await;
            *guard += counted;
        }));
    }
    for h in handles {
        h.await.unwrap();
    }
    let total = *vault.lock().unwrap();
    total
}

/// A slow, blocking computation. Don't run it directly on the runtime.
pub fn crunch(n: u64) -> u64 {
    (1..=n).map(|i| i % 7).sum()
}

pub async fn heavy_sum(n: u64) -> u64 {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn deposits() {
        assert_eq!(deposit_all(vec![5, 10, 1, 4]).await, 20);
        assert_eq!(deposit_all(vec![]).await, 0);
    }

    #[tokio::test]
    async fn heavy() {
        assert_eq!(heavy_sum(14).await, 42);
        assert_eq!(heavy_sum(3_000_000).await, crunch(3_000_000));
    }
}
