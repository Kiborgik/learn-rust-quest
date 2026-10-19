use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 28,
    title: "The Tribunal: testing tools",
    notes: "starters/lesson28/README.md",
    boss_trophy: ("boss_l28", "Voice of the Tribunal"),
    exercises: &[
        Exercise {
            id: "l28_01_rstest",
            lesson: 28,
            code: "28-1",
            title: "Trial by Table",
            path: "exercises/lesson28/01_rstest/src/lib.rs",
            story: "The Tribunal judges code by trying it, case after case. The first judge \
                    keeps a table, and the rank ladder fails at every step.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "The failing cases are 499 and 999: `100..499` stops BEFORE 499. Use `..=`: \
                 `0..=99`, `100..=499`, `500..=999`.",
                "fixture: `Party { members: vec![(\"Ferris\".to_string(), 120), (\"Lumi\".to_string(), 1500), \
                 (\"Corro\".to_string(), 40)] }`",
            ],
            quiz: Some(Quiz {
                question: "Where should table test cases concentrate?",
                options: &[
                    "Random values in the middle",
                    "Boundaries and edges: 99/100, 0, the maximum",
                    "Only the happy path",
                    "As many cases as possible, anywhere",
                ],
                answer: 1,
                explain: "Off-by-one bugs live at the edges. One case per side of every boundary.",
            }),
        },
        Exercise {
            id: "l28_02_proptest",
            lesson: 28,
            code: "28-2",
            title: "Trial by Chaos",
            path: "exercises/lesson28/02_proptest/src/lib.rs",
            story: "The second judge doesn't pick cases. She throws thousands of random ones \
                    and keeps the smallest one that breaks your code.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "split loses `total % n` coins. Give one extra coin to each of the first `total % n` heroes.",
                "`let (share, extra) = (total / n, total % n); (0..n).map(|i| share + u32::from(i < extra)).collect()`",
                "clamp_level: `as u8` cuts big numbers. Clamp first, convert after: `level.clamp(1, 99) as u8`.",
            ],
            quiz: Some(Quiz {
                question: "What does proptest's SHRINKING give you?",
                options: &[
                    "Faster tests",
                    "The smallest input that still fails, so the bug is easy to see",
                    "Smaller binaries",
                    "Fewer test cases",
                ],
                answer: 1,
                explain: "A huge random input is hard to debug. Shrinking simplifies it step by step \
                          while it keeps failing.",
            }),
        },
        Exercise {
            id: "l28_03_insta",
            lesson: 28,
            code: "28-3",
            title: "The Painted Record",
            path: "exercises/lesson28/03_insta/src/lib.rs",
            story: "The third judge paints what she sees and compares it with the last \
                    painting. One picture is wrong, and one canvas is still blank.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "The board diff shows [x] and [ ] swapped. Swap the two chars in `let mark = ...`.",
                "Run the tests and look at the `+new results` lines for `receipt`.",
                "Paste them between `@r\"` and `\"`, one per line, indented like the board snapshot.",
            ],
            quiz: Some(Quiz {
                question: "A snapshot test fails after you changed the output format on purpose. What now?",
                options: &[
                    "Delete the test",
                    "Review the diff; if the new output is right, update the snapshot",
                    "Revert your change",
                    "Mark it #[ignore]",
                ],
                answer: 1,
                explain: "Snapshots flag every change. You decide if it's a bug or the new truth.",
            }),
        },
        Exercise {
            id: "l28_04_fakes",
            lesson: 28,
            code: "28-4",
            title: "Stand-ins",
            path: "exercises/lesson28/04_fakes/src/lib.rs",
            story: "The innkeeper's prices depend on time, and the Tribunal can't wait two days \
                    to check a bill. Let them hand the inn a clock they control.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`pub struct Inn<C: Clock> { clock: C, guests: HashMap<String, u64> }`",
                "`impl<C: Clock> Inn<C> { pub fn new(clock: C) -> Inn<C> { Inn { clock, guests: HashMap::new() } } ... }`",
                "Replace both `SystemClock.now()` calls with `self.clock.now()`.",
            ],
            quiz: Some(Quiz {
                question: "What makes code that reads the clock testable?",
                options: &[
                    "Sleeping in tests",
                    "Taking the clock as a parameter behind a trait, so tests pass a fake",
                    "Mocking the operating system",
                    "Global mutable time",
                ],
                answer: 1,
                explain: "Dependency injection: the program passes the real clock, tests pass one \
                          they control.",
            }),
        },
        Exercise {
            id: "l28_05_mockall",
            lesson: 28,
            code: "28-5",
            title: "The Puppeteer",
            path: "exercises/lesson28/05_mockall/src/lib.rs",
            story: "The fourth judge pulls strings: she knows exactly which questions your code \
                    asks the bank, and how often. Paying the party is far too chatty.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Start with `if wages.is_empty() { return Ok(0); }` and `let total: u32 = wages.iter().sum();`",
                "`let have = bank.balance(account); if have < total { return Err(format!(\"not enough gold: \
                 need {total}, have {have}\")); }`",
                "`bank.withdraw(account, total)?; Ok(total)`",
            ],
            quiz: Some(Quiz {
                question: "When is a mock better than a simple fake?",
                options: &[
                    "Always",
                    "When the calls themselves matter: which ones, how many, with what",
                    "When the trait has one method",
                    "Never",
                ],
                answer: 1,
                explain: "Mocks check the conversation. For plain answers, a fake is simpler and \
                          survives refactors better.",
            }),
        },
        Exercise {
            id: "l28_06_boss",
            lesson: 28,
            code: "BOSS",
            title: "The Tribunal",
            path: "exercises/lesson28/06_boss/src/lib.rs",
            story: "All four judges sit together. The guild vault stands accused, and each \
                    judge has a verdict. Read them, then make the vault innocent.",
            check: Check::CargoTest,
            xp: 400,
            hints: &[
                "HEAD 1: full weeks round DOWN: `(to - from) / 7`.",
                "HEAD 2: `d.gold * weeks` overflows u32. Do the math in u64: \
                 `let gold = d.gold as u64; gold + gold * weeks as u64 / 100`",
                "HEAD 3: wrap the announce in `if gold >= BIG_DEPOSIT { ... }`.",
                "HEAD 4: `sorted.sort_by(|a, b| (a.day, &a.hero).cmp(&(b.day, &b.hero)));`",
            ],
            quiz: Some(Quiz {
                question: "In debug builds, what happens when u32 arithmetic overflows?",
                options: &[
                    "It wraps around silently",
                    "It panics",
                    "It saturates at u32::MAX",
                    "It becomes u64",
                ],
                answer: 1,
                explain: "Debug builds panic on overflow; release builds wrap. Use wider types or \
                          checked_*/saturating_* when big values are possible.",
            }),
        },
    ],
};
