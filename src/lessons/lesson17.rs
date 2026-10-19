use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 17,
    title: "The Town Square: CLI apps with clap",
    notes: "starters/lesson17/README.md",
    boss_trophy: ("boss_l17", "Toolmaker"),
    exercises: &[
        Exercise {
            id: "l17_01_parser",
            lesson: 17,
            code: "17-1",
            title: "The Town Crier",
            path: "exercises/lesson17/01_parser/src/main.rs",
            story: "In the town square, everything happens by command. The town crier \
                    needs to understand what people shout at him.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "\"no function named `try_parse_from`\": the struct needs `#[derive(Parser, Debug)]`.",
                "For `--times` and `-t`: put `#[arg(short, long, default_value_t = 1)]` above the field.",
            ],
            quiz: Some(Quiz {
                question: "What's the first item in `Cli::try_parse_from([...])`?",
                options: &[
                    "the first argument",
                    "the program name",
                    "the subcommand",
                    "ignored",
                ],
                answer: 1,
                explain: "Like argv[0], the first item is the program name. clap skips it \
                          when matching arguments.",
            }),
        },
        Exercise {
            id: "l17_02_flags",
            lesson: 17,
            code: "17-2",
            title: "Flags and Banners",
            path: "exercises/lesson17/02_flags/src/main.rs",
            story: "Banners for the parade: some loud, some with titles, some covered in \
                    tags. The painter takes orders from the command line.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "A field without `#[arg]` is positional. A positional bool makes clap panic.",
                "`#[arg(short, long)] loud: bool,`",
                "`#[arg(short = 'g', long = \"tag\")] tags: Vec<String>,`",
            ],
            quiz: Some(Quiz {
                question: "What type gives you an optional `--title <TITLE>`?",
                options: &["String", "bool", "Option<String>", "Vec<String>"],
                answer: 2,
                explain: "Option means \"may be missing\". A plain String would be required.",
            }),
        },
        Exercise {
            id: "l17_03_subcommands",
            lesson: 17,
            code: "17-3",
            title: "Many Commands",
            path: "exercises/lesson17/03_subcommands/src/main.rs",
            story: "The quest board takes many kinds of orders. Each order is a subcommand, \
                    each subcommand a variant.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "The odd error about `Command: Clone` means clap thinks `command` is a plain \
                 value. Tell it it's a subcommand: `#[command(subcommand)]` above the field.",
                "`match &cli.command { Command::Attack { target } => format!(\"{} attacks {target}\", cli.hero), ... }`",
                "`Command::Rest { hours } => format!(\"{} rests {hours} hours\", cli.hero)`",
            ],
            quiz: Some(Quiz {
                question: "A variant named `LongRest` becomes which subcommand?",
                options: &["LongRest", "longrest", "long-rest", "long_rest"],
                answer: 2,
                explain: "clap converts variant names to kebab-case by default.",
            }),
        },
        Exercise {
            id: "l17_04_values",
            lesson: 17,
            code: "17-4",
            title: "Only Valid Choices",
            path: "exercises/lesson17/04_values/src/main.rs",
            story: "The guild registrar refuses bards and heroes of level zero. Let the \
                    parser say no before your code has to.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "The long error about `value_parser` means clap doesn't know how to read a `Class` \
                 from text. Add `ValueEnum` to its derive list.",
                "`#[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=99))]`",
            ],
            quiz: Some(Quiz {
                question: "Why check ranges with a value_parser instead of in your own code?",
                options: &[
                    "It's faster",
                    "Errors come out like every other clap error, and --help can show it",
                    "Your code can't check ranges",
                    "It's required for u8",
                ],
                answer: 1,
                explain: "Validation in the parser gives consistent messages and keeps your \
                          own code working with known-good values.",
            }),
        },
        Exercise {
            id: "l17_05_help",
            lesson: 17,
            code: "17-5",
            title: "Read the Manual",
            path: "exercises/lesson17/05_help/src/main.rs",
            story: "Nobody uses a tool they can't figure out. Every good tool has a manual \
                    page and a version number.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Add a doc comment above the struct: `/// Roll dice like a hero`.",
                "Add `#[command(version, about)]` above the struct. `version` reads Cargo.toml.",
            ],
            quiz: Some(Quiz {
                question: "What does `Cli::parse()` do when the user passes `--help`?",
                options: &[
                    "Returns Err",
                    "Prints the help and exits with code 0",
                    "Panics",
                    "Ignores it",
                ],
                answer: 1,
                explain: "parse() handles help and version itself. try_parse_from() returns \
                          them as errors so tests can check them.",
            }),
        },
        Exercise {
            id: "l17_06_output",
            lesson: 17,
            code: "17-6",
            title: "Write to Any Scroll",
            path: "exercises/lesson17/06_output/src/main.rs",
            story: "The potion counter shouts its results into the void. Teach it to write \
                    to whatever scroll it's given.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Replace `println!(...)` with `writeln!(out, ...)?`.",
                "`writeln!` needs `use std::io::Write;` (already there) and returns a Result.",
            ],
            quiz: Some(Quiz {
                question: "Which type is a handy `impl Write` for capturing output in a test?",
                options: &["String", "Vec<u8>", "&str", "Box<str>"],
                answer: 1,
                explain: "`Vec<u8>` implements io::Write. Turn it into text with \
                          `String::from_utf8`.",
            }),
        },
        Exercise {
            id: "l17_07_boss",
            lesson: 17,
            code: "BOSS",
            title: "The Ledger Lich",
            path: "exercises/lesson17/07_boss/src/main.rs",
            story: "The Ledger Lich counts every coin in the kingdom. To pass its audit, \
                    build a real tool: parsed, saved, and impossible to fool.",
            check: Check::CargoTest,
            xp: 400,
            hints: &[
                "`#[derive(Parser)] struct Cli { #[command(subcommand)] command: Command }` and \
                 `enum Command { Add { item: String, qty: u32, #[arg(long, default_value_t = 0)] price: u32 }, \
                 Remove { item: String, qty: u32 }, List { #[arg(long, value_enum, default_value_t = Sort::Name)] sort: Sort }, Total }`",
                "`#[derive(Serialize, Deserialize, Default)] struct Ledger { items: BTreeMap<String, Entry> }` \
                 with `struct Entry { qty: u32, price: u32 }`. New state: `if state.is_empty() \
                 { Ledger::default() } else { serde_json::from_str(state)? }`.",
                "Remove: `let Some(e) = ledger.items.get_mut(&item) else { bail!(\"no item {item}\") };` then check \
                 qty, subtract, and remove the entry at 0.",
                "Sort by qty: `.sorted_by(|a, b| b.1.qty.cmp(&a.1.qty).then(a.0.cmp(b.0)))`. \
                 Finish with `Ok(serde_json::to_string(&ledger)?)`.",
            ],
            quiz: Some(Quiz {
                question: "Why does `run` take the state as a string and return the new one, \
                           instead of reading and writing the file itself?",
                options: &[
                    "Files are slow in Rust",
                    "So tests can run it without touching the disk",
                    "clap requires it",
                    "serde can't read files",
                ],
                answer: 1,
                explain: "Keeping I/O at the edges (in main) makes the logic easy to test, \
                          the same idea as writing to `impl Write`.",
            }),
        },
    ],
};
