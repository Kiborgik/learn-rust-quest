# Lesson 20 · The Clockwork Gate ⏳

> What async really is, built from std only. After this, tokio is just a
> very good version of what you wrote.

## The Future trait

```rust
trait Future {
    type Output;
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Self::Output>;
}
enum Poll<T> { Ready(T), Pending }
```

- An executor calls `poll`.
- `Pending` means "not yet". Before returning it, make sure `cx.waker()`
  gets woken when progress is possible.
- `wake()` means "poll me again".

## async / await

```rust
// returns impl Future<Output = u32>
async fn dig(d: u32) -> u32 { d * 10 }
let n = dig(1).await;                      // inside async code
let fut = async move { x + 1 };            // async block
```

Futures are lazy: nothing runs until polled.

## Pin

Async futures can borrow their own data, so they must not move once
polled. `Pin` guarantees that.

```rust
// on the stack
let f = std::pin::pin!(fut);
// on the heap
let f: Pin<Box<dyn Future<Output = u32>>> = Box::pin(fut);
```

## A runtime in one paragraph

Tasks are boxed futures. A queue holds tasks that are ready. The loop
pops a task and polls it once. A task's waker puts it back in the queue.
Real runtimes add I/O and timers that call `wake`.

## Concurrency vs parallelism

| | What | Tool |
|---|---|---|
| concurrency | many tasks interleaved, maybe on one thread | async |
| parallelism | many cores at once | threads, rayon |

Async shines when tasks mostly WAIT (network, disk, timers).

## Coming from other languages

| Elsewhere | Rust |
|---|---|
| JS Promise (starts running immediately) | future (lazy, needs polling) |
| C# Task / Java CompletableFuture | future + an executor you choose |
| built-in event loop (JS, Python asyncio) | pick a runtime: tokio, smol... |
| `async () => {}` | `async \|\| {}` (Rust 1.85+) |

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
