// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 20-4 · Two at Once                               175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `.await` waits for one thing at a time. To make progress on two
//   futures at once on ONE thread, poll them both, turn by turn. That's
//   `join`, and you can write it yourself:
//
//     struct Join<A: Future, B: Future> {
//         a: Pin<Box<A>>,
//         b: Pin<Box<B>>,
//         a_out: Option<A::Output>,     // results arrive at different times
//         b_out: Option<B::Output>,
//     }
//
//   poll():
//     • if `a` isn't done yet, poll it; on Ready, store the value
//     • same for `b`
//     • both stored? → Ready((a, b)), else Pending
//
//   Never poll a future again after it returned Ready. That's why the
//   Options double as "done" flags.
//
//   No threads involved. This is CONCURRENCY (interleaving), not
//   parallelism. One thread juggles many tasks, which is exactly what
//   async runtimes do with thousands of network connections.
//
//   `yield_now()` (given) is a future that returns Pending once, so other
//   work gets a turn, like a polite "after you".
//
// 🎯 YOUR TASK
//
//   Implement `poll` for `Join`. The test checks that the two workers
//   really take turns.

use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

pub struct Join<A: Future, B: Future> {
    a: Pin<Box<A>>,
    b: Pin<Box<B>>,
    a_out: Option<A::Output>,
    b_out: Option<B::Output>,
}

pub fn join<A: Future, B: Future>(a: A, b: B) -> Join<A, B> {
    Join { a: Box::pin(a), b: Box::pin(b), a_out: None, b_out: None }
}

impl<A: Future, B: Future> Future for Join<A, B>
where
    A::Output: Unpin,
    B::Output: Unpin,
{
    type Output = (A::Output, B::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.get_mut();
        todo!()
    }
}

// ─── Helpers (given) ─────────────────────────────────────────────────────────

struct YieldNow(bool);

impl Future for YieldNow {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
        if self.0 {
            return Poll::Ready(());
        }
        self.0 = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

fn yield_now() -> YieldNow {
    YieldNow(false)
}

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

async fn worker(name: &str, steps: u32, log: Arc<Mutex<Vec<String>>>) -> u32 {
    for i in 1..=steps {
        log.lock().unwrap().push(format!("{name}{i}"));
        yield_now().await;
    }
    steps * 10
}

fn main() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let both = join(worker("a", 2, log.clone()), worker("b", 2, log.clone()));
    println!("{:?} {:?}", block_on(both), log.lock().unwrap());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn takes_turns() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let result = block_on(join(worker("a", 3, log.clone()), worker("b", 2, log.clone())));
        assert_eq!(result, (30, 20));
        assert_eq!(*log.lock().unwrap(), vec!["a1", "b1", "a2", "b2", "a3"]);
    }

    #[test]
    fn instant() {
        assert_eq!(block_on(join(async { 1 }, async { "x" })), (1, "x"));
    }
}
