//! Client: typed access to the board over HTTP. (Given: you wrote this in 26-4.)

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
    http: reqwest::Client,
    base: String,
}

impl GuildClient {
    pub fn new(base: &str) -> GuildClient {
        let http = reqwest::Client::builder().timeout(Duration::from_secs(5)).build().expect("http client");
        GuildClient { http, base: base.trim_end_matches('/').to_string() }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base)
    }

    pub async fn list(&self, done: Option<bool>) -> Result<Vec<Quest>, ClientError> {
        let mut req = self.http.get(self.url("/quests"));
        if let Some(done) = done {
            req = req.query(&[("done", done)]);
        }
        Ok(check(req.send().await?).await?.json().await?)
    }

    pub async fn add(&self, title: &str, reward: i64) -> Result<Quest, ClientError> {
        let new = NewQuest { title: title.to_string(), reward };
        let resp = self.http.post(self.url("/quests")).json(&new).send().await?;
        Ok(check(resp).await?.json().await?)
    }

    pub async fn get(&self, id: i64) -> Result<Quest, ClientError> {
        let resp = self.http.get(self.url(&format!("/quests/{id}"))).send().await?;
        Ok(check(resp).await?.json().await?)
    }

    pub async fn finish(&self, id: i64) -> Result<Quest, ClientError> {
        let resp = self.http.post(self.url(&format!("/quests/{id}/finish"))).send().await?;
        Ok(check(resp).await?.json().await?)
    }

    pub async fn summary(&self) -> Result<Summary, ClientError> {
        let resp = self.http.get(self.url("/summary")).send().await?;
        Ok(check(resp).await?.json().await?)
    }
}
