// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 21-4 · First Past the Post                       175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `timeout` gives up on a future after a while:
//
//     use tokio::time::timeout;
//     match timeout(Duration::from_secs(2), ask()).await {
//         Ok(answer) => ...,           // finished in time
//         Err(_elapsed) => ...,        // too slow
//     }
//
//   `select!` waits for the FIRST of several futures and drops the rest
//   (dropping a future cancels it: it simply won't be polled again):
//
//     tokio::select! {
//         a = ride("red", 3) => println!("red wins: {a}"),
//         b = ride("blue", 2) => println!("blue wins: {b}"),
//     }
//
//   This is how async code does cancellation: lose the race, get dropped.
//   Keep that in mind: work done inside a cancelled future can stop at
//   any `.await`.
//
// 🎯 YOUR TASK
//
//   1. `ask_with_patience`: the oracle's answer, or Err("the oracle is
//      silent") if it takes longer than `patience` seconds.
//   2. `race`: the name of the rider who arrives first.

use std::time::Duration;
use tokio::time::{sleep, timeout};

async fn oracle(delay: u64) -> String {
    sleep(Duration::from_secs(delay)).await;
    "yes".into()
}

async fn ride(name: &'static str, secs: u64) -> &'static str {
    sleep(Duration::from_secs(secs)).await;
    name
}

pub async fn ask_with_patience(delay: u64, patience: u64) -> Result<String, &'static str> {
    todo!()
}

pub async fn race(red_secs: u64, blue_secs: u64) -> &'static str {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test(start_paused = true)]
    async fn patience() {
        assert_eq!(ask_with_patience(1, 5).await, Ok("yes".to_string()));
        let start = Instant::now();
        assert_eq!(ask_with_patience(60, 5).await, Err("the oracle is silent"));
        assert_eq!(start.elapsed().as_secs(), 5);
    }

    #[tokio::test(start_paused = true)]
    async fn racing() {
        assert_eq!(race(3, 2).await, "blue");
        let start = Instant::now();
        assert_eq!(race(1, 100).await, "red");
        assert_eq!(start.elapsed().as_secs(), 1);
    }
}
