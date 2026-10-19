# Lesson 5 · The Shapeshifter Swamp 🐍

> Enums with data, `Option`, and the full power of patterns.

## Enums are sum types

```rust
enum Shape {
    Circle(f64),                 // tuple-like
    Rect { w: f64, h: f64 },     // struct-like
    Point,                       // unit
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => 3.14 * r * r,
            Shape::Rect { w, h } => w * h,
            Shape::Point => 0.0,
        }
    }
}
```

## `Option<T>`: no null

```rust
fn find(id: u32) -> Option<&'static str> {
    if id == 1 { Some("Ferris") } else { None }
}
let name = find(2).unwrap_or("nobody");
```

## Pattern toolbox

| Pattern | Meaning |
|---|---|
| `_` | anything, ignore it |
| `x` | anything, bind it to `x` |
| `1 \| 2` | either |
| `3..=9` | inclusive range |
| `Some(x)` | enum variant with data |
| `Point { x, .. }` | struct, ignore the other fields |
| `(a, 0)` | tuple |
| `n @ 1..=9` | match a range AND bind the value |
| `x if x > 5` | guard |

## Shorthands

```rust
if let Some(x) = opt { ... } else { ... }
let Some(x) = opt else { return; };
while let Some(top) = stack.pop() { ... }
let fire = matches!(spell, Spell::Fire(_));
```

## Coming from other languages

| Elsewhere | Rust |
|---|---|
| `null` / `undefined` / `nil` | `Option<T>` (checked by the compiler) |
| class hierarchy + `instanceof` | `enum` + `match` |
| `switch` without exhaustiveness | `match` is always exhaustive |
| TypeScript discriminated unions | almost the same idea, with checked exhaustiveness |
| Kotlin `sealed class` + `when` | the same idea |

- Matching on `&Enum` gives you references to the fields; use `*x` to get the value.
- Arm order matters: a general arm placed above a specific one hides it. Read the "unreachable pattern" warnings.

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
