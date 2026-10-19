# Lesson 18 · The Observatory 🔭

> `rayon`: parallel iterators. Change `iter()` to `par_iter()`.

```rust
use rayon::prelude::*;

let total: u64 = v.par_iter().map(|x| x * x).sum();
let names: Vec<String> =
    heroes.par_iter().map(|h| h.name.clone()).collect(); // keeps order
v.par_iter_mut().for_each(|x| *x += 1);
v.par_sort_by_key(|h| h.level);
let (a, b) = rayon::join(|| left(), || right());
```

| Std | rayon |
|---|---|
| `iter()` / `iter_mut()` / `into_iter()` | `par_iter()` / `par_iter_mut()` / `into_par_iter()` |
| `chunks(n)` | `par_chunks(n)` |
| `sort*()` | `par_sort*()` |
| `fold(init, f)` | `fold(|| init, f)` then `reduce(|| init, merge)` |

## Rules of thumb

- CPU-bound work on lots of data: yes. Waiting on network or disk: no,
  that's async.
- `collect` keeps input order. Side effects in `for_each` don't.
- `reduce` needs an associative operation.
- Count with `fold` + `reduce`, not with a shared `Mutex<HashMap>`.
- Measure: for small inputs the splitting costs more than it saves.

## Pools

```rust
let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build()?;
pool.install(|| data.par_iter().sum::<u64>());
```

## Coming from other languages

- Java `parallelStream()`, .NET `AsParallel()` / `Parallel.For`, Python
  `multiprocessing.Pool.map`: same idea.
- The difference: data races don't compile. Shared state needs `Sync`.

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
