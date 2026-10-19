# Lesson 9 · The River of Iterators 🌊

> Closures, the `Fn` traits, and iterator chains.

## Closures

```rust
let add = |a: i32, b: i32| a + b;
let bonus = 5;
let boosted = |x| x + bonus;              // captures `bonus`
fn make(n: i32) -> impl Fn(i32) -> i32 { move |x| x * n }
```

| Trait | The closure... | Accept with |
|---|---|---|
| `Fn` | reads captures | `impl Fn(A) -> B` |
| `FnMut` | mutates captures | `mut f: impl FnMut(A) -> B` |
| `FnOnce` | consumes captures | `impl FnOnce(A) -> B` |

## Iterators

```rust
v.iter()        // &T
v.iter_mut()    // &mut T
v.into_iter()   // T (consumes v)
```

Iterators are lazy: nothing happens until a consumer runs them.

### Adapters (return a new iterator)

| Adapter | Example |
|---|---|
| `map` | `.map(\|x\| x * 2)` |
| `filter` | `.filter(\|x\| **x > 3)` |
| `filter_map` | `.filter_map(\|s\| s.parse().ok())` |
| `flat_map` | `.flat_map(\|w\| w.chars())` |
| `enumerate` | `(index, item)` |
| `zip` | pair with another iterator |
| `chain` | concatenate |
| `take` / `skip` | first n / drop n |
| `take_while` / `skip_while` | while a condition holds |
| `rev` | backwards |
| `copied` / `cloned` | `&T` → `T` |

### Consumers (produce a value)

| Consumer | Returns |
|---|---|
| `collect::<Vec<_>>()` | a collection (Vec, String, HashMap...) |
| `sum` / `product` | a number |
| `count` | `usize` |
| `min` / `max` / `max_by_key` | `Option` |
| `find` / `position` | `Option` |
| `any` / `all` | `bool` |
| `fold(init, f)` | anything |
| `for_each(f)` | `()` |

```rust
let top3: Vec<&str> = heroes.iter()
    .filter(|h| h.alive)
    .map(|h| h.name.as_str())
    .take(3)
    .collect();
```

## Coming from other languages

| Elsewhere | Rust |
|---|---|
| JS `arr.map().filter()` (eager, allocates each step) | lazy, fused into a single loop |
| C# LINQ / Java Streams | very similar, and zero-cost |
| Python generators | `impl Iterator` + `next` |
| `reduce` | `fold` (or `reduce` for no initial value) |

- Iterator chains compile down to the same machine code as a hand-written loop.
- `filter` on `.iter()` sees `&&T`. Use `.copied()` or patterns like `|&&x|`.
- `collect` needs to know the target type: annotate the variable or use turbofish.

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
