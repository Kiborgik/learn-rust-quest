use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 14,
    title: "The Error Forge: thiserror & anyhow",
    notes: "starters/lesson14/README.md",
    boss_trophy: ("boss_l14", "Wraith Banisher"),
    exercises: &[
        Exercise {
            id: "l14_01_thiserror",
            lesson: 14,
            code: "14-1",
            title: "Errors Without the Boilerplate",
            path: "exercises/lesson14/01_thiserror/src/lib.rs",
            story: "The Error Forge makes error types to order. You describe the message; \
                    the forge hammers out the Display and Error impls.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "\"missing #[error(\\\"...\\\")] display attribute\": every variant needs one.",
                "Named fields go in braces: `#[error(\"need {needed} gold, have {have}\")]`.",
                "Tuple fields use their index: `#[error(\"unknown item: {0}\")]`.",
            ],
            quiz: Some(Quiz {
                question: "What does `#[derive(Error)]` from thiserror add at runtime?",
                options: &[
                    "A hidden allocation per error",
                    "Nothing beyond normal Display and Error impls",
                    "A backtrace in every error",
                    "Automatic logging",
                ],
                answer: 1,
                explain: "thiserror only generates the code you'd write by hand. The type \
                          is a plain enum or struct.",
            }),
        },
        Exercise {
            id: "l14_02_from",
            lesson: 14,
            code: "14-2",
            title: "The Automatic Conversion",
            path: "exercises/lesson14/02_from/src/lib.rs",
            story: "A parse error falls into the forge and comes out as a proper save \
                    error, with its origin still stamped on it.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "\"`?` couldn't convert the error to `SaveError`\": there is no `From<ParseIntError>`.",
                "Mark the field: `BadLevel(#[from] ParseIntError)`.",
                "`#[from]` also makes the field the `source()`, so the last test passes too.",
            ],
            quiz: Some(Quiz {
                question: "Which attribute lets `?` convert a ParseIntError into your variant?",
                options: &["#[source]", "#[error]", "#[from]", "#[transparent]"],
                answer: 2,
                explain: "`#[from]` generates `impl From<Inner>`. `#[source]` only records the \
                          cause, without a conversion.",
            }),
        },
        Exercise {
            id: "l14_03_anyhow",
            lesson: 14,
            code: "14-3",
            title: "Any Error Will Do",
            path: "exercises/lesson14/03_anyhow/src/lib.rs",
            story: "The quartermaster doesn't care what went wrong with the ledger, only \
                    that someone hears about it.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`String` has no `From<ParseIntError>`. Change the return type to \
                 `anyhow::Result<u32>`.",
                "hero_level: `for line in roster.lines() { let Some((n, lvl)) = line.split_once(':') \
                 else { continue }; if n == name { return Ok(lvl.trim().parse()?); } }`",
                "After the loop: `Err(anyhow!(\"no hero named {name}\"))`.",
            ],
            quiz: Some(Quiz {
                question: "Which one fits a reusable library's public API best?",
                options: &[
                    "anyhow::Result<T>",
                    "Result<T, String>",
                    "an error enum made with thiserror",
                    "panic!",
                ],
                answer: 2,
                explain: "Callers of a library want to match on specific errors. anyhow is \
                          for applications that just report them.",
            }),
        },
        Exercise {
            id: "l14_04_context",
            lesson: 14,
            code: "14-4",
            title: "Breadcrumbs",
            path: "exercises/lesson14/04_context/src/lib.rs",
            story: "\"invalid digit found in string\". Somewhere. In some file. The guards \
                    want breadcrumbs that lead back to the problem.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`.parse::<u32>().context(\"bad level in save\")?`",
                "load_hero: `read_level(save).with_context(|| format!(\"loading {name}\"))`",
                "`{err:#}` prints every layer joined by \": \", outermost first.",
            ],
            quiz: Some(Quiz {
                question: "Why prefer `with_context(|| format!(...))` over `context(format!(...))`?",
                options: &[
                    "It compiles faster",
                    "The message is only built when there is an error",
                    "It adds a backtrace",
                    "context() doesn't accept Strings",
                ],
                answer: 1,
                explain: "The closure runs only on the error path, so the happy path doesn't \
                          pay for formatting.",
            }),
        },
        Exercise {
            id: "l14_05_bail",
            lesson: 14,
            code: "14-5",
            title: "Turn Back!",
            path: "exercises/lesson14/05_bail/src/lib.rs",
            story: "The dungeon guard has two rules and a short temper. Break one and you're \
                    sent home with a note explaining why.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`ensure!(level >= 5, \"level {level} is too low\");` then \
                 `if keys == 0 { bail!(\"you have no keys\"); }`",
                "Success: `Ok(format!(\"{name} enters with {keys} keys\"))`.",
                "`err.downcast_ref::<ParseIntError>().is_some()`",
            ],
            quiz: Some(Quiz {
                question: "What does `ensure!(cond, \"msg\")` do when cond is false?",
                options: &[
                    "Panics",
                    "Returns Err(anyhow!(\"msg\")) from the function",
                    "Logs a warning",
                    "Nothing in release builds",
                ],
                answer: 1,
                explain: "Unlike assert!, ensure! returns an error instead of panicking.",
            }),
        },
        Exercise {
            id: "l14_06_chains",
            lesson: 14,
            code: "14-6",
            title: "Chains of Cause",
            path: "exercises/lesson14/06_chains/src/lib.rs",
            story: "The rescue failed because the potion was empty because the alchemist \
                    was asleep. Every failure has a cause, and the cause has one too.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "A field named `cause` isn't picked up automatically. Add `#[source]` above it.",
                "`let mut out = vec![]; let mut cur: Option<&dyn StdError> = Some(err); \
                 while let Some(e) = cur { out.push(e.to_string()); cur = e.source(); } out`",
            ],
            quiz: Some(Quiz {
                question: "Which thiserror field is treated as the source WITHOUT any attribute?",
                options: &[
                    "a field named `cause`",
                    "a field named `source`",
                    "the first field",
                    "any field implementing Error",
                ],
                answer: 1,
                explain: "A field called `source` is picked up by name. Others need \
                          `#[source]` or `#[from]`.",
            }),
        },
        Exercise {
            id: "l14_07_boss",
            lesson: 14,
            code: "BOSS",
            title: "The Config Wraith",
            path: "exercises/lesson14/07_boss/src/lib.rs",
            story: "A cold wind blows through the server room. The Config Wraith feeds on \
                    vague error messages. Starve it.",
            check: Check::CargoTest,
            xp: 400,
            hints: &[
                "`#[derive(Debug, Error)] pub enum ConfigError { #[error(\"line {0}: expected key = value\")] \
                 Malformed(usize), ... #[error(\"bad number for {key}\")] BadNumber { key: String, \
                 source: ParseIntError } }`",
                "Loop with `for (i, raw) in text.lines().enumerate()`, skip blank and '#' lines, \
                 `let (k, v) = line.split_once('=').ok_or(ConfigError::Malformed(i + 1))?;`",
                "Numbers: `v.parse().map_err(|source| ConfigError::BadNumber { key: k.into(), source })?`. \
                 Keep `Option`s and finish with `.ok_or(ConfigError::MissingKey(\"name\"))?`.",
                "load: `let cfg = parse_config(text).with_context(|| format!(\"loading {file}\"))?; \
                 ensure!((1..=99).contains(&cfg.level), \"level {} is out of range\", cfg.level); Ok(cfg)`",
            ],
            quiz: Some(Quiz {
                question: "A library function fails deep inside an app. Who should add the \
                           context \"loading hero.cfg\"?",
                options: &[
                    "The library, in its error enum",
                    "The application, where it knows which file it's loading",
                    "Nobody, the source error is enough",
                    "The operating system",
                ],
                answer: 1,
                explain: "The caller knows the bigger picture. Libraries return precise errors, \
                          and apps wrap them with context.",
            }),
        },
    ],
};
