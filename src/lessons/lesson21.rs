use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 21,
    title: "The Courier Guild: async with tokio",
    notes: "starters/lesson21/README.md",
    boss_trophy: ("boss_l21", "Guildmaster Courier"),
    exercises: &[
        Exercise {
            id: "l21_01_runtime",
            lesson: 21,
            code: "21-1",
            title: "Start the Engine",
            path: "exercises/lesson21/01_runtime/src/main.rs",
            story: "The Courier Guild runs on a great engine called tokio. Nothing moves \
                    until someone starts it.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "\"`main` function is not allowed to be `async`\": add `#[tokio::main]` above it.",
                "\"async functions cannot be used for tests\": replace `#[test]` with \
                 `#[tokio::test(start_paused = true)]` on both tests.",
            ],
            quiz: Some(Quiz {
                question: "Why is `std::thread::sleep` a bug inside async code?",
                options: &[
                    "It doesn't compile",
                    "It blocks the worker thread, so other tasks on it stop too",
                    "It sleeps for the wrong time",
                    "It panics on tokio",
                ],
                answer: 1,
                explain: "Async tasks share threads. Blocking one thread freezes every task \
                          scheduled on it. Use tokio::time::sleep.",
            }),
        },
        Exercise {
            id: "l21_02_spawn",
            lesson: 21,
            code: "21-2",
            title: "Send the Scouts",
            path: "exercises/lesson21/02_spawn/src/lib.rs",
            story: "The guild sends a scout to every region at once. Each scout must carry \
                    its own map: the office copy stays home.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Spawned tasks must be 'static: they can't borrow `regions`.",
                "Loop by value: `for region in regions { ... }` and use `async move`.",
                "`tokio::spawn(async move { scout(&region).await })`",
            ],
            quiz: Some(Quiz {
                question: "What does `handle.await` return for a spawned task?",
                options: &[
                    "The task's output directly",
                    "Result<Output, JoinError>, Err if the task panicked or was cancelled",
                    "Option<Output>",
                    "Nothing",
                ],
                answer: 1,
                explain: "A JoinHandle resolves to a Result, so a panic in a task doesn't take \
                          down the caller.",
            }),
        },
        Exercise {
            id: "l21_03_join",
            lesson: 21,
            code: "21-3",
            title: "All Hands",
            path: "exercises/lesson21/03_join/src/lib.rs",
            story: "The smith, the alchemist and the treasurer each have work to do. They \
                    don't have to wait for each other.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`let (sword, potion, gold) = tokio::join!(forge(), brew(), count_gold());`",
                "`let (a, b) = tokio::try_join!(check_armor(armor_ok), check_boots(boots_ok))?; Ok(a + b)`",
            ],
            quiz: Some(Quiz {
                question: "`join!(a(), b())`: do a and b run on different threads?",
                options: &[
                    "Yes, always",
                    "No, they're interleaved on the current task",
                    "Only in release mode",
                    "Only with spawn_blocking",
                ],
                answer: 1,
                explain: "join! polls both futures inside the current task: concurrency \
                          without new tasks. spawn is for separate tasks.",
            }),
        },
        Exercise {
            id: "l21_04_select",
            lesson: 21,
            code: "21-4",
            title: "First Past the Post",
            path: "exercises/lesson21/04_select/src/lib.rs",
            story: "Two riders race for the guild's prize, and the oracle answers only if \
                    you're patient enough.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`timeout(Duration::from_secs(patience), oracle(delay)).await.map_err(|_| \"the oracle is silent\")`",
                "`tokio::select! { r = ride(\"red\", red_secs) => r, b = ride(\"blue\", blue_secs) => b }`",
            ],
            quiz: Some(Quiz {
                question: "What happens to the losing branch of a `select!`?",
                options: &[
                    "It keeps running in the background",
                    "It's dropped, which cancels it",
                    "It panics",
                    "It runs after the winner",
                ],
                answer: 1,
                explain: "Futures that lose a select are dropped and never polled again. \
                          That's async cancellation.",
            }),
        },
        Exercise {
            id: "l21_05_channels",
            lesson: 21,
            code: "21-5",
            title: "Carrier Pigeons",
            path: "exercises/lesson21/05_channels/src/lib.rs",
            story: "Pigeons fly reports back to the guild hall. For questions, the keeper \
                    sends back a single bird with the answer.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`let (tx, mut rx) = mpsc::channel(8);` then for each pigeon \
                 `let tx = tx.clone(); tokio::spawn(async move { tx.send(i * 10).await.unwrap(); });`",
                "Call `drop(tx)` after the loop, then `while let Some(v) = rx.recv().await { sum += v }`.",
                "`let (tx, rx) = oneshot::channel(); tokio::spawn(async move { let _ = tx.send(question.len()); }); rx.await.unwrap()`",
            ],
            quiz: Some(Quiz {
                question: "Why use a BOUNDED channel?",
                options: &[
                    "It's faster",
                    "Backpressure: senders wait when it's full, so memory stays in check",
                    "Unbounded channels don't exist",
                    "Only bounded channels can be cloned",
                ],
                answer: 1,
                explain: "With a bound, a fast producer slows down to the consumer's pace \
                          instead of piling up messages.",
            }),
        },
        Exercise {
            id: "l21_06_mutex",
            lesson: 21,
            code: "21-6",
            title: "Don't Hold the Lock",
            path: "exercises/lesson21/06_mutex/src/lib.rs",
            story: "A courier wanders off with the vault key in his pocket. Nobody else can \
                    open it until he's back.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "\"future cannot be sent between threads safely\": the std guard lives across \
                 `.await`.",
                "Easiest fix: `let counted = count_coins(a).await; *vault.lock().unwrap() += counted;`",
                "`tokio::task::spawn_blocking(move || crunch(n)).await.unwrap()`",
            ],
            quiz: Some(Quiz {
                question: "When is std::sync::Mutex fine in async code?",
                options: &[
                    "Never",
                    "When the lock is held briefly and never across an .await",
                    "Only in tests",
                    "Only with spawn_blocking",
                ],
                answer: 1,
                explain: "A short critical section with no await inside is fine, and even \
                          faster. Use tokio's Mutex when you must hold it across awaits.",
            }),
        },
        Exercise {
            id: "l21_07_boss",
            lesson: 21,
            code: "BOSS",
            title: "The Courier Guild",
            path: "exercises/lesson21/07_boss/src/lib.rs",
            story: "The guildmaster slams a stack of parcels on the desk. \"All of them. \
                    Fast. And never more couriers out than we have.\"",
            check: Check::CargoTest,
            xp: 400,
            hints: &[
                "`let sem = Arc::new(Semaphore::new(couriers));` per parcel: `let sem = sem.clone(); \
                 tokio::spawn(async move { let _permit = sem.acquire_owned().await.unwrap(); deliver(p).await })`",
                "Collect the handles in a Vec, then await them in order: \
                 `for h in handles { out.push(h.await.unwrap()); }`",
                "timeout: `match timeout(Duration::from_secs(max_secs), deliver(p.clone())).await \
                 { Ok(r) => r, Err(_) => Err(format!(\"parcel {} timed out\", p.id)) }`",
                "fastest: spawn `async move { deliver(p).await; id }` into a JoinSet, then \
                 `set.join_next().await.map(|r| r.unwrap())`. Dropping the set cancels the rest.",
            ],
            quiz: Some(Quiz {
                question: "What does a Semaphore with 3 permits guarantee?",
                options: &[
                    "At most 3 tasks exist",
                    "At most 3 holders of a permit at the same time",
                    "Tasks run in order",
                    "3 threads are used",
                ],
                answer: 1,
                explain: "Tasks can be many, but only 3 can hold a permit at once. Others wait \
                          in acquire().",
            }),
        },
    ],
};
