use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 5,
    title: "The Shapeshifter Swamp: enums & pattern matching",
    notes: "starters/lesson05/README.md",
    boss_trophy: ("boss_l05", "Hydra Hunter"),
    exercises: &[
        Exercise {
            id: "l5_01_enums",
            lesson: 5,
            code: "5-1",
            title: "Many Forms",
            path: "exercises/lesson05/01_enums.rs",
            story: "The Shapeshifter Swamp changes form beneath your feet: mud, \
                    forest, rock, water. Your travel planner forgot that rivers exist.",
            check: Check::Output("Total travel cost: 11\nSquelch!"),
            xp: 125,
            hints: &[
                "\"non-exhaustive patterns: `Terrain::River` not covered\": add an arm for it.",
                "`Terrain::River => 1,`",
                "`==` on an enum needs `#[derive(PartialEq)]` above `enum Terrain`.",
            ],
            quiz: Some(Quiz {
                question: "You add a new variant `Desert` to `enum Terrain`. What happens to \
                           existing `match` expressions without a `_` arm?",
                options: &[
                    "They silently skip Desert",
                    "They panic at runtime on Desert",
                    "They fail to compile until Desert is handled",
                    "Desert matches the first arm",
                ],
                answer: 2,
                explain: "Matches must be exhaustive, so the compiler points at every match \
                          that needs updating.",
            }),
        },
        Exercise {
            id: "l5_02_enum_data",
            lesson: 5,
            code: "5-2",
            title: "Shapes with Substance",
            path: "exercises/lesson05/02_enum_data.rs",
            story: "A swamp witch sells spells. Each one is a different kind of thing \
                    with its own ingredients, yet they all fit in one spellbook.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`Heal` is a struct-like variant, so it's built with braces: \
                 `Spell::Heal { amount: 20 }`.",
                "`match self { Spell::Fireball(d) => d / 2, Spell::Heal { amount } => amount + 5, ... }`",
                "`Spell::Teleport { x, y } => (x.abs() + y.abs()) as u32`",
            ],
            quiz: Some(Quiz {
                question: "Given `enum E { A(u32), B { v: u32 } }`, which creates a valid value?",
                options: &["E::A { 0: 5 } only", "E::B(5)", "E::A(5)", "E::B { 5 }"],
                answer: 2,
                explain: "Tuple-like variants use parentheses, so `E::A(5)` is right. \
                          Struct-like variants need named fields: `E::B { v: 5 }`.",
            }),
        },
        Exercise {
            id: "l5_03_option",
            lesson: 5,
            code: "5-3",
            title: "Maybe a Treasure",
            path: "exercises/lesson05/03_option.rs",
            story: "The swamp is full of chests. Some hold gold, some hold nothing at \
                    all. In this land, \"nothing\" is a real answer, not a crash.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "The function returns `Option<u32>`, so wrap the values: `1 => Some(50)`.",
                "`find_treasure(chest).unwrap_or(0)`",
                "`match find_treasure(chest) { Some(g) => Some(g * 2), None => None }`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let a: Option<u32> = None;\n    let b = Some(3);\n    println!(\"{}\", a.unwrap_or(10) + b.unwrap_or(10));",
                options: &["13", "3", "20", "It panics"],
                answer: 0,
                explain: "`a` is None, so it falls back to 10. `b` is Some(3), so it gives 3. \
                          10 + 3 = 13.",
            }),
        },
        Exercise {
            id: "l5_04_match",
            lesson: 5,
            code: "5-4",
            title: "The Exhaustive Oracle",
            path: "exercises/lesson05/04_match.rs",
            story: "The Swamp Oracle rates every creature's threat level. It has \
                    forgotten about ghosts, and it underestimates fire elementals.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "Add `Creature::Ghost => 0,`. Then read the WARNING about an \
                 unreachable pattern.",
                "The general `Elemental(_, power)` arm catches fire elementals first. \
                 Put the `Element::Fire` arm ABOVE it.",
                "`Creature::Elemental(e, _) => format!(\"{:?} Elemental\", e)`, \
                 `Creature::Dragon { name, .. } => name.clone()`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let p = (0, 5);\n    let s = match p {\n        (0, y) => y * 2,\n        (x, 5) => x,\n        _ => 99,\n    };\n    println!(\"{s}\");",
                options: &["0", "99", "5", "10"],
                answer: 3,
                explain: "The first arm `(0, y)` matches with y = 5, giving 10. \
                          Later arms are never tried.",
            }),
        },
        Exercise {
            id: "l5_05_if_let",
            lesson: 5,
            code: "5-5",
            title: "Quick Glances",
            path: "exercises/lesson05/05_if_let.rs",
            story: "Some situations don't need a full ritual. A quick glance is \
                    enough: is there something here or not?",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`if let Some(item) = drop { format!(\"You found: {item}\") } else { \
                 String::from(\"Nothing here\") }`",
                "`let Some(n) = find_even(nums) else { return -1; };`",
                "`while let Some(item) = stack.pop() { ... }`",
                "`let mut c = 0; for s in spells { if matches!(s, Spell::Fire(_)) { c += 1; } } c`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let mut v = vec![1, 2, 3];\n    let mut s = 0;\n    while let Some(x) = v.pop() {\n        if x == 2 { break; }\n        s += x;\n    }\n    println!(\"{s} {}\", v.len());",
                options: &["3 1", "6 0", "1 2", "3 2"],
                answer: 0,
                explain: "pop() yields 3 (s = 3), then 2, which breaks the loop. Only [1] is \
                          left, so the length is 1.",
            }),
        },
        Exercise {
            id: "l5_06_guards",
            lesson: 5,
            code: "5-6",
            title: "Guarded Gates",
            path: "exercises/lesson05/06_guards.rs",
            story: "The gates of the Hydra's lair let only some through. The guards \
                    check more than shape: they check conditions.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`match n { x if x < 0 => \"negative\", 0 => \"zero\", x if x % 2 == 0 => \
                 \"even\", _ => \"odd\" }`",
                "Bind with `@`: `l @ 1..=9 => format!(\"Rookie {l}\")`",
                "In `judge`, put the `(20, _)` arm FIRST.",
                "`match hero { Hero { hp: 0, .. } => \"fallen\", Hero { mana: 0, .. } => \
                 \"drained\", Hero { level, .. } if *level >= 10 => \"elite\", _ => \"ready\" }`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let n = 15;\n    let s = match n {\n        x @ 10..=20 if x % 2 == 0 => \"even teen-ish\",\n        10..=20 => \"odd teen-ish\",\n        _ => \"other\",\n    };\n    println!(\"{s}\");",
                options: &[
                    "even teen-ish",
                    "other",
                    "It doesn't compile",
                    "odd teen-ish",
                ],
                answer: 3,
                explain: "15 is in range, but the guard (even?) fails, so matching continues \
                          to the next arm.",
            }),
        },
        Exercise {
            id: "l5_07_boss",
            lesson: 5,
            code: "BOSS",
            title: "The Hydra of Variants",
            path: "exercises/lesson05/07_boss.rs",
            story: "The swamp boils. The Hydra rises: a head for every variant, and \
                    it bites any code that forgets one.",
            check: Check::Test,
            xp: 300,
            hints: &[
                "`fn parse_dir(word: &str) -> Option<Dir> { match word { \"n\" | \"north\" => \
                 Some(Dir::North), ..., _ => None } }`",
                "`impl Hero { fn new() -> Self {...} fn apply(&mut self, cmd: &Command) -> String \
                 { match cmd { Command::Move(dir, steps) => { match dir { Dir::North => self.y += \
                 steps, ... } format!(\"Moved to ({}, {})\", self.x, self.y) } ... } } }`",
                "Attack: `Command::Attack { target, .. } if self.mana < 2 => format!(\"Too tired \
                 to attack {target}\")` as a guarded arm before the normal one.",
                "`run`: `let mut h = Hero::new(); for c in cmds { if *c == Command::Quit { break; } \
                 h.apply(c); } h`. Bonus: `if let Command::Attack { target, .. } = c { return \
                 Some(target.as_str()); }`",
            ],
            quiz: Some(Quiz {
                question: "What does `matches!(Some(7), Some(x) if x > 5)` evaluate to?",
                options: &["Some(7)", "true", "false", "It doesn't compile"],
                answer: 1,
                explain: "`matches!` returns a bool: the pattern matches AND the guard \
                          7 > 5 holds.",
            }),
        },
    ],
};
