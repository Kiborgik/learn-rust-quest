use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 19,
    title: "The Hall of Forms: advanced traits & types",
    notes: "starters/lesson19/README.md",
    boss_trophy: ("boss_l19", "Riddle Breaker"),
    exercises: &[
        Exercise {
            id: "l19_01_associated",
            lesson: 19,
            code: "19-1",
            title: "The Bag of Holding",
            path: "exercises/lesson19/01_associated.rs",
            story: "In the Hall of Forms, every container knows what it holds. A bag that \
                    can't say is no bag at all.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "\"not all trait items implemented, missing: `Item`\": add `type Item = String;` \
                 to the Backpack impl.",
                "Wallet: `impl Container for Wallet { type Item = u32; fn get(&self, i: usize) \
                 -> Option<&u32> { self.coins.get(i) } }`",
                "In the trait: `fn count(&self) -> usize { let mut n = 0; while self.get(n).is_some() \
                 { n += 1; } n }`",
            ],
            quiz: Some(Quiz {
                question: "When is an associated type better than a generic trait parameter?",
                options: &[
                    "When a type can implement the trait many ways",
                    "When there's exactly one sensible choice per implementing type",
                    "Always",
                    "Only for iterators",
                ],
                answer: 1,
                explain: "Associated types fix one answer per impl (Iterator::Item). Generic \
                          parameters allow many impls (From<u32>, From<&str>, ...).",
            }),
        },
        Exercise {
            id: "l19_02_operators",
            lesson: 19,
            code: "19-2",
            title: "Coin Arithmetic",
            path: "exercises/lesson19/02_operators.rs",
            story: "The treasurer wants to add plain numbers to gold, multiply purses, and \
                    top them up with +=.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "`impl Add<u32> for Gold { type Output = Gold; fn add(self, rhs: u32) -> Gold { Gold(self.0 + rhs) } }`",
                "`impl Mul<u32> for Gold { type Output = Gold; fn mul(self, rhs: u32) -> Gold { Gold(self.0 * rhs) } }`",
                "`impl AddAssign<u32> for Gold { fn add_assign(&mut self, rhs: u32) { self.0 += rhs; } }`",
            ],
            quiz: Some(Quiz {
                question: "In `trait Add<Rhs = Self>`, what does `= Self` mean?",
                options: &[
                    "Rhs must be Self",
                    "Rhs is Self unless you say otherwise",
                    "The output is Self",
                    "It's a runtime default value",
                ],
                answer: 1,
                explain: "A default type parameter: `impl Add for T` means `impl Add<T> for T`.",
            }),
        },
        Exercise {
            id: "l19_03_supertraits",
            lesson: 19,
            code: "19-3",
            title: "Prerequisites",
            path: "exercises/lesson19/03_supertraits.rs",
            story: "Only those with a name and a proper title may call themselves champions.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "The default method calls `self.name()` and formats `{self}`. Require both: \
                 `trait Champion: Named + fmt::Display`.",
                "Knight needs Display: `impl fmt::Display for Knight { fn fmt(&self, f: &mut \
                 fmt::Formatter) -> fmt::Result { write!(f, \"Sir {}\", self.name) } }`",
            ],
            quiz: Some(Quiz {
                question: "Two traits in scope both have `fn name(&self)`. How do you call one?",
                options: &[
                    "x.name() picks the first",
                    "Named::name(&x)",
                    "x.Named::name()",
                    "It's impossible",
                ],
                answer: 1,
                explain: "Call it through the trait: `Named::name(&x)`, or fully qualified \
                          `<Knight as Named>::name(&x)`.",
            }),
        },
        Exercise {
            id: "l19_04_blanket",
            lesson: 19,
            code: "19-4",
            title: "A Blanket for Everyone",
            path: "exercises/lesson19/04_blanket.rs",
            story: "Winter in the Hall. One blanket, woven well enough, covers everyone \
                    who can show themselves.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "Delete `impl Describe for i32`. A blanket impl would conflict with it.",
                "`impl<T: fmt::Display> Describe for T { fn describe(&self) -> String { format!(\"<<{self}>>\") } }`",
            ],
            quiz: Some(Quiz {
                question: "Why do you get `.to_string()` after implementing Display?",
                options: &[
                    "The compiler adds it to every type",
                    "std has a blanket impl: ToString for every T: Display",
                    "Display and ToString are the same trait",
                    "#[derive] adds it",
                ],
                answer: 1,
                explain: "`impl<T: Display + ?Sized> ToString for T` in std covers your type \
                          automatically.",
            }),
        },
        Exercise {
            id: "l19_05_newtype",
            lesson: 19,
            code: "19-5",
            title: "The Orphan Rule",
            path: "exercises/lesson19/05_newtype.rs",
            story: "You can't stitch your own crest onto the king's banner. Sew a banner of \
                    your own around it instead.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "E0117: Display and Vec both come from std. Add `struct Inventory(Vec<Item>);`.",
                "Change the impl to `impl fmt::Display for Inventory` and use `self.0.iter()`.",
                "`impl Deref for Inventory { type Target = Vec<Item>; fn deref(&self) -> &Vec<Item> { &self.0 } }`",
            ],
            quiz: Some(Quiz {
                question: "Which impl does the orphan rule ALLOW?",
                options: &[
                    "impl Display for Vec<u8>",
                    "impl MyTrait for Vec<u8>",
                    "impl Iterator for String",
                    "impl Clone for std::fs::File",
                ],
                answer: 1,
                explain: "The trait or the type must be yours. MyTrait is local, so implementing \
                          it for a std type is fine.",
            }),
        },
        Exercise {
            id: "l19_06_phantom",
            lesson: 19,
            code: "19-6",
            title: "Ghost Types",
            path: "exercises/lesson19/06_phantom.rs",
            story: "Gold and silver look the same in the ledger: just numbers. A ghost in \
                    the type system keeps them apart.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "E0392: `C` is never used. Add a field `currency: PhantomData<C>`.",
                "Update `new`: `Money { amount, currency: PhantomData }`.",
                "`impl Money<Gold> { fn to_silver(self) -> Money<Silver> { Money::new(self.amount * 100) } }`",
            ],
            quiz: Some(Quiz {
                question: "How much memory does a `PhantomData<T>` field take?",
                options: &["as much as T", "8 bytes", "zero", "one byte"],
                answer: 2,
                explain: "PhantomData is zero-sized. It only exists for the type checker.",
            }),
        },
        Exercise {
            id: "l19_07_boss",
            lesson: 19,
            code: "BOSS",
            title: "The Sphinx of Forms",
            path: "exercises/lesson19/07_boss.rs",
            story: "At the end of the Hall sits the Sphinx. It doesn't ask what you know. \
                    It asks what your types can prove.",
            check: Check::Test,
            xp: 400,
            hints: &[
                "`trait Monster: fmt::Display { type Loot; const DANGER: u32; fn drop_loot(&self) -> Self::Loot; }`",
                "`impl Monster for Dragon { type Loot = Hoard; const DANGER: u32 = 50; fn drop_loot(&self) \
                 -> Hoard { Hoard(vec![format!(\"{}'s scale\", self.name), \"gold pile\".into()]) } }`",
                "`impl<M: Monster> Threat for M { fn threat(&self) -> String { format!(\"{self}: danger {}\", M::DANGER) } }`",
                "`fn total_danger<M: Monster>(ms: &[M]) -> u32 { M::DANGER * ms.len() as u32 }`. \
                 Hoard Add: `Hoard(self.0.into_iter().chain(rhs.0).collect())`.",
            ],
            quiz: Some(Quiz {
                question: "How do you read an associated const in generic code?",
                options: &["self.DANGER", "M::DANGER", "DANGER", "Monster.DANGER"],
                answer: 1,
                explain: "Associated consts belong to the type: `M::DANGER` or \
                          `<M as Monster>::DANGER`.",
            }),
        },
    ],
};
