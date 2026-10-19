use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 6,
    title: "The Merchant's Bazaar: collections",
    notes: "starters/lesson06/README.md",
    boss_trophy: ("boss_l06", "Baron Breaker"),
    exercises: &[
        Exercise {
            id: "l6_01_vec",
            lesson: 6,
            code: "6-1",
            title: "The Backpack",
            path: "exercises/lesson06/01_vec.rs",
            story: "Past the swamp lies a vast bazaar. Before you shop, you need a \
                    backpack that can actually grow.",
            check: Check::Output(
                "Backpack: [\"rope\", \"torch\", \"map\"]\nDropped: map\nSlot 5: empty\nItems left: 2",
            ),
            xp: 125,
            hints: &[
                "`push` and `pop` change the Vec, so it must be `let mut backpack`.",
                "`pop()` returns an `Option`. For this line `backpack.pop().unwrap()` is fine \
                 (we know the bag isn't empty), or use `match`.",
                "`backpack[5]` panics if out of range and doesn't return an Option. \
                 Use `backpack.get(5)`.",
                "`.get()` gives `Option<&&str>`, so the arms don't match `\"empty\"`. \
                 Write `Some(item) => *item`.",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let mut v = vec![10, 20, 30];\n    v.remove(0);\n    v.push(40);\n    println!(\"{:?} {:?}\", v.get(2), v.get(3));",
                options: &[
                    "Some(40) None",
                    "Some(30) Some(40)",
                    "40 It panics",
                    "Some(30) None",
                ],
                answer: 0,
                explain: "After remove(0) and push(40), v = [20, 30, 40]. Index 2 is 40, \
                          and index 3 doesn't exist, so it's None (no panic with get).",
            }),
        },
        Exercise {
            id: "l6_02_vec_iter",
            lesson: 6,
            code: "6-2",
            title: "Inventory Check",
            path: "exercises/lesson06/02_vec_iter.rs",
            story: "The quartermaster checks, repairs and duplicates your gear. He \
                    keeps tripping over the rules about who may touch what.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`for d in durability` yields `&mut u32`. Change the VALUE: `*d += 10`.",
                "In `duplicate_first`, `first` borrows the Vec while `push` modifies it. \
                 Copy the number out instead: `let first = weights[0];`",
                "`n + \"!\"` needs an owned String on the left. Loop by value \
                 (`for n in names`) to own each String.",
            ],
            quiz: Some(Quiz {
                question: "Which loop lets you use `v` again afterwards AND modify its elements?",
                options: &[
                    "for x in v",
                    "for x in &v",
                    "for x in &mut v",
                    "for x in v.clone()",
                ],
                answer: 2,
                explain: "`&mut v` borrows mutably. `v` is moved by the first option, \
                          and the others don't modify the original.",
            }),
        },
        Exercise {
            id: "l6_03_strings",
            lesson: 6,
            code: "6-3",
            title: "The Scribe's Workshop",
            path: "exercises/lesson06/03_strings.rs",
            story: "A scribe copies runes from many lands. Some runes take one byte, \
                    some take four. Counting them is harder than it looks.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "No indexing into strings. `name.chars().next()` gives an `Option<char>`; \
                 add `.unwrap_or('?')`.",
                "`.len()` counts bytes. `s.chars().count()` counts characters.",
                "`s.chars().rev().collect()` builds a String from reversed chars \
                 (or loop and `push` each char).",
                "Acronym: `let mut out = String::new(); for w in phrase.split_whitespace() { \
                 if let Some(c) = w.chars().next() { out.push(c.to_ascii_uppercase()); } } out`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let s = \"añb\";\n    println!(\"{} {}\", s.len(), s.chars().count());",
                options: &["3 3", "4 3", "3 4", "4 4"],
                answer: 1,
                explain: "'ñ' takes 2 bytes in UTF-8, so the string is 4 bytes but 3 characters.",
            }),
        },
        Exercise {
            id: "l6_04_hashmap",
            lesson: 6,
            code: "6-4",
            title: "The Price Ledger",
            path: "exercises/lesson06/04_hashmap.rs",
            story: "The bazaar's prices are written in a ledger, a book where every \
                    item name points to a price. Right now the ledger is blank.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`map.insert(String::from(\"sword\"), 50);` and so on.",
                "`get` returns `Option<&u32>`: `map.get(item).copied().unwrap_or(0)`",
                "`if let Some(p) = map.get_mut(item) { *p += amount; true } else { false }`",
                "`let mut t = 0; for p in map.values() { t += p; } t`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let mut m = HashMap::new();\n    m.insert(\"a\", 1);\n    let old = m.insert(\"a\", 2);\n    println!(\"{:?} {}\", old, m[\"a\"]);",
                options: &["None 2", "Some(1) 2", "Some(2) 2", "None 1"],
                answer: 1,
                explain: "`insert` overwrites and returns the previous value, if any.",
            }),
        },
        Exercise {
            id: "l6_05_entry",
            lesson: 6,
            code: "6-5",
            title: "Counting Coins",
            path: "exercises/lesson06/05_entry.rs",
            story: "A money changer pours a sack of mixed coins on the table and asks \
                    for a count of each type. He's tired of counting to one.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`insert(.., 1)` always overwrites with 1. Use the entry API to add to \
                 the existing count instead.",
                "`*counts.entry(coin.to_string()).or_insert(0) += 1;`",
                "`let mut g = HashMap::new(); for w in text.split_whitespace() { \
                 g.entry(w.len()).or_insert_with(Vec::new).push(w.to_string()); } g`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let mut m = HashMap::new();\n    for c in \"abca\".chars() {\n        *m.entry(c).or_insert(10) += 1;\n    }\n    println!(\"{}\", m[&'a']);",
                options: &["2", "11", "12", "1"],
                answer: 2,
                explain: "The first 'a' inserts 10, then adds 1. The second 'a' adds 1 again: 12.",
            }),
        },
        Exercise {
            id: "l6_06_sets",
            lesson: 6,
            code: "6-6",
            title: "Unique Relics",
            path: "exercises/lesson06/06_sets.rs",
            story: "A relic collector only wants items he doesn't have yet, and he \
                    wants his catalogue in alphabetical order. Every time.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`insert` returns `true` when the value is NEW. A duplicate is when it \
                 returns `false`: `if !seen.insert(*r)`.",
                "HashMap iteration order is random. Switch `counts` to `BTreeMap::new()`.",
                "`let set: BTreeSet<u32> = values.iter().copied().collect(); \
                 set.into_iter().collect()`, or insert each value in a loop.",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let mut s = BTreeSet::new();\n    for n in [3, 1, 3, 2] { s.insert(n); }\n    println!(\"{:?}\", s);",
                options: &[
                    "{3, 1, 2}",
                    "{1, 2, 3}",
                    "{3, 1, 3, 2}",
                    "the order is random",
                ],
                answer: 1,
                explain: "A BTreeSet removes duplicates and keeps its values sorted.",
            }),
        },
        Exercise {
            id: "l6_07_boss",
            lesson: 6,
            code: "BOSS",
            title: "The Bazaar Baron",
            path: "exercises/lesson06/07_boss.rs",
            story: "The Bazaar Baron owns every stall but one: yours. Out-trade him \
                    and the market is free.",
            check: Check::Test,
            xp: 300,
            hints: &[
                "`struct Shop { stock: HashMap<String, u32>, gold: u32, log: Vec<String> }`. \
                 restock: `*self.stock.entry(item.to_string()).or_insert(0) += qty;`",
                "sell: `let have = self.quantity(item); if have < qty { return false; }` then \
                 update. `if have == qty { self.stock.remove(item); } else { self.stock.insert(...) }`",
                "report: copy into a `BTreeMap`, build `format!(\"{k} x{v}\")` parts and \
                 `.join(\", \")`.",
                "best_seller: count items into a BTreeMap<String, u32>, then loop keeping \
                 the max with a strict `>` (so the alphabetically-first one wins ties).",
            ],
            quiz: Some(Quiz {
                question: "You need word counts printed in a stable, sorted order. \
                           Which collection fits best?",
                options: &[
                    "HashMap<String, u32>",
                    "Vec<String>",
                    "HashSet<String>",
                    "BTreeMap<String, u32>",
                ],
                answer: 3,
                explain: "BTreeMap keeps its keys sorted, so iteration order is deterministic.",
            }),
        },
    ],
};
