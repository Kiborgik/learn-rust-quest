use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 2,
    title: "The Winding Caves: control flow",
    notes: "starters/lesson02/README.md",
    boss_trophy: ("boss_l02", "Troll Toppler"),
    exercises: &[
        Exercise {
            id: "l2_01_if_else",
            lesson: 2,
            code: "2-1",
            title: "Fork in the Tunnel",
            path: "exercises/lesson02/01_if_else.rs",
            story: "Beyond the village lie the Winding Caves. The very first tunnel \
                    splits in two, and only a hero who can make decisions gets through.",
            check: Check::Output(
                "The tunnel is lit.\nYou take the golden tunnel.\nFerris feels nervous.",
            ),
            xp: 100,
            hints: &[
                "Rust has no truthiness: an integer is not a bool. Compare it: `torches > 0`.",
                "Both branches of an `if` expression must have the same type. \
                 The `else` branch returns `0` (an integer) but the other returns text.",
                "`else { \"the dark tunnel\" }`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let n = 4;\n    let s = if n % 2 == 0 { \"even\" } else { \"odd\" };\n    println!(\"{s}\");",
                options: &["odd", "It doesn't compile", "even", "true"],
                answer: 2,
                explain: "`if` is an expression: the chosen branch's value is bound to `s`. \
                          4 % 2 == 0, so it's \"even\".",
            }),
        },
        Exercise {
            id: "l2_02_loop",
            lesson: 2,
            code: "2-2",
            title: "The Endless Echo",
            path: "exercises/lesson02/02_loop.rs",
            story: "Ferris shouts into a bottomless cavern. The echo keeps coming back \
                    louder and louder. Count the shouts, and catch the loudest echo.",
            check: Check::Output("After 7 shouts the echo reached 128."),
            xp: 100,
            hints: &[
                "The error says `()` doesn't implement `Display`: the loop currently \
                 evaluates to `()` because `break;` carries no value.",
                "`break some_value;` makes the whole `loop` evaluate to `some_value`.",
                "`break echo;`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let mut i = 0;\n    let v = loop {\n        i += 3;\n        if i > 10 { break i - 1; }\n    };\n    println!(\"{v}\");",
                options: &["11", "12", "10", "9"],
                answer: 0,
                explain: "i goes 3, 6, 9, 12. At 12 the loop breaks with 12 - 1 = 11.",
            }),
        },
        Exercise {
            id: "l2_03_while",
            lesson: 2,
            code: "2-3",
            title: "Torchlight Countdown",
            path: "exercises/lesson02/03_while.rs",
            story: "Your torch is sputtering. Count down the last seconds of light before \
                    the cave goes dark, but don't count one second too many!",
            check: Check::Output("5...\n4...\n3...\n2...\n1...\nDarkness!"),
            xp: 100,
            hints: &[
                "`torch` changes inside the loop, so it needs to be declared `mut`.",
                "Once it compiles, compare the output: there's one line too many. \
                 Look at the loop condition.",
                "`while torch > 0 {`",
            ],
            quiz: Some(Quiz {
                question: "How many times does \"tick\" print?\n\n    let mut n = 10;\n    while n > 1 {\n        println!(\"tick\");\n        n /= 2;\n    }",
                options: &["5", "4", "3", "2"],
                answer: 2,
                explain: "n goes 10 → 5 → 2 → 1. The body runs for 10, 5 and 2: three ticks. \
                          Integer division truncates.",
            }),
        },
        Exercise {
            id: "l2_04_for_ranges",
            lesson: 2,
            code: "2-4",
            title: "Counting Crystals",
            path: "exercises/lesson02/04_for_ranges.rs",
            story: "The cave walls glitter with crystals. A greedy goblin offers to \
                    count them for you. He is, unsurprisingly, terrible at math.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "`1..n` stops BEFORE n. To include n use `1..=n`.",
                "`for i in (2..=n).step_by(2) { total += i; }`",
                "`for i in (1..=n).rev() { if i % k == 0 { return i; } }` then `0` \
                 after the loop.",
                "`let mut sum = 0; for g in gems { sum += g; } sum`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    for i in (0..10).step_by(4) {\n        print!(\"{i} \");\n    }",
                options: &["0 4 8 ", "4 8 ", "0 4 8 12 ", "0 4 "],
                answer: 0,
                explain: "The range 0..10 stops before 10; stepping by 4 yields 0, 4 and 8.",
            }),
        },
        Exercise {
            id: "l2_05_labels",
            lesson: 2,
            code: "2-5",
            title: "The Nested Mines",
            path: "exercises/lesson02/05_labels.rs",
            story: "Deep in the caves, abandoned mines run in rows of tunnels. Some \
                    tunnels have collapsed, and some gems are cursed. Dig carefully.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "A plain `break` only exits the inner `for cell in row` loop.",
                "Label the outer loop: `'mine: for row in mine {` and use `break 'mine;`.",
                "For `safe_rows`: `let mut safe = 0; 'rows: for row in mine { for cell in row \
                 { if cell == 13 { continue 'rows; } } safe += 1; } safe`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let mut count = 0;\n    'outer: for a in 0..3 {\n        for b in 0..3 {\n            if b == 1 { continue 'outer; }\n            count += 1;\n        }\n    }\n    println!(\"{count}\");",
                options: &["9", "6", "0", "3"],
                answer: 3,
                explain: "For each `a`, only b = 0 increments count before `continue 'outer` \
                          skips to the next `a`. 3 values of a → 3.",
            }),
        },
        Exercise {
            id: "l2_06_match_basics",
            lesson: 2,
            code: "2-6",
            title: "The Sorting Stone",
            path: "exercises/lesson02/06_match_basics.rs",
            story: "An ancient stone judges every roll of the dice. Lately it has been \
                    refusing some numbers entirely, and it's being stingy with the \
                    'rare' label too.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "\"non-exhaustive patterns\": a `u32` can be any number, so `match` needs \
                 a catch-all `_ => \"invalid\"` arm at the end.",
                "Once it compiles, `rarity(19)` returns the wrong thing. `15..19` \
                 EXCLUDES 19; use `15..=19`.",
                "`match enemy_level { 0 => 0, 1..=5 => 10, 6..=10 => 25, _ => 100 }`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let x = 5;\n    let s = match x {\n        1..=4 => \"low\",\n        4..=9 => \"mid\",\n        _ => \"high\",\n    };\n    println!(\"{s}\");",
                options: &["low", "mid", "high", "It doesn't compile"],
                answer: 1,
                explain: "Arms are checked top to bottom: 5 isn't in 1..=4 but is in 4..=9. \
                          Overlapping ranges are allowed (rustc may warn about them).",
            }),
        },
        Exercise {
            id: "l2_07_boss",
            lesson: 2,
            code: "BOSS",
            title: "The Cave Troll",
            path: "exercises/lesson02/07_boss.rs",
            story: "A massive Cave Troll blocks the only exit. It chants, it stomps, it \
                    roars, and it won't move until you prove you can tame every loop.",
            check: Check::Test,
            xp: 250,
            hints: &[
                "Check the most specific case first: `if n % 15 == 0` before `% 3` / `% 5`.",
                "Collatz: `let mut steps = 0; while n != 1 { n = if n % 2 == 0 { n / 2 } \
                 else { 3 * n + 1 }; steps += 1; } steps` (make the parameter `mut n: u64`).",
                "Grades: `match score { 90..=100 => 'S', 75..=89 => 'A', 50..=74 => 'B', \
                 0..=49 => 'C', _ => '?' }`",
                "Bonus: `let mut i = 1; loop { if i * i > n { break i * i; } i += 1; }`",
            ],
            quiz: Some(Quiz {
                question: "Which of these does NOT compile?",
                options: &[
                    "let a = loop { break 5; };",
                    "let b = if true { 1 } else { 2 };",
                    "let c = while false { };",
                    "let d = if 1 { 1 } else { 2 };",
                ],
                answer: 3,
                explain: "Conditions must be `bool`, and `1` is an integer. (`while` evaluates \
                          to `()`, which is a perfectly valid, if useless, value.)",
            }),
        },
    ],
};
