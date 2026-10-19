// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 21-2 · Send the Scouts                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `tokio::spawn` starts a TASK: a future that runs on its own, in the
//   background, maybe on another worker thread. You get a JoinHandle:
//
//     let handle = tokio::spawn(async move {
//         scout("north").await              // runs concurrently with the caller
//     });
//     let report = handle.await.unwrap();   // Err if the task panicked
//
//   Like `thread::spawn` (lesson 12), the future must be:
//     • 'static: it may outlive the caller, so no borrowed locals
//       → use `async move` and owned data (clone what you need)
//     • Send: it may move between threads
//
//   Tasks are cheap: thousands are fine (they're futures, not OS threads).
//
// 🎯 YOUR TASK
//
//   `scout_all` should scout every region at the same time, then return
//   the reports in the order of `regions`. It doesn't compile: the tasks
//   borrow `regions`. Fix it.

use std::time::Duration;
use tokio::time::sleep;

pub async fn scout(region: &str) -> String {
    sleep(Duration::from_secs(region.len() as u64)).await;
    format!("{region}: clear")
}

pub async fn scout_all(regions: Vec<String>) -> Vec<String> {
    let mut handles = Vec::new();
    for region in &regions {
        handles.push(tokio::spawn(async { scout(region).await }));
    }
    let mut reports = Vec::new();
    for h in handles {
        reports.push(h.await.unwrap());
    }
    reports
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test(start_paused = true)]
    async fn scouting() {
        let start = Instant::now();
        let regions = vec!["north".to_string(), "far east".to_string(), "sea".to_string()];
        let reports = scout_all(regions).await;
        assert_eq!(reports, vec!["north: clear", "far east: clear", "sea: clear"]);
        // all at once: as long as the slowest (8 s), not the sum (16 s)
        assert_eq!(start.elapsed().as_secs(), 8);
    }
}
