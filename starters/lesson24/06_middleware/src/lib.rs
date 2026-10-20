// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 24-6 · The Gatekeeper                            175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   MIDDLEWARE runs around your handlers: before (check auth, log) and
//   after (add headers, time it). The simplest kind is an async fn:
//
//     use axum::{extract::Request, middleware::{self, Next}, response::Response};
//
//     async fn timer(req: Request, next: Next) -> Response {
//         let start = Instant::now();
//         let res = next.run(req).await;        // call the inner handler
//         println!("took {:?}", start.elapsed());
//         res
//     }
//
//   The Response can be changed on its way out, for example a header:
//
//     let mut res = next.run(req).await;
//     res.headers_mut().insert("x-hi", HeaderValue::from_static("hi"));
//
//   Return early to stop the request (here `Result<Response, StatusCode>`;
//   the Err becomes the response):
//
//     async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
//         if !allowed(&req) { return Err(StatusCode::UNAUTHORIZED); }
//         Ok(next.run(req).await)
//     }
//
//   `.layer(middleware::from_fn(auth))` wraps the routes of the Router
//   it's called on. Split the app with `.nest` to protect only a part:
//
//     let admin = Router::new().route("/stats", get(stats)).layer(middleware::from_fn(auth));
//     let app = Router::new().route("/", get(home)).nest("/admin", admin);
//     // GET /admin/stats needs auth, GET / doesn't
//
// 🎯 YOUR TASK
//
//   1. The guard locks the whole hall, so nobody can even enter. Only
//      /vault/* should need the `x-guild-token: s3cret` header.
//   2. Write `stamp`: add the header `x-guild: open` to EVERY response
//      (401s too), and apply it to the whole app.

#[cfg(test)]
mod testkit;

use axum::Router;
use axum::extract::Request;
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::routing::get;

async fn hall() -> &'static str {
    "Welcome"
}

async fn gold() -> &'static str {
    "1000 gold"
}

async fn require_token(req: Request, next: Next) -> Result<Response, StatusCode> {
    match req.headers().get("x-guild-token") {
        Some(token) if token == "s3cret" => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn stamp(req: Request, next: Next) -> Response {
    todo!()
}

pub fn app() -> Router {
    let vault = Router::new().route("/gold", get(gold));
    Router::new().route("/", get(hall)).nest("/vault", vault).layer(middleware::from_fn(require_token))
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{get, send};

    #[tokio::test]
    async fn the_hall_is_open() {
        let r = get(&app(), "/").await;
        assert_eq!(r.status, StatusCode::OK);
        assert_eq!(r.body, "Welcome");
    }

    #[tokio::test]
    async fn the_vault_is_locked() {
        let r = get(&app(), "/vault/gold").await;
        assert_eq!(r.status, StatusCode::UNAUTHORIZED);
        let r = send(&app(), "GET", "/vault/gold", &[("x-guild-token", "guess")], "").await;
        assert_eq!(r.status, StatusCode::UNAUTHORIZED);
        let r = send(&app(), "GET", "/vault/gold", &[("x-guild-token", "s3cret")], "").await;
        assert_eq!(r.status, StatusCode::OK);
        assert_eq!(r.body, "1000 gold");
    }

    #[tokio::test]
    async fn everything_is_stamped() {
        assert_eq!(get(&app(), "/").await.headers["x-guild"], "open");
        assert_eq!(get(&app(), "/vault/gold").await.headers["x-guild"], "open");
    }
}
