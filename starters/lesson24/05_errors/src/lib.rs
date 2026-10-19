// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 24-5 · Polite Refusals                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Handlers can return `Result<T, E>` when both T and E implement
//   `IntoResponse`. Make one error type for your API and teach it how to
//   become a response, once:
//
//     enum ApiError { NotFound, Forbidden }
//
//     impl IntoResponse for ApiError {
//         fn into_response(self) -> Response {
//             let (status, msg) = match self {
//                 ApiError::NotFound => (StatusCode::NOT_FOUND, "not found"),
//                 ApiError::Forbidden => (StatusCode::FORBIDDEN, "go away"),
//             };
//             (status, Json(json!({ "error": msg }))).into_response()
//         }
//     }
//
//     async fn hero(Path(id): Path<u32>) -> Result<Json<Hero>, ApiError> {
//         let hero = find(id).ok_or(ApiError::NotFound)?;     // ? works here
//         Ok(Json(hero))
//     }
//
//   Every handler now fails the same way, with a clear status and a JSON
//   body clients can read.
//
// 🎯 YOUR TASK
//
//   `into_response` turns every error into a vague 500. Map them:
//     NotFound(id)   → 404  {"error": "hero {id} not found"}
//     BadLevel(lvl)  → 400  {"error": "level {lvl} is not in 1..=100"}

#[cfg(test)]
mod testkit;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, put};
use axum::{Json, Router};
use serde::Serialize;
use serde_json::json;

#[derive(Clone, Serialize)]
pub struct Hero {
    id: u32,
    name: &'static str,
    level: u32,
}

const HEROES: [Hero; 2] = [Hero { id: 1, name: "Ferris", level: 12 }, Hero { id: 2, name: "Lumi", level: 30 }];

pub enum ApiError {
    NotFound(u32),
    BadLevel(u32),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
    }
}

fn find(id: u32) -> Result<Hero, ApiError> {
    HEROES.iter().find(|h| h.id == id).cloned().ok_or(ApiError::NotFound(id))
}

async fn hero(Path(id): Path<u32>) -> Result<Json<Hero>, ApiError> {
    Ok(Json(find(id)?))
}

/// PUT /heroes/{id}/level/{level}
async fn set_level(Path((id, level)): Path<(u32, u32)>) -> Result<Json<Hero>, ApiError> {
    let mut hero = find(id)?;
    if !(1..=100).contains(&level) {
        return Err(ApiError::BadLevel(level));
    }
    hero.level = level;
    Ok(Json(hero))
}

pub fn app() -> Router {
    Router::new().route("/heroes/{id}", get(hero)).route("/heroes/{id}/level/{level}", put(set_level))
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{get, send};

    #[tokio::test]
    async fn found() {
        let r = get(&app(), "/heroes/1").await;
        assert_eq!(r.status, StatusCode::OK);
        assert_eq!(r.json(), json!({"id": 1, "name": "Ferris", "level": 12}));
        let r = send(&app(), "PUT", "/heroes/2/level/31", &[], "").await;
        assert_eq!(r.json()["level"], 31);
    }

    #[tokio::test]
    async fn not_found() {
        let r = get(&app(), "/heroes/9").await;
        assert_eq!(r.status, StatusCode::NOT_FOUND);
        assert_eq!(r.json(), json!({"error": "hero 9 not found"}));
        let r = send(&app(), "PUT", "/heroes/9/level/5", &[], "").await;
        assert_eq!(r.status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn bad_level() {
        let r = send(&app(), "PUT", "/heroes/1/level/500", &[], "").await;
        assert_eq!(r.status, StatusCode::BAD_REQUEST);
        assert_eq!(r.json(), json!({"error": "level 500 is not in 1..=100"}));
    }
}
