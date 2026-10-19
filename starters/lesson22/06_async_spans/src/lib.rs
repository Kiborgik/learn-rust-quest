// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 22-6 · Tangled Threads of Story                  175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `span.enter()` marks the CURRENT THREAD as "inside the span" until the
//   guard drops. In async code that's a bug: at every `.await` the task
//   pauses, and another task runs on the same thread, still "inside"
//   your span. Logs get tangled.
//
//     async fn deliver(id: u32) {
//         let span = info_span!("courier", id);
//         let _guard = span.enter();           // ✘ wrong across .await
//         yield_now().await;
//         info!("delivered");
//     }
//
//   For futures, attach the span to the FUTURE instead. It's entered
//   only while that future is being polled:
//
//     use tracing::Instrument;
//
//     async { ... }.instrument(info_span!("courier", id)).await
//
//   `#[instrument]` on an async fn does exactly this for you.
//
// 🎯 YOUR TASK
//
//   Two couriers run at the same time. Their logs are tangled: fix
//   `deliver` so every line has exactly its own courier span.

pub mod capture;

use tokio::task::yield_now;
use tracing::{Instrument, info, info_span};

pub async fn deliver(id: u32) {
    let span = info_span!("courier", id);
    let _guard = span.enter();
    info!("picked up parcel {id}");
    yield_now().await;
    info!("delivered parcel {id}");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::capture;

    #[test]
    fn untangled() {
        let log = capture(|| {
            let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
            rt.block_on(async {
                tokio::join!(deliver(1), deliver(2));
            });
        });
        let lines: Vec<&str> = log.lines().collect();
        assert_eq!(lines.len(), 4, "{log}");
        for line in lines {
            let id = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            assert!(line.contains(&format!("courier{{id={id}}}: ")), "wrong span: {line}");
            assert_eq!(line.matches("courier{").count(), 1, "tangled spans: {line}");
        }
    }
}
