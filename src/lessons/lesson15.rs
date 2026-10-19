use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 15,
    title: "The Archive: serde, JSON & TOML",
    notes: "starters/lesson15/README.md",
    boss_trophy: ("boss_l15", "Archive Keeper"),
    exercises: &[
        Exercise {
            id: "l15_01_serialize",
            lesson: 15,
            code: "15-1",
            title: "Into the Scroll",
            path: "exercises/lesson15/01_serialize/src/lib.rs",
            story: "The Archive stores every hero on scrolls of JSON. First you must learn \
                    to write yourself down.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "\"the trait `Serialize` is not implemented for `Hero`\": derive it.",
                "`Item` is inside `Hero`, so it needs `#[derive(Serialize)]` too.",
                "`serde_json::to_string_pretty(hero).unwrap()`",
            ],
            quiz: Some(Quiz {
                question: "How does serde know the fields of your struct?",
                options: &[
                    "Runtime reflection",
                    "The derive macro generates code at compile time",
                    "It reads the source file",
                    "You list them in Cargo.toml",
                ],
                answer: 1,
                explain: "`#[derive(Serialize)]` writes a Serialize impl for your type at \
                          compile time. Rust has no runtime reflection.",
            }),
        },
        Exercise {
            id: "l15_02_deserialize",
            lesson: 15,
            code: "15-2",
            title: "Reading Old Scrolls",
            path: "exercises/lesson15/02_deserialize/src/lib.rs",
            story: "The oldest scrolls were written by a careless scribe who used odd \
                    names. You can read them, if you know the old words.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "The error says \"missing field `health`\": the JSON calls it \"hp\".",
                "`#[serde(rename = \"hp\")] pub health: u32,`",
                "`#[serde(rename = \"type\")] pub kind: String,`",
            ],
            quiz: Some(Quiz {
                question: "The JSON has an extra field your struct doesn't have. By default...",
                options: &[
                    "Deserializing fails",
                    "The field is ignored",
                    "It panics",
                    "It's stored in a hidden map",
                ],
                answer: 1,
                explain: "Unknown fields are ignored unless you add \
                          `#[serde(deny_unknown_fields)]`.",
            }),
        },
        Exercise {
            id: "l15_03_attributes",
            lesson: 15,
            code: "15-3",
            title: "Fine Print",
            path: "exercises/lesson15/03_attributes/src/lib.rs",
            story: "A JavaScript wizard sends you settings scrolls. They're written in \
                    camelCase, and half the lines are missing.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Above the struct: `#[serde(rename_all = \"camelCase\")]`.",
                "Also above the struct: `#[serde(default)]`, which uses your `impl Default`.",
                "On `nickname`: `#[serde(skip_serializing_if = \"Option::is_none\")]`.",
            ],
            quiz: Some(Quiz {
                question: "`#[serde(default)]` on the whole struct takes missing values from...",
                options: &[
                    "zero for every field",
                    "the struct's `Default` impl",
                    "the previous value",
                    "an environment variable",
                ],
                answer: 1,
                explain: "On the container it uses `Default::default()` of the struct. On a \
                          single field it uses that field type's default.",
            }),
        },
        Exercise {
            id: "l15_04_enums",
            lesson: 15,
            code: "15-4",
            title: "Many Shapes, One Scroll",
            path: "exercises/lesson15/04_enums/src/lib.rs",
            story: "The game server speaks in spells, each one labelled with its type. \
                    Your enum must speak the same tongue.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Compare your output with the expected one: the tag goes in a \"type\" field.",
                "`#[serde(tag = \"type\")]` makes the enum internally tagged.",
                "Add `rename_all = \"snake_case\"`: `#[serde(tag = \"type\", rename_all = \"snake_case\")]`.",
            ],
            quiz: Some(Quiz {
                question: "How does `#[serde(untagged)]` decide which variant to use?",
                options: &[
                    "By a \"type\" field",
                    "It tries each variant in order until one fits",
                    "By the variant's index",
                    "It always picks the first",
                ],
                answer: 1,
                explain: "Untagged enums try every variant in declaration order. It's flexible, \
                          but errors get vague and it's slower.",
            }),
        },
        Exercise {
            id: "l15_05_value",
            lesson: 15,
            code: "15-5",
            title: "Shapeless Data",
            path: "exercises/lesson15/05_value/src/lib.rs",
            story: "Some scrolls follow no format at all. You read them the way a scout \
                    reads a map: one landmark at a time.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`save[\"party\"][0][\"name\"].as_str()`. Missing keys give Null, then None.",
                "`save[\"party\"].as_array().map_or(0, |p| p.iter().map(|h| h[\"gold\"].as_u64() \
                 .unwrap_or(0)).sum())`",
                "`json!({ \"event\": kind, \"xp\": xp })`",
            ],
            quiz: Some(Quiz {
                question: "What is `json!({\"a\": 1})[\"b\"][\"c\"]`?",
                options: &["a panic", "Value::Null", "None", "a compile error"],
                answer: 1,
                explain: "Indexing a Value never panics. Missing keys and wrong types give Null.",
            }),
        },
        Exercise {
            id: "l15_06_toml",
            lesson: 15,
            code: "15-6",
            title: "The Config Tome",
            path: "exercises/lesson15/06_toml/src/lib.rs",
            story: "The Archive's config is written in TOML, like every Cargo.toml in the \
                    land. Same serde, different ink.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "The test reads `cfg.party`. Add `pub party: Vec<Member>` to Config.",
                "Next error: unknown variant `hard`. The variants are `Hard` etc.",
                "On Difficulty: `#[serde(rename_all = \"lowercase\")]`.",
            ],
            quiz: Some(Quiz {
                question: "In TOML, what does `[[party]]` create?",
                options: &[
                    "a nested table",
                    "one element of an array of tables (a Vec)",
                    "a comment",
                    "an enum variant",
                ],
                answer: 1,
                explain: "Each `[[party]]` block adds one table to the `party` array, which \
                          maps to `Vec<Member>`.",
            }),
        },
        Exercise {
            id: "l15_07_boss",
            lesson: 15,
            code: "BOSS",
            title: "The Archivist",
            path: "exercises/lesson15/07_boss/src/lib.rs",
            story: "Beneath the stacks waits the Archivist, keeper of every save ever made. \
                    It will not accept a format that forgets the past.",
            check: Check::CargoTest,
            xp: 400,
            hints: &[
                "Derive `Serialize, Deserialize` on all three types. On Item: \
                 `#[serde(tag = \"kind\", rename_all = \"snake_case\")]`. On achievements: \
                 `#[serde(default)]`.",
                "`serde_json::to_string_pretty(s).unwrap()` and \
                 `serde_json::from_str(text).map_err(|e| e.to_string())`.",
                "check_version: `if s.version > CURRENT_VERSION { return Err(format!(\"save version \
                 {} is newer than this game ({CURRENT_VERSION})\", s.version)); } Ok(s)`",
                "load_any: `if text.trim_start().starts_with('{') { load_json(text) } else { \
                 toml::from_str(text).map_err(|e| e.to_string()).and_then(check_version) }`",
            ],
            quiz: Some(Quiz {
                question: "You add a field to your save struct. How do old saves keep loading?",
                options: &[
                    "They can't, it's a breaking change",
                    "Mark the new field `#[serde(default)]`",
                    "Use `#[serde(skip)]` on old fields",
                    "Switch to TOML",
                ],
                answer: 1,
                explain: "With `default`, a missing field gets its default value instead of \
                          an error. That's the usual way to evolve a format.",
            }),
        },
    ],
};
