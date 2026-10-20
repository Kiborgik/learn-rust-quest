use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 4,
    title: "The Guild Hall: structs & methods",
    notes: "starters/lesson04/README.md",
    boss_trophy: ("boss_l04", "Guild Champion"),
    exercises: &[
        Exercise {
            id: "l4_01_structs",
            lesson: 4,
            code: "4-1",
            title: "Guild Registration",
            path: "exercises/lesson04/01_structs.rs",
            story: "Across the bridge stands the Guild Hall. Every adventurer gets a \
                    record, and the clerk refuses incomplete forms.",
            check: Check::Output("Ferris the Crabsmith (level 3)\nActive: true"),
            xp: 100,
            hints: &[
                "\"missing field `active`\": every field must be given a value.",
                "`Adventurer { name, class, level: 1, active: true }`",
                "To change `hero.level`, the whole binding must be mutable: `let mut hero = ...`",
            ],
            quiz: Some(Quiz {
                question: "Which is valid Rust?",
                options: &[
                    "struct P { x: i32, y: i32 }  let p = P { x: 1 };",
                    "struct P { x: i32, y: i32 }  let p = P { y: 2, x: 1 };",
                    "struct P { mut x: i32 }",
                    "struct P { x: i32 = 0 }",
                ],
                answer: 1,
                explain: "Fields can be given in any order, but all of them are required. \
                          There are no per-field `mut` or default values.",
            }),
        },
        Exercise {
            id: "l4_02_tuple_structs",
            lesson: 4,
            code: "4-2",
            title: "Coins of the Realm",
            path: "exercises/lesson04/02_tuple_structs.rs",
            story: "The guild treasurer counts gold and silver. Both are just numbers, \
                    and a careless clerk once paid 50 gold instead of 50 silver.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "`g` is a `Gold`, not a number. The number inside is `g.0`.",
                "`total_value` expects `(Gold, Silver)`: `total_value(Gold(2), Silver(50))`.",
                "`p.0.abs() + p.1.abs() + p.2.abs()`",
            ],
            quiz: Some(Quiz {
                question: "Given `struct Meters(f64); struct Feet(f64);`, which compiles?",
                options: &[
                    "let m: Meters = Feet(3.0);",
                    "let x = Meters(1.0) + Meters(2.0);",
                    "let v: f64 = Meters(2.0).0 * 2.0;",
                    "let m = Meters(1);",
                ],
                answer: 2,
                explain: "`.0` reaches the inner f64. Newtypes don't get `+` automatically, \
                          and `1` is an integer, not an f64.",
            }),
        },
        Exercise {
            id: "l4_03_methods",
            lesson: 4,
            code: "4-3",
            title: "Skills & Abilities",
            path: "exercises/lesson04/03_methods.rs",
            story: "The guild alchemist sells you a potion with built-in abilities. \
                    Unfortunately, checking whether it's empty makes it vanish.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "`is_empty(self)` CONSUMES the potion. Reading only needs `&self`.",
                "`drink` changes `self.doses`, so it needs `&mut self`.",
                "`fn label(&self) -> String { format!(\"{} [{}]\", self.name, self.doses) }`",
            ],
            quiz: Some(Quiz {
                question: "A method that should only READ the struct takes...",
                options: &["self", "&mut self", "&self", "Self"],
                answer: 2,
                explain: "`&self` borrows immutably. `&mut self` allows changes, \
                          `self` consumes the instance, and `Self` is the type itself.",
            }),
        },
        Exercise {
            id: "l4_04_associated",
            lesson: 4,
            code: "4-4",
            title: "The Constructor's Oath",
            path: "exercises/lesson04/04_associated.rs",
            story: "Founding a guild requires an oath: a proper constructor. The \
                    guild charter also caps membership at fifty.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "The `name` field is a `String` but the parameter is `&str`: \
                 `name: name.to_string()`.",
                "Associated constants are accessed through the type: `Self::MAX_MEMBERS`.",
                "`fn founders() -> Self { let mut g = Self::new(\"Founders\"); g.members = 3; g }`",
            ],
            quiz: Some(Quiz {
                question: "How do you call an associated function `new` on type `Guild`?",
                options: &["Guild.new()", "new Guild()", "Guild::new()", "Guild->new()"],
                answer: 2,
                explain: "Associated functions (no `self`) are called with `Type::name()`. \
                          Methods are called with `value.name()`.",
            }),
        },
        Exercise {
            id: "l4_05_derive",
            lesson: 4,
            code: "4-5",
            title: "Debug Sight",
            path: "exercises/lesson04/05_derive.rs",
            story: "The guild's appraiser can't see, compare or copy your items. Teach \
                    them a few tricks with a single magic line per item.",
            check: Check::Test,
            xp: 100,
            hints: &[
                "Put `#[derive(...)]` directly above each `struct` line.",
                "`Item` needs `Debug, Clone, PartialEq`.",
                "`Coord` must be copyable: `#[derive(Debug, Clone, Copy, PartialEq)]`.",
            ],
            quiz: Some(Quiz {
                question: "Why can't `Item { name: String, weight: u32 }` derive `Copy`?",
                options: &[
                    "Structs can never be Copy",
                    "u32 is not Copy",
                    "String is not Copy",
                    "It needs Debug first",
                ],
                answer: 2,
                explain: "A type can only be Copy if ALL its fields are Copy. String owns \
                          heap memory, so it isn't.",
            }),
        },
        Exercise {
            id: "l4_06_update_syntax",
            lesson: 4,
            code: "4-6",
            title: "Copycat Armor",
            path: "exercises/lesson04/06_update_syntax.rs",
            story: "The guild armorer copies existing armor with small tweaks. But the \
                    copies keep stealing the original's nameplate...",
            check: Check::Test,
            xp: 100,
            hints: &[
                "`..base` needs an owned `Armor`, but `base` is a `&Armor`. Use `..base.clone()`.",
                "In `lighten`, `..heavy` moves `heavy.name` into `light`. Give `light` \
                 its own copy: `Armor { weight: 5, ..heavy.clone() }`.",
                "`let Armor { name, defense, .. } = armor; format!(\"{name}: {defense}\")`",
            ],
            quiz: Some(Quiz {
                question: "Does this compile?\n\n    #[derive(Debug)]\n    struct S { a: String, b: u32 }\n    let x = S { a: String::from(\"hi\"), b: 1 };\n    let y = S { a: String::from(\"yo\"), ..x };\n    println!(\"{:?}\", x);",
                options: &[
                    "No: x was partially moved",
                    "Yes: only the u32 `b` was copied from x",
                    "No: `..x` must come first",
                    "No: S isn't Clone",
                ],
                answer: 1,
                explain: "`y` provides its own `a`, so only `b` (a Copy u32) comes from x. \
                          Nothing is moved, and x stays fully usable.",
            }),
        },
        Exercise {
            id: "l4_07_boss",
            lesson: 4,
            code: "BOSS",
            title: "The Guildmaster's Trial",
            path: "exercises/lesson04/07_boss.rs",
            story: "The Guildmaster slams a blank ledger on the table. \"Design your own \
                    record, from nothing. If a single method misbehaves, you're out.\"",
            check: Check::Test,
            xp: 250,
            hints: &[
                "Start with `#[derive(Debug, Clone, PartialEq)] struct Adventurer { name: String, \
                 level: u32, hp: u32, max_hp: u32, xp: u32 }` and an `impl Adventurer`.",
                "`self.hp = self.hp.saturating_sub(amount);` and \
                 `self.hp = (self.hp + amount).min(self.max_hp);`",
                "`self.xp += amount; while self.xp >= self.level * 100 { self.xp -= self.level * 100; \
                 self.level += 1; self.max_hp += 10; self.hp = self.max_hp; }`",
                "`let rank = match self.level { 0..=4 => \"Novice\", 5..=9 => \"Veteran\", \
                 _ => \"Legend\" }; format!(\"{} the {}\", self.name, rank)`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let hp: u32 = 5;\n    println!(\"{}\", hp.saturating_sub(9));",
                options: &["-4", "0", "It panics", "4294967292"],
                answer: 1,
                explain: "`saturating_sub` stops at the type's minimum (0 for u32) instead \
                          of overflowing.",
            }),
        },
    ],
};
