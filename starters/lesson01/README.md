# Lesson 1 · The Village of Bindings 🏘️

> Variables, mutability, shadowing, basic types and functions.

Skim this before (or while) playing. Every concept has a tiny example. Try
pasting them into an exercise and breaking them on purpose.

## 1. Hello, world

```rust
fn main() {
    println!("Hello, {}!", "world");
}
```

- `fn main()`: the program's entry point.
- `println!`: a **macro** (note the `!`). `{}` placeholders are filled by the arguments.

## 2. Variables with `let`

```rust
let hero = "Ferris";      // type inferred (&str)
let hp: u32 = 100;        // type written explicitly
println!("{hero}: {hp}"); // names can go right inside {}
```

A variable must have a value before it is read. The compiler checks this.

## 3. Mutability with `mut`

```rust
let gold = 10;
// gold = 20;        // ✘ immutable by default
let mut purse = 10;
purse += 5;          // ✔
```

## 4. Shadowing

```rust
let level = "7";                        // &str
let level: u32 = level.parse().unwrap(); // new variable, new type (u32)
let level = level + 1;                   // 8
```

| | `mut` | shadowing (`let` again) |
|---|---|---|
| Same variable? | yes | no, a new one |
| Can change type? | no | yes |

## 5. Types

| Kind | Types | Example |
|---|---|---|
| Signed int | `i8 i16 i32 i64 i128 isize` | `-5` |
| Unsigned int | `u8 u16 u32 u64 u128 usize` | `255u8` |
| Float | `f32 f64` | `1.5` |
| Boolean | `bool` | `true` |
| Character | `char` | `'R'`, `'🦀'` |
| Text | `&str` | `"hello"` |
| Tuple | `(T, U, ...)` | `(1, 'a')` → `.0`, `.1` |
| Array | `[T; N]` | `[1, 2, 3]` → `[0]` |

Rust **never** silently converts types: `let x: i32 = 2.5;` is an error.

### Number gotchas

| Situation | Rust |
|---|---|
| `u32` into a `u64` variable | error: write `x as u64` or `u64::from(x)` |
| `7 / 2` | `3` (integer division truncates) |
| `250u8 + 10` | compile error or **panic** (debug), never silent |
| want a cap / wrap | `saturating_add`, `wrapping_add`, `checked_add` |
| `300 as u8` | `44`: `as` silently drops bits, use `u8::try_from` to be safe |
| `3.99 as i32` | `3` (truncates toward zero) |

## 6. Functions & expressions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b       // last expression, no `;` → the return value
}
```

- Parameter types are mandatory.
- `a + b;` (with `;`) would return `()` instead. A classic first mistake.
- Blocks are expressions too:

```rust
let v = {
    let a = 2;
    a * 3       // v = 6
};
```

## Coming from other languages

| You might write | In Rust |
|---|---|
| `var x = 5` / `int x = 5` | `let x = 5;` (immutable!) |
| `x = 6` on any variable | only if declared `let mut x` |
| implicit `int` → `long` | explicit `as` / `From` |
| `return a + b;` | `a + b` as the last line, no `;` |
| `printf("%d", x)` / f-strings | `println!("{x}")` / `println!("{}", x)` |
| silent integer overflow (C, Java) | panic in debug, explicit `wrapping_*` if you want it |

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
