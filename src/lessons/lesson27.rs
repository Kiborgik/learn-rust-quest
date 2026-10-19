use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 27,
    title: "The Spellwright: macro_rules!",
    notes: "starters/lesson27/README.md",
    boss_trophy: ("boss_l27", "Author of the Grimoire"),
    exercises: &[
        Exercise {
            id: "l27_01_first_macro",
            lesson: 27,
            code: "27-1",
            title: "First Incantation",
            path: "exercises/lesson27/01_first_macro.rs",
            story: "The Spellwright writes spells that write code. Your first one squares a \
                    number, and already it misfires.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "\"cannot find macro `square`\": move the `macro_rules! square` block above `fn main`.",
                "`$x * $x` pastes `roll()` twice. Store it once: `let v = $x; v * v`.",
                "A block that returns a value needs double braces in the rule: `($x:expr) => {{ let v = $x; v * v }};`",
            ],
            quiz: Some(Quiz {
                question: "`twice!(roll())` expands to `roll() + roll()`. How often does roll run?",
                options: &["once", "twice", "zero times", "it doesn't compile"],
                answer: 1,
                explain: "Macros paste code, not values. Bind the argument to a local once if it \
                          appears more than once.",
            }),
        },
        Exercise {
            id: "l27_02_repetition",
            lesson: 27,
            code: "27-2",
            title: "Many Voices",
            path: "exercises/lesson27/02_repetition.rs",
            story: "A spell for one target is a cantrip. Real spells take as many targets as \
                    you dare to name.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "hashmap: add `$(,)?` after the repetition: `($($key:expr => $value:expr),* $(,)?)`.",
                "max_of: add a second rule `($x:expr, $($rest:expr),+) => {{ ... }};` below the first.",
                "In it: `let a = $x; let b = max_of!($($rest),+); if a > b { a } else { b }`",
            ],
            quiz: Some(Quiz {
                question: "What does `$( ... ),+` match?",
                options: &[
                    "exactly one item",
                    "zero or more items, comma separated",
                    "one or more items, comma separated",
                    "an optional comma",
                ],
                answer: 2,
                explain: "`*` is zero or more, `+` one or more, `?` zero or one. The comma is the \
                          separator.",
            }),
        },
        Exercise {
            id: "l27_03_fragments",
            lesson: 27,
            code: "27-3",
            title: "Shapes of Words",
            path: "exercises/lesson27/03_fragments.rs",
            story: "A spell that builds whole structures must know a name from a type from \
                    a value. Get them mixed up and nothing stands.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "Use `$name:ident`, `$field:ident` and `$ty:ty` in the pattern.",
                "In the impl: `pub fn field_names() -> &'static [&'static str] { &[$( stringify!($field) ),*] }`",
            ],
            quiz: Some(Quiz {
                question: "Which fragment specifier matches a struct or function NAME?",
                options: &["expr", "ty", "ident", "literal"],
                answer: 2,
                explain: "ident is a bare name. ty is a type, expr a value-producing expression.",
            }),
        },
        Exercise {
            id: "l27_04_hygiene",
            lesson: 27,
            code: "27-4",
            title: "Clean Hands",
            path: "exercises/lesson27/04_hygiene.rs",
            story: "A good spell leaves no mess behind, and it shows up wherever it's called. \
                    These two do neither.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "bind: the variable name must come from the caller: `let $name = $value;`.",
                "Inside `mod spells`, after the macro: `pub(crate) use bolt;`",
            ],
            quiz: Some(Quiz {
                question: "A macro does `let tmp = ...;` and the caller also has a `tmp`. What happens?",
                options: &[
                    "The caller's tmp is overwritten",
                    "Compile error: duplicate name",
                    "Nothing: macro hygiene keeps them apart",
                    "Undefined behavior",
                ],
                answer: 2,
                explain: "Names invented inside a macro_rules! body live in their own scope.",
            }),
        },
        Exercise {
            id: "l27_05_recursion",
            lesson: 27,
            code: "27-5",
            title: "Echoes",
            path: "exercises/lesson27/05_recursion.rs",
            story: "In the echo cave a spell calls itself, smaller each time, until silence. \
                    Miscount the echoes and the cave answers wrong.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "count: the empty input is zero, so the end rule should give `0usize`.",
                "sum: `() => { 0 };` and `($x:expr $(, $rest:expr)*) => { $x + sum!($($rest),*) };`",
            ],
            quiz: Some(Quiz {
                question: "What stops a recursive macro_rules! macro?",
                options: &[
                    "A return statement",
                    "A rule that matches the smallest input without calling itself",
                    "The borrow checker",
                    "It stops after 3 calls",
                ],
                answer: 1,
                explain: "Like recursion in functions: a base case. Without one you hit the \
                          recursion limit.",
            }),
        },
        Exercise {
            id: "l27_06_impl_gen",
            lesson: 27,
            code: "27-6",
            title: "The Copying Spell",
            path: "exercises/lesson27/06_impl_gen.rs",
            story: "The scribes copy the same impl for every number type by hand. One spell \
                    could do it all, and never make a typo.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "Pattern `($($t:ty),* $(,)?)`, and wrap the whole `impl Zero for $t { ... }` in `$( ... )*`.",
                "`impl_zero!(i32, i64, u8, f64);`",
                "`macro_rules! impl_kind { ($($t:ty => $label:literal),* $(,)?) => { $( impl Kind for $t \
                 { fn kind() -> &'static str { $label } } )* }; }`",
            ],
            quiz: Some(Quiz {
                question: "When is a macro better than a generic impl like `impl<T: Num> Zero for T`?",
                options: &[
                    "Always, macros are faster",
                    "When each type needs slightly different code that generics can't express",
                    "Never",
                    "Only for traits with no methods",
                ],
                answer: 1,
                explain: "Prefer generics. Use a macro when the per-type code differs, like `0 as $t`.",
            }),
        },
        Exercise {
            id: "l27_07_boss",
            lesson: 27,
            code: "BOSS",
            title: "The Grimoire",
            path: "exercises/lesson27/07_boss.rs",
            story: "The scribes want their own language for quest logs, and the compiler \
                    should reject every malformed line. Write the grimoire.",
            check: Check::Test,
            xp: 400,
            hints: &[
                "Main rule: `($($title:literal => $gold:literal gold $([$flag:ident])?),* $(,)?) => { vec![ ... ] };`",
                "Inside the vec: `$( Entry { title: $title, gold: $gold, done: quest_log!(@done $($flag)?) } ),*`",
                "Put the `(@done) => { false };` and `(@done done) => { true };` rules FIRST.",
                "ensure: `($cond:expr, $($msg:tt)+) => { if !$cond { return Err(format!($($msg)+)); } };`",
            ],
            quiz: Some(Quiz {
                question: "Why must the `@done` helper rules come before the main rule?",
                options: &[
                    "Rust requires helpers first",
                    "Rules are tried in order; the main rule could otherwise try to match `@done ...`",
                    "For speed",
                    "They don't have to",
                ],
                answer: 1,
                explain: "The first matching rule wins. Specific rules go above general ones.",
            }),
        },
    ],
};
