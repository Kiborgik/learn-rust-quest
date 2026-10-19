# Lesson 21 · The Courier Guild 🕊️

> tokio: the async runtime most Rust services use.

```toml
# + "test-util" for paused time
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() { ... }

// fake clock: sleeps are instant
#[tokio::test(start_paused = true)]
async fn works() { ... }
```

## Toolbox

| Need | Tool |
|---|---|
| wait without blocking | `tokio::time::sleep(d).await` |
| background task | `tokio::spawn(async move { ... })` → `JoinHandle` |
| several at once, wait for all | `tokio::join!(a, b)` / `try_join!` |
| first one wins | `tokio::select! { x = a => .., y = b => .. }` |
| give up after a while | `tokio::time::timeout(d, fut)` |
| many tasks, first finished | `JoinSet` + `join_next().await` |
| limit concurrency | `Semaphore::new(n)` + `acquire_owned()` |
| messages | `mpsc::channel(n)` (bounded), `oneshot` for replies |
| state held across `.await` | `tokio::sync::Mutex` |
| blocking/CPU-heavy code | `tokio::task::spawn_blocking` |

## Rules

- No blocking calls in async code (`std::thread::sleep`, heavy loops,
  blocking I/O). Use async versions or `spawn_blocking`.
- Don't hold a `std::sync::MutexGuard` across `.await`.
- `spawn` needs `'static + Send`: use `async move` and owned data.
- Dropping a future cancels it. Code after an `.await` may never run.

## Coming from other languages

| Elsewhere | tokio |
|---|---|
| `Promise.all` / `Task.WhenAll` | `join!` / `JoinSet` |
| `Promise.race` / `Task.WhenAny` | `select!` |
| Go goroutines + channels | `spawn` + `mpsc` |
| `asyncio.Semaphore` | `tokio::sync::Semaphore` |
| `asyncio.wait_for` | `tokio::time::timeout` |

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
