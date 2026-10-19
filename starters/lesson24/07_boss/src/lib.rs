// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Quest Board Colossus                      400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The guild wants its quest board online. Build the whole REST API.
//
//     GET    /quests              → 200 [Quest, ...]   (oldest first)
//     GET    /quests?done=true    → 200 only finished (false: only open)
//     POST   /quests {"title"}    → 201 Quest           (title trimmed)
//     GET    /quests/{id}         → 200 Quest | 404
//     PATCH  /quests/{id}/done    → 200 Quest (now done) | 404
//     DELETE /quests/{id}         → 204 (empty body) | 404
//
//   A Quest is {"id": 1, "title": "Slay the slime", "done": false}.
//
//   ⚔️  HEAD 1: shared state. Ids start at 1 and are never reused, even
//        after a delete.
//   ⚔️  HEAD 2: every route above, with the right status codes.
//   ⚔️  HEAD 3: errors as JSON through `ApiError`:
//          NotFound(id) → 404 {"error": "quest {id} not found"}
//          EmptyTitle   → 400 {"error": "title is empty"}
//   ⚔️  HEAD 4: `app()` builds a fresh board every time it's called.
//
//   Everything you need is in this lesson's earlier quests.

#[cfg(test)]
mod testkit;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, patch};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quest {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct NewQuest {
    pub title: String,
}

#[derive(Deserialize)]
pub struct Filter {
    pub done: Option<bool>,
}

pub enum ApiError {
    NotFound(u32),
    EmptyTitle,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Clone, Default)]
pub struct Board {
    // your fields
}

// your handlers

pub fn app() -> Router {
    todo!()
}

// ─── The Colossus's tests: don't change these! ───────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{get, post_json, send};

    async fn add(app: &Router, title: &str) -> testkit::Reply {
        post_json(app, "/quests", &json!({ "title": title }).to_string()).await
    }

    #[tokio::test]
    async fn head_1_and_2_create_and_list() {
        let app = app();
        let r = add(&app, "  Slay the slime  ").await;
        assert_eq!(r.status, StatusCode::CREATED);
        assert_eq!(r.json(), json!({"id": 1, "title": "Slay the slime", "done": false}));
        add(&app, "Find the cat").await;
        let all = get(&app, "/quests").await.json();
        assert_eq!(all[0]["title"], "Slay the slime");
        assert_eq!(all[1]["id"], 2);
        assert_eq!(all.as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn head_2_get_finish_and_filter() {
        let app = app();
        add(&app, "a").await;
        add(&app, "b").await;
        assert_eq!(get(&app, "/quests/2").await.json()["title"], "b");
        let r = send(&app, "PATCH", "/quests/2/done", &[], "").await;
        assert_eq!(r.status, StatusCode::OK);
        assert_eq!(r.json(), json!({"id": 2, "title": "b", "done": true}));
        assert_eq!(get(&app, "/quests?done=true").await.json(), json!([{"id": 2, "title": "b", "done": true}]));
        assert_eq!(get(&app, "/quests?done=false").await.json(), json!([{"id": 1, "title": "a", "done": false}]));
    }

    #[tokio::test]
    async fn head_1_and_2_delete() {
        let app = app();
        for t in ["a", "b", "c"] {
            add(&app, t).await;
        }
        let r = send(&app, "DELETE", "/quests/3", &[], "").await;
        assert_eq!(r.status, StatusCode::NO_CONTENT);
        assert_eq!(r.body, "");
        assert_eq!(add(&app, "d").await.json()["id"], 4);
        assert_eq!(get(&app, "/quests").await.json().as_array().unwrap().len(), 3);
    }

    #[tokio::test]
    async fn head_3_errors() {
        let app = app();
        let r = get(&app, "/quests/9").await;
        assert_eq!(r.status, StatusCode::NOT_FOUND);
        assert_eq!(r.json(), json!({"error": "quest 9 not found"}));
        assert_eq!(send(&app, "PATCH", "/quests/9/done", &[], "").await.status, StatusCode::NOT_FOUND);
        assert_eq!(send(&app, "DELETE", "/quests/9", &[], "").await.status, StatusCode::NOT_FOUND);
        let r = add(&app, "   ").await;
        assert_eq!(r.status, StatusCode::BAD_REQUEST);
        assert_eq!(r.json(), json!({"error": "title is empty"}));
    }

    #[tokio::test]
    async fn head_4_fresh_boards() {
        add(&app(), "lost").await;
        assert_eq!(get(&app(), "/quests").await.json(), json!([]));
    }
}
