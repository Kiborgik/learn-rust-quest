// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 23-6 · Patience and Persistence                  175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Networks are slow and flaky. Two habits every client needs:
//
//   TIMEOUTS: by default a request can wait forever. Set one on the
//   Client (applies to every request) or per request:
//
//     let client = reqwest::Client::builder()
//         .timeout(Duration::from_secs(5))
//         .build()?;
//
//     client.get(url).timeout(Duration::from_millis(500)).send().await
//
//     err.is_timeout()          // tell timeouts from other failures
//
//   RETRIES: some failures are temporary (503 Service Unavailable, a
//   dropped connection). Try again a few times, waiting a bit longer
//   each time ("backoff"):
//
//     for attempt in 1..=3 {
//         match try_once().await {
//             Ok(v) => return Ok(v),
//             Err(e) if attempt < 3 => sleep(Duration::from_millis(50 * attempt)).await,
//             Err(e) => return Err(e),
//         }
//     }
//
//   Only retry what's safe to repeat. A GET is fine. A POST that charges
//   a credit card is not!
//
// 🎯 YOUR TASK
//
//   1. `impatient_client`: a Client with a 300 ms timeout.
//   2. `fetch_with_retry`: GET the url; on a 503, wait 50 ms × attempt and
//      try again, up to `attempts` tries total. Return the body on 2xx.
//      Any other status, or 503 on the last try, is an error
//      (`error_for_status`).

pub mod mock;

use reqwest::StatusCode;
use std::time::Duration;
use tokio::time::sleep;

pub fn impatient_client() -> reqwest::Client {
    todo!()
}

pub async fn fetch_with_retry(client: &reqwest::Client, url: &str, attempts: u32) -> Result<String, reqwest::Error> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockServer;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn times_out() {
        let s = MockServer::start_with_delay(Duration::from_secs(3), |_| (200, "late".into())).await;
        let err = impatient_client().get(&s.url).send().await.unwrap_err();
        assert!(err.is_timeout(), "{err}");
    }

    #[tokio::test]
    async fn retries_until_it_works() {
        let calls = Arc::new(AtomicU32::new(0));
        let c2 = calls.clone();
        let s = MockServer::start(move |_| {
            if c2.fetch_add(1, Ordering::SeqCst) < 2 { (503, "busy".into()) } else { (200, "finally".into()) }
        })
        .await;
        let body = fetch_with_retry(&reqwest::Client::new(), &s.url, 5).await.unwrap();
        assert_eq!(body, "finally");
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn gives_up() {
        let s = MockServer::start(|_| (503, "busy".into())).await;
        let err = fetch_with_retry(&reqwest::Client::new(), &s.url, 2).await.unwrap_err();
        assert_eq!(err.status(), Some(StatusCode::SERVICE_UNAVAILABLE));
        assert_eq!(s.requests().len(), 2);
    }

    #[tokio::test]
    async fn no_retry_on_404() {
        let s = MockServer::start(|_| (404, "nope".into())).await;
        assert!(fetch_with_retry(&reqwest::Client::new(), &s.url, 5).await.is_err());
        assert_eq!(s.requests().len(), 1);
    }
}
