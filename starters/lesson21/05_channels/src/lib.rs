// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 21-5 · Carrier Pigeons                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   tokio's channels are like std's (lesson 12), but `.await` instead of
//   blocking:
//
//     use tokio::sync::{mpsc, oneshot};
//
//     let (tx, mut rx) = mpsc::channel(8);     // BOUNDED: send waits when full
//     tokio::spawn(async move { tx.send(42).await.unwrap(); });
//     while let Some(msg) = rx.recv().await {  // None once all senders drop
//         ...
//     }
//
//   The bound is backpressure: a fast producer can't flood memory.
//
//   `oneshot` carries exactly one value, the usual way to get a reply:
//
//     let (reply_tx, reply_rx) = oneshot::channel();
//     tokio::spawn(async move { reply_tx.send(compute()).unwrap(); });
//     let answer = reply_rx.await.unwrap();
//
//   A common pattern ("actor"): one task owns some state and serves
//   requests from an mpsc channel, each request carrying a oneshot
//   sender for the answer.
//
// 🎯 YOUR TASK
//
//   1. `gather`: spawn `n` pigeon tasks; pigeon `i` (1..=n) sends i * 10
//      through a bounded mpsc channel. Return the sum of everything
//      received.
//   2. `ask_keeper`: spawn a task that computes `question.len()` and
//      sends it back over a oneshot channel. Return the answer.

use tokio::sync::{mpsc, oneshot};

pub async fn gather(n: u32) -> u32 {
    todo!()
}

pub async fn ask_keeper(question: String) -> usize {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn pigeons() {
        assert_eq!(gather(4).await, 100);
        assert_eq!(gather(0).await, 0);
        assert_eq!(gather(100).await, 50_500);
    }

    #[tokio::test]
    async fn keeper() {
        assert_eq!(ask_keeper("where is the key?".into()).await, 17);
    }
}
