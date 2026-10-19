// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 23-4 · Send a Raven                              175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Sending JSON: `.json(&value)` serializes it and sets
//   `content-type: application/json`:
//
//     #[derive(Serialize)]
//     struct NewHero<'a> { name: &'a str, class: &'a str }
//
//     let created: Created = client
//         .post(format!("{base}/heroes"))
//         .json(&NewHero { name: "Ferris", class: "crab" })
//         .send().await?
//         .json().await?;
//
//   Other verbs work the same: `.put()`, `.patch()`, `.delete()`.
//   For HTML forms there's `.form(&pairs)`.
//
// 🎯 YOUR TASK
//
//   `register` must POST a `NewHero` as JSON to {base}/heroes and return
//   the `id` from the JSON reply `{"id": 7}`.

pub mod mock;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct NewHero {
    pub name: String,
    pub level: u32,
}

#[derive(Deserialize)]
struct Created {
    id: u32,
}

pub async fn register(client: &reqwest::Client, base: &str, hero: &NewHero) -> Result<u32, reqwest::Error> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockServer;

    #[tokio::test]
    async fn registering() {
        let s = MockServer::start(|req| {
            let v: serde_json::Value = serde_json::from_str(&req.body).unwrap_or_default();
            if req.method == "POST" && v["name"] == "Ferris" && v["level"] == 3 {
                (201, r#"{"id": 7}"#.into())
            } else {
                (400, r#"{"error": "bad hero"}"#.into())
            }
        })
        .await;
        let c = reqwest::Client::new();
        let id = register(&c, &s.url, &NewHero { name: "Ferris".into(), level: 3 }).await.unwrap();
        assert_eq!(id, 7);
        let req = &s.requests()[0];
        assert_eq!(req.path, "/heroes");
        assert_eq!(req.headers["content-type"], "application/json");
    }
}
