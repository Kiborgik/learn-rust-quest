// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 23-3 · The Right Questions                       175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The request builder adds everything before you `.send()`:
//
//     client
//         .get(format!("{base}/heroes"))
//         .query(&[("class", "mage"), ("min_level", "5")])   // ?class=mage&min_level=5
//         .header("x-guild-token", token)                    // a custom header
//         .bearer_auth(api_key)                              // Authorization: Bearer ...
//         .send()
//         .await?;
//
//   `.query()` takes anything serde can turn into key/value pairs, and
//   handles URL escaping ("dark elf" → dark+elf). Don't build query
//   strings by hand with format!.
//
// 🎯 YOUR TASK
//
//   `search` must GET {base}/heroes with the query parameters `class` and
//   `min_level`, and the header `x-guild-token`. Return the body text.

pub mod mock;

pub async fn search(
    client: &reqwest::Client,
    base: &str,
    token: &str,
    class: &str,
    min_level: u32,
) -> Result<String, reqwest::Error> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockServer;

    #[tokio::test]
    async fn searching() {
        let s = MockServer::start(|req| {
            if req.headers.get("x-guild-token").map(String::as_str) == Some("s3cret") {
                (200, "found 2 heroes".into())
            } else {
                (401, "who are you?".into())
            }
        })
        .await;
        let c = reqwest::Client::new();
        let body = search(&c, &s.url, "s3cret", "dark elf", 5).await.unwrap();
        assert_eq!(body, "found 2 heroes");
        let req = &s.requests()[0];
        assert_eq!(req.method, "GET");
        assert_eq!(req.path, "/heroes?class=dark+elf&min_level=5");
    }
}
