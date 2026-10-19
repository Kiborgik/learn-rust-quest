// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 24-1 · Open the Gates                            175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `axum` is a web framework built on tokio. A server is a `Router`
//   that maps (method, path) to a HANDLER, which is just an async fn:
//
//     use axum::{Router, routing::get};
//
//     async fn hello() -> &'static str { "hi" }
//
//     let app = Router::new()
//         .route("/", get(hello))              // GET /
//         .route("/items", get(list).post(add)) // two methods, one path
//         .fallback(not_found);                 // anything else
//
//   A handler returns anything that implements `IntoResponse`: a &str or
//   String (200, text), a StatusCode, or a tuple that adds to it:
//
//     async fn not_found() -> (StatusCode, &'static str) {
//         (StatusCode::NOT_FOUND, "nothing here")
//     }
//
//   To serve it for real (not needed in this lesson):
//
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
//     axum::serve(listener, app).await?;
//
//   The tests skip the network: they call the Router directly (see
//   testkit.rs). A wrong method gets 405, an unknown path gets 404.
//
// 🎯 YOUR TASK
//
//   1. GET /health must answer "ok". Right now the route is wrong.
//   2. Write `not_found` (404, body "no such room") and make it the
//      fallback for every unknown path.

#[cfg(test)]
mod testkit;

use axum::Router;
use axum::http::StatusCode;
use axum::routing::{get, post};

async fn hall() -> &'static str {
    "Welcome to the guild hall"
}

async fn health() -> &'static str {
    "ok"
}

pub fn app() -> Router {
    Router::new().route("/", get(hall)).route("/health", post(health))
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{get, send};

    #[tokio::test]
    async fn the_hall() {
        let r = get(&app(), "/").await;
        assert_eq!(r.status, StatusCode::OK);
        assert_eq!(r.body, "Welcome to the guild hall");
    }

    #[tokio::test]
    async fn health_check() {
        let r = get(&app(), "/health").await;
        assert_eq!(r.status, StatusCode::OK);
        assert_eq!(r.body, "ok");
    }

    #[tokio::test]
    async fn wrong_method() {
        let r = send(&app(), "DELETE", "/", &[], "").await;
        assert_eq!(r.status, StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn lost_visitor() {
        let r = get(&app(), "/dungeon").await;
        assert_eq!(r.status, StatusCode::NOT_FOUND);
        assert_eq!(r.body, "no such room");
    }
}
