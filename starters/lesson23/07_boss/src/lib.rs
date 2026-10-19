// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Registry Hydra                            400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Guild Registry is a REST API with many heads. Write a client
//   library for it that a whole guild could rely on.
//
//   The API:
//     GET  /heroes                  → 200 [Hero, ...]
//     GET  /heroes?class=mage       → 200 only heroes of that class
//     GET  /heroes/{id}             → 200 Hero | 404
//     POST /heroes  {"name","class"}→ 201 Hero (with its new id) | 409 if the name exists
//   A Hero is {"id": 1, "name": "Ferris", "class": "crab", "level": 12}.
//
//   ⚔️  HEAD 1: `Registry::new(base)`: one reqwest::Client with a 2 s
//        timeout, kept in the struct.
//
//   ⚔️  HEAD 2: `list(class: Option<&str>)`: the `class` query
//        parameter only when given.
//
//   ⚔️  HEAD 3: `get(id)`: Ok(None) on 404; other non-2xx are errors.
//
//   ⚔️  HEAD 4: `create(name, class)`: POST JSON; on 409 return the error
//        "hero {name} already exists" (anyhow::bail!).
//
//   ⚔️  HEAD 5: `strongest()`: the hero with the highest level, using
//        `list`. Every error carries context: "listing heroes".

use anyhow::{Context, Result, bail};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub mod mock;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hero {
    pub id: u32,
    pub name: String,
    pub class: String,
    pub level: u32,
}

pub struct Registry {
    // your fields
}

impl Registry {
    pub fn new(base: &str) -> Result<Registry> {
        todo!()
    }

    pub async fn list(&self, class: Option<&str>) -> Result<Vec<Hero>> {
        todo!()
    }

    pub async fn get(&self, id: u32) -> Result<Option<Hero>> {
        todo!()
    }

    pub async fn create(&self, name: &str, class: &str) -> Result<Hero> {
        todo!()
    }

    pub async fn strongest(&self) -> Result<Option<Hero>> {
        todo!()
    }
}

// ─── The Hydra's tests: don't change these! ──────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockServer;
    use std::sync::{Arc, Mutex};

    fn hero(id: u32, name: &str, class: &str, level: u32) -> Hero {
        Hero { id, name: name.into(), class: class.into(), level }
    }

    async fn registry_server() -> MockServer {
        let db = Arc::new(Mutex::new(vec![
            hero(1, "Ferris", "crab", 12),
            hero(2, "Lumi", "mage", 30),
            hero(3, "Corro", "mage", 7),
        ]));
        MockServer::start(move |req| {
            let mut db = db.lock().unwrap();
            let (path, query) = req.path.split_once('?').unwrap_or((&req.path, ""));
            match (req.method.as_str(), path) {
                ("GET", "/heroes") => {
                    let class = query.strip_prefix("class=");
                    let list: Vec<&Hero> = db.iter().filter(|h| class.is_none_or(|c| h.class == c)).collect();
                    (200, serde_json::to_string(&list).unwrap())
                }
                ("GET", p) if p.starts_with("/heroes/") => {
                    let id: u32 = p["/heroes/".len()..].parse().unwrap_or(0);
                    if id == 500 {
                        return (500, "boom".into());
                    }
                    match db.iter().find(|h| h.id == id) {
                        Some(h) => (200, serde_json::to_string(h).unwrap()),
                        None => (404, "".into()),
                    }
                }
                ("POST", "/heroes") => {
                    let v: serde_json::Value = serde_json::from_str(&req.body).unwrap();
                    let name = v["name"].as_str().unwrap_or("").to_string();
                    if db.iter().any(|h| h.name == name) {
                        return (409, "".into());
                    }
                    let h = hero(db.len() as u32 + 1, &name, v["class"].as_str().unwrap_or(""), 1);
                    db.push(h.clone());
                    (201, serde_json::to_string(&h).unwrap())
                }
                _ => (404, "".into()),
            }
        })
        .await
    }

    #[tokio::test]
    async fn head_1_and_2_list() {
        let s = registry_server().await;
        let r = Registry::new(&s.url).unwrap();
        assert_eq!(r.list(None).await.unwrap().len(), 3);
        let mages: Vec<String> = r.list(Some("mage")).await.unwrap().into_iter().map(|h| h.name).collect();
        assert_eq!(mages, vec!["Lumi", "Corro"]);
        assert_eq!(s.requests()[0].path, "/heroes");
        assert_eq!(s.requests()[1].path, "/heroes?class=mage");
    }

    #[tokio::test]
    async fn head_3_get() {
        let s = registry_server().await;
        let r = Registry::new(&s.url).unwrap();
        assert_eq!(r.get(1).await.unwrap(), Some(hero(1, "Ferris", "crab", 12)));
        assert_eq!(r.get(99).await.unwrap(), None);
        assert!(r.get(500).await.is_err());
    }

    #[tokio::test]
    async fn head_4_create() {
        let s = registry_server().await;
        let r = Registry::new(&s.url).unwrap();
        assert_eq!(r.create("Ada", "bard").await.unwrap(), hero(4, "Ada", "bard", 1));
        assert_eq!(r.create("Ferris", "crab").await.unwrap_err().to_string(), "hero Ferris already exists");
    }

    #[tokio::test]
    async fn head_5_strongest() {
        let s = registry_server().await;
        let r = Registry::new(&s.url).unwrap();
        assert_eq!(r.strongest().await.unwrap().unwrap().name, "Lumi");
        let dead = Registry::new("http://127.0.0.1:9").unwrap();
        let err = dead.strongest().await.unwrap_err();
        assert!(format!("{err:#}").starts_with("listing heroes: "), "{err:#}");
    }
}
