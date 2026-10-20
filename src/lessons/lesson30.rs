use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 30,
    title: "The Guild Charter: API design idioms",
    notes: "starters/lesson30/README.md",
    boss_trophy: ("boss_l30", "Author of the Charter"),
    exercises: &[
        Exercise {
            id: "l30_01_builder",
            lesson: 30,
            code: "30-1",
            title: "The Artisan's Order",
            path: "exercises/lesson30/01_builder.rs",
            story: "The guild's artisans take orders with many options. Nobody wants to fill \
                    in eight blanks for a simple sword.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "`pub fn reward(mut self, gold: u32) -> Self { self.reward = Some(gold); self }` and the same \
                 for the others.",
                "title: `pub fn title(mut self, title: impl Into<String>) -> Self { self.title = Some(title.into()); self }`",
                "build: `title: self.title.ok_or(\"a quest needs a title\")?` (the ? turns &str into String), \
                 `reward: self.reward.unwrap_or(10)`, ...",
            ],
            quiz: Some(Quiz {
                question: "Why do builder setters take `self` by value and return `Self`?",
                options: &[
                    "It's faster",
                    "So calls chain: builder().a(1).b(2).build()",
                    "Rust forbids &mut self in builders",
                    "To make the builder Copy",
                ],
                answer: 1,
                explain: "Each call moves the builder through and hands it back, so one expression \
                          builds the whole thing.",
            }),
        },
        Exercise {
            id: "l30_02_typestate",
            lesson: 30,
            code: "30-2",
            title: "Doors That Know",
            path: "exercises/lesson30/02_typestate.rs",
            story: "The charter hall's doors know if they're locked. Try to walk through a \
                    locked one and the compiler stops you, not the guard.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "On a wrong code, give the same door back: `Err(self)`.",
                "`impl Door<Unlocked> { pub fn open(self) -> Door<Open> { self.into_state() } pub fn lock(self) \
                 -> Door<Locked> { self.into_state() } }`",
                "`impl Door<Open> { pub fn enter(&self, hero: &str) -> String { format!(\"{hero} walks in\") } \
                 pub fn close(self) -> Door<Unlocked> { self.into_state() } }`",
            ],
            quiz: Some(Quiz {
                question: "What does typestate give you over a `state: State` field?",
                options: &[
                    "Faster code",
                    "Wrong transitions become compile errors instead of runtime checks",
                    "Less memory",
                    "Nothing",
                ],
                answer: 1,
                explain: "Each state is a different type with only the valid methods. And \
                          PhantomData costs zero bytes.",
            }),
        },
        Exercise {
            id: "l30_03_flexible_args",
            lesson: 30,
            code: "30-3",
            title: "Open Hands",
            path: "exercises/lesson30/03_flexible_args.rs",
            story: "Recruits arrive with names on scraps of paper, in lists, in long processions. \
                    A good clerk takes them as they come.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "new and add: `impl Into<String>`, then `.into()` when storing.",
                "add_all: `heroes: impl IntoIterator<Item = impl Into<String>>` and keep the `for` loop.",
                "has: `hero: impl AsRef<str>`, then compare `m == hero.as_ref()`.",
            ],
            quiz: Some(Quiz {
                question: "You only READ a string argument. Best parameter type?",
                options: &[
                    "String",
                    "impl Into<String>",
                    "&str or impl AsRef<str>",
                    "Box<str>",
                ],
                answer: 2,
                explain: "Into<String> is for values you'll store. For reading, borrow: no \
                          allocation needed.",
            }),
        },
        Exercise {
            id: "l30_04_cow",
            lesson: 30,
            code: "30-4",
            title: "Borrow or Forge",
            path: "exercises/lesson30/04_cow.rs",
            story: "The smith forges a new blade for every request, even when the old one was \
                    fine. Lend when you can, forge only when you must.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "`let trimmed = name.trim(); if trimmed.chars().any(char::is_uppercase) { Cow::Owned(trimmed.to_lowercase()) } \
                 else { Cow::Borrowed(trimmed) }`",
                "escape: `if !(text.contains('&') || text.contains('<') || text.contains('>')) \
                 { return Cow::Borrowed(text); }`",
                "Then `Cow::Owned(text.replace('&', \"&amp;\").replace('<', \"&lt;\").replace('>', \"&gt;\"))`. \
                 Replace & first!",
            ],
            quiz: Some(Quiz {
                question: "When is `Cow<str>` a good return type?",
                options: &[
                    "Always instead of String",
                    "When the result is usually the input unchanged, and only sometimes new",
                    "For huge strings only",
                    "When the caller must own the result",
                ],
                answer: 1,
                explain: "The common case costs nothing; the rare case allocates.",
            }),
        },
        Exercise {
            id: "l30_05_iter_traits",
            lesson: 30,
            code: "30-5",
            title: "The Procession",
            path: "exercises/lesson30/05_iter_traits.rs",
            story: "The guild wants its party to march like any other column: in a for loop, \
                    gathered by collect, joined by extend.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "`pub fn iter(&self) -> std::slice::Iter<'_, Hero> { self.heroes.iter() }`",
                "Two IntoIterator impls: for `&'a Party` (Item = &'a Hero, slice::Iter) and for `Party` \
                 (Item = Hero, std::vec::IntoIter<Hero>).",
                "FromIterator: `Party { heroes: iter.into_iter().collect() }`; Extend: `self.heroes.extend(iter)`.",
            ],
            quiz: Some(Quiz {
                question: "Which trait makes `let p: Party = iter.collect();` work?",
                options: &[
                    "IntoIterator",
                    "Extend",
                    "FromIterator<Hero>",
                    "From<Vec<Hero>>",
                ],
                answer: 2,
                explain: "collect() is generic over FromIterator. Implement it and collect can build \
                          your type.",
            }),
        },
        Exercise {
            id: "l30_06_extension",
            lesson: 30,
            code: "30-6",
            title: "New Tricks for Old Types",
            path: "exercises/lesson30/06_extension.rs",
            story: "Numbers don't know they're gold, and iterators can't count treasure. Teach \
                    them, without touching the standard library.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "`impl GoldExt for u32 { fn gold(self) -> Gold { Gold(self) } }`",
                "In the trait: `fn total_gold(self) -> Gold { Gold(self.map(|g| g.0).sum()) }`",
                "Blanket impl: `impl<I: Iterator<Item = Gold>> TotalGold for I {}`",
            ],
            quiz: Some(Quiz {
                question: "Why may you implement your own trait for u32 but not std's Display for Vec<T>?",
                options: &[
                    "u32 is special",
                    "The orphan rule: the trait or the type must be yours",
                    "Display is sealed",
                    "You can do both",
                ],
                answer: 1,
                explain: "Your trait + foreign type is fine. Foreign trait + foreign type needs a \
                          newtype.",
            }),
        },
        Exercise {
            id: "l30_07_boss",
            lesson: 30,
            code: "BOSS",
            title: "The Guild Charter",
            path: "exercises/lesson30/07_boss.rs",
            story: "Other guilds will copy your roster code. Write it so well that it feels \
                    like it came with the language.",
            check: Check::Test,
            xp: 400,
            hints: &[
                "Builder fields: `guild: Option<String>, max_size: Option<usize>`. build: \
                 `let guild = self.guild.unwrap_or_default();` and NoGuildName if `guild.trim().is_empty()`.",
                "recruit: check `contains` first (AlreadyIn), then `len() >= max_size` (Full), then push and `Ok(self)`.",
                "Display: `write!(f, \"{}:\", self.guild)?;` then `\" {}\"` with `self.members.join(\", \")` if any.",
                "FromStr: `s.split_once(':')`, build with the trimmed guild, then recruit each trimmed, non-empty \
                 name from `members.split(',')`. Map every failure to `BadFormat(s.to_string())`.",
            ],
            quiz: Some(Quiz {
                question: "What lets `?` turn your RosterError into Box<dyn Error>?",
                options: &[
                    "Debug alone",
                    "Implementing std::error::Error (which needs Display + Debug)",
                    "Clone",
                    "Nothing, it always works",
                ],
                answer: 1,
                explain: "std has `impl<E: Error> From<E> for Box<dyn Error>`, so any Error type \
                          converts with ?.",
            }),
        },
    ],
};
