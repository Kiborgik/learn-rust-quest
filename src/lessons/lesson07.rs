use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 7,
    title: "The Fortress of Failure: error handling",
    notes: "starters/lesson07/README.md",
    boss_trophy: ("boss_l07", "Gatebreaker"),
    exercises: &[
        Exercise {
            id: "l7_01_panic",
            lesson: 7,
            code: "7-1",
            title: "When All Is Lost",
            path: "exercises/lesson07/01_panic.rs",
            story: "The road to the Fortress of Failure crosses a rotten bridge. Some \
                    failures you simply can't recover from, but you can at least \
                    scream something useful on the way down.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`#[should_panic(expected = \"...\")]` checks that the panic message \
                 CONTAINS that text.",
                "`if weight > 100 { panic!(\"the bridge collapsed under {weight} kg\"); }`",
                "`find_door(room).expect(\"no secret door here\")`",
            ],
            quiz: Some(Quiz {
                question: "Which situation is the BEST fit for `panic!`?",
                options: &[
                    "The user typed letters into a number field",
                    "A config file is missing",
                    "An internal invariant was broken: a bug in your own code",
                    "A network request timed out",
                ],
                answer: 2,
                explain: "Panics are for bugs and impossible states. Expected failures \
                          should be returned as `Result` so the caller can handle them.",
            }),
        },
        Exercise {
            id: "l7_02_result",
            lesson: 7,
            code: "7-2",
            title: "Two Paths",
            path: "exercises/lesson07/02_result.rs",
            story: "At the fortress gate every action has two possible outcomes: \
                    success or a polite, informative refusal.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "Both outcomes must be wrapped: the success value is `Ok(gold - price)`.",
                "An `if` without `else` can't produce a value, so use `return Err(...);` \
                 or add an `else` branch.",
                "`match text.parse::<u32>() { Ok(0) => Err(\"level must be at least 1\".to_string()), \
                 Ok(n) => Ok(n), Err(_) => Err(\"not a number\".to_string()) }`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let r: Result<u32, String> = \"8\".parse::<u32>()\n        .map_err(|e| e.to_string());\n    match r {\n        Ok(n) if n > 5 => println!(\"big\"),\n        Ok(_) => println!(\"small\"),\n        Err(e) => println!(\"{e}\"),\n    }",
                options: &["small", "8", "an error message", "big"],
                answer: 3,
                explain: "\"8\" parses to Ok(8), and the guard 8 > 5 holds.",
            }),
        },
        Exercise {
            id: "l7_03_question_mark",
            lesson: 7,
            code: "7-3",
            title: "The ? Rune",
            path: "exercises/lesson07/03_question_mark.rs",
            story: "Carved on the fortress walls is a single glowing rune: `?`. \
                    Whoever masters it never writes the same `match` twice.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`parse()` returns a Result. Add `?` to unwrap it or return the error: \
                 `let a: u32 = a.trim().parse()?;`",
                "stat_sum: `let mut total = 0; for pair in stats.split_whitespace() { \
                 let (_, v) = pair.split_once('=').unwrap(); total += v.parse::<u32>()?; } Ok(total)`",
                "`let last = text.split_whitespace().last()?; Some(last.len())`",
            ],
            quiz: Some(Quiz {
                question: "Where can you use `s.parse::<u32>()?`?",
                options: &[
                    "In any function",
                    "Only in `main`",
                    "In a function returning `Option<u32>`",
                    "In a function returning a `Result` whose error type can be built from ParseIntError",
                ],
                answer: 3,
                explain: "`?` returns the error early, converting it with `From`, so the \
                          function's return type must be able to hold it.",
            }),
        },
        Exercise {
            id: "l7_04_combinators",
            lesson: 7,
            code: "7-4",
            title: "Shifting Shapes",
            path: "exercises/lesson07/04_combinators.rs",
            story: "The fortress alchemist transforms Options into Results and back \
                    again with a flick of the wrist, and never writes a single `match`.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`inventory().get(item).copied().map(|n| n * 2)`",
                "`doubled(item).ok_or(format!(\"no {item}\"))`",
                "`text.parse::<u32>().map_err(|_| \"bad arrow count\".to_string())`",
                "`text.parse::<u32>().ok().and_then(checked)` and \
                 `inventory().get(item).copied().unwrap_or(0)`",
            ],
            quiz: Some(Quiz {
                question: "What does `Some(4).and_then(|x| if x > 5 { Some(x) } else { None })` give?",
                options: &["Some(4)", "None", "Some(None)", "false"],
                answer: 1,
                explain: "`and_then` runs the closure and returns its result directly, \
                          without nesting. 4 > 5 is false, so the result is None.",
            }),
        },
        Exercise {
            id: "l7_05_custom_error",
            lesson: 7,
            code: "7-5",
            title: "Name Your Foe",
            path: "exercises/lesson07/05_custom_error.rs",
            story: "The fortress mage refuses to say WHY a spell fizzled. Give every \
                    failure a name and a voice.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "Copy the `impl fmt::Display for DoorError` recipe from the comments and \
                 adapt the arms.",
                "`CastError::NotEnoughMana { needed, have } => write!(f, \"need {needed} mana, have {have}\")`",
                "`impl std::error::Error for CastError {}`, and in `cast`: \
                 `if mana < cost { return Err(CastError::NotEnoughMana { needed: cost, have: mana }); }`",
            ],
            quiz: Some(Quiz {
                question: "Which trait gives a type `.to_string()` and `{}` formatting?",
                options: &["Debug", "Display", "Error", "ToOwned"],
                answer: 1,
                explain: "Implementing `Display` enables `{}`, and `.to_string()` comes \
                          for free through a blanket impl.",
            }),
        },
        Exercise {
            id: "l7_06_from",
            lesson: 7,
            code: "7-6",
            title: "The Conversion Circle",
            path: "exercises/lesson07/06_from.rs",
            story: "In the fortress's inner circle, errors of one kind can be \
                    transmuted into another. The `?` rune knows the spell, if you \
                    teach it.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "\"`?` couldn't convert the error to `SaveError`\": write \
                 `impl From<ParseIntError> for SaveError { fn from(e: ParseIntError) -> Self \
                 { SaveError::Corrupt(e) } }`",
                "After that it compiles, but one test wants the raw parse message for the \
                 bonus: errors must not be wrapped as SaveError there.",
                "`fn total_gold(save: &str, bonus: &str) -> Result<u32, Box<dyn Error>>`",
            ],
            quiz: Some(Quiz {
                question: "What happens when `main` returns `Err(e)`?",
                options: &[
                    "Nothing, the error is ignored",
                    "The program panics with a backtrace",
                    "It doesn't compile, main must return ()",
                    "The error's Debug form is printed and the process exits with a non-zero code",
                ],
                answer: 3,
                explain: "`main` can return `Result<(), E>` where E: Debug. On Err it prints \
                          `Error: {e:?}` and exits with code 1.",
            }),
        },
        Exercise {
            id: "l7_07_boss",
            lesson: 7,
            code: "BOSS",
            title: "The Fortress Gatekeeper",
            path: "exercises/lesson07/07_boss.rs",
            story: "The final gate. The Gatekeeper reads every pass with a cold, \
                    precise eye, and for every flaw it names the exact reason.",
            check: Check::Test,
            xp: 300,
            hints: &[
                "Loop over `text.split(';')`. For each field: \
                 `let (k, v) = field.split_once('=').ok_or(PassError::Malformed(field.to_string()))?;` \
                 then `match k { \"name\" => name = Some(v.to_string()), ... _ => {} }`.",
                "After the loop: `let name = name.ok_or(PassError::MissingField(\"name\"))?;`. \
                 For level, keep the raw text and parse it: `let level: u32 = level_text.parse()?;`.",
                "`impl From<ParseIntError> for PassError { fn from(e: ParseIntError) -> Self \
                 { PassError::BadLevel(e) } }`",
                "`fn enter(text: &str) -> Result<String, PassError> { let p = parse_pass(text)?; \
                 if p.level < 10 { return Err(...) } Ok(format!(\"Welcome, {} of {}!\", p.name, p.guild)) }`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    fn f() -> Option<u32> {\n        let a = Some(2)?;\n        let b: Option<u32> = None;\n        let c = b?;\n        Some(a + c)\n    }\n    println!(\"{:?}\", f());",
                options: &["Some(2)", "None", "It panics", "Some(0)"],
                answer: 1,
                explain: "`b?` hits a None, so the function returns None right there.",
            }),
        },
    ],
};
