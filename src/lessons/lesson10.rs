use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 10,
    title: "The Timeless Library: lifetimes",
    notes: "starters/lesson10/README.md",
    boss_trophy: ("boss_l10", "Lich Slayer"),
    exercises: &[
        Exercise {
            id: "l10_01_dangling",
            lesson: 10,
            code: "10-1",
            title: "Ghost References",
            path: "exercises/lesson10/01_dangling.rs",
            story: "The Timeless Library is haunted by ghost references: pointers to \
                    books that were burned long ago. The librarians want them banished.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "A local String dies when the function returns, so you can't return a \
                 reference to it. Return the String itself: `-> String` and `name`.",
                "In `longest_scroll`, `long` is dropped at the inner `}` while `best` \
                 still points at it.",
                "Move `let long = String::from(\"BOOOOOOO\");` above the inner block \
                 (or remove the block entirely).",
            ],
            quiz: Some(Quiz {
                question: "Why does `fn f() -> &str { let s = String::from(\"x\"); &s }` fail?",
                options: &[
                    "&str can't be returned from functions",
                    "`s` is dropped at the end of f, so the reference would dangle",
                    "String can't be borrowed as &str",
                    "It needs `mut`",
                ],
                answer: 1,
                explain: "The owner `s` dies when f returns. Rust rejects any reference \
                          that could outlive its value.",
            }),
        },
        Exercise {
            id: "l10_02_annotations",
            lesson: 10,
            code: "10-2",
            title: "Marking Time",
            path: "exercises/lesson10/02_annotations.rs",
            story: "Library rules: every borrowed book must carry a stamp saying \
                    which shelf it came from and how long it may stay out.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "`fn stronger<'a>(a: &'a str, b: &'a str) -> &'a str`",
                "`first_rune` returns words from `spell` only: \
                 `fn first_rune<'a>(spell: &'a str, rune: &str) -> &'a str`",
                "`pick` never returns `ignore`, so don't tie it to `'a`: \
                 `fn pick<'a>(keep: &'a str, ignore: &str) -> &'a str`",
            ],
            quiz: Some(Quiz {
                question: "What does `fn f<'a>(x: &'a str, y: &'a str) -> &'a str` promise?",
                options: &[
                    "The result lives forever",
                    "x and y have the same length",
                    "The result is valid only while BOTH x and y are valid",
                    "x and y are copied into the result",
                ],
                answer: 2,
                explain: "'a is the overlap of both inputs' lifetimes. The result may borrow \
                          from either one, so it's limited by both.",
            }),
        },
        Exercise {
            id: "l10_03_elision",
            lesson: 10,
            code: "10-3",
            title: "Unwritten Rules",
            path: "exercises/lesson10/03_elision.rs",
            story: "Most library stamps are filled in automatically by a clerk who \
                    follows three simple rules. Sometimes the clerk guesses wrong.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "`motto` and `last_word` are fine as they are (rules 3 and 2).",
                "`echo` may return either `request` or the motto. Tie both to one lifetime: \
                 `fn echo<'a>(&'a self, request: &'a str) -> &'a str`",
                "`which_has` returns `a` or `b`, never `keyword`: \
                 `fn which_has<'a>(a: &'a str, b: &'a str, keyword: &str) -> &'a str`",
            ],
            quiz: Some(Quiz {
                question: "Which signature compiles WITHOUT explicit lifetimes?",
                options: &[
                    "fn f(a: &str, b: &str) -> &str",
                    "fn f() -> &str",
                    "fn f(a: &str, n: u32) -> &str",
                    "fn f(a: &str, b: &str, c: &str) -> &str",
                ],
                answer: 2,
                explain: "Only ONE input is a reference, so rule 2 gives its lifetime to the \
                          output. The others are ambiguous or have no input lifetime.",
            }),
        },
        Exercise {
            id: "l10_04_struct_lifetimes",
            lesson: 10,
            code: "10-4",
            title: "Bookmarks",
            path: "exercises/lesson10/04_struct_lifetimes.rs",
            story: "A bookmark doesn't copy the page, it just points at it. So a \
                    bookmark is useless once the book is gone.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "`struct Bookmark<'a> { page: u32, passage: &'a str }`",
                "`impl<'a> Bookmark<'a> { ... }`",
                "`let end = text.find('.').map(|i| i + 1).unwrap_or(text.len()); \
                 Bookmark { page: 1, passage: &text[..end] }`",
            ],
            quiz: Some(Quiz {
                question: "Given `struct Q<'a> { t: &'a str }`, what does `Q<'a>` guarantee?",
                options: &[
                    "Q owns its text",
                    "A Q value can't outlive the text it points to",
                    "The text is copied into Q",
                    "Q lives for the whole program",
                ],
                answer: 1,
                explain: "The lifetime parameter ties the struct to its borrowed data. The \
                          compiler stops the data from being dropped while a Q still uses it.",
            }),
        },
        Exercise {
            id: "l10_05_static",
            lesson: 10,
            code: "10-5",
            title: "Eternal Scrolls",
            path: "exercises/lesson10/05_static.rs",
            story: "Some scrolls in the Library are carved in stone and will outlast \
                    everyone. The rest are paper that you can own, but not forever.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "Text literals are `&'static str`: `fn rank_name(level: usize) -> &'static str`.",
                "`static RANKS: [&str; 3] = [\"Squire\", \"Knight\", \"Paladin\"];`",
                "Store OWNED Strings: `fn remember(vault: &mut Vault<String>, memory: String)`.",
            ],
            quiz: Some(Quiz {
                question: "Does `String` satisfy the bound `T: 'static`?",
                options: &[
                    "No, a String is freed at some point",
                    "Yes, it owns its data and borrows nothing",
                    "Only String literals do",
                    "Only inside `static` items",
                ],
                answer: 1,
                explain: "`T: 'static` means \"contains no non-static references\". Owned \
                          types always satisfy it, however long they actually live.",
            }),
        },
        Exercise {
            id: "l10_06_multiple",
            lesson: 10,
            code: "10-6",
            title: "Tangled Timelines",
            path: "exercises/lesson10/06_multiple.rs",
            story: "A search party enters the stacks with a scrap of paper (the query) \
                    and comes out holding a page from an ancient tome (the result). \
                    The scrap can go in the bin, but the page is still valid.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "`struct Finder<'t, 'q> { text: &'t str, query: &'q str }`",
                "`impl<'t, 'q> Finder<'t, 'q> { fn find(&self) -> Option<&'t str> { ... } }`",
                "`self.text.split_whitespace().map(|w| if w.contains(self.query) { \
                 format!(\"[{w}]\") } else { w.to_string() }).collect::<Vec<_>>().join(\" \")`",
            ],
            quiz: Some(Quiz {
                question: "In `fn f<'a, 'b>(x: &'a str, y: &'b str) -> &'a str`, what can the body return?",
                options: &[
                    "only something borrowed from x (or a 'static str)",
                    "only something borrowed from y",
                    "either x or y",
                    "nothing, it doesn't compile",
                ],
                answer: 0,
                explain: "The output is tied to 'a. Returning y would need 'b: 'a, which isn't \
                          declared. 'static data always works.",
            }),
        },
        Exercise {
            id: "l10_07_boss",
            lesson: 10,
            code: "BOSS",
            title: "The Librarian Lich",
            path: "exercises/lesson10/07_boss.rs",
            story: "Deep in the Timeless Library the Lich turns its hollow gaze to you. \
                    \"Show me you can read without copying a single letter.\"",
            check: Check::Test,
            xp: 350,
            hints: &[
                "`fn parse(text: &str) -> Scroll<'_> { let mut it = text.lines(); \
                 let title = it.next().unwrap_or(\"\"); Scroll { title, lines: it.collect() } }`",
                "`impl<'a> Scroll<'a> { fn longest_line(&self) -> Option<&'a str> { ... } }`. \
                 `max_by_key` returns the LAST max on ties, so loop with a strict `>` instead.",
                "`fn find(&self, word: &str) -> Option<&'a str> { self.lines.iter().copied() \
                 .find(|l| l.contains(word)) }`",
                "Tokens::next: `let s = self.rest.trim_start(); if s.is_empty() { return None; } \
                 let end = s.find(' ').unwrap_or(s.len()); self.rest = &s[end..]; Some(&s[..end])`",
            ],
            quiz: Some(Quiz {
                question: "Which statement about lifetimes is TRUE?",
                options: &[
                    "Annotating 'a makes a value live longer",
                    "Lifetimes exist at runtime as counters",
                    "Lifetimes only describe relationships the compiler checks; they don't change behaviour",
                    "Every reference needs an explicit lifetime",
                ],
                answer: 2,
                explain: "Lifetimes are purely compile-time descriptions. They never extend, \
                          shorten or track anything at runtime.",
            }),
        },
    ],
};
