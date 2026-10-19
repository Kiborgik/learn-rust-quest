// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 21-1 · Start the Engine                          175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   tokio is the runtime most async Rust uses: the executor from last
//   lesson, plus timers, networking, files, channels and a thread pool.
//
//   `main` can't be async by itself (who would poll it?). The attribute
//   builds a runtime and runs your async main on it:
//
//     #[tokio::main]
//     async fn main() {
//         let msg = brew("tea", 1).await;
//     }
//
//   Same for tests:
//
//     #[tokio::test]
//     async fn brews() { assert_eq!(brew("tea", 1).await, "tea is ready"); }
//
//   Timers: `tokio::time::sleep(Duration::from_secs(5)).await` waits
//   WITHOUT blocking the thread (other tasks keep running). Never use
//   `std::thread::sleep` in async code: it freezes the whole worker.
//
//   Fast tests with fake time:
//
//     #[tokio::test(start_paused = true)]    // the clock only moves when
//     async fn quick() { ... }               // every task is waiting, and
//                                            // then jumps ahead instantly
//
// 🎯 YOUR TASK
//
//   1. Make `main` run on tokio.
//   2. Make the tests async tokio tests with paused time.

use std::time::Duration;
use tokio::time::{Instant, sleep};

async fn brew(potion: &str, secs: u64) -> String {
    sleep(Duration::from_secs(secs)).await;
    format!("{potion} is ready")
}

async fn main() {
    println!("{}", brew("tea", 1).await);
}

// ─── Tests: don't change these (except their attributes)! ───────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn brews() {
        assert_eq!(brew("tea", 1).await, "tea is ready");
    }

    #[test]
    async fn long_brew_is_instant_with_paused_time() {
        let start = Instant::now();
        assert_eq!(brew("elixir", 3600).await, "elixir is ready");
        assert_eq!(start.elapsed().as_secs(), 3600);
    }
}
