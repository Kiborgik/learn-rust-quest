// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Clockwork Oracle                          400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Oracle answers only to those who understand how waiting works.
//   Build the pieces of a tiny async runtime yourself. No helpers this
//   time: std only.
//
//   ⚔️  HEAD 1: `yield_now()`: a future that returns Pending once (after
//        waking itself), then Ready(()).
//
//   ⚔️  HEAD 2: `block_on(fut)`: run a future to completion on this
//        thread. Waker = a type implementing `Wake` that unparks the
//        thread; between polls, `thread::park()`.
//
//   ⚔️  HEAD 3: `join_all(futs) -> JoinAll<T>`: a future over
//        `Vec<Pin<Box<dyn Future<Output = T>>>>` that polls every
//        unfinished one on each poll, and gives all outputs IN ORDER.
//        (Keep a `Vec<Option<T>>` of results.)
//
//   ⚔️  HEAD 4: `CountPolls<F>`: wraps a future; its output is
//        (inner output, how many times the inner future was polled).
//
//   ⚔️  HEAD 5: `with_budget(fut, max) -> Budget<F>`: Ready(Some(v)) if
//        the inner future finishes within `max` polls, Ready(None) as
//        soon as it has used them up.
//
//   Tip: storing the inner future as `Pin<Box<F>>` keeps your wrapper
//   `Unpin`, so `self.get_mut()` works in poll.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Your code goes here ↓



fn main() {
    println!("The Oracle ticks. Run the tests to consult it.");
}

// ─── The Oracle's tests: don't change these! ─────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    async fn slow(steps: u32, value: u32) -> u32 {
        for _ in 0..steps {
            yield_now().await;
        }
        value
    }

    #[test]
    fn head_1_and_2_yield_and_block() {
        assert_eq!(block_on(async { 5 }), 5);
        assert_eq!(block_on(slow(3, 9)), 9);
    }

    #[test]
    fn head_3_join_all() {
        let futs: Vec<Pin<Box<dyn Future<Output = u32>>>> =
            vec![Box::pin(slow(3, 1)), Box::pin(slow(0, 2)), Box::pin(slow(1, 3))];
        assert_eq!(block_on(join_all(futs)), vec![1, 2, 3]);
        assert_eq!(block_on(join_all(Vec::<Pin<Box<dyn Future<Output = u32>>>>::new())), vec![]);
    }

    #[test]
    fn head_4_count_polls() {
        assert_eq!(block_on(CountPolls::new(slow(0, 7))), (7, 1));
        assert_eq!(block_on(CountPolls::new(slow(4, 7))), (7, 5));
    }

    #[test]
    fn head_5_budget() {
        assert_eq!(block_on(with_budget(slow(2, 7), 3)), Some(7));
        assert_eq!(block_on(with_budget(slow(5, 7), 3)), None);
        assert_eq!(block_on(CountPolls::new(with_budget(slow(5, 7), 3))), (None, 3));
    }
}
