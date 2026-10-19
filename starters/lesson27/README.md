# Lesson 27 · The Spellwright 🪄

> `macro_rules!`: code that writes code.

```rust
macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}
```

## Patterns

| Piece | Meaning |
|---|---|
| `$x:expr` | an expression |
| `$n:ident` | a name |
| `$t:ty` | a type |
| `$l:literal` | a literal |
| `$b:block`, `$p:pat`, `$tt:tt` | a block, a pattern, any one token tree |
| `$( ... ),*` | zero or more, comma separated (`+` one or more, `?` optional) |
| `$(,)?` | optional trailing comma |

Rules are tried top to bottom. Put specific ones first.

## Gotchas

- A macro exists only below its definition in the file.
- Arguments are pasted as code: one used twice runs twice. Bind it once.
- Return a value from a block: `=> {{ let v = $x; v * v }}`.
- Hygiene: a macro's own local names are invisible to the caller. Let the
  caller pass names in as `ident`.
- Use from other modules: `pub(crate) use my_macro;` after the definition.
  For other crates: `#[macro_export]` and `$crate::path`.
- Some fragments can only be followed by certain tokens (an `expr` can't
  be followed by `{`). The compiler tells you which.

## When to use one

Reach for a function or generics first. Macros fit when you need:
variable argument lists, new syntax (small DSLs), or generating items
(impls for many types, structs from a list).

Bigger jobs use **procedural macros** (`#[derive(...)]`, attributes), a
separate crate with `syn` and `quote`. That's how serde, clap and
thiserror work.

## Coming from other languages

| Elsewhere | Rust |
|---|---|
| C `#define` | `macro_rules!`, but it works on syntax trees, has hygiene, and is checked |
| Lisp macros | close in spirit, pattern-based |
| C# source generators / Java annotation processors | procedural macros |

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
