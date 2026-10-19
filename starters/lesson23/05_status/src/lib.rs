// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 23-5 · Read the Seal                             175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   ⚠️ A 404 or 500 reply is NOT an error to reqwest. The request worked;
//   the server just said no. `send()` returns Ok, and you'd happily read
//   an error page as if it were data.
//
//   Check the status yourself:
//
//     let resp = client.get(url).send().await?;
//     resp.status()                          // StatusCode
//     resp.status().is_success()             // 2xx?
//     resp.status() == StatusCode::NOT_FOUND
//
//   Or turn every 4xx/5xx into an Err:
//
//     let resp = client.get(url).send().await?.error_for_status()?;
//
//   Often "not found" isn't an error for your program at all, just
//   "nothing there". Map it to None, and treat the other failures as
//   errors.
//
// 🎯 YOUR TASK
//
//   1. `scroll_text` returns the error page text on a 500. Make it an
//      error instead.
//   2. `find_hero`: Ok(Some(hero)) on 200, Ok(None) on 404, Err for any
//      other failure.

pub mod mock;

use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Hero {
    pub name: String,
    pub level: u32,
}

pub async fn scroll_text(client: &reqwest::Client, url: &str) -> Result<String, reqwest::Error> {
    client.get(url).send().await?.text().await
}

pub async fn find_hero(client: &reqwest::Client, base: &str, id: u32) -> Result<Option<Hero>, reqwest::Error> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockServer;

    async fn server() -> MockServer {
        MockServer::start(|req| match req.path.as_str() {
            "/scroll" => (200, "ancient words".into()),
            "/cursed" => (500, "internal curse".into()),
            "/heroes/1" => (200, r#"{"name":"Ferris","level":12}"#.into()),
            "/heroes/666" => (503, "the registry is on fire".into()),
            _ => (404, "no such page".into()),
        })
        .await
    }

    #[tokio::test]
    async fn scrolls() {
        let s = server().await;
        let c = reqwest::Client::new();
        assert_eq!(scroll_text(&c, &format!("{}/scroll", s.url)).await.unwrap(), "ancient words");
        let err = scroll_text(&c, &format!("{}/cursed", s.url)).await.unwrap_err();
        assert_eq!(err.status(), Some(StatusCode::INTERNAL_SERVER_ERROR));
    }

    #[tokio::test]
    async fn heroes() {
        let s = server().await;
        let c = reqwest::Client::new();
        assert_eq!(
            find_hero(&c, &s.url, 1).await.unwrap(),
            Some(Hero { name: "Ferris".into(), level: 12 })
        );
        assert_eq!(find_hero(&c, &s.url, 2).await.unwrap(), None);
        assert!(find_hero(&c, &s.url, 666).await.is_err());
    }
}
