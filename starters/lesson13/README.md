# Lesson 13 · The Workshop 🛠️

> Real Cargo projects: modules, visibility, library crates, tests, docs,
> dependencies.

From this lesson on, each quest is a small Cargo package in its own folder.

## Package layout

```
my_app/
├── Cargo.toml       # name, version, edition, [dependencies]
├── Cargo.lock       # exact versions used (commit it for apps)
├── src/
│   ├── main.rs      # binary crate root
│   ├── lib.rs       # library crate root (optional)
│   └── tools.rs     # a module
└── tests/           # integration tests, one crate per file
```

| Command | Does |
|---|---|
| `cargo new name` | create a package |
| `cargo run` | build and run |
| `cargo test` | unit, integration and doc tests |
| `cargo add rand` | add a dependency |
| `cargo doc --open` | build and open the docs |
| `cargo fmt` | format the code |
| `cargo clippy` | lint the code |

## Modules

```rust
mod tools;              // loads src/tools.rs (or src/tools/mod.rs)
mod inline { pub fn f() {} }

use crate::tools::count;
use super::sibling;     // from the parent module
pub use inner::Thing;   // re-export
```

## Visibility

| Keyword | Visible to |
|---|---|
| (none) | this module and its children |
| `pub(super)` | the parent module |
| `pub(crate)` | the whole crate |
| `pub` | everyone, including other crates |

A private field means outsiders must use your constructor.

## Library + binary

Put the logic in `lib.rs` and keep `main.rs` small. The binary and
`tests/*.rs` import the library by crate name (package name with `-` → `_`).

## Dependencies

```toml
[dependencies]
rand = "0.9"                                   # >=0.9.0, <0.10.0
serde = { version = "1", features = ["derive"] }
```

## Doc tests

```rust
/// ```
/// use my_crate::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

## Coming from other languages

| Elsewhere | Rust |
|---|---|
| npm / pip / Maven | Cargo + crates.io |
| `package.json` / `pom.xml` | `Cargo.toml` |
| lock file | `Cargo.lock` |
| `import` scans folders | `mod` declares the tree explicitly |
| `public`/`private` per class | `pub` per item, privacy per module |
| `__init__.py` / index.js | `lib.rs` or `mod.rs` |

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
