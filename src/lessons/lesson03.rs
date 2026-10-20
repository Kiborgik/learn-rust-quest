use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 3,
    title: "The Bridge of Ownership: ownership & borrowing",
    notes: "starters/lesson03/README.md",
    boss_trophy: ("boss_l03", "Golem Breaker"),
    exercises: &[
        Exercise {
            id: "l3_01_move",
            lesson: 3,
            code: "3-1",
            title: "Hand-Me-Down Sword",
            path: "exercises/lesson03/01_move.rs",
            story: "At the Bridge of Ownership every item has exactly one keeper. \
                    Your father hands you his old sword, and the bridge remembers who \
                    holds it now.",
            check: Check::Output(
                "Father's sword: Rusty Blade\nThe smith inspects Rusty Blade.\nFerris now wields Rusty Blade.",
            ),
            xp: 100,
            hints: &[
                "\"borrow of moved value: `sword`\": after `let heirloom = sword;` only \
                 `heirloom` owns the String.",
                "Print `{heirloom}` on the \"Father's sword\" line.",
                "`inspect` takes ownership and RETURNS it. Catch it: \
                 `let heirloom = inspect(heirloom);`",
            ],
            quiz: Some(Quiz {
                question: "Which line causes a compile error?\n\n    let a = String::from(\"x\");   // line 1\n    let b = a;                    // line 2\n    let c = b.len();              // line 3\n    let d = a.len();              // line 4",
                options: &["line 2", "line 3", "line 4", "none, it compiles"],
                answer: 2,
                explain: "Line 2 moves the String from `a` to `b`. Using `a` afterwards \
                          (line 4) is a use-after-move error.",
            }),
        },
        Exercise {
            id: "l3_02_clone_copy",
            lesson: 3,
            code: "3-2",
            title: "The Mirror Forge",
            path: "exercises/lesson03/02_clone_copy.rs",
            story: "The Mirror Forge can duplicate any item, but magic items must be \
                    duplicated on purpose. Simple trinkets copy themselves.",
            check: Check::Output(
                "hp=100, backup=100\npos=(3, 4), saved=(3, 4)\nspell=Fireball, scroll=Fireball\nFor the crab!\nFor the crab!\noriginal=Shield +1, copy=Shield",
            ),
            xp: 100,
            hints: &[
                "Integers and tuples of integers are `Copy`, so those lines are fine. \
                 `String` is not.",
                "`let scroll = spell.clone();` keeps `spell` usable.",
                "`shout(war_cry.clone());` for the first call; the second can move it.",
            ],
            quiz: Some(Quiz {
                question: "Which assignment MOVES instead of copying?",
                options: &[
                    "let b = a;  // a: (i32, bool)",
                    "let b = a;  // a: [u8; 4]",
                    "let b = a;  // a: char",
                    "let b = a;  // a: String",
                ],
                answer: 3,
                explain: "String owns heap memory, so it is not `Copy`. Tuples and arrays \
                          of Copy types, and chars, are all `Copy`.",
            }),
        },
        Exercise {
            id: "l3_03_borrow",
            lesson: 3,
            code: "3-3",
            title: "Borrowed, Not Stolen",
            path: "exercises/lesson03/03_borrow.rs",
            story: "The librarian lets you read the spellbook, but you must give it \
                    back. Reading doesn't mean owning.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "The callers pass `&spell`, a reference. Make the parameters references too: \
                 `spell: &String`.",
                "Methods like `.len()` and `.starts_with()` work the same through a reference.",
                "`fn total_length(a: &String, b: &String) -> usize { a.len() + b.len() }`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    fn f(s: &String) -> usize { s.len() }\n\n    let s = String::from(\"crab\");\n    let n = f(&s) + f(&s);\n    println!(\"{n} {s}\");",
                options: &["It doesn't compile", "8 crab", "4 crab", "8"],
                answer: 1,
                explain: "Borrowing doesn't move, so `s` can be lent twice and still printed. \
                          4 + 4 = 8.",
            }),
        },
        Exercise {
            id: "l3_04_mut_borrow",
            lesson: 3,
            code: "3-4",
            title: "The One-Writer Rule",
            path: "exercises/lesson03/04_mut_borrow.rs",
            story: "The bridge's ancient law: many may read the scroll, OR one may \
                    write on it, never both at once. Three apprentices broke the law.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "`add_gold` changes the value behind the reference, so it must be \
                 `purse: &mut u32`.",
                "In `forge`, the two `&mut` borrows overlap. Use `first` completely \
                 before creating `second`.",
                "In `polish`, `before` is still used (printed) after the `push_str`. \
                 Move the `println!` above the `push_str`.",
            ],
            quiz: Some(Quiz {
                question: "Does this compile?\n\n    let mut s = String::from(\"a\");\n    let r1 = &s;\n    let r2 = &s;\n    println!(\"{r1}{r2}\");\n    let r3 = &mut s;\n    r3.push_str(\"b\");",
                options: &[
                    "No: r1 and r2 conflict",
                    "No: r3 conflicts with r1",
                    "Yes",
                    "No: s must not be mut",
                ],
                answer: 2,
                explain: "Two shared borrows are fine, and they end at their last use \
                          (the println!). After that, a mutable borrow is allowed.",
            }),
        },
        Exercise {
            id: "l3_05_string_str",
            lesson: 3,
            code: "3-5",
            title: "Scrolls and Inscriptions",
            path: "exercises/lesson03/05_string_str.rs",
            story: "Scrolls (String) are yours to write on. Inscriptions (&str) are \
                    carved into the walls: you may read them, never change them.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "`shout` should take `name: &str`. That accepts literals AND `&String`.",
                "You can't `+` two `&str`. Use `format!(\"Sir {name}\")`, or \
                 `String::from(\"Sir \") + name`.",
                "`new_scroll` must return an owned String: `\"Blank scroll\".to_string()`.",
            ],
            quiz: Some(Quiz {
                question: "Which function signature is the most idiomatic for READING text?",
                options: &[
                    "fn f(s: String)",
                    "fn f(s: &String)",
                    "fn f(s: &str)",
                    "fn f(s: &mut String)",
                ],
                answer: 2,
                explain: "`&str` accepts literals, `&String` and slices, and it doesn't \
                          take ownership.",
            }),
        },
        Exercise {
            id: "l3_06_slices",
            lesson: 3,
            code: "3-6",
            title: "Slicing the Map",
            path: "exercises/lesson03/06_slices.rs",
            story: "A huge treasure map is too heavy to carry. Instead you look at \
                    just the part you need through a magic window: a slice.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "`first_word`: `&s[0..i]` stops BEFORE the space; `i + 1` includes it.",
                "`middle`: `&items[1..items.len() - 1]`",
                "`sum`: `let mut t = 0; for n in numbers { t += n; } t`",
                "`keep_first_word`: convert to an owned String BEFORE clearing: \
                 `let word = first_word(&s).to_string(); s.clear(); word`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let a = [1, 2, 3, 4, 5];\n    let s = &a[1..4];\n    println!(\"{} {}\", s.len(), s[0]);",
                options: &["3 2", "4 1", "3 1", "4 2"],
                answer: 0,
                explain: "1..4 covers indexes 1, 2, 3 (three items: [2, 3, 4]). \
                          The slice's own index 0 is 2.",
            }),
        },
        Exercise {
            id: "l3_07_boss",
            lesson: 3,
            code: "BOSS",
            title: "The Borrow Checker Golem",
            path: "exercises/lesson03/07_boss.rs",
            story: "The bridge rumbles to life. The Borrow Checker Golem rises from \
                    its stones. It doesn't hit hard, but it rejects every wrong \
                    signature you write.",
            check: Check::Test,
            xp: 250,
            hints: &[
                "Look at each call: `count_char(\"banana\", 'a')` → `text: &str`; \
                 `add_title(&mut name)` → `name: &mut String`; \
                 `double_all(&mut nums)` → `nums: &mut [i32]`.",
                "`take_and_shout(s: String) -> String { format!(\"{}!\", s.to_uppercase()) }`",
                "`longest_len(words: &[&str]) -> usize`: loop and keep the max \
                 `if w.len() > best { best = w.len(); }`.",
                "Bonus: `let mid = loot.len() / 2;` then sum `&loot[..mid]` and `&loot[mid..]`.",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    fn grow(v: &mut i32) { *v *= 3; }\n\n    let mut x = 2;\n    grow(&mut x);\n    grow(&mut x);\n    println!(\"{x}\");",
                options: &["6", "18", "2", "It doesn't compile"],
                answer: 1,
                explain: "Each call triples x through the mutable reference: 2 → 6 → 18. \
                          The borrows don't overlap, so this is fine.",
            }),
        },
    ],
};
