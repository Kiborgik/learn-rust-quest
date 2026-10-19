use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 20,
    title: "The Clockwork Gate: how async works",
    notes: "starters/lesson20/README.md",
    boss_trophy: ("boss_l20", "Oracle Unwound"),
    exercises: &[
        Exercise {
            id: "l20_01_future",
            lesson: 20,
            code: "20-1",
            title: "A Promise of Later",
            path: "exercises/lesson20/01_future.rs",
            story: "Beyond the Clockwork Gate, nothing happens right away. Every answer is \
                    a promise, kept only when someone asks again.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "`let this = self.get_mut(); this.polls += 1;` then check `this.left`.",
                "`if this.left == 0 { return Poll::Ready(\"liftoff\"); }`",
                "Otherwise: `this.left -= 1; cx.waker().wake_by_ref(); Poll::Pending`",
            ],
            quiz: Some(Quiz {
                question: "A future returns Pending but never arranges a wake(). What happens?",
                options: &[
                    "The executor polls it again soon anyway",
                    "It may never be polled again: the task hangs",
                    "It panics",
                    "The compiler rejects it",
                ],
                answer: 1,
                explain: "Executors only re-poll woken tasks. Pending without a wake-up means \
                          waiting forever.",
            }),
        },
        Exercise {
            id: "l20_02_async_await",
            lesson: 20,
            code: "20-2",
            title: "async and .await",
            path: "exercises/lesson20/02_async_await.rs",
            story: "The gatekeeper writes his promises in a shorthand that the clockwork \
                    turns into gears for him.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "\"cannot add `u32` to `impl Future`\": `dig(1)` is still a future. Add `.await`.",
                "`let mut t = 0; for d in depths { t += dig(*d).await; } t`",
                "`async move { x + 1 }`",
            ],
            quiz: Some(Quiz {
                question: "What does calling `dig(1)` (an async fn) do by itself?",
                options: &[
                    "Runs dig on a new thread",
                    "Runs dig and blocks",
                    "Creates a future; nothing runs until it's awaited or polled",
                    "Schedules it on the global runtime",
                ],
                answer: 2,
                explain: "Futures are lazy. Without .await or an executor, the body never runs.",
            }),
        },
        Exercise {
            id: "l20_03_pin",
            lesson: 20,
            code: "20-3",
            title: "Pinned in Place",
            path: "exercises/lesson20/03_pin.rs",
            story: "Some gears hold onto their own teeth. Move them, and they shatter. So \
                    they're pinned to the wall.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "`dyn Future<Output = u32>` cannot be unpinned, so a plain Box can't be awaited.",
                "Use `Vec<Pin<Box<dyn Future<Output = u32>>>>` in both signatures.",
                "Build them with `Box::pin(dig(1))` instead of `Box::new(...)`.",
            ],
            quiz: Some(Quiz {
                question: "Why do some futures need to be pinned?",
                options: &[
                    "To make them faster",
                    "They may hold references into themselves, which moving would break",
                    "Pin makes them thread-safe",
                    "The OS requires it",
                ],
                answer: 1,
                explain: "async blocks can borrow their own locals across .await. Pin promises \
                          the future won't move, so those borrows stay valid.",
            }),
        },
        Exercise {
            id: "l20_04_join",
            lesson: 20,
            code: "20-4",
            title: "Two at Once",
            path: "exercises/lesson20/04_join.rs",
            story: "Two clockwork workers, one key to wind them. Turn one, then the other, \
                    and both get there.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "`if this.a_out.is_none() { if let Poll::Ready(v) = this.a.as_mut().poll(cx) { this.a_out = Some(v); } }`",
                "Do the same for b.",
                "`if this.a_out.is_some() && this.b_out.is_some() { Poll::Ready((this.a_out.take().unwrap(), \
                 this.b_out.take().unwrap())) } else { Poll::Pending }`",
            ],
            quiz: Some(Quiz {
                question: "join on one thread is concurrency, not parallelism. What's the difference?",
                options: &[
                    "None",
                    "Concurrency interleaves tasks; parallelism runs them at the same instant",
                    "Parallelism is only for async",
                    "Concurrency needs more threads",
                ],
                answer: 1,
                explain: "One thread can juggle many tasks (concurrency). Parallelism needs \
                          several cores working at once.",
            }),
        },
        Exercise {
            id: "l20_05_executor",
            lesson: 20,
            code: "20-5",
            title: "Build a Runtime",
            path: "exercises/lesson20/05_executor.rs",
            story: "The gate's great engine is a queue of sleeping tasks and a bell. Ring \
                    the bell and a task gets back in line.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "wake: `let _ = self.queue.send(self.clone());`",
                "run: `while let Ok(task) = self.ready.try_recv() { let mut slot = task.future.lock().unwrap(); ... }`",
                "Inside: `if let Some(mut fut) = slot.take() { let waker = Waker::from(task.clone()); \
                 let mut cx = Context::from_waker(&waker); if fut.as_mut().poll(&mut cx).is_pending() \
                 { *slot = Some(fut); } }`",
            ],
            quiz: Some(Quiz {
                question: "In this executor, what does a task's `wake()` do?",
                options: &[
                    "Polls the future right away",
                    "Puts the task back in the ready queue",
                    "Starts a new thread",
                    "Drops the task",
                ],
                answer: 1,
                explain: "Waking just means \"poll me again\": back into the queue. The run loop \
                          does the polling.",
            }),
        },
        Exercise {
            id: "l20_06_async_closures",
            lesson: 20,
            code: "20-6",
            title: "Try, Try Again",
            path: "exercises/lesson20/06_async_closures.rs",
            story: "The gate's lock sticks. A patient hero tries again, a few times, before \
                    giving up.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "Loop `for _ in 0..tries` and call `attempt().await` each time.",
                "Keep the last error: `let mut last = Err(String::new());` then \
                 `match attempt().await { Ok(v) => return Ok(v), Err(e) => last = Err(e) }`.",
                "After the loop: `last`.",
            ],
            quiz: Some(Quiz {
                question: "An async closure changes a captured counter. Which trait should you accept?",
                options: &["AsyncFn", "AsyncFnMut", "Fn", "FnOnce"],
                answer: 1,
                explain: "Same rules as lesson 9: mutating captures needs the Mut version.",
            }),
        },
        Exercise {
            id: "l20_07_boss",
            lesson: 20,
            code: "BOSS",
            title: "The Clockwork Oracle",
            path: "exercises/lesson20/07_boss.rs",
            story: "At the heart of the gate, the Oracle ticks. It speaks only to those who \
                    can build its gears from nothing.",
            check: Check::Test,
            xp: 400,
            hints: &[
                "yield_now: a `struct YieldNow(bool)` future, like quest 20-4's helper. \
                 block_on: copy the pattern from quest 20-2.",
                "`pub struct JoinAll<T> { futs: Vec<Pin<Box<dyn Future<Output = T>>>>, out: Vec<Option<T>> }`. \
                 In poll, for each index with `out[i].is_none()`, poll and store; Ready when all are Some.",
                "`pub struct CountPolls<F: Future> { inner: Pin<Box<F>>, polls: usize }`: \
                 count, poll inner, map Ready(v) to Ready((v, polls)).",
                "Budget: like CountPolls, but if Pending and `used == max`, return `Poll::Ready(None)`.",
            ],
            quiz: Some(Quiz {
                question: "What does an async runtime like tokio add on top of what you built?",
                options: &[
                    "The Future trait itself",
                    "I/O and timers that call wake() at the right time, plus multi-threaded scheduling",
                    "The async keyword",
                    "Pin",
                ],
                answer: 1,
                explain: "The language gives futures and async/await. Runtimes add executors that \
                          know when sockets and timers are ready.",
            }),
        },
    ],
};
