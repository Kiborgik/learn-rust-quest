// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 20-5 · Build a Runtime                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A real executor runs many TASKS. The core idea fits in a few lines:
//
//   • A task = a boxed future + a way to get back in line.
//   • The executor has a queue of tasks that are ready to be polled.
//   • Waking a task = putting it back in the queue.
//
//     struct Task {
//         future: Mutex<Option<BoxFuture>>,   // None once finished
//         queue: Sender<Arc<Task>>,           // where to go when woken
//     }
//
//     impl Wake for Task {
//         fn wake(self: Arc<Self>) {
//             self.queue.send(self.clone())...  // "poll me again"
//         }
//     }
//
//   The run loop: take a task from the queue, build a Waker from it
//   (`Waker::from(task.clone())`), poll its future once. If Pending, put
//   the future back in the slot; the task will be queued again when
//   woken. If Ready, drop it: that task is done.
//
//   That's the heart of tokio, async-std and smol. They add I/O and
//   timers that call wake() at the right moment, plus multiple threads.
//
// 🎯 YOUR TASK
//
//   1. Implement `Task::wake`: send the task back into the queue.
//   2. Implement `Executor::run`: keep polling queued tasks until the
//      queue is empty (`try_recv`).

use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

struct Task {
    future: Mutex<Option<BoxFuture>>,
    queue: Sender<Arc<Task>>,
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        todo!()
    }
}

pub struct Executor {
    queue: Sender<Arc<Task>>,
    ready: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn new() -> Self {
        let (queue, ready) = channel();
        Executor { queue, ready }
    }

    pub fn spawn(&self, fut: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Some(Box::pin(fut))),
            queue: self.queue.clone(),
        });
        self.queue.send(task).unwrap();
    }

    /// Polls tasks until no task is ready.
    pub fn run(&self) {
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

async fn yield_now() {
    YieldNow(false).await
}

fn main() {
    let ex = Executor::new();
    ex.spawn(async { println!("hello from a task") });
    ex.run();
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interleaves_tasks() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let ex = Executor::new();
        for name in ["a", "b", "c"] {
            let log = log.clone();
            ex.spawn(async move {
                for i in 1..=2 {
                    log.lock().unwrap().push(format!("{name}{i}"));
                    yield_now().await;
                }
            });
        }
        ex.run();
        assert_eq!(*log.lock().unwrap(), vec!["a1", "b1", "c1", "a2", "b2", "c2"]);
    }

    #[test]
    fn many_tasks_finish() {
        let done = Arc::new(Mutex::new(0));
        let ex = Executor::new();
        for _ in 0..100 {
            let done = done.clone();
            ex.spawn(async move {
                yield_now().await;
                *done.lock().unwrap() += 1;
            });
        }
        ex.run();
        assert_eq!(*done.lock().unwrap(), 100);
    }
}
