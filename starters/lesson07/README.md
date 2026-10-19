# Lesson 7 · The Fortress of Failure 🏯

> `panic!`, `Result`, `?`, combinators, custom errors and `From`.

## Two kinds of errors

| | Tool | Example |
|---|---|---|
| Bug / impossible state | `panic!`, `unwrap`, `expect` | broken invariant |
| Expected failure | `Result<T, E>` | bad input, missing file |

## Result and `?`

```rust
fn add(a: &str, b: &str) -> Result<u32, ParseIntError> {
    // Err → return early (converted with From)
    let x: u32 = a.parse()?;
    let y: u32 = b.parse()?;
    Ok(x + y)
}
```

`?` works on `Option` too, inside functions returning `Option`.

## Combinators cheat sheet

| Method | Does |
|---|---|
| `map(f)` | transform the success value |
| `map_err(f)` | transform the error |
| `and_then(f)` | chain a step that can fail |
| `ok_or(e)` / `ok_or_else(f)` | Option → Result |
| `ok()` | Result → Option |
| `unwrap_or(d)` / `unwrap_or_default()` | fallback value |
| `is_ok()` / `is_some()` | bool checks |

## Custom errors

```rust
#[derive(Debug)]
enum AppError { NotFound, Bad(ParseIntError) }

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "not found"),
            AppError::Bad(e) => write!(f, "bad number: {e}"),
        }
    }
}
impl std::error::Error for AppError {}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Bad(e) }
}
```

For applications and scripts: `fn main() -> Result<(), Box<dyn Error>>`.
In real projects, the `thiserror` crate (for libraries) and `anyhow` (for apps) remove most of this boilerplate.

## Coming from other languages

| Elsewhere | Rust |
|---|---|
| `throw` / `try` / `catch` | `return Err(..)` / `?` / `match` |
| checked exceptions | the `Result` type in the signature |
| Go `if err != nil { return err }` | `?` |
| `NullPointerException` | `Option` + `unwrap` (explicit, greppable) |

- There are no exceptions: control flow is always visible.
- Searching for `unwrap()` finds every place your program might crash.
- `#[must_use]` on `Result` means ignoring one triggers a warning.

## 🎮 Controls

| Key | Action |
|---|---|
| *save the file* | auto re-check |
| `Enter` | re-check now |
| `h` | reveal a hint (that quest gives half XP) |
| `m` | quest map & achievements |
| `q` | save & quit |
| `restore` | put this quest's file back to its starting state (asks first) |
| `reset` | start over: wipe progress and restore all files (asks first) |
