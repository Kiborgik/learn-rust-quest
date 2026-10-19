//! Test helper (given, don't change): runs the real board server on a
//! free local port, with a fresh in-memory database.

use crate::api::router;
use crate::store::Store;

/// Starts a board in the background and returns its base URL.
pub async fn spawn_board() -> String {
    let store = Store::open("sqlite::memory:").await.unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move { axum::serve(listener, router(store)).await.unwrap() });
    url
}
