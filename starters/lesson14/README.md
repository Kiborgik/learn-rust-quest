# Lesson 14 · The Error Forge ⚒️

> The two crates nearly every Rust project uses for errors.

| Crate | For | Gives you |
|---|---|---|
| `thiserror` | libraries | derive `Display` + `Error` for your own enums |
| `anyhow` | applications | one error type for everything, plus context |

## thiserror

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("file is empty")]
    Empty,
    #[error("bad number: {0}")]
    BadNumber(#[from] std::num::ParseIntError),   // From impl + source
    #[error("bad value for {key}")]
    BadValue { key: String, #[source] cause: std::num::ParseIntError },
}
```

| Attribute | Does |
|---|---|
| `#[error("...")]` | the Display message, can use fields |
| `#[from]` | `impl From<T>`, so `?` converts; also sets source |
| `#[source]` | marks the cause (a field named `source` counts too) |
| `#[error(transparent)]` | forward message and source to the inner error |

## anyhow

```rust
use anyhow::{anyhow, bail, ensure, Context, Result};

fn load(path: &str) -> Result<Config> {
    let text = read(path).with_context(|| format!("loading {path}"))?;
    ensure!(!text.is_empty(), "{path} is empty");
    if text.starts_with('{') { bail!("JSON is not supported yet"); }
    parse(&text).context("parsing config")
}

// prints "Error: ... Caused by: ..."
fn main() -> anyhow::Result<()> { ... }
```

| Print | Shows |
|---|---|
| `{err}` | top message only |
| `{err:#}` | the chain on one line, joined by `: ` |
| `{err:?}` | full report with "Caused by:" lines |

`err.downcast_ref::<T>()` gets a specific error type back out.

## Coming from other languages

- Exceptions carry a stack of "caused by"; here it's `source()`, explicit.
- Java checked exceptions ≈ thiserror enums in signatures.
- Catch-all `catch (Exception e)` ≈ `anyhow::Error`, but without unwinding.
- Go's `fmt.Errorf("loading %s: %w", path, err)` ≈ `.with_context(...)`.

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
