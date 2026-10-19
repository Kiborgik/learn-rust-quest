// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 26-3 · The Front Desk                            200 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The API is a THIN layer: parse the request, call the store, turn the
//   result into a response. No rules live here. They're in the domain.
//
//     async fn get_one(State(store): State<Store>, Path(id): Path<i64>)
//         -> Result<Json<Quest>, AppError>
//     {
//         Ok(Json(store.get(id).await?))
//     }
//
//   All the HTTP meaning of errors sits in ONE place, the IntoResponse
//   impl. Pick codes that tell the client what to do:
//
//     422 Unprocessable Entity   your input breaks a rule, fix it
//     404 Not Found              no such thing
//     409 Conflict               fine request, wrong moment (already done)
//     500 Internal Server Error  our fault, try later
//
//   ⚠️ Never send database error text to clients. It can leak table
//   names and paths. Send a plain "internal error" (and log the details,
//   lesson 22).
//
// 🎯 YOUR TASK
//
//   1. `into_response`: 422 for EmptyTitle/BadReward, 404 for NotFound,
//      409 for AlreadyDone, 500 for Db. Body: {"error": "<message>"}, and
//      the message for Db is just "internal error".
//   2. `router`:
//        GET  /health              → "ok"
//        GET  /quests?done=true    → [Quest]   (done is optional)
//        POST /quests              → 201 Quest  (body: NewQuest JSON)
//        GET  /quests/{id}         → Quest
//        POST /quests/{id}/finish  → Quest
//        GET  /summary             → Summary

use crate::domain::{BoardError, NewQuest, Quest, Summary};
use crate::store::{AppError, Store};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Filter {
    pub done: Option<bool>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        todo!()
    }
}

pub fn router(store: Store) -> Router {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{get, post_json, send};

    async fn app() -> Router {
        router(Store::open("sqlite::memory:").await.unwrap())
    }

    #[tokio::test]
    async fn health() {
        assert_eq!(get(&app().await, "/health").await.body, "ok");
    }

    #[tokio::test]
    async fn posting_and_reading() {
        let app = app().await;
        let r = post_json(&app, "/quests", r#"{"title":"Slay the slime","reward":50}"#).await;
        assert_eq!(r.status, StatusCode::CREATED);
        assert_eq!(r.json(), json!({"id": 1, "title": "Slay the slime", "reward": 50, "done": false}));
        post_json(&app, "/quests", r#"{"title":"Find the cat","reward":5}"#).await;
        assert_eq!(get(&app, "/quests/2").await.json()["title"], "Find the cat");
        assert_eq!(get(&app, "/quests").await.json().as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn finishing_and_summary() {
        let app = app().await;
        post_json(&app, "/quests", r#"{"title":"a","reward":10}"#).await;
        post_json(&app, "/quests", r#"{"title":"b","reward":20}"#).await;
        let r = send(&app, "POST", "/quests/1/finish", &[], "").await;
        assert_eq!(r.status, StatusCode::OK);
        assert_eq!(r.json()["done"], true);
        assert_eq!(get(&app, "/quests?done=false").await.json()[0]["title"], "b");
        assert_eq!(
            get(&app, "/summary").await.json(),
            json!({"open": 1, "done": 1, "gold_paid": 10, "gold_waiting": 20})
        );
    }

    #[tokio::test]
    async fn errors() {
        let app = app().await;
        let r = post_json(&app, "/quests", r#"{"title":" ","reward":10}"#).await;
        assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(r.json(), json!({"error": "title can't be empty"}));
        let r = post_json(&app, "/quests", r#"{"title":"x","reward":0}"#).await;
        assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);
        let r = get(&app, "/quests/7").await;
        assert_eq!(r.status, StatusCode::NOT_FOUND);
        assert_eq!(r.json(), json!({"error": "quest 7 not found"}));
        post_json(&app, "/quests", r#"{"title":"a","reward":10}"#).await;
        send(&app, "POST", "/quests/1/finish", &[], "").await;
        let r = send(&app, "POST", "/quests/1/finish", &[], "").await;
        assert_eq!(r.status, StatusCode::CONFLICT);
        assert_eq!(r.json(), json!({"error": "quest 1 is already done"}));
    }

    #[tokio::test]
    async fn database_errors_stay_secret() {
        let r = AppError::Db(sqlx::Error::Protocol("table quests at C:/secret/db".into())).into_response();
        assert_eq!(r.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = axum::body::to_bytes(r.into_body(), 1024).await.unwrap();
        assert_eq!(body, r#"{"error":"internal error"}"#);
    }
}
