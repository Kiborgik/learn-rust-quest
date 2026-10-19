use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 18,
    title: "The Observatory: data parallelism with rayon",
    notes: "starters/lesson18/README.md",
    boss_trophy: ("boss_l18", "Stargazer"),
    exercises: &[
        Exercise {
            id: "l18_01_par_iter",
            lesson: 18,
            code: "18-1",
            title: "One Word Faster",
            path: "exercises/lesson18/01_par_iter/src/lib.rs",
            story: "The observatory has many telescopes and one tired astronomer. Time to \
                    put every lens to work at once.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "\"no method named `par_iter`\": the trait isn't in scope. Add `use rayon::prelude::*;`.",
                "`(0..limit).into_par_iter().filter(|n| is_prime(*n)).count()`",
            ],
            quiz: Some(Quiz {
                question: "Why can't a rayon closure push into a plain `Vec` from outside?",
                options: &[
                    "Vec isn't allowed in rayon",
                    "Many threads would mutate it at once: the borrow checker refuses",
                    "rayon closures can't capture anything",
                    "It can, it's just slower",
                ],
                answer: 1,
                explain: "par_iter closures run on several threads, so they may only share \
                          `Sync` data, which rules out data races. Collect instead.",
            }),
        },
        Exercise {
            id: "l18_02_collect",
            lesson: 18,
            code: "18-2",
            title: "Order From Chaos",
            path: "exercises/lesson18/02_collect/src/lib.rs",
            story: "A thousand scouts report back in random order, but the star chart must \
                    keep its order.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`heroes.par_iter().map(|(n, l)| format!(\"{n} (lvl {l})\")).collect()`",
                "`gems.par_chunks(size).map(|c| c.iter().sum()).collect()`",
                "`hp.par_iter_mut().for_each(|h| *h = (*h + amount).min(100));`",
            ],
            quiz: Some(Quiz {
                question: "`v.par_iter().map(f).collect::<Vec<_>>()`: what order are the results in?",
                options: &[
                    "random",
                    "the same order as v",
                    "sorted",
                    "the order threads finished",
                ],
                answer: 1,
                explain: "rayon tracks indexes, so collect keeps the input order even though \
                          the work runs out of order.",
            }),
        },
        Exercise {
            id: "l18_03_reduce",
            lesson: 18,
            code: "18-3",
            title: "Many Hands, One Answer",
            path: "exercises/lesson18/03_reduce/src/lib.rs",
            story: "Each telescope counts its own patch of sky. Someone has to add up the \
                    tallies without anyone fighting over the pen.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`hits.par_iter().copied().reduce(|| 0, |a, b| a.max(b))`",
                "census: `.fold(HashMap::new, |mut m, k| { *m.entry(k.to_string()).or_insert(0) += 1; m })`",
                "then `.reduce(HashMap::new, |mut a, b| { for (k, v) in b { *a.entry(k).or_insert(0) += v; } a })`",
            ],
            quiz: Some(Quiz {
                question: "Which operation is NOT safe for a parallel `reduce`?",
                options: &[
                    "addition",
                    "max",
                    "subtraction",
                    "merging two HashMaps of counts",
                ],
                answer: 2,
                explain: "reduce merges pieces in an unknown tree shape, so the operation must \
                          be associative. (a-b)-c differs from a-(b-c).",
            }),
        },
        Exercise {
            id: "l18_04_sort",
            lesson: 18,
            code: "18-4",
            title: "The Great Sorting",
            path: "exercises/lesson18/04_sort/src/lib.rs",
            story: "Fifty thousand heroes want to know their rank by sundown.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Use `heroes.par_sort_by(...)` from rayon's prelude.",
                "`heroes.par_sort_by(|a, b| b.level.cmp(&a.level).then(a.name.cmp(&b.name)));`",
            ],
            quiz: Some(Quiz {
                question: "What does a STABLE sort guarantee?",
                options: &[
                    "It never panics",
                    "Equal elements keep their original relative order",
                    "It runs in constant time",
                    "It uses no extra memory",
                ],
                answer: 1,
                explain: "Stable sorts keep ties in input order. Unstable sorts may swap them, \
                          and are usually a bit faster.",
            }),
        },
        Exercise {
            id: "l18_05_join",
            lesson: 18,
            code: "18-5",
            title: "Divide and Conquer",
            path: "exercises/lesson18/05_join/src/lib.rs",
            story: "Split the sky in two. Hand each half to a different astronomer. Split \
                    again, until every patch is small enough to count alone.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`let (a, b) = v.split_at(v.len() / 2); let (x, y) = rayon::join(|| tree_max(a), || tree_max(b)); x.max(y)`",
                "count_paths: `if w + h > 12 { let (a, b) = rayon::join(|| count_paths(w - 1, h), \
                 || count_paths(w, h - 1)); a + b } else { count_paths(w - 1, h) + count_paths(w, h - 1) }`",
            ],
            quiz: Some(Quiz {
                question: "Why add a cutoff (\"small inputs: just loop\") to a recursive join?",
                options: &[
                    "join panics on small slices",
                    "Splitting has a cost; tiny tasks are faster done directly",
                    "To avoid stack overflow only",
                    "It's required by rayon",
                ],
                answer: 1,
                explain: "Each split costs a little scheduling work. Below some size, that \
                          overhead beats the gain.",
            }),
        },
        Exercise {
            id: "l18_06_pools",
            lesson: 18,
            code: "18-6",
            title: "Hire Your Own Crew",
            path: "exercises/lesson18/06_pools/src/lib.rs",
            story: "The king lends you two astronomers, not the whole guild. Work with the \
                    crew you're given.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();`",
                "`pool.install(|| (job(), rayon::current_num_threads()))`",
            ],
            quiz: Some(Quiz {
                question: "Inside `pool.install(|| ...)`, where do par_iter calls run?",
                options: &[
                    "on the global pool",
                    "on `pool`",
                    "on the calling thread only",
                    "on a new pool each time",
                ],
                answer: 1,
                explain: "install makes `pool` the current pool for everything inside the closure.",
            }),
        },
        Exercise {
            id: "l18_07_boss",
            lesson: 18,
            code: "BOSS",
            title: "The Swarm of Stars",
            path: "exercises/lesson18/07_boss/src/lib.rs",
            story: "A million stars break loose and swarm across the sky. The astronomer's \
                    one candle won't last the night. Light every core.",
            check: Check::CargoTest,
            xp: 400,
            hints: &[
                "Head 1 is 18-3's census, with `flat_map_iter(|t| t.split_whitespace())` (or a loop \
                 inside fold) and `.to_lowercase()`.",
                "`stars.par_iter().max_by_key(|s| s.brightness)` and \
                 `stars.par_iter().filter(|s| s.brightness >= min).map(|s| s.name.as_str()).collect()`",
                "mandel: `(0..size).into_par_iter().map(|row| (0..size).filter(|&col| inside(col, row)).count()).sum()` \
                 with the test's inner loop as `inside`.",
                "`masses.par_chunks(4096).map(|c| c.iter().sum::<u64>()).sum()`",
            ],
            quiz: Some(Quiz {
                question: "When does rayon usually NOT make code faster?",
                options: &[
                    "CPU-heavy work on big data",
                    "Tiny inputs or work that waits on I/O",
                    "Sorting large slices",
                    "Independent calculations per item",
                ],
                answer: 1,
                explain: "rayon is for CPU-bound work. For small inputs the overhead wins; for \
                          waiting on network or disk, use async (next lessons).",
            }),
        },
    ],
};
