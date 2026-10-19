# Lesson 2 · The Winding Caves 🕳️

> `if`, `loop`, `while`, `for`, labels and a first taste of `match`.

## 1. `if` is an expression

```rust
let hp = 30;
let state = if hp < 50 { "weak" } else { "strong" };  // like ?: in C/JS
```

- No parentheses around the condition; braces are mandatory.
- The condition **must** be `bool` (`if 1 {}` is an error).
- Both branches must have the same type.

## 2. `loop` with a value

```rust
let mut n = 1;
let first_big = loop {
    n *= 2;
    if n > 100 { break n; }   // the loop evaluates to n
};
```

## 3. `while`

```rust
let mut t = 3;
while t > 0 { t -= 1; }      // no ++ / -- in Rust
```

## 4. `for` and ranges

| Range | Yields |
|---|---|
| `0..3` | 0, 1, 2 |
| `0..=3` | 0, 1, 2, 3 |
| `(1..=3).rev()` | 3, 2, 1 |
| `(0..10).step_by(5)` | 0, 5 |

```rust
for coin in [10, 20, 30] { /* arrays are iterable */ }
```

## 5. Labels

```rust
'outer: for x in 0..5 {
    for y in 0..5 {
        if x + y == 6 { break 'outer; }
        if y > x { continue 'outer; }
    }
}
```

## 6. `match` (basics)

```rust
let s = match n {
    0 => "none",
    1 | 2 => "few",
    3..=9 => "some",
    _ => "lots",          // must be exhaustive
};
```

## 7. `todo!()`

`todo!()` compiles anywhere, whatever type is expected, and panics at runtime. It's handy for stubbing functions.

## Coming from other languages

| You might write | In Rust |
|---|---|
| `if (x) {...}` with a number | `if x != 0 {...}` |
| `cond ? a : b` | `if cond { a } else { b }` |
| `for (i = 0; i < n; i++)` | `for i in 0..n` |
| `i++` | `i += 1` |
| `switch` + `break` | `match` (no fall-through) |
| `do { } while (c)` | `loop { ...; if !c { break; } }` |

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
