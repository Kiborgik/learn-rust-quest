// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 24-3 · Scrolls In, Scrolls Out                   175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `Json<T>` works both ways:
//
//     // as an extractor: parse the request body into T (T: Deserialize)
//     async fn create(Json(new): Json<NewHero>) -> ... { ... }
//
//     // as a response: serialize T (T: Serialize), content-type: application/json
//     Json(hero)
//
//   Bad input is answered before your code runs:
//     no `content-type: application/json`   → 415 Unsupported Media Type
//     not valid JSON                         → 400 Bad Request
//     valid JSON, wrong shape (missing field) → 422 Unprocessable Entity
//
//   A tuple response sets more than the body. Status first, then headers,
//   then the body:
//
//     (StatusCode::CREATED, [(header::LOCATION, "/heroes/1")], Json(hero))
//
//   The easy return type for a tuple like that: `-> impl IntoResponse`
//   (the trait from 24-1, imported from `axum::response`).
//
//   ⚠️ An extractor that reads the BODY (Json, String) must be the LAST
//   argument. The body can only be read once.
//
// 🎯 YOUR TASK
//
//   POST /heroes should answer 201 Created with a `location: /heroes/1`
//   header and the new hero as JSON. Right now it says 200 and no header.

#[cfg(test)]
mod testkit;

use axum::http::{StatusCode, header};
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewHero {
    name: String,
    class: String,
}

#[derive(Serialize)]
pub struct Hero {
    id: u32,
    name: String,
    class: String,
    level: u32,
}

async fn create(Json(new): Json<NewHero>) -> Json<Hero> {
    Json(Hero { id: 1, name: new.name, class: new.class, level: 1 })
}

pub fn app() -> Router {
    Router::new().route("/heroes", post(create))
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{post_json, send};
    use serde_json::json;

    #[tokio::test]
    async fn created() {
        let r = post_json(&app(), "/heroes", r#"{"name":"Ferris","class":"crab"}"#).await;
        assert_eq!(r.status, StatusCode::CREATED);
        assert_eq!(r.headers["location"], "/heroes/1");
        assert_eq!(r.headers["content-type"], "application/json");
        assert_eq!(r.json(), json!({"id": 1, "name": "Ferris", "class": "crab", "level": 1}));
    }

    #[tokio::test]
    async fn axum_guards_the_door() {
        let app = app();
        let r = post_json(&app, "/heroes", r#"{"name":"Ferris"}"#).await;
        assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);
        let r = post_json(&app, "/heroes", "{oops").await;
        assert_eq!(r.status, StatusCode::BAD_REQUEST);
        let r = send(&app, "POST", "/heroes", &[], r#"{"name":"Ferris","class":"crab"}"#).await;
        assert_eq!(r.status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }
}
