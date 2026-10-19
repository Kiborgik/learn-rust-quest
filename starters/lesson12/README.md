# Lesson 12 · The Hive of Threads 🐝

> Threads, `move`, channels, `Mutex`, `Arc`, scoped threads and atomics.

## Spawning

```rust
// `move`: the thread owns `data`
let h = thread::spawn(move || heavy_work(data));
// wait for it (Err if it panicked)
let result = h.join().unwrap();
```

## Channels

```rust
let (tx, rx) = mpsc::channel();
for i in 0..3 {
    let tx = tx.clone();
    thread::spawn(move || tx.send(i).unwrap());
}
drop(tx);                     // otherwise the loop below never ends!
let total: i32 = rx.iter().sum();
```

## Shared state

```rust
let counter = Arc::new(Mutex::new(0));
let c = Arc::clone(&counter);
thread::spawn(move || *c.lock().unwrap() += 1);
```

## Scoped threads (borrowing allowed)

```rust
thread::scope(|s| {
    for chunk in data.chunks(1000) {
        s.spawn(move || process(chunk));
    }
});  // everything is joined here
```

## Send and Sync

| Trait | Meaning | Not implemented by |
|---|---|---|
| `Send` | safe to **move** to another thread | `Rc`, raw pointers |
| `Sync` | safe to **share** `&T` between threads | `Cell`, `RefCell`, `Rc` |

The compiler derives these for you. Misuse is a **compile error**, not a data race in production.

## Pick the tool

| Situation | Tool |
|---|---|
| run work in the background, get a result | `spawn` + `join` |
| parallel work on borrowed local data | `thread::scope` |
| pass messages / pipelines | `mpsc::channel` |
| shared mutable state | `Arc<Mutex<T>>` |
| many readers, rare writers | `Arc<RwLock<T>>` |
| simple counters and flags | `AtomicU32`, `AtomicBool` |
| data parallelism on collections | the `rayon` crate (`par_iter()`) |

## Coming from other languages

- **Java/C#:** `Mutex<T>` owns its data, so you *can't* forget to lock.
- **Go:** channels feel familiar. `tx.clone()` = multiple producers, and receiving ends once all senders are dropped.
- **JS/Python:** real parallelism, no GIL.
- Rust prevents **data races** at compile time, but not deadlocks or logic races.

## 🎓 What's next after Rust Quest?

1. **Build something real:** `cargo new my_app`, then a CLI tool (crates: `clap`, `serde`, `anyhow`).
2. **Modules & crates:** `mod`, `pub`, `use`, workspaces, publishing to crates.io.
3. **Testing:** integration tests in `tests/`, doc tests, `cargo clippy`.
4. **Macros:** `macro_rules!` declarative macros.
5. **Async:** `async`/`await` with `tokio` for network services.
6. **Unsafe & FFI:** `unsafe`, raw pointers, calling C. Read "The Rustonomicon".
7. Reading: *The Rust Book*, *Rust for Rustaceans*, *Rust Atomics and Locks*.

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
