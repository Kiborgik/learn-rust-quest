use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 29,
    title: "The Forbidden Archive: unsafe and FFI",
    notes: "starters/lesson29/README.md",
    boss_trophy: ("boss_l29", "Keeper of Forbidden Things"),
    exercises: &[
        Exercise {
            id: "l29_01_raw_pointers",
            lesson: 29,
            code: "29-1",
            title: "Two Hands, One Blade",
            path: "exercises/lesson29/01_raw_pointers.rs",
            story: "Behind the archive door the rules still apply, but you may vouch for things \
                    the guards can't see. Hold both halves of one blade, carefully.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "`let ptr = values.as_mut_ptr();` then build both halves with `slice::from_raw_parts_mut`.",
                "`unsafe { (slice::from_raw_parts_mut(ptr, mid), slice::from_raw_parts_mut(ptr.add(mid), len - mid)) }`",
                "first_two_mut: check `len() < 2`, then `let (a, b) = split_at_mut(values, 1); Some((&mut a[0], &mut b[0]))`",
            ],
            quiz: Some(Quiz {
                question: "What does an `unsafe` block turn off?",
                options: &[
                    "The borrow checker",
                    "Nothing: it unlocks a few extra operations you promise to use correctly",
                    "All type checks",
                    "Bounds checks everywhere",
                ],
                answer: 1,
                explain: "Borrow and type checks still run. Unsafe allows raw pointer derefs, unsafe \
                          calls, static mut, unsafe traits and unions.",
            }),
        },
        Exercise {
            id: "l29_02_contracts",
            lesson: 29,
            code: "29-2",
            title: "The Contract",
            path: "exercises/lesson29/02_contracts.rs",
            story: "Every forbidden scroll comes with a contract. Read it, keep it, and sign \
                    where you kept it.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "`// SAFETY: just checked n <= v.len().` then `Some(unsafe { sum_first_unchecked(v, n) })`",
                "average_first: `if n == 0 { return None; } Some(sum_first(v, n)? / n as u32)`",
            ],
            quiz: Some(Quiz {
                question: "What is undefined behavior?",
                options: &[
                    "A panic",
                    "A compile error",
                    "Breaking an unsafe contract: the program may do anything, even seem fine",
                    "A runtime error with a message",
                ],
                answer: 2,
                explain: "UB has no guaranteed symptom. That's why unsafe code needs clear contracts \
                          and tools like Miri.",
            }),
        },
        Exercise {
            id: "l29_03_calling_c",
            lesson: 29,
            code: "29-3",
            title: "Words from the Old Tongue",
            path: "exercises/lesson29/03_calling_c.rs",
            story: "The oldest scrolls speak C. Ask them simple questions, and make sure your \
                    words end where C expects them to.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "Edition 2024: `unsafe extern \"C\" { ... }`",
                "c_abs: `unsafe { abs(x) }` with a SAFETY comment.",
                "c_strlen: `let c = CString::new(text).ok()?; Some(unsafe { strlen(c.as_ptr()) })`",
            ],
            quiz: Some(Quiz {
                question: "What's wrong with `strlen(CString::new(s).unwrap().as_ptr())`?",
                options: &[
                    "Nothing",
                    "The CString is dropped before strlen reads it: a dangling pointer",
                    "CString has no as_ptr",
                    "strlen needs a Rust string",
                ],
                answer: 1,
                explain: "The temporary dies at the end of the statement. Keep the CString in a \
                          variable while C uses it.",
            }),
        },
        Exercise {
            id: "l29_04_callbacks",
            lesson: 29,
            code: "29-4",
            title: "The Old Tongue Calls Back",
            path: "exercises/lesson29/04_callbacks.rs",
            story: "An ancient sorting engine does the heavy work, but it keeps asking you \
                    which of two things comes first. Answer in its language.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "desc: `let (a, b) = unsafe { (*(a as *const i32), *(b as *const i32)) }; b.cmp(&a) as c_int`",
                "by_level_then_id: cast to `*const Hero`, then `(a.level, a.id).cmp(&(b.level, b.id)) as c_int`.",
                "sort_heroes: like sort_desc, with `size_of::<Hero>()` and your comparator.",
            ],
            quiz: Some(Quiz {
                question: "A Rust callback called from C panics. What happens?",
                options: &[
                    "C catches it",
                    "The panic unwinds into C normally",
                    "The program aborts",
                    "It returns 0",
                ],
                answer: 2,
                explain: "Unwinding into C is not allowed. An `extern \"C\" fn` that panics aborts \
                          the process.",
            }),
        },
        Exercise {
            id: "l29_05_exporting",
            lesson: 29,
            code: "29-5",
            title: "Speaking Their Language",
            path: "exercises/lesson29/05_exporting.rs",
            story: "Now the old scrolls want to call YOU. Lay out your words the way they \
                    read, under names they can find.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "Use `#[unsafe(no_mangle)]` on both functions, and add `#[repr(C)]` to Hero.",
                "heal: `let Some(hero) = (unsafe { hero.as_mut() }) else { return false; };` then \
                 `hero.hp = hero.hp.saturating_add(amount).min(MAX_HP); true`",
                "title: `match level { 0..10 => c\"novice\", 10..50 => c\"veteran\", _ => c\"legend\" }.as_ptr()`",
            ],
            quiz: Some(Quiz {
                question: "Why does a struct shared with C need #[repr(C)]?",
                options: &[
                    "To make it Copy",
                    "Rust may reorder fields; repr(C) fixes C's order and padding",
                    "C can't read Rust integers",
                    "For speed",
                ],
                answer: 1,
                explain: "Rust's default layout is unspecified. repr(C) promises the layout a C \
                          compiler would use.",
            }),
        },
        Exercise {
            id: "l29_06_statics",
            lesson: 29,
            code: "29-6",
            title: "The Shared Altar",
            path: "exercises/lesson29/06_statics.rs",
            story: "Every pilgrim adds a mark to the altar at once. The old way of keeping \
                    count is forbidden now, and for good reason.",
            check: Check::Test,
            xp: 175,
            hints: &[
                "`static VISITS: AtomicU32 = AtomicU32::new(0);` and `VISITS.fetch_add(1, Ordering::Relaxed) + 1`",
                "`static LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());` and `LOG.lock().unwrap().push(..)`",
            ],
            quiz: Some(Quiz {
                question: "What's the safe replacement for `static mut COUNTER: u32`?",
                options: &[
                    "A local variable",
                    "static COUNTER: AtomicU32",
                    "const COUNTER: u32",
                    "Box<u32>",
                ],
                answer: 1,
                explain: "Atomics change safely from many threads. For other data, a static Mutex \
                          or LazyLock.",
            }),
        },
        Exercise {
            id: "l29_07_boss",
            lesson: 29,
            code: "BOSS",
            title: "The Forbidden Archive",
            path: "exercises/lesson29/07_boss.rs",
            story: "The innermost vault holds a stack with no heap beneath it. Its empty slots \
                    are truly empty, and only your bookkeeping knows which are full.",
            check: Check::Test,
            xp: 400,
            hints: &[
                "new: `ArrayStack { items: [const { MaybeUninit::uninit() }; N], len: 0 }`",
                "push: `if self.len == N { return Err(value); } self.items[self.len].write(value); self.len += 1;`",
                "pop: lower len first, then `unsafe { self.items[self.len].assume_init_read() }`. \
                 as_slice: `std::slice::from_raw_parts(self.items.as_ptr().cast::<T>(), self.len)`",
                "`impl<T, const N: usize> Drop for ArrayStack<T, N>`: for each slot in \
                 `&mut self.items[..self.len]`, `unsafe { slot.assume_init_drop() }`.",
            ],
            quiz: Some(Quiz {
                question: "Why doesn't Rust drop the contents of a MaybeUninit<T> automatically?",
                options: &[
                    "It's a bug",
                    "It can't know if the slot holds a value; you track that",
                    "MaybeUninit is always empty",
                    "Drop doesn't work with arrays",
                ],
                answer: 1,
                explain: "Dropping an empty slot would be UB. The owner (your len) decides what to \
                          drop.",
            }),
        },
    ],
};
