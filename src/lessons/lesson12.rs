use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 12,
    title: "The Hive of Threads: fearless concurrency",
    notes: "starters/lesson12/README.md",
    boss_trophy: ("boss_l12", "Queen's Bane"),
    exercises: &[
        Exercise {
            id: "l12_01_spawn",
            lesson: 12,
            code: "12-1",
            title: "Summoning Workers",
            path: "exercises/lesson12/01_spawn.rs",
            story: "The final land: the Hive of Threads, where a thousand workers \
                    buzz at once. First, learn to summon a worker and wait for its return.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "A `JoinHandle<u64>` is not a `u64`. Wait for the thread: `handle.join().unwrap()`.",
                "Spawn into a Vec: `let handles: Vec<_> = (1..=count).map(|i| thread::spawn(move \
                 || i * 100)).collect();`",
                "Then `handles.into_iter().map(|h| h.join().unwrap()).sum()`",
            ],
            quiz: Some(Quiz {
                question: "What type does `thread::spawn(|| \"done\".to_string())` return?",
                options: &[
                    "String",
                    "Result<String, _>",
                    "JoinHandle<String>",
                    "Thread",
                ],
                answer: 2,
                explain: "spawn returns a handle immediately. `join()` later gives \
                          `Result<String, _>`.",
            }),
        },
        Exercise {
            id: "l12_02_move",
            lesson: 12,
            code: "12-2",
            title: "Packing Supplies",
            path: "exercises/lesson12/02_move.rs",
            story: "Workers leaving the hive must carry their own supplies. Nobody \
                    can borrow from a cupboard that might be gone by the time they look.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "E0373: the closure borrows `backpack`. Add `move`: `thread::spawn(move || backpack.len())`.",
                "In the loop, the first iteration moves `map` away. Clone it per thread: \
                 `let map = map.clone();` as the first line in the loop body.",
                "Shadowing a clone inside the loop leaves the outer `map` untouched, so you \
                 can still return it at the end.",
            ],
            quiz: Some(Quiz {
                question: "Why does `thread::spawn` require a `move` closure when using local variables?",
                options: &[
                    "Threads are slow without it",
                    "The thread may outlive the function, so it can't hold borrows of its locals",
                    "move makes the closure thread-safe automatically",
                    "It's only a style convention",
                ],
                answer: 1,
                explain: "spawn requires `'static` data. Moving gives the thread ownership, \
                          so nothing can dangle.",
            }),
        },
        Exercise {
            id: "l12_03_channels",
            lesson: 12,
            code: "12-3",
            title: "Message Bees",
            path: "exercises/lesson12/03_channels.rs",
            story: "Bees don't share one big honey pot. Each one flies out and brings \
                    back a message. The hive just has to know when everyone is home.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "Every bee needs its own sender: put `let tx = tx.clone();` at the top of the loop body.",
                "`for nectar in rx` only ends when EVERY sender is dropped, including \
                 the original `tx`. If the run times out, that's why.",
                "Add `drop(tx);` after the spawning loop.",
            ],
            quiz: Some(Quiz {
                question: "When does `for msg in rx { }` stop?",
                options: &[
                    "After the first message",
                    "When the channel is empty",
                    "When all senders have been dropped and the channel is empty",
                    "After 1 second of silence",
                ],
                answer: 2,
                explain: "An empty channel just waits. The iteration only ends once no sender \
                          can ever send again.",
            }),
        },
        Exercise {
            id: "l12_04_mutex",
            lesson: 12,
            code: "12-4",
            title: "The Royal Seal",
            path: "exercises/lesson12/04_mutex.rs",
            story: "The royal honey jar is sealed. Only the bee holding the seal may \
                    open it, and the seal must be passed on when they're done.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "`*self.honey.lock().unwrap() += amount;`",
                "`let mut h = self.honey.lock().unwrap(); if *h < amount { return false; } \
                 *h -= amount; true`",
                "Royal tax, one lock: `let mut h = self.honey.lock().unwrap(); let tax = *h / 10; \
                 *h -= tax; tax`",
            ],
            quiz: Some(Quiz {
                question: "When is a `MutexGuard`'s lock released?",
                options: &[
                    "When you call unlock()",
                    "When the guard is dropped",
                    "After 1 second",
                    "Never, until the program ends",
                ],
                answer: 1,
                explain: "RAII: the guard unlocks in its Drop. Rust has no `unlock()` method.",
            }),
        },
        Exercise {
            id: "l12_05_arc",
            lesson: 12,
            code: "12-5",
            title: "Shared Honey",
            path: "exercises/lesson12/05_arc.rs",
            story: "A swarm of bees fills one shared jar. The old counting trick (Rc) \
                    falls apart when everyone counts at once.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "\"`Rc<Mutex<u32>>` cannot be sent between threads safely\": replace \
                 `Rc` with `Arc` everywhere.",
                "names: `let names = Arc::new(Mutex::new(Vec::new()));` then spawn bees with \
                 `let names = Arc::clone(&names);` + `names.lock().unwrap().push(format!(\"bee {i}\"))`.",
                "After joining: `let mut v = names.lock().unwrap().clone(); v.sort(); v`",
            ],
            quiz: Some(Quiz {
                question: "Which of these is NOT `Send` (can't be moved to another thread)?",
                options: &["Arc<Mutex<i32>>", "String", "Rc<i32>", "Vec<u8>"],
                answer: 2,
                explain: "Rc's counter isn't atomic, so two threads updating it could corrupt \
                          it. The compiler forbids sending it.",
            }),
        },
        Exercise {
            id: "l12_06_scoped",
            lesson: 12,
            code: "12-6",
            title: "Scoped Swarms",
            path: "exercises/lesson12/06_scoped.rs",
            story: "A swarm that promises to return before sunset may borrow anything \
                    in the hive: no copies, no ceremonies.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "`thread::scope(|s| { let left = s.spawn(|| ...); let right = s.spawn(|| ...); \
                 left.join().unwrap() + right.join().unwrap() })`",
                "count_big: `thread::scope(|s| { for chunk in data.chunks(chunk_size) { \
                 let counter = &counter; s.spawn(move || { ... }); } });`",
                "Inside each thread: `for &x in chunk { if x > 50 { counter.fetch_add(1, \
                 Ordering::Relaxed); } }`",
            ],
            quiz: Some(Quiz {
                question: "Why can scoped threads borrow local variables while `thread::spawn` can't?",
                options: &[
                    "Scoped threads copy everything",
                    "The scope guarantees all its threads are joined before it returns",
                    "Scoped threads run one at a time",
                    "They use Arc internally",
                ],
                answer: 1,
                explain: "Since every scoped thread ends inside the scope, borrows of data \
                          that outlives the scope are safe.",
            }),
        },
        Exercise {
            id: "l12_07_boss",
            lesson: 12,
            code: "BOSS",
            title: "The Hive Queen",
            path: "exercises/lesson12/07_boss.rs",
            story: "The final battle. The Hive Queen rises from her throne of honeycomb, \
                    commanding ten thousand threads at once. Every data race you ever \
                    feared stands behind her, and the compiler stands behind you.",
            check: Check::Test,
            xp: 350,
            hints: &[
                "Head 1: `thread::scope(|s| { let hs: Vec<_> = texts.iter().map(|t| s.spawn(move || \
                 { /* build a BTreeMap for t */ })).collect(); /* merge each h.join().unwrap() */ })`",
                "Head 2: two `mpsc::channel()`s. Stage 1 owns `tx_a`, stage 2 owns `rx_a` + `tx_b`. \
                 Both loops end because each sender is moved into a thread that finishes.",
                "Head 3: per worker, `loop { let job = queue.lock().unwrap().pop(); match job { \
                 Some(j) => results.lock().unwrap().push(j * 2), None => break } }`. \
                 Finally sort a clone of the results.",
                "Head 4: spawn with an owned String: `let s = input.to_string(); thread::spawn(move || \
                 s.parse::<u64>().map_err(|_| format!(\"bad input: {s}\")))`. Then join IN ORDER \
                 and use `?` on each result.",
            ],
            quiz: Some(Quiz {
                question: "What does Rust's \"fearless concurrency\" guarantee at compile time?",
                options: &[
                    "No deadlocks",
                    "No data races",
                    "Threads always finish in order",
                    "No race conditions of any kind",
                ],
                answer: 1,
                explain: "Send/Sync and the borrow rules rule out DATA races. Deadlocks and \
                          higher-level logic races are still possible, so careful design matters.",
            }),
        },
    ],
};
