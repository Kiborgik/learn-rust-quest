// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 26-4 · The Courier                               200 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The CLI (next quest) shouldn't know about URLs and status codes. It
//   wants `client.finish(3).await?`. So wrap the API in a typed client.
//
//   The domain types already derive Serialize + Deserialize, so the
//   client and the server share them. Change a field once and both sides
//   follow. (In bigger projects these types live in a shared crate.)
//
//   Turn API errors back into something useful. The server sends
//   {"error": "..."}; keep that message and the status:
//
//     async fn check(resp: reqwest::Response) -> Result<reqwest::Response, ClientError> {
//         if resp.status().is_success() {
//             return Ok(resp);
//         }
//         let status = resp.status().as_u16();
//         let message = match resp.json::<ErrorBody>().await {
//             Ok(body) => body.error,
//             Err(_) => "unknown error".to_string(),
//         };
//         Err(ClientError::Api { status, message })
//     }
//
// 🎯 YOUR TASK
//
//   Write GuildClient: `new` (one reqwest Client with a 5 s timeout) and
//   list / add / get / finish / summary, one per API route.
//   `check` is given.
//
//   The tests run the REAL server from the last quest on a local port.

use crate::domain::{NewQuest, Quest, Summary};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("{message} (HTTP {status})")]
    Api { status: u16, message: String },
    #[error("can't reach the board: {0}")]
    Http(#[from] reqwest::Error),
}

#[derive(Deserialize)]
struct ErrorBody {
    error: String,
}

async fn check(resp: reqwest::Response) -> Result<reqwest::Response, ClientError> {
    if resp.status().is_success() {
        return Ok(resp);
    }
    let status = resp.status().as_u16();
    let message = match resp.json::<ErrorBody>().await {
        Ok(body) => body.error,
        Err(_) => "unknown error".to_string(),
    };
    Err(ClientError::Api { status, message })
}

pub struct GuildClient {
    // your fields
}

impl GuildClient {
    pub fn new(base: &str) -> GuildClient {
        todo!()
    }

    pub async fn list(&self, done: Option<bool>) -> Result<Vec<Quest>, ClientError> {
        todo!()
    }

    pub async fn add(&self, title: &str, reward: i64) -> Result<Quest, ClientError> {
        todo!()
    }

    pub async fn get(&self, id: i64) -> Result<Quest, ClientError> {
        todo!()
    }

    pub async fn finish(&self, id: i64) -> Result<Quest, ClientError> {
        todo!()
    }

    pub async fn summary(&self) -> Result<Summary, ClientError> {
        todo!()
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::spawn_board;

    #[tokio::test]
    async fn round_trip() {
        let c = GuildClient::new(&spawn_board().await);
        let q = c.add("Slay the slime", 50).await.unwrap();
        assert_eq!(q, Quest { id: 1, title: "Slay the slime".into(), reward: 50, done: false });
        c.add("Find the cat", 5).await.unwrap();
        assert_eq!(c.get(2).await.unwrap().title, "Find the cat");
        assert!(c.finish(1).await.unwrap().done);
        assert_eq!(c.list(None).await.unwrap().len(), 2);
        assert_eq!(c.list(Some(true)).await.unwrap()[0].id, 1);
        assert_eq!(c.list(Some(false)).await.unwrap()[0].id, 2);
        let s = c.summary().await.unwrap();
        assert_eq!((s.open, s.done, s.gold_paid), (1, 1, 50));
    }

    #[tokio::test]
    async fn api_errors_come_back_typed() {
        let c = GuildClient::new(&spawn_board().await);
        let err = c.get(9).await.unwrap_err();
        assert!(matches!(&err, ClientError::Api { status: 404, .. }), "{err:?}");
        assert_eq!(err.to_string(), "quest 9 not found (HTTP 404)");
        let err = c.add("x", 0).await.unwrap_err();
        assert_eq!(err.to_string(), "reward must be 1..=1000 gold, got 0 (HTTP 422)");
    }

    #[tokio::test]
    async fn nobody_home() {
        let c = GuildClient::new("http://127.0.0.1:9");
        assert!(matches!(c.summary().await, Err(ClientError::Http(_))));
    }
}
