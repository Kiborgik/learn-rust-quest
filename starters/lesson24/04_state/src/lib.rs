// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 24-4 · The Shared Ledger                         175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Handlers run on many tasks at once, so shared data needs the tools
//   from lesson 12: Arc to share, Mutex to change.
//
//     #[derive(Clone, Default)]
//     struct AppState { visits: Arc<Mutex<u32>> }   // Clone = cheap Arc copy
//
//     async fn visit(State(state): State<AppState>) -> String {
//         let mut n = state.visits.lock().unwrap();
//         *n += 1;
//         format!("visit #{n}")
//     }
//
//     Router::new()
//         .route("/visit", get(visit))
//         .with_state(AppState::default())    // ← hand the state in
//
//   A Router that still needs state has the type `Router<AppState>`.
//   `.with_state(...)` gives you back a plain, ready `Router`.
//
//   ⚠️ Don't hold a std Mutex lock across an `.await`. Lock, copy or
//   change what you need, and let the guard drop. (tokio's Mutex can be
//   held across awaits, but you rarely need it.)
//
// 🎯 YOUR TASK
//
//   1. It doesn't compile: the state is never handed to the Router.
//   2. Write `count`: GET /heroes/count → how many heroes, as text.

#[cfg(test)]
mod testkit;

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct AppState {
    heroes: Arc<Mutex<Vec<String>>>,
}

async fn list(State(state): State<AppState>) -> Json<Vec<String>> {
    Json(state.heroes.lock().unwrap().clone())
}

/// POST /heroes with the name as a plain text body.
async fn add(State(state): State<AppState>, name: String) -> StatusCode {
    state.heroes.lock().unwrap().push(name);
    StatusCode::CREATED
}

async fn count(State(state): State<AppState>) -> String {
    todo!()
}

pub fn app() -> Router {
    Router::new().route("/heroes", get(list).post(add)).route("/heroes/count", get(count))
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{get, send};
    use serde_json::json;

    #[tokio::test]
    async fn ledger() {
        let app = app();
        assert_eq!(get(&app, "/heroes/count").await.body, "0");
        for name in ["Ferris", "Lumi"] {
            assert_eq!(send(&app, "POST", "/heroes", &[], name).await.status, StatusCode::CREATED);
        }
        assert_eq!(get(&app, "/heroes").await.json(), json!(["Ferris", "Lumi"]));
        assert_eq!(get(&app, "/heroes/count").await.body, "2");
    }

    #[tokio::test]
    async fn each_app_has_its_own_ledger() {
        let a = app();
        send(&a, "POST", "/heroes", &[], "Ferris").await;
        assert_eq!(get(&app(), "/heroes/count").await.body, "0");
    }
}
