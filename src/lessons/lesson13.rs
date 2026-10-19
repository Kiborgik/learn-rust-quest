use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 13,
    title: "The Workshop: Cargo, modules & crates",
    notes: "starters/lesson13/README.md",
    boss_trophy: ("boss_l13", "Tower Builder"),
    exercises: &[
        Exercise {
            id: "l13_01_workshop",
            lesson: 13,
            code: "13-1",
            title: "Open the Workshop",
            path: "exercises/lesson13/01_workshop/src/main.rs",
            story: "With the Hive behind you, you open a workshop of your own. From now on \
                    you build real projects, with folders, manifests and modules.",
            check: Check::CargoRun("Ferris's Workshop v1.0.0\nTools on the wall: 3"),
            xp: 150,
            hints: &[
                "\"failed to resolve: use of unresolved module `tools`\": the file exists, \
                 but nothing declares it. Add `mod tools;` at the top of main.rs.",
                "The version comes from Cargo.toml through `env!(\"CARGO_PKG_VERSION\")`.",
                "In Cargo.toml: `version = \"1.0.0\"`.",
            ],
            quiz: Some(Quiz {
                question: "You add `src/utils.rs` but never write `mod utils;`. What happens?",
                options: &[
                    "Cargo compiles it automatically",
                    "It's ignored: not part of the module tree",
                    "Compile error: unused file",
                    "It becomes a second binary",
                ],
                answer: 1,
                explain: "The module tree starts at main.rs/lib.rs and only follows `mod` \
                          declarations. Undeclared files are simply not compiled.",
            }),
        },
        Exercise {
            id: "l13_02_visibility",
            lesson: 13,
            code: "13-2",
            title: "Private Parts",
            path: "exercises/lesson13/02_visibility/src/forge.rs",
            story: "The forge has rules: anyone may use a sword, but only the smith decides \
                    how sharp it is.",
            check: Check::CargoTest,
            xp: 150,
            hints: &[
                "Items are private to their module by default. The tests live outside `forge`.",
                "Make `new`, `sharpen` and `MAX_SHARPNESS` public with `pub`.",
                "Keep `sharpness: u32` private. Callers read it through `pub fn sharpness(&self)`.",
            ],
            quiz: Some(Quiz {
                question: "A `pub struct` has one private field. Can another module write \
                           `Gem { name, value }` to build it?",
                options: &[
                    "Yes, `pub` on the struct is enough",
                    "Only with `#[derive(Default)]`",
                    "No, every field must be visible to use the literal syntax",
                    "Only inside `unsafe`",
                ],
                answer: 2,
                explain: "With a private field, outsiders must go through your constructor. \
                          That's how Rust types protect their invariants.",
            }),
        },
        Exercise {
            id: "l13_03_paths",
            lesson: 13,
            code: "13-3",
            title: "Nested Rooms",
            path: "exercises/lesson13/03_paths/src/main.rs",
            story: "The workshop has rooms inside rooms. Knowing how to get from one to \
                    another is half the job.",
            check: Check::CargoTest,
            xp: 150,
            hints: &[
                "Inside `storage`, the sibling module `forge` is reached through the parent: \
                 `super::forge::heat()`.",
                "An absolute path works too: `crate::workshop::forge::heat()`.",
                "Inside `mod workshop`, add `pub use forge::heat;`.",
            ],
            quiz: Some(Quiz {
                question: "Inside `mod a { mod b { ... } }`, what does `super::x` refer to?",
                options: &["`b::x`", "`a::x`", "`crate::x` always", "`std::x`"],
                answer: 1,
                explain: "`super` is the parent module. From inside `b`, the parent is `a`.",
            }),
        },
        Exercise {
            id: "l13_04_library",
            lesson: 13,
            code: "13-4",
            title: "The Library Crate",
            path: "exercises/lesson13/04_library/src/lib.rs",
            story: "A good tool is shared. You move the workshop's catalog into a library \
                    so other programs (and tests) can use it.",
            check: Check::CargoTest,
            xp: 150,
            hints: &[
                "\"module `catalog` is private\": the integration test is an outside user. \
                 In lib.rs write `pub mod catalog;`.",
                "`find` returns the first matching book: `BOOKS.iter().find(...)`.",
                "`BOOKS.iter().find(|b| b.title.starts_with(prefix))`",
            ],
            quiz: Some(Quiz {
                question: "Your package is named `dice-tower`. How do integration tests import it?",
                options: &[
                    "use dice-tower::roll;",
                    "use crate::roll;",
                    "use dice_tower::roll;",
                    "mod dice_tower;",
                ],
                answer: 2,
                explain: "The crate name is the package name with `-` replaced by `_`. \
                          Integration tests are separate crates, so `crate::` won't work.",
            }),
        },
        Exercise {
            id: "l13_05_dependency",
            lesson: 13,
            code: "13-5",
            title: "Borrowing Tools",
            path: "exercises/lesson13/05_dependency/src/main.rs",
            story: "Why forge your own dice? The merchants of crates.io have the finest \
                    ones. You just have to put them on your list.",
            check: Check::CargoTest,
            xp: 150,
            hints: &[
                "\"use of unresolved module or unlinked crate `rand`\": add it under \
                 `[dependencies]` in Cargo.toml: `rand = \"0.9\"`.",
                "The first build downloads the crate. It needs internet once.",
                "`1..sides` excludes `sides`. Use `1..=sides`.",
            ],
            quiz: Some(Quiz {
                question: "`rand = \"0.9\"` in Cargo.toml allows which versions?",
                options: &[
                    "exactly 0.9.0",
                    ">=0.9.0 and <0.10.0",
                    "any version >= 0.9",
                    "the newest version, always",
                ],
                answer: 1,
                explain: "Cargo uses caret requirements: compatible updates only. For 0.x \
                          crates, the minor number counts as the breaking one. Cargo.lock \
                          pins the exact version you got.",
            }),
        },
        Exercise {
            id: "l13_06_docs",
            lesson: 13,
            code: "13-6",
            title: "Scrolls of Documentation",
            path: "exercises/lesson13/06_docs/src/lib.rs",
            story: "The workshop's scrolls show examples of every tool. Here the examples \
                    are checked, so a wrong scroll is caught right away.",
            check: Check::CargoTest,
            xp: 150,
            hints: &[
                "A doc test is its own small program. It needs to import your function.",
                "Add `/// use l13_06_docs::split_loot;` as the first line of that example.",
                "title_case: `text.split(' ').map(|w| ...capitalize w...).collect::<Vec<_>>().join(\" \")`.",
            ],
            quiz: Some(Quiz {
                question: "Which command runs the code examples inside `///` comments?",
                options: &["cargo doc", "cargo test", "cargo run", "cargo clippy"],
                answer: 1,
                explain: "`cargo test` runs doc tests along with unit and integration tests. \
                          `cargo doc` only builds the HTML docs.",
            }),
        },
        Exercise {
            id: "l13_07_boss",
            lesson: 13,
            code: "BOSS",
            title: "The Dice Tower",
            path: "exercises/lesson13/07_boss/src/lib.rs",
            story: "At the edge of the workshop rises the Dice Tower. It decides every fate \
                    in the realm, and it was built with a missing floor.",
            check: Check::CargoTest,
            xp: 350,
            hints: &[
                "Head 1: add `pub mod tower;` to lib.rs. Head 2: \
                 `(sides >= 2).then_some(Die { sides })` and `rng.random_range(1..=self.sides)`.",
                "parse: `let text = text.trim(); if text.is_empty() { return Err(Empty) }` then \
                 `let (count, rest) = text.split_once('d').ok_or(ParseError::MissingD)?;`",
                "Split the bonus: `match rest.find(['+', '-']) { Some(i) => (&rest[..i], &rest[i..]), \
                 None => (rest, \"\") }`. Parse \"+3\"/\"-1\" by stripping the sign.",
                "min = count + bonus, max = count * sides + bonus. Tower: \
                 `text.split(',').map(parse).collect::<Result<Vec<_>, _>>()`.",
            ],
            quiz: Some(Quiz {
                question: "What does `iter.collect::<Result<Vec<T>, E>>()` do with a list of Results?",
                options: &[
                    "Keeps only the Ok values",
                    "Returns Ok(vec) if all are Ok, otherwise the first Err",
                    "Panics on the first Err",
                    "Returns a Vec of Results",
                ],
                answer: 1,
                explain: "Collecting into Result stops at the first error. It's the iterator \
                          version of using `?` in a loop.",
            }),
        },
    ],
};
