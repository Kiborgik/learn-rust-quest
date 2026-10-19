// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 24-2 · Doors and Signs                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Handlers get request data through EXTRACTORS: arguments whose types
//   say what to pull out of the request. axum fills them in for you.
//
//     use axum::extract::{Path, Query};
//
//     // GET /heroes/7
//     .route("/heroes/{id}", get(hero))
//     async fn hero(Path(id): Path<u32>) -> String { format!("#{id}") }
//
//     // several captures: /heroes/{id}/items/{slot}  →  Path<(u32, String)>
//
//     // GET /search?class=mage&page=2
//     #[derive(Deserialize)]
//     struct Search { class: String, page: Option<u32> }   // Option = may be missing
//     async fn search(Query(s): Query<Search>) -> String { ... }
//
//   `Path(id): Path<u32>` is a pattern in the argument list: it unpacks
//   the wrapper right away.
//
//   If the data doesn't fit the type (/heroes/abc for a u32), axum
//   answers 400 for you and your handler never runs.
//
//   ⚠️ axum 0.8 writes captures as `{id}`. Older versions used `:id`, and
//   you'll still see that in old blog posts.
//
// 🎯 YOUR TASK
//
//   1. The app panics on start. Read the message and fix the route.
//   2. GET /greet?name=Ferris → "Hello, Ferris!". With no name:
//      "Hello, stranger!"

#[cfg(test)]
mod testkit;

use axum::Router;
use axum::extract::{Path, Query};
use axum::routing::get;
use serde::Deserialize;

async fn hero(Path(id): Path<u32>) -> String {
    format!("hero #{id}")
}

#[derive(Deserialize)]
pub struct Greet {
    name: Option<String>,
}

async fn greet(Query(q): Query<Greet>) -> String {
    todo!()
}

pub fn app() -> Router {
    Router::new().route("/heroes/:id", get(hero)).route("/greet", get(greet))
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::get;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn path() {
        let r = get(&app(), "/heroes/7").await;
        assert_eq!(r.status, StatusCode::OK);
        assert_eq!(r.body, "hero #7");
    }

    #[tokio::test]
    async fn bad_path_is_400() {
        let r = get(&app(), "/heroes/seven").await;
        assert_eq!(r.status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn query() {
        assert_eq!(get(&app(), "/greet?name=Ferris").await.body, "Hello, Ferris!");
        assert_eq!(get(&app(), "/greet?name=dark%20elf").await.body, "Hello, dark elf!");
        assert_eq!(get(&app(), "/greet").await.body, "Hello, stranger!");
    }
}
