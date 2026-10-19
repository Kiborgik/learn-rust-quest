# Lesson 24 · The Guild Hall 🏰

> Web servers with `axum`.

```toml
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

```rust
#[derive(Clone, Default)]
struct AppState { heroes: Arc<Mutex<Vec<Hero>>> }

async fn hero(
    State(s): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Hero>, ApiError> {
    let heroes = s.heroes.lock().unwrap();
    heroes
        .iter()
        .find(|h| h.id == id)
        .cloned()
        .map(Json)
        .ok_or(ApiError::NotFound(id))
}

let app = Router::new()
    .route("/heroes/{id}", get(hero).delete(remove))
    .with_state(AppState::default());

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
axum::serve(listener, app).await?;
```

## Extractors

| Argument | Gives you | Fails with |
|---|---|---|
| `Path(id): Path<u32>` | `/heroes/{id}` captures | 400 |
| `Query(q): Query<T>` | `?a=1&b=2` into a struct | 400 |
| `State(s): State<S>` | shared app state | compile error if missing |
| `headers: HeaderMap` | request headers | never |
| `Json(v): Json<T>` | JSON body (must be last) | 400 / 415 / 422 |
| `body: String` | raw body (must be last) | 400 if not UTF-8 |

## Responses

Anything `IntoResponse`: `&str`, `String`, `Json(v)`, `StatusCode`,
tuples like `(StatusCode, headers, body)`, and `Result<T, E>` when both
sides are `IntoResponse`. Give your API one error type that implements it.

## Middleware

`axum::middleware::from_fn(f)` with `async fn f(req: Request, next: Next)`.
`.layer()` wraps the routes already in that Router. For ready-made layers
(CORS, compression, timeouts, request logs) see the `tower-http` crate.

## Testing

A Router is a tower `Service`. Tests call it with `oneshot(request)`, no
port needed (see `testkit.rs`).

## Coming from other languages

| Elsewhere | axum |
|---|---|
| Express `app.get("/x/:id", fn)` | `.route("/x/{id}", get(f))` |
| `req.params.id` | `Path(id): Path<u32>` |
| `req.body` with a schema | `Json(v): Json<T>` with serde |
| Flask/FastAPI dependency injection | extractors and `State` |
| Express middleware `(req, res, next)` | `from_fn(async fn(req, next))` |

## 🎮 Controls

| Key | Action |
|---|---|
| *save the file* | auto re-check |
| `Enter` | re-check now |
| `h` | reveal a hint (that quest gives half XP) |
| `m` | quest map & achievements |
| `q` | save & quit |
| `restore` | put this quest's files back to the start (asks first) |
| `reset` | start over: wipe progress and restore all files (asks first) |
