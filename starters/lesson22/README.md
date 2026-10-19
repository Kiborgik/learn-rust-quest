# Lesson 22 · The Watchtower 🗼

> `tracing`: structured logs, spans, and filters.

```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

```rust
fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env(),
        )
        .init(); // RUST_LOG=info cargo run
    info!(port = 8080, "server started");
}
```

## Events

```rust
use tracing::{trace, debug, info, warn, error};
// fields first, then the message
warn!(hero = %name, hp, "low health");
```

| Sigil | Records with |
|---|---|
| `x` | the value itself (numbers, bools, strings) |
| `%x` | Display |
| `?x` | Debug |

## Spans

```rust
let span = info_span!("quest", name = %quest);
let _guard = span.enter();                 // sync code only

#[instrument(skip(password), fields(user = %name))]
fn login(name: &str, password: &str) { ... }

async_work().instrument(info_span!("job", id)).await;   // async code
```

Never hold `span.enter()` across `.await`: use `.instrument()` or
`#[instrument]`.

## Filters (RUST_LOG)

| Directive | Shows |
|---|---|
| `warn` | warn + error |
| `info,my_app::db=debug` | info everywhere, debug in one module |
| `off` | nothing |

## Coming from other languages

| Elsewhere | tracing |
|---|---|
| log4j / NLog / Python logging | events + subscriber |
| MDC / logging context | spans |
| OpenTelemetry traces | spans (with `tracing-opentelemetry`) |
| structured logging (Serilog) | fields |

The `log` crate is older and simpler. tracing can read its events, so
libraries using `log` still show up.

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
