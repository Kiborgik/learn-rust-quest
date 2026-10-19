use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 16,
    title: "The Scriptorium: itertools & regex",
    notes: "starters/lesson16/README.md",
    boss_trophy: ("boss_l16", "Golem Scribe"),
    exercises: &[
        Exercise {
            id: "l16_01_itertools",
            lesson: 16,
            code: "16-1",
            title: "More Tools for the Belt",
            path: "exercises/lesson16/01_itertools/src/lib.rs",
            story: "The scribes of the Scriptorium sort, merge and join lists all day. \
                    Their secret: a belt full of extra iterator tools.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`use itertools::Itertools;` is already there. Its methods work on any iterator.",
                "roster: `names.iter().sorted().dedup().join(\", \")`",
                "first_seen: `.unique().map(|s| s.to_string()).collect()`. \
                 podium: `scores.iter().copied().k_smallest(3).collect()`",
            ],
            quiz: Some(Quiz {
                question: "What does `[1, 1, 2, 1].iter().dedup()` yield?",
                options: &["1, 2", "1, 2, 1", "2", "1, 1, 2, 1"],
                answer: 1,
                explain: "dedup only removes ADJACENT repeats. Use `unique()` for a global \
                          \"each value once\".",
            }),
        },
        Exercise {
            id: "l16_02_grouping",
            lesson: 16,
            code: "16-2",
            title: "Sorting the Spoils",
            path: "exercises/lesson16/02_grouping/src/lib.rs",
            story: "After the battle, the spoils need counting: by monster, by streak, \
                    by how fast the heroes grew.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`log.iter().copied().counts()`",
                "`rolls.iter().chunk_by(|r| **r).into_iter().map(|(_, run)| run.count()).max() \
                 .unwrap_or(0)`",
                "`levels.iter().tuple_windows().map(|(a, b)| b - a).max().unwrap_or(0).max(0)`",
            ],
            quiz: Some(Quiz {
                question: "`[1, 1, 2, 1].iter().chunk_by(|x| **x)` makes how many groups?",
                options: &["2", "3", "4", "1"],
                answer: 1,
                explain: "chunk_by groups consecutive runs: [1, 1], [2], [1]. Sort first if you \
                          want one group per key.",
            }),
        },
        Exercise {
            id: "l16_03_regex",
            lesson: 16,
            code: "16-3",
            title: "Rune Patterns",
            path: "exercises/lesson16/03_regex/src/lib.rs",
            story: "The scrolls are full of dice runes: 2d6, 1d20, 10d100. A pattern can \
                    pick them out, if the pattern is right.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`\\d` matches one digit. \"12d20\" has two-digit numbers.",
                "Use `\\d+` (one or more): `r\"(\\d+)d(\\d+)\"`.",
                "all_dice: `re.find_iter(text).map(|m| m.as_str().to_string()).collect()`",
            ],
            quiz: Some(Quiz {
                question: "Why write regexes as `r\"\\d+\"` instead of `\"\\d+\"`?",
                options: &[
                    "Raw strings are faster",
                    "In a raw string, `\\` isn't an escape, so `\\d` reaches the regex as is",
                    "Regex::new requires raw strings",
                    "It enables Unicode",
                ],
                answer: 1,
                explain: "In a normal string you'd need \"\\\\d\". Raw strings keep backslashes as \
                          they are.",
            }),
        },
        Exercise {
            id: "l16_04_lazylock",
            lesson: 16,
            code: "16-4",
            title: "Carve It Once",
            path: "exercises/lesson16/04_lazylock/src/lib.rs",
            story: "The apprentice re-carves the same rune for every page. The master carves \
                    it once and hangs it on the wall.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "E0015: a `static` can't call `Regex::new` at compile time.",
                "`static CALL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r\"\\b[a-z]+!\").unwrap());` \
                 plus `use std::sync::LazyLock;`.",
                "`CALL.find_iter(text).count()`",
            ],
            quiz: Some(Quiz {
                question: "When does the closure inside `LazyLock::new` run?",
                options: &[
                    "At compile time",
                    "When the program starts",
                    "The first time the static is used, once",
                    "Every time it's used",
                ],
                answer: 2,
                explain: "LazyLock runs it on first access, keeps the value, and is safe to use \
                          from many threads.",
            }),
        },
        Exercise {
            id: "l16_05_captures",
            lesson: 16,
            code: "16-5",
            title: "Named Runes",
            path: "exercises/lesson16/05_captures/src/lib.rs",
            story: "A rune with names is easier to read than one with numbers. The scribes \
                    also use runes to fill in letters by template.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`let c = HIT.captures(line)?; Some(Hit { hero: c[\"hero\"].into(), foe: c[\"foe\"].into(), \
                 damage: c[\"dmg\"].parse().ok()? })`",
                "fill: `VAR.replace_all(template, |c: &Captures| ...).into_owned()`",
                "In the closure: `vars.get(&c[\"key\"]).cloned().unwrap_or_else(|| c[0].to_string())`",
            ],
            quiz: Some(Quiz {
                question: "`re.replace_all(text, \"x\")` returns a `Cow<str>`. Why not String?",
                options: &[
                    "To make it async",
                    "It borrows the input when nothing was replaced, so no allocation",
                    "Cow is required by Regex",
                    "Strings can't hold Unicode",
                ],
                answer: 1,
                explain: "Cow (clone-on-write) is Borrowed when there was no match and Owned \
                          only when it had to build a new string.",
            }),
        },
        Exercise {
            id: "l16_06_combos",
            lesson: 16,
            code: "16-6",
            title: "Every Combination",
            path: "exercises/lesson16/06_combos/src/lib.rs",
            story: "The tailor wants every outfit, the captain every duo, the medic the \
                    range of wounds. Three requests, three one-liners.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`hats.iter().cartesian_product(cloaks).map(|(h, c)| format!(\"{h} + {c}\")).collect()`",
                "`party.iter().tuple_combinations().map(|(a, b)| format!(\"{a} & {b}\")).collect()` \
                 (or `.combinations(2)`)",
                "`hits.iter().copied().minmax().into_option()`",
            ],
            quiz: Some(Quiz {
                question: "How many items does `(1..=4).combinations(2)` produce?",
                options: &["4", "6", "8", "16"],
                answer: 1,
                explain: "4 choose 2 = 6 pairs, each in increasing order: [1,2] [1,3] [1,4] \
                          [2,3] [2,4] [3,4].",
            }),
        },
        Exercise {
            id: "l16_07_boss",
            lesson: 16,
            code: "BOSS",
            title: "The Chronicle Golem",
            path: "exercises/lesson16/07_boss/src/lib.rs",
            story: "In the deepest room of the Scriptorium, a stone golem writes battle logs \
                    nobody can read. Make sense of its chronicle.",
            check: Check::CargoTest,
            xp: 400,
            hints: &[
                "Pattern: `r\"\\[r(?<round>\\d+)\\] (?<hero>\\w+) hits (?<foe>\\w+) for (?<dmg>\\d+)(?<crit> \\(crit\\))?\"` \
                 and `crit: c.name(\"crit\").is_some()`.",
                "by hero: sum into a HashMap or use `into_grouping_map`, then \
                 `.sorted_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)))`.",
                "per round: `&hits.iter().chunk_by(|h| h.round)` then sum each run's damage.",
                "report: reuse the other functions. `rounds` = damage_per_round(...).len().",
            ],
            quiz: Some(Quiz {
                question: "Why compile the boss's regex inside a LazyLock?",
                options: &[
                    "Regex::new is not allowed in functions",
                    "So it's built once instead of on every parse call",
                    "LazyLock makes the regex faster to match",
                    "To make it mutable",
                ],
                answer: 1,
                explain: "Matching is fast, compiling is not. Build it once and reuse it.",
            }),
        },
    ],
};
