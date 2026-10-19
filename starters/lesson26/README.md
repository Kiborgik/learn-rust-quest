# Lesson 26 · Capstone: the Guild Board 🏆

> One real program, built in layers: a quest board with a database, a REST
> API, an HTTP client and a CLI.

```text
guildboard (CLI, clap)          guildboard-server (config, start, stop)
        │                                     │
   client (reqwest) ── HTTP/JSON ──▶  api (axum)
                                              │
                                       store (sqlx, SQLite)
                                              │
                  domain (rules, serde, thiserror; no IO)
```

Each quest adds one layer. The next quest ships your layer solved, so you
always build on working code.

| Quest | Layer | Uses |
|---|---|---|
| 26-1 | domain: types, rules, errors | serde, thiserror |
| 26-2 | store: SQLite, transactions | sqlx |
| 26-3 | api: routes, error → status | axum |
| 26-4 | client: typed calls | reqwest |
| 26-5 | cli: commands, testable output | clap, anyhow |
| BOSS | server: config, start, graceful stop | toml, tokio |

## Ideas that hold it together

- **Rules live in one place.** The domain decides; every other layer just
  moves data in and out.
- **Each layer has its own error,** converted at the border with `#[from]`
  or `IntoResponse`. Clients get clear messages, never database details.
- **Shared types.** Client and server use the same `Quest`, so they can't
  drift apart quietly.
- **Thin edges.** `main.rs` is three lines; everything testable is in the
  library.
- **Tests at every level:** pure unit tests, in-memory database, in-memory
  router, and finally a real server on a free port.

## Play with it

After the boss, in `exercises/lesson26/06_boss/`:

```sh
cargo run --bin guildboard-server -- board.toml     # terminal 1
cargo run --bin guildboard -- add "Slay the slime" -r 50
cargo run --bin guildboard -- list
cargo run --bin guildboard -- finish 1
cargo run --bin guildboard -- summary
```

`board.toml` uses a file database, so quests survive restarts.

## Where to go next

- Add `tracing` spans to the handlers (lesson 22) and `tower-http`'s
  `TraceLayer`.
- Retry on 503 in the client (lesson 23).
- Use `sqlx::migrate!` instead of the inline schema (lesson 25).
- Split it into a workspace: `guild-core`, `guild-server`, `guild-cli`.

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
