# Lesson 23 · The Raven Post 🐦

> HTTP clients with `reqwest`.

```toml
# default features include TLS (https)
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```

(The quests turn TLS off to build faster; they only talk to a local test
server.)

```rust
let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(5))
    .build()?;

let heroes: Vec<Hero> = client
    .get(format!("{base}/heroes"))
    .query(&[("class", "mage")])
    .bearer_auth(token)
    .send().await?
    .error_for_status()?          // 4xx/5xx → Err
    .json().await?;

let created: Hero = client
    .post(url)
    .json(&new_hero)
    .send().await?
    .json().await?;
```

## Checklist for real clients

| Habit | How |
|---|---|
| one Client, reused | create once, clone (cheap) or share |
| timeouts | `Client::builder().timeout(..)` |
| check status | `error_for_status()` or `status()` |
| 404 as "not found" | map to `Ok(None)` |
| retry only safe calls | GET with backoff on 503 / timeouts |
| typed data | serde structs, not `Value` |
| context on errors | `anyhow::Context` |

## Testing clients

Never hit the real internet in tests. Run a local server (these quests
use a tiny one in `mock.rs`; crates like `wiremock` or `httpmock` do it
properly) and point the client at it.

## Coming from other languages

| Elsewhere | reqwest |
|---|---|
| `fetch(url).then(r => r.json())` | `client.get(url).send().await?.json().await?` |
| Python requests | similar API, async, typed |
| `HttpClient` (C#/Java) | `Client` (pool + settings) |
| `raise_for_status()` | `error_for_status()` |

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
