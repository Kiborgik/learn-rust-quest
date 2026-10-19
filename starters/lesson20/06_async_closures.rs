// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 20-6 · Try, Try Again                            175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   ASYNC CLOSURES (Rust 1.85+) are closures whose body is async:
//
//     let fetch = async |id: u32| -> u32 { id * 2 };
//     let n = fetch(21).await;                    // 42
//
//   To accept one, use the async closure traits, the async versions of
//   Fn / FnMut / FnOnce from lesson 9:
//
//     async fn twice(f: impl AsyncFn(u32) -> u32) -> u32 {
//         f(1).await + f(2).await
//     }
//
//   `AsyncFnMut` lets the closure change what it captured (a counter,
//   a log), and needs `mut f`:
//
//     async fn retry(mut attempt: impl AsyncFnMut() -> Result<u32, String>) { ... }
//
//   Retrying with a limit is the classic use: networks fail, and a
//   second try often works.
//
// 🎯 YOUR TASK
//
//   Implement `retry`: call `attempt` up to `tries` times. Return the
//   first Ok. If every try fails, return the LAST error.
//   (tries is at least 1)

use std::future::Future;
use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

async fn retry(tries: u32, mut attempt: impl AsyncFnMut() -> Result<u32, String>) -> Result<u32, String> {
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
    let mut calls = 0;
    let result = block_on(retry(5, async || {
        calls += 1;
        if calls < 3 { Err(format!("try {calls} failed")) } else { Ok(calls) }
    }));
    println!("{result:?}");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succeeds_eventually() {
        let mut calls = 0;
        let r = block_on(retry(5, async || {
            calls += 1;
            if calls < 3 { Err(format!("try {calls} failed")) } else { Ok(calls * 10) }
        }));
        assert_eq!(r, Ok(30));
        assert_eq!(calls, 3);
    }

    #[test]
    fn gives_up() {
        let mut calls = 0;
        let r = block_on(retry(2, async || {
            calls += 1;
            Err(format!("try {calls} failed"))
        }));
        assert_eq!(r, Err("try 2 failed".to_string()));
        assert_eq!(calls, 2);
    }

    #[test]
    fn first_try() {
        assert_eq!(block_on(retry(3, async || Ok(7))), Ok(7));
    }
}
