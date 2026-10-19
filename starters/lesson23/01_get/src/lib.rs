// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 23-1 · A Letter from Afar                        175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `reqwest` is the go-to HTTP client. It's async (built on tokio):
//
//     let body = reqwest::get("http://example.com/motd")   // send GET
//         .await?                                            // → Response
//         .text()                                            // read the body...
//         .await?;                                           // ...also async!
//
//   Two awaits: one for the response HEADERS, one for the BODY. The
//   body may be large and arrive slowly, so reading it is a separate
//   async step.
//
//   For more than one request, create a `Client` once and reuse it. It
//   keeps connections open and shares settings:
//
//     let client = reqwest::Client::new();
//     let text = client.get(url).send().await?.text().await?;
//
//   Errors are `reqwest::Error`: connection refused, timeouts, bad
//   bodies...
//
//   (The tests run a tiny local server, mock.rs, so nothing touches the
//   internet.)
//
// 🎯 YOUR TASK
//
//   `fetch_motd` should GET `{base}/motd` and return the body text.
//   It forgets an `.await`. Then write `fetch_both` with one shared Client.

pub mod mock;

pub async fn fetch_motd(base: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(format!("{base}/motd")).await?;
    let body = response.text();
    Ok(body)
}

/// GET {base}/motd and {base}/news with ONE Client, joined by " | ".
pub async fn fetch_both(base: &str) -> Result<String, reqwest::Error> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockServer;

    async fn server() -> MockServer {
        MockServer::start(|req| match req.path.as_str() {
            "/motd" => (200, "Welcome, traveller".into()),
            "/news" => (200, "Dragon spotted".into()),
            _ => (404, "".into()),
        })
        .await
    }

    #[tokio::test]
    async fn motd() {
        let s = server().await;
        assert_eq!(fetch_motd(&s.url).await.unwrap(), "Welcome, traveller");
    }

    #[tokio::test]
    async fn both() {
        let s = server().await;
        assert_eq!(fetch_both(&s.url).await.unwrap(), "Welcome, traveller | Dragon spotted");
        assert_eq!(s.requests().len(), 2);
    }

    #[tokio::test]
    async fn nobody_home() {
        assert!(fetch_motd("http://127.0.0.1:9").await.is_err());
    }
}
