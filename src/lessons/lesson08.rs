use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 8,
    title: "The Academy of Forms: traits & generics",
    notes: "starters/lesson08/README.md",
    boss_trophy: ("boss_l08", "Archmage"),
    exercises: &[
        Exercise {
            id: "l8_01_traits",
            lesson: 8,
            code: "8-1",
            title: "Learning a Trade",
            path: "exercises/lesson08/01_traits.rs",
            story: "Beyond the fortress stands the Academy of Forms, where every \
                    student must learn a trade. A trade is a promise: \"I can do these things.\"",
            check: Check::Test,
            xp: 125,
            hints: &[
                "\"not all trait items implemented, missing: `cost`\": add `fn cost(&self) -> u32 \
                 { 10 * self.skill }` to the Blacksmith impl.",
                "`impl Craft for Alchemist { fn product(&self) -> String { ... } fn cost(&self) \
                 -> u32 { 5 + self.potions_brewed } }`",
                "`impl Craft for u32 { ... fn cost(&self) -> u32 { *self } }`",
            ],
            quiz: Some(Quiz {
                question: "Which is the correct syntax to implement trait `Fly` for `Bird`?",
                options: &[
                    "impl Bird for Fly { ... }",
                    "impl Fly for Bird { ... }",
                    "struct Bird implements Fly { ... }",
                    "impl Bird: Fly { ... }",
                ],
                answer: 1,
                explain: "`impl Trait for Type`, kept separate from the struct definition.",
            }),
        },
        Exercise {
            id: "l8_02_default_methods",
            lesson: 8,
            code: "8-2",
            title: "Inherited Wisdom",
            path: "exercises/lesson08/02_default_methods.rs",
            story: "Academy mentors share common teachings, but the eccentric Sage \
                    insists on doing things his own way.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "A default method is written INSIDE the trait, with a body.",
                "`fn teach(&self) -> String { format!(\"{} teaches {}\", self.name(), self.subject()) }`",
                "`fn wisdom(&self) -> u32 { self.years() * 10 }` in the trait, and \
                 `fn wisdom(&self) -> u32 { 1000 }` in `impl Mentor for Sage`.",
            ],
            quiz: Some(Quiz {
                question: "A trait has 1 required method and 3 default methods. How many \
                           methods MUST an implementor write?",
                options: &["0", "1", "3", "4"],
                answer: 1,
                explain: "Only the required one. The defaults come for free and can \
                          optionally be overridden.",
            }),
        },
        Exercise {
            id: "l8_03_generics",
            lesson: 8,
            code: "8-3",
            title: "One Spell, Many Targets",
            path: "exercises/lesson08/03_generics.rs",
            story: "Why learn a separate spell for every enemy? A generic spell works \
                    on anything, as long as the target meets its conditions.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "\"binary operation `>` cannot be applied to type `T`\": add the bound \
                 `fn largest<T: PartialOrd + Copy>`.",
                "`struct Chest<T> { item: T }` and `impl<T> Chest<T> { fn new(item: T) -> Self \
                 { .. } fn open(self) -> T { .. } }`",
                "`impl Chest<u32> { fn is_heavy(&self) -> bool { self.item > 100 } }`",
            ],
            quiz: Some(Quiz {
                question: "What does Rust do at compile time with `fn id<T>(x: T) -> T` \
                           called with an i32 and a String?",
                options: &[
                    "Boxes both values behind a pointer",
                    "Generates two specialised copies of the function",
                    "Converts both to a common type",
                    "Checks types at runtime",
                ],
                answer: 1,
                explain: "Monomorphisation: one copy per concrete type, so there's no \
                          runtime cost.",
            }),
        },
        Exercise {
            id: "l8_04_bounds",
            lesson: 8,
            code: "8-4",
            title: "Bound by Oath",
            path: "exercises/lesson08/04_bounds.rs",
            story: "Academy rule: you may only ask something of a type if it has \
                    sworn an oath (a trait bound) that it can do it.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`banner` uses `{}`: `fn banner<T: Display>(value: T)`.",
                "`twins` calls `.clone()`: `fn twins<T: Clone>(value: T)`.",
                "`compare` uses `==`: `where A: Debug + PartialEq`.",
                "`if b.volume() > a.volume() { b.name() } else { a.name() }`",
            ],
            quiz: Some(Quiz {
                question: "Which signature is equivalent to `fn f(x: impl Display)`?",
                options: &[
                    "fn f(x: dyn Display)",
                    "fn f<T: Display>(x: T)",
                    "fn f(x: Box<Display>)",
                    "fn f(x: &Display)",
                ],
                answer: 1,
                explain: "`impl Trait` in argument position is shorthand for an anonymous \
                          generic parameter with that bound.",
            }),
        },
        Exercise {
            id: "l8_05_std_traits",
            lesson: 8,
            code: "8-5",
            title: "Royal Protocols",
            path: "exercises/lesson08/05_std_traits.rs",
            story: "At court, every guest must follow the royal protocols: how to \
                    introduce yourself, how to be converted, added and ranked.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`impl fmt::Display for Gold { fn fmt(&self, f: &mut fmt::Formatter) -> \
                 fmt::Result { write!(f, \"{}g\", self.0) } }`",
                "`impl From<u32> for Gold { fn from(n: u32) -> Self { Gold(n) } }`. \
                 You get `.into()` for free.",
                "`impl Add for Gold { type Output = Gold; fn add(self, o: Gold) -> Gold { Gold(self.0 + o.0) } }`",
                "Rank: `#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]`. Settings: \
                 `impl Default for Settings { fn default() -> Self { Settings { difficulty: \
                 \"normal\".into(), volume: 5 } } }`",
            ],
            quiz: Some(Quiz {
                question: "You implement `From<u32> for Gold`. What else do you get automatically?",
                options: &[
                    "Display for Gold",
                    "Into<Gold> for u32",
                    "From<Gold> for u32",
                    "Add for Gold",
                ],
                answer: 1,
                explain: "The standard library gives `Into<Gold> for u32` for free once \
                          `From<u32> for Gold` exists. So implement `From`, not `Into`.",
            }),
        },
        Exercise {
            id: "l8_06_dyn",
            lesson: 8,
            code: "8-6",
            title: "The Masquerade",
            path: "exercises/lesson08/06_dyn.rs",
            story: "A masquerade ball: foxes, owls, who knows what's behind each mask. \
                    The host doesn't care, as long as everyone can dance.",
            check: Check::Test,
            xp: 125,
            hints: &[
                "`Vec<Box<Fox>>` only holds foxes. Use `Vec<Box<dyn Mask>>`.",
                "`let mut t = 0; for g in guests { t += g.charm(); } t`",
                "`impl Mask` must be ONE concrete type. Return `Box<dyn Mask>` and wrap \
                 each branch in `Box::new(...)`.",
            ],
            quiz: Some(Quiz {
                question: "Why does `Vec<dyn Mask>` not compile, while `Vec<Box<dyn Mask>>` does?",
                options: &[
                    "dyn types can't be stored at all",
                    "`dyn Mask` has no known size; a Box is a fixed-size pointer",
                    "Vec only accepts Copy types",
                    "Box makes the trait object-safe",
                ],
                answer: 1,
                explain: "Values of different types differ in size. A Vec needs a uniform \
                          element size, and pointers like Box provide it.",
            }),
        },
        Exercise {
            id: "l8_07_boss",
            lesson: 8,
            code: "BOSS",
            title: "The Archmage's Exam",
            path: "exercises/lesson08/07_boss.rs",
            story: "The Archmage sets the final exam: build a whole spell system \
                    from nothing but traits, generics and trait objects.",
            check: Check::Test,
            xp: 300,
            hints: &[
                "`trait Spell { fn name(&self) -> String; fn mana_cost(&self) -> u32; \
                 fn cast(&self, target: &mut Target); fn describe(&self) -> String { \
                 format!(\"{} ({} mana)\", self.name(), self.mana_cost()) } }`",
                "Fireball cast: `target.hp = target.hp.saturating_sub(self.power);` \
                 Heal: `target.hp = (target.hp + self.amount).min(target.max_hp);`",
                "cheapest: `if spells.is_empty() { return None; } let mut best = &spells[0]; \
                 for s in spells { if s.mana_cost() < best.mana_cost() { best = s; } } Some(best)`",
                "Display: `let mut lines = Vec::new(); for s in &self.spells { \
                 lines.push(s.describe()); } write!(f, \"{}\", lines.join(\"\\n\"))`",
            ],
            quiz: Some(Quiz {
                question: "When should you prefer `Box<dyn Trait>` over generics?",
                options: &[
                    "Always, it's faster",
                    "When one collection must hold values of DIFFERENT concrete types",
                    "When the trait has default methods",
                    "Never, generics can do everything",
                ],
                answer: 1,
                explain: "Generics fix one concrete type per use. Trait objects let you mix \
                          types at runtime, at the cost of a vtable call.",
            }),
        },
    ],
};
