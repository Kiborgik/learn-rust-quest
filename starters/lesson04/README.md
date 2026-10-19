# Lesson 4 · The Guild Hall 🏰

> Structs, methods, associated functions and `derive`.

## Structs

```rust
struct Monster { name: String, hp: u32 }
let m = Monster { name: String::from("Slime"), hp: 12 };
// update syntax (moves m.name!)
let mut m2 = Monster { hp: 50, ..m };
let Monster { name, hp } = m2;              // destructuring
```

| Kind | Example | Access |
|---|---|---|
| Named fields | `struct P { x: i32 }` | `p.x` |
| Tuple struct | `struct Gold(u32);` | `g.0` |
| Unit struct | `struct Marker;` | (no fields) |

## Methods & associated functions

```rust
impl Monster {
    const MAX_HP: u32 = 999;

    // Monster::new(..)
    fn new(name: &str) -> Self { Self { name: name.into(), hp: 10 } }
    fn is_alive(&self) -> bool { self.hp > 0 }       // read
    // modify
    fn hit(&mut self, d: u32) { self.hp = self.hp.saturating_sub(d); }
    fn loot(self) -> String { self.name }            // consume
}
```

## Derive

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord(i32, i32);
```

| Derive | Enables |
|---|---|
| `Debug` | `{:?}`, `{:#?}` |
| `Clone` | `.clone()` |
| `Copy` | implicit copy (all fields must be Copy) |
| `PartialEq` | `==`, `assert_eq!` |
| `Default` | `Type::default()` (all zero/empty) |

## Coming from other languages

- **No classes and no inheritance.** You get data (`struct`) plus behaviour (`impl`). Shared behaviour comes from traits (lesson 8).
- **No constructors.** `new` is just a convention; you can have `with_capacity`, `from_file`, and so on.
- **No `this`.** `self` is explicit, and its form (`&self` / `&mut self` / `self`) is an ownership decision.
- **Mutability is per binding**, not per field.
- **Visibility:** everything here is in one file, so `pub` doesn't matter yet. In real crates, fields are private by default.

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
