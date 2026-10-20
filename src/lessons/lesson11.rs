use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 11,
    title: "The Clockwork Tower: smart pointers",
    notes: "starters/lesson11/README.md",
    boss_trophy: ("boss_l11", "Titan Toppler"),
    exercises: &[
        Exercise {
            id: "l11_01_box",
            lesson: 11,
            code: "11-1",
            title: "The Box of Holding",
            path: "exercises/lesson11/01_box.rs",
            story: "The Clockwork Tower's first floor holds a magical box: bigger on \
                    the inside, and able to contain things that contain themselves.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "E0072: `Expr` contains `Expr` directly, so its size would be infinite. \
                 Use `Add(Box<Expr>, Box<Expr>)`, and the same for Mul and Neg.",
                "`match e { Num(n) => *n, Add(a, b) => eval(a) + eval(b), ... }`. \
                 `&Box<Expr>` auto-derefs to `&Expr`.",
                "`count_numbers`: `Num(_) => 1, Add(a, b) | Mul(a, b) => count_numbers(a) + \
                 count_numbers(b), Neg(x) => count_numbers(x)`",
            ],
            quiz: Some(Quiz {
                question: "`std::mem::size_of::<T>()` gives the size of a type in bytes. \
                           What is `std::mem::size_of::<Box<[u8; 1000]>>()` on a 64-bit machine?",
                options: &["1000", "8", "1008", "0"],
                answer: 1,
                explain: "A Box is just a pointer (8 bytes). The 1000 bytes live on the heap.",
            }),
        },
        Exercise {
            id: "l11_02_deref_drop",
            lesson: 11,
            code: "11-2",
            title: "Gears That Turn",
            path: "exercises/lesson11/02_deref_drop.rs",
            story: "Every gear in the tower announces itself when it starts turning, \
                    and should announce when it stops. Some never say goodbye.",
            check: Check::Output(
                "Gear Alpha starts turning\nGear Bronze starts turning\nGear Copper starts turning\nGear Bronze is 6 letters long\nGear Bronze stops\nMain loop done\nGear Copper stops\nGear Alpha stops",
            ),
            xp: 150,
            hints: &[
                "`impl Deref for Gear { type Target = String; fn deref(&self) -> &String { &self.name } }`",
                "`impl Drop for Gear { fn drop(&mut self) { println!(\"Gear {} stops\", self.name); } }`",
                "Call `drop(b);` right after `print_len(...)`. The others are dropped \
                 automatically in reverse order at the end of main.",
            ],
            quiz: Some(Quiz {
                question: "In what order are `x`, `y`, `z` dropped?\n\n    fn main() {\n        let x = Noisy(\"x\");\n        let y = Noisy(\"y\");\n        let z = Noisy(\"z\");\n    }",
                options: &["x y z", "z y x", "y x z", "random"],
                answer: 1,
                explain: "Locals are dropped in reverse order of declaration, like a stack.",
            }),
        },
        Exercise {
            id: "l11_03_rc",
            lesson: 11,
            code: "11-3",
            title: "Shared Blueprints",
            path: "exercises/lesson11/03_rc.rs",
            story: "Three parties of engineers are repairing the tower. There's only \
                    ONE blueprint, and photocopiers haven't been invented yet.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "The field must hold a shared pointer: `plan: Rc<Blueprint>`.",
                "Give each party its own handle: `plan: Rc::clone(&plan)`.",
                "`Rc::strong_count(plan)`",
            ],
            quiz: Some(Quiz {
                question: "What does this print?\n\n    let a = Rc::new(1);\n    let b = Rc::clone(&a);\n    {\n        let c = Rc::clone(&b);\n    }\n    println!(\"{}\", Rc::strong_count(&a));",
                options: &["1", "3", "2", "0"],
                answer: 2,
                explain: "`a` and `b` are still alive. `c` was dropped at the end of its block, \
                          so the count is 2.",
            }),
        },
        Exercise {
            id: "l11_04_refcell",
            lesson: 11,
            code: "11-4",
            title: "The Hidden Mechanism",
            path: "exercises/lesson11/04_refcell.rs",
            story: "Behind a sealed panel, a clockwork logs every turn of the tower, \
                    even though nobody can open the panel to write in the log.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "Field types: `turns: Cell<u32>`, `log: RefCell<Vec<String>>`. In `new`: \
                 `Cell::new(0)` and `RefCell::new(Vec::new())`.",
                "`self.turns.set(self.turns.get() + 1);` \
                 `self.log.borrow_mut().push(...)`, `self.log.borrow().len()`",
                "In `report`, the `log` guard is still alive when you call `borrow_mut`. \
                 Add `drop(log);` before it, or compute the length in one expression.",
            ],
            quiz: Some(Quiz {
                question: "What happens here?\n\n    let c = RefCell::new(5);\n    let a = c.borrow_mut();\n    let b = c.borrow_mut();",
                options: &[
                    "Compile error: two mutable borrows",
                    "Runtime panic: already borrowed",
                    "b waits until a is dropped",
                    "Works fine",
                ],
                answer: 1,
                explain: "RefCell checks the borrow rules at RUNTIME, and breaking them panics.",
            }),
        },
        Exercise {
            id: "l11_05_rc_refcell",
            lesson: 11,
            code: "11-5",
            title: "The Shared Workshop",
            path: "exercises/lesson11/05_rc_refcell.rs",
            story: "The tower's workshop has one shelf of parts that every engineer \
                    can put things on and take things off.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "`stash: Rc<RefCell<Stash>>`",
                "`self.stash.borrow_mut().push(item.to_string());` and \
                 `self.stash.borrow_mut().pop()`",
                "`stash.borrow().len()`",
            ],
            quiz: Some(Quiz {
                question: "Which combination gives you shared ownership AND mutation in single-threaded code?",
                options: &["Box<Cell<T>>", "Rc<T>", "Rc<RefCell<T>>", "&mut Rc<T>"],
                answer: 2,
                explain: "Rc provides several owners, and RefCell provides mutation through \
                          a shared reference.",
            }),
        },
        Exercise {
            id: "l11_06_weak",
            lesson: 11,
            code: "11-6",
            title: "Weak Links",
            path: "exercises/lesson11/06_weak.rs",
            story: "The rooms of the tower point at each other, and the tower can never \
                    be demolished. Some links must be weaker than others.",
            check: Check::Test,
            xp: 150,
            hints: &[
                "`parent: RefCell<Weak<Room>>` and in `new`: `RefCell::new(Weak::new())`.",
                "`*child.parent.borrow_mut() = Rc::downgrade(parent);`",
                "`room.parent.borrow().upgrade().map(|p| p.name.clone())`",
            ],
            quiz: Some(Quiz {
                question: "What does `weak.upgrade()` return after every `Rc` to the value has been dropped?",
                options: &["A dangling pointer", "None", "It panics", "The last value"],
                answer: 1,
                explain: "A Weak doesn't keep the value alive. Once it's gone, upgrade() \
                          safely returns None.",
            }),
        },
        Exercise {
            id: "l11_07_boss",
            lesson: 11,
            code: "BOSS",
            title: "The Clockwork Titan",
            path: "exercises/lesson11/07_boss.rs",
            story: "The top of the tower splits open. The Clockwork Titan rises: gears \
                    within gears, a heart of springs. Map its mechanism and stop it.",
            check: Check::Test,
            xp: 350,
            hints: &[
                "`Gear::new` returns `Rc::new(Gear { name: name.into(), teeth, parent: \
                 RefCell::new(Weak::new()), children: RefCell::new(vec![]) })`. \
                 `attach` is the same as in quest 11-6.",
                "`gear.teeth + gear.children.borrow().iter().map(total_teeth).sum::<u32>()`",
                "path: `let mut out = vec![gear.name.clone()]; let mut cur = gear.parent.borrow().upgrade(); \
                 while let Some(p) = cur { out.push(p.name.clone()); cur = p.parent.borrow().upgrade(); } out`",
                "spin_all: `for g in gears { counter.spin(); spin_all(&g.children.borrow(), counter); }`. \
                 power: `Amplify(c) => 2 * power(c)`.",
            ],
            quiz: Some(Quiz {
                question: "In a tree with `Rc` parent → child links, what should child → parent links be?",
                options: &["Rc", "Box", "Weak", "&'static"],
                answer: 2,
                explain: "Weak back-pointers avoid reference cycles, so the tree can still \
                          be freed.",
            }),
        },
    ],
};
