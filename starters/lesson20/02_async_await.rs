// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 20-2 · async and .await                          175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Writing `poll` by hand is tedious. `async fn` makes the compiler
//   write the future for you:
//
//     async fn dig(depth: u32) -> u32 {      // returns impl Future<Output = u32>
//         depth * 10
//     }
//
//     async fn expedition() -> u32 {
//         let a = dig(1).await;              // wait for it, get the u32
//         let b = dig(2).await;
//         a + b
//     }
//
//   Calling an async fn does NOT run it. It builds a future (a state
//   machine). `.await` (inside async code) or an executor (`block_on`)
//   actually drives it.
//
//     let f = dig(1);        // nothing happened yet; f is a future
//     let n = f.await;       // now it runs
//
//   Async blocks make a future inline, like a closure:
//
//     let fut = async move { x + 1 };        // impl Future<Output = ...>
//
//   Forgetting `.await` is a classic: you get "cannot add `impl Future`
//   to `u32`" or a warning about an unused future.
//
// 🎯 YOUR TASK
//
//   1. `expedition` forgets to await something. Fix it.
//   2. Implement `total_depth` (await each dig in a loop).
//   3. Implement `later` with an `async move` block.

use std::future::Future;
use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

async fn dig(depth: u32) -> u32 {
    depth * 10
}

async fn expedition() -> u32 {
    let first = dig(1);
    let second = dig(2).await;
    first + second
}

/// Sum of dig(d) for every depth.
async fn total_depth(depths: &[u32]) -> u32 {
    todo!()
}

/// A future that, when awaited, gives x + 1.
fn later(x: u32) -> impl Future<Output = u32> {
    todo!()
}

// ─── A tiny executor (given) ─────────────────────────────────────────────────

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
        thread::park();
    }
}

fn main() {
    println!("{}", block_on(expedition()));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expeditions() {
        assert_eq!(block_on(expedition()), 30);
        assert_eq!(block_on(total_depth(&[1, 2, 3])), 60);
        assert_eq!(block_on(later(41)), 42);
    }
}
