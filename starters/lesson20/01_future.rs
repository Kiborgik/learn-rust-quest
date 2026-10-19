// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 20-1 · A Promise of Later                        175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A FUTURE is a value that will be ready later. It's just a trait with
//   one method:
//
//     trait Future {
//         type Output;
//         fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
//     }
//
//     enum Poll<T> { Ready(T), Pending }
//
//   Someone (an EXECUTOR) calls `poll`. The future either finishes
//   (`Ready(value)`) or says "not yet" (`Pending`).
//
//   The deal: when you return Pending, you must make sure someone will
//   call `wake()` on the waker in `cx` once it's worth polling again.
//   Otherwise the executor sleeps forever. Real futures hand the waker to
//   a timer or the OS ("wake me when this socket has data"). A future
//   that just wants another turn wakes itself right away:
//
//     cx.waker().wake_by_ref();
//     Poll::Pending
//
//   (`self: Pin<&mut Self>` will make sense in quest 20-3. For an Unpin
//   type like ours, `let this = self.get_mut();` gives a plain &mut.)
//
//   `block_on` below is a tiny executor: poll, sleep until woken, repeat.
//   It complains if you return Pending without waking.
//
// 🎯 YOUR TASK
//
//   Implement `poll` for Countdown. Each poll counts itself in `polls`.
//   While `left > 0`: decrease `left`, wake, return Pending.
//   When `left` is 0: return Ready("liftoff").

use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};
use std::time::{Duration, Instant};

struct Countdown {
    left: u32,
    polls: u32,
}

impl Future for Countdown {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<&'static str> {
        todo!()
    }
}

// ─── A tiny executor (given) ─────────────────────────────────────────────────

struct ThreadWaker {
    thread: Thread,
    woken: AtomicBool,
}

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.woken.store(true, Ordering::SeqCst);
        self.thread.unpark();
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let signal = Arc::new(ThreadWaker { thread: thread::current(), woken: AtomicBool::new(false) });
    let waker = Waker::from(signal.clone());
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
        let start = Instant::now();
        while !signal.woken.swap(false, Ordering::SeqCst) {
            if start.elapsed() > Duration::from_secs(1) {
                panic!("the future returned Pending but never called wake(), so nobody will poll it again");
            }
            thread::park_timeout(Duration::from_millis(50));
        }
    }
}

fn main() {
    println!("{}", block_on(Countdown { left: 3, polls: 0 }));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_down() {
        assert_eq!(block_on(Countdown { left: 3, polls: 0 }), "liftoff");
        assert_eq!(block_on(Countdown { left: 0, polls: 0 }), "liftoff");
    }

    #[test]
    fn polled_the_right_number_of_times() {
        let mut c = Countdown { left: 2, polls: 0 };
        let mut fut = Pin::new(&mut c);
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        assert!(fut.as_mut().poll(&mut cx).is_pending());
        assert!(fut.as_mut().poll(&mut cx).is_pending());
        assert_eq!(fut.as_mut().poll(&mut cx), Poll::Ready("liftoff"));
        assert_eq!(c.polls, 3);
    }
}
