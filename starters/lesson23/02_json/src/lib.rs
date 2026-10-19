// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 23-2 · Typed Replies                             175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Most APIs speak JSON. With the `json` feature, reqwest uses serde
//   (lesson 15) to turn the body straight into your type:
//
//     #[derive(Deserialize)]
//     struct Hero { name: String, level: u32 }
//
//     let hero: Hero = client.get(url).send().await?.json().await?;
//     let all = client.get(url).send().await?.json::<Vec<Hero>>().await?;
//
//   If the JSON doesn't match the type (missing field, wrong type), you
//   get a reqwest::Error describing the decoding problem: no silent junk.
//
// 🎯 YOUR TASK
//
//   1. Make `Hero` deserializable.
//   2. Implement `party`: GET {base}/party (a JSON array of heroes) and
//      return the names of heroes with level >= `min_level`.

pub mod mock;

use serde::Deserialize;

#[derive(Debug, PartialEq)]
pub struct Hero {
    pub name: String,
    pub level: u32,
}

pub async fn hero(client: &reqwest::Client, base: &str) -> Result<Hero, reqwest::Error> {
    client.get(format!("{base}/hero")).send().await?.json().await
}

pub async fn party(client: &reqwest::Client, base: &str, min_level: u32) -> Result<Vec<String>, reqwest::Error> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockServer;

    async fn server() -> MockServer {
        MockServer::start(|req| match req.path.as_str() {
            "/hero" => (200, r#"{"name":"Ferris","level":12,"class":"crab"}"#.into()),
            "/party" => (
                200,
                r#"[{"name":"Ferris","level":12},{"name":"Corro","level":3},{"name":"Lumi","level":7}]"#.into(),
            ),
            "/broken" => (200, r#"{"name":"Ferris"}"#.into()),
            _ => (404, "".into()),
        })
        .await
    }

    #[tokio::test]
    async fn one_hero() {
        let s = server().await;
        let c = reqwest::Client::new();
        assert_eq!(hero(&c, &s.url).await.unwrap(), Hero { name: "Ferris".into(), level: 12 });
    }

    #[tokio::test]
    async fn the_party() {
        let s = server().await;
        let c = reqwest::Client::new();
        assert_eq!(party(&c, &s.url, 5).await.unwrap(), vec!["Ferris", "Lumi"]);
    }

    #[tokio::test]
    async fn bad_json_is_an_error() {
        let s = server().await;
        let c = reqwest::Client::new();
        let r: Result<Hero, _> = c.get(format!("{}/broken", s.url)).send().await.unwrap().json().await;
        assert!(r.is_err());
    }
}
