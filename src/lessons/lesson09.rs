use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 9,
    title: "The River of Iterators: closures & iterators",
    notes: "starters/lesson09/README.md",
    boss_trophy: ("boss_l09", "Serpent Tamer"),
    exercises: &[
        Exercise {
            id: "l9_01_closures",
            lesson: 9,
            code: "9-1",
            title: "Pocket Spells",
            path: "exercises/lesson09/01_closures.rs",
            story: "On the banks of the River of Iterators, a hermit teaches pocket \
                    spells: tiny nameless functions that remember where they were cast.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "Parameters need types, and closures are typed by trait bounds: \
                 `fn apply(f: impl Fn(u32) -> u32, x: u32) -> u32`.",
                "The closure in `power_up` ignores `multiplier`: `|x| x * multiplier`.",
                "`let mut v = x; for _ in 0..n { v = f(v); } v`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let base = 10;\n    let f = |x: i32| x + base;\n    let base = 100;\n    println!(\"{}\", f(1));",
                options: &["101", "11", "It doesn't compile", "1"],
                answer: 1,
                explain: "The closure captured the FIRST `base` (10). The later `let base` \
                          is a new, shadowing variable.",
            }),
        },
        Exercise {
            id: "l9_02_fn_traits",
            lesson: 9,
            code: "9-2",
            title: "The Three Vows",
            path: "exercises/lesson09/02_fn_traits.rs",
            story: "Every pocket spell takes one of three vows: to only look, to \
                    change things, or to consume what it holds.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "A closure that changes a captured variable is `FnMut`, and calling it \
                 needs a mutable binding: `let mut record = ...`.",
                "The returned closure borrows `factor`, which dies when the function \
                 returns. Use `move |x| x * factor`.",
                "`log.push(..)` mutates, so the bound must be `mut f: impl FnMut()`.",
            ],
            quiz: Some(Quiz {
                question: "Which trait does this closure implement at most?\n\n    let s = String::from(\"hi\");\n    let c = move || s;",
                options: &["Fn", "FnMut", "FnOnce", "None of them"],
                answer: 2,
                explain: "Calling it moves `s` OUT of the closure, so it can only run once.",
            }),
        },
        Exercise {
            id: "l9_03_iter_basics",
            lesson: 9,
            code: "9-3",
            title: "Downstream",
            path: "exercises/lesson09/03_iter_basics.rs",
            story: "The river flows one drop at a time. Ask for the next drop and \
                    you get it, until the river runs dry.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "`(it.next(), it.next())`",
                "`sum` can produce many types. Annotate: `let sum: u32 = ...`.",
                "`map` is lazy, so nothing runs. Write `for s in stats.iter_mut() { *s += 10; }`.",
                "`let mut names = Vec::new(); for (name, _) in party.into_iter() { names.push(name); } names`",
            ],
            quiz: Some(Quiz {
                question: "How many times does this print \"x\"?\n\n    let v = vec![1, 2, 3];\n    let it = v.iter().map(|n| { println!(\"x\"); n * 2 });",
                options: &["3", "0", "1", "6"],
                answer: 1,
                explain: "Iterators are lazy. Nothing consumes `it`, so the closure never runs.",
            }),
        },
        Exercise {
            id: "l9_04_adapters",
            lesson: 9,
            code: "9-4",
            title: "Filtering the Current",
            path: "exercises/lesson09/04_adapters.rs",
            story: "Fishermen along the river use nets, sieves and dams to shape the \
                    current. Combine them and the river does your work.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "`nums.iter().map(|x| x * x).collect()` and \
                 `powers.iter().copied().filter(|&p| p > 50).collect()`",
                "`names.iter().enumerate().map(|(i, n)| format!(\"{}. {}\", i + 1, n)).collect()`",
                "`names.iter().zip(scores.iter()).map(|(n, s)| (n.to_string(), *s)).collect()`",
                "`items.iter().rev().take(n).copied().collect()`; \
                 `text.split_whitespace().map(|w| w.to_uppercase()).collect::<Vec<_>>().join(\"-\")`",
            ],
            quiz: Some(Quiz {
                question: "What does this produce?\n\n    let v: Vec<i32> = (1..=6)\n        .filter(|x| x % 2 == 1)\n        .map(|x| x * 10)\n        .skip(1)\n        .collect();",
                options: &["[10, 30, 50]", "[30, 50]", "[20, 40, 60]", "[40, 60]"],
                answer: 1,
                explain: "The odd numbers 1, 3, 5 become 10, 30, 50, and skipping one leaves [30, 50].",
            }),
        },
        Exercise {
            id: "l9_05_consumers",
            lesson: 9,
            code: "9-5",
            title: "The Delta",
            path: "exercises/lesson09/05_consumers.rs",
            story: "Where the river meets the sea, all its streams end up in one \
                    place: a single answer.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "`fish.iter().map(|f| f.weight).sum()` and \
                 `fish.iter().max_by_key(|f| f.weight).map(|f| f.name)`",
                "`fish.iter().any(|f| f.weight > limit)` and \
                 `fish.iter().position(|f| f.name == name)`",
                "`log.iter().filter_map(|s| s.parse::<u32>().ok()).sum()`",
                "`nums.iter().fold(1, |acc, x| acc * x)` and \
                 `words.iter().flat_map(|w| w.chars()).collect()`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let r = [5, 1, 4].iter().fold(String::new(), |acc, n| format!(\"{acc}{n}\"));\n    println!(\"{r}\");",
                options: &["10", "514", "541", "\"\""],
                answer: 1,
                explain: "fold starts with an empty String and appends each number in order: \"514\".",
            }),
        },
        Exercise {
            id: "l9_06_custom_iter",
            lesson: 9,
            code: "9-6",
            title: "Build Your Own Stream",
            path: "exercises/lesson09/06_custom_iter.rs",
            story: "A river spirit offers you a deal: dig one channel, \"next\", and \
                    every waterway in the land becomes yours.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "Fibonacci: `let value = self.current; self.current = self.next; \
                 self.next += value; Some(value)`",
                "`Steps` is missing its associated type: `type Item = u32;`.",
                "`Fibonacci::new().take_while(|&x| x < limit).filter(|x| x % 2 == 0).sum()`",
            ],
            quiz: Some(Quiz {
                question: "Which method MUST you write to implement `Iterator`?",
                options: &["iter", "next", "map", "collect"],
                answer: 1,
                explain: "Only `next` (plus `type Item`). All other methods are defaults \
                          built on top of it.",
            }),
        },
        Exercise {
            id: "l9_07_boss",
            lesson: 9,
            code: "BOSS",
            title: "The River Serpent",
            path: "exercises/lesson09/07_boss.rs",
            story: "The River Serpent coils out of the water, its scales scrolling with \
                    endless battle records. Only a true master of streams can read them.",
            check: Check::Test,
            xp: 350,
            hints: &[
                "`log.iter().map(|h| h.damage).sum()`",
                "`log.iter().fold(BTreeMap::new(), |mut m, h| { *m.entry(h.hero.clone()).or_insert(0) \
                 += h.damage; m })`",
                "`damage_by_hero(log).into_iter().max_by_key(|(_, d)| *d).map(|(name, _)| name)`. \
                 Note that on a tie, max_by_key returns the LAST maximum.",
                "Average: `if log.is_empty() { None } else { Some(total_damage(log) as f64 / log.len() as f64) }`. \
                 Segments: store the `next` value; in `next()`, return it and compute the following one.",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let v = [3, 7, 2, 7];\n    let i = v.iter().enumerate()\n        .max_by_key(|(_, x)| **x)\n        .map(|(i, _)| i);\n    println!(\"{:?}\", i);",
                options: &["Some(1)", "Some(3)", "Some(7)", "None"],
                answer: 1,
                explain: "`max_by_key` returns the LAST of several equal maxima, so index 3.",
            }),
        },
    ],
};
