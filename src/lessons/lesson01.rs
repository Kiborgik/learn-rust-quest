use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 1,
    title: "The Village of Bindings: variables, types & functions",
    notes: "starters/lesson01/README.md",
    boss_trophy: ("dragon_slayer", "Dragon Slayer"),
    exercises: &[
        // ───────────────────────────────────────────────────────── 1-1
        Exercise {
            id: "l1_01_hello",
            lesson: 1,
            code: "1-1",
            title: "Wake up, Ferris",
            path: "exercises/lesson01/01_hello.rs",
            story: "Ferris the crab wakes up in a strange village. The first thing \
                    any adventurer must do is introduce themselves... loudly.",
            check: Check::Output("Hello, Rustacean!\nFerris has 10 legs."),
            xp: 100,
            hints: &[
                "Read the compiler error carefully. Rust's compiler is your friend. \
                 It often tells you *exactly* what to change.",
                "`println` is a macro, and macros are called with a `!`: `println!(...)`.",
                "Second line: `println!(\"Ferris has {} legs.\", 10);`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    println!(\"{} + {} = {}\", 2, 2, 2 + 2);",
                options: &["{} + {} = {}", "2 + 2 = 4", "2 2 4", "It doesn't compile"],
                answer: 1,
                explain: "Each `{}` is replaced, in order, by the next argument. \
                          `2 + 2` is evaluated first, so the last one becomes 4.",
            }),
        },
        // ───────────────────────────────────────────────────────── 1-2
        Exercise {
            id: "l1_02_variables",
            lesson: 1,
            code: "1-2",
            title: "Name Your Hero",
            path: "exercises/lesson01/02_variables.rs",
            story: "The village elder wants to write your name in the Book of Heroes. \
                    But the ink only sticks to names declared with `let`.",
            check: Check::Output("Ferris enters the dungeon with 100 HP"),
            xp: 100,
            hints: &[
                "In Rust, every new variable is introduced with `let`.",
                "A variable must have a value before you read it. Give `health` a value: \
                 `let health = 100;`",
            ],
            quiz: Some(Quiz {
                question: "Which line creates a variable `coins` holding 3?",
                options: &[
                    "coins = 3;",
                    "let coins = 3;",
                    "var coins = 3;",
                    "int coins = 3;",
                ],
                answer: 1,
                explain: "`let` is the only way to declare a variable in Rust. \
                          The type (i32 here) is inferred automatically.",
            }),
        },
        // ───────────────────────────────────────────────────────── 1-3
        Exercise {
            id: "l1_03_mutability",
            lesson: 1,
            code: "1-3",
            title: "The Ever-Changing Purse",
            path: "exercises/lesson01/03_mutability.rs",
            story: "You find a treasure chest! But your purse is enchanted with \
                    *immutability*: the gold inside can never change... unless you say so.",
            check: Check::Output("Starting gold: 10\nGold now: 30"),
            xp: 100,
            hints: &[
                "Variables are immutable by default. The error says \
                 \"cannot assign twice to immutable variable\".",
                "Add the `mut` keyword: `let mut gold = 10;`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let mut a = 1;\n    a = a + 1;\n    a *= 10;\n    println!(\"{a}\");",
                options: &["11", "20", "2", "It doesn't compile"],
                answer: 1,
                explain: "a = 1, then 1 + 1 = 2, then 2 * 10 = 20. `a *= 10` is short for `a = a * 10`.",
            }),
        },
        // ───────────────────────────────────────────────────────── 1-4
        Exercise {
            id: "l1_04_shadowing",
            lesson: 1,
            code: "1-4",
            title: "Shadow of the Scroll",
            path: "exercises/lesson01/04_shadowing.rs",
            story: "A messenger hands you a scroll: your new level, written as *text*. \
                    You need it as a *number* to level up. Shadows to the rescue!",
            check: Check::Output("Level up! You are now level 8"),
            xp: 100,
            hints: &[
                "The compiler says \"type annotations needed\": `parse()` can produce \
                 many number types (u8, i32, u64, f64...), so it can't guess which one.",
                "Annotate the new variable: `let level: u32 = level.trim().parse().expect(...)`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let x = 2;\n    {\n        let x = x * 10;\n        println!(\"inner: {x}\");\n    }\n    println!(\"outer: {x}\");",
                options: &[
                    "inner: 20  /  outer: 20",
                    "inner: 20  /  outer: 2",
                    "inner: 2  /  outer: 2",
                    "It doesn't compile",
                ],
                answer: 1,
                explain: "The inner `x` only shadows the outer one until the end of the `{ }` \
                          block. Afterwards the original `x` (2) is visible again.",
            }),
        },
        // ───────────────────────────────────────────────────────── 1-5
        Exercise {
            id: "l1_05_types",
            lesson: 1,
            code: "1-5",
            title: "The Type Smith",
            path: "exercises/lesson01/05_types.rs",
            story: "The village blacksmith forges values into *types*. But somebody \
                    mixed up the molds. Every piece of your gear is broken!",
            check: Check::Output(
                "HP: 300, speed: 1.5, alive: true, rune: R\nPosition: x=10 y=-3\nLast item: potion\nTreasury: 10000001234\nRage: 255",
            ),
            xp: 100,
            hints: &[
                "Fix errors one at a time, top to bottom. A `u8` can only hold 0..=255 \
                 (use `u16` or `u32` for 300).",
                "`1.5` is a float (use `f64`), `true` is a bool, and chars use \
                 SINGLE quotes: 'R'. Double quotes make a string.",
                "Tuples are accessed with `.0`, `.1`... and arrays are indexed from 0, \
                 so the last of 3 items is `[2]`.",
                "Treasury: `pocket_gold as u64 + royal_vault`. Rage: `rage.saturating_add(10)`.",
            ],
            quiz: Some(Quiz {
                question: "Which of these does NOT compile?",
                options: &[
                    "let a: f64 = 3.0;",
                    "let b: char = 'x';",
                    "let c: i32 = 2.5;",
                    "let d: (u8, bool) = (1, true);",
                ],
                answer: 2,
                explain: "Rust never silently converts a float into an integer. \
                          2.5 is a float literal, so it can't be an i32.",
            }),
        },
        // ───────────────────────────────────────────────────────── 1-6
        Exercise {
            id: "l1_06_functions",
            lesson: 1,
            code: "1-6",
            title: "Spells & Functions",
            path: "exercises/lesson01/06_functions.rs",
            story: "The village wizard teaches you to write spells. A spell takes \
                    ingredients (parameters) and produces a result (return value). \
                    Get one semicolon wrong and... *poof*.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "Function parameters always need a type: `fn heal(hp: u32, potion: u32)`.",
                "A trailing `;` turns an expression into a statement that returns `()`. \
                 Remove the semicolon after `attack - defense`.",
                "fn gold_after_battle(gold: u32, enemies: u32) -> u32 { gold + enemies * 15 }",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    fn triple(n: i32) -> i32 { n * 3 }\n\n    let v = {\n        let a = 2;\n        triple(a) + 1\n    };\n    println!(\"{v}\");",
                options: &["6", "7", "9", "It doesn't compile"],
                answer: 1,
                explain: "A `{ }` block is an expression: its value is its last line \
                          (without `;`). triple(2) + 1 = 7.",
            }),
        },
        // ───────────────────────────────────────────────────────── 1-7 BOSS
        Exercise {
            id: "l1_07_boss",
            lesson: 1,
            code: "BOSS",
            title: "The Compiler Dragon",
            path: "exercises/lesson01/07_boss.rs",
            story: "The ground shakes. The Compiler Dragon descends upon the village! \
                    Six heads, each guarding a trap of numbers and types. \
                    No starter code this time. Everything you learned is your weapon.",
            check: Check::Test,
            xp: 250,
            hints: &[
                "Read the tests at the bottom: they show the exact function names, \
                 parameter types and expected results.",
                "`level % 5 == 0` is already a `bool`. Fahrenheit: `c * 1.8 + 32.0` \
                 (float literals need the `.0`).",
                "hp_percent: multiply FIRST, then divide, then cast: \
                 `(hp * 100 / max_hp) as u8`. critical_hit: `base.saturating_mul(multiplier)`.",
                "average: `(hits[0] + hits[1] + hits[2] + hits[3]) as f64 / 4.0`. \
                 dodge: `(pos.1, pos.0)`.",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let x: i32 = 300;\n    println!(\"{}\", x as u8);",
                options: &["255", "44", "300", "It panics"],
                answer: 1,
                explain: "`as` between integers keeps the low bits: 300 - 256 = 44. \
                          It never panics, so prefer `u8::try_from(x)` when data loss matters.",
            }),
        },
    ],
};
