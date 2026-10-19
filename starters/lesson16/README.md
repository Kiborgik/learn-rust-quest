# Lesson 16 · The Scriptorium 🖋️

> `itertools` for iterator chains, `regex` for patterns, `LazyLock` for
> building things once.

## itertools

```rust
use itertools::Itertools;
```

| Method | Example result |
|---|---|
| `sorted()` / `sorted_by_key(f)` | sorted iterator |
| `unique()` | each value once, first seen |
| `dedup()` | drop adjacent repeats |
| `join(", ")` | `"a, b"` |
| `counts()` | `HashMap<item, usize>` |
| `chunk_by(key)` | runs of equal keys |
| `tuple_windows::<(_, _)>()` | overlapping pairs |
| `tuple_combinations()` / `combinations(k)` | unordered picks |
| `cartesian_product(other)` | every pair |
| `minmax()` | min and max in one pass |
| `k_smallest(k)` | the k smallest |

## regex

```rust
use regex::Regex;
let re = Regex::new(r"(?<count>\d+)d(?<sides>\d+)").unwrap();
re.is_match("2d6");
re.find_iter(text).map(|m| m.as_str());
let c = re.captures("12d20").unwrap();   // &c["count"] == "12"
re.replace_all(text, "$count dice");
```

| Syntax | Means |
|---|---|
| `\d` `\w` `\s` | digit, word char, whitespace |
| `+` `*` `?` | 1+, 0+, optional |
| `^` `$` | start, end |
| `[a-z]` | one of |
| `(?<name>...)` | named group |
| `\b` | word boundary |

The regex crate runs in linear time: safe on untrusted input. No
lookaround or backreferences.

## LazyLock

```rust
use std::sync::LazyLock;
static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d+").unwrap());
```

Built on first use, then shared. Thread-safe. In std since Rust 1.80.

## Coming from other languages

- JS/Python regex mostly carry over. Named groups: `(?<name>...)` or `(?P<name>...)`.
- No backtracking engine, so no ReDoS: the trade-off is no lookbehind.
- itertools ≈ Python's `itertools` + `more-itertools`, as methods.

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
