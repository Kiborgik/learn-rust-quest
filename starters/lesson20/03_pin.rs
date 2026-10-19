// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 20-3 · Pinned in Place                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The future made by an `async fn` can hold references into ITSELF
//   (a local variable, and a borrow of it, both living across an
//   `.await`). If such a value moved in memory, those references would
//   dangle. So once it's been polled, it must stay put: it's PINNED.
//
//   `Pin<P>` wraps a pointer and promises the value behind it won't move.
//   That's why `poll` takes `self: Pin<&mut Self>`.
//
//   In everyday code you meet Pin in two places:
//
//     let fut = pin!(some_future);                // pin on the stack
//     let fut: Pin<Box<dyn Future<Output = u32>>> = Box::pin(some_future);
//                                                 // pin on the heap
//
//   The second one is how you store DIFFERENT futures together (each
//   async fn has its own unnamed type):
//
//     let jobs: Vec<Pin<Box<dyn Future<Output = u32>>>> = vec![
//         Box::pin(dig(1)),
//         Box::pin(async { 5 }),
//     ];
//
//   A plain `Box<dyn Future>` won't do: you can't `.await` it, because
//   `dyn Future` might not be safe to move ("cannot be unpinned").
//
//   Types that are safe to move anyway implement `Unpin` (almost all
//   normal types do). Pin only restricts the ones that don't.
//
// 🎯 YOUR TASK
//
//   `quests` builds a list of different futures, and `run_all` awaits
//   them in order. Make it compile with pinned boxes.

use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

async fn dig(depth: u32) -> u32 {
    depth * 10
}

fn quests() -> Vec<Box<dyn Future<Output = u32>>> {
    vec![Box::new(dig(1)), Box::new(async { 5 }), Box::new(dig(3))]
}

async fn run_all(quests: Vec<Box<dyn Future<Output = u32>>>) -> u32 {
    let mut total = 0;
    for q in quests {
        total += q.await;
    }
    total
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
    println!("{}", block_on(run_all(quests())));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_quests() {
        assert_eq!(block_on(run_all(quests())), 45);
        assert_eq!(block_on(run_all(Vec::new())), 0);
    }
}
