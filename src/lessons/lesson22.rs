use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 22,
    title: "The Watchtower: logging with tracing",
    notes: "starters/lesson22/README.md",
    boss_trophy: ("boss_l22", "Saboteur Seeker"),
    exercises: &[
        Exercise {
            id: "l22_01_events",
            lesson: 22,
            code: "22-1",
            title: "The Watchtower Log",
            path: "exercises/lesson22/01_events/src/lib.rs",
            story: "The watchtower keeps a log of everything it sees, marked by how worried \
                    the guard was when he wrote it down.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Replace each `println!` with a tracing macro: `info!`, `warn!` or `error!`.",
                "Import them: `use tracing::{error, info, warn};`",
            ],
            quiz: Some(Quiz {
                question: "What happens to `info!` events if no subscriber is installed?",
                options: &[
                    "They print to stdout",
                    "They're discarded, cheaply",
                    "The program panics",
                    "They're queued",
                ],
                answer: 1,
                explain: "Without a subscriber, events go nowhere. Libraries can log freely; \
                          the application decides what to keep.",
            }),
        },
        Exercise {
            id: "l22_02_fields",
            lesson: 22,
            code: "22-2",
            title: "Facts, Not Sentences",
            path: "exercises/lesson22/02_fields/src/lib.rs",
            story: "The watch captain can't search a pile of sentences. She wants facts \
                    she can sort and count.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Fields go before the message: `info!(field = value, ..., \"message\")`.",
                "`info!(hero = %hero.name, damage, hp = hero.hp, \"took damage\");`",
            ],
            quiz: Some(Quiz {
                question: "In `info!(user = %name)`, what does `%` mean?",
                options: &[
                    "modulo",
                    "record the value with Display",
                    "record with Debug",
                    "hide the value",
                ],
                answer: 1,
                explain: "`%` records with Display, `?` with Debug. Plain values must be \
                          recordable primitives or strings.",
            }),
        },
        Exercise {
            id: "l22_03_spans",
            lesson: 22,
            code: "22-3",
            title: "Chapters of the Story",
            path: "exercises/lesson22/03_spans/src/lib.rs",
            story: "A log line on its own says little. Knowing which quest and which step \
                    it came from says everything.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`let span = info_span!(\"quest\", name = %name); let _guard = span.enter();` at the top.",
                "In the loop: `let step = info_span!(\"step\", n); let _s = step.enter();` before `info!`.",
            ],
            quiz: Some(Quiz {
                question: "Why does `let _ = span.enter();` not work?",
                options: &[
                    "enter() needs await",
                    "`_` drops the guard immediately, so you leave the span right away",
                    "spans can't be entered twice",
                    "It works fine",
                ],
                answer: 1,
                explain: "`_` doesn't bind, so the guard is dropped at once. Use `_guard` to \
                          keep it alive until the end of the scope.",
            }),
        },
        Exercise {
            id: "l22_04_instrument",
            lesson: 22,
            code: "22-4",
            title: "Instrument Everything",
            path: "exercises/lesson22/04_instrument/src/lib.rs",
            story: "The alchemist wants every brew recorded, but her secret recipe must \
                    never appear in the log.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Put `#[instrument]` above `brew`.",
                "`#[instrument(skip(secret_recipe))]`",
            ],
            quiz: Some(Quiz {
                question: "By default, how does #[instrument] record arguments?",
                options: &["Display", "Debug", "it doesn't", "as JSON"],
                answer: 1,
                explain: "Arguments are recorded with Debug, so strings show quotes. Use skip \
                          or fields(...) to control it.",
            }),
        },
        Exercise {
            id: "l22_05_filter",
            lesson: 22,
            code: "22-5",
            title: "Turn Down the Noise",
            path: "exercises/lesson22/05_filter/src/lib.rs",
            story: "The town is chatty, the dungeon is dangerous. Listen closely to one, \
                    and only to alarms from the other.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Every line gets through because nothing filters. Replace `.with_max_level(...)`.",
                "`.with_env_filter(tracing_subscriber::EnvFilter::new(directives))`",
            ],
            quiz: Some(Quiz {
                question: "`RUST_LOG=info,my_app::db=debug` means...",
                options: &[
                    "debug everywhere",
                    "info everywhere, plus debug for the my_app::db module",
                    "only my_app::db",
                    "info only for my_app::db",
                ],
                answer: 1,
                explain: "Directives combine: a default level plus per-target overrides.",
            }),
        },
        Exercise {
            id: "l22_06_async_spans",
            lesson: 22,
            code: "22-6",
            title: "Tangled Threads of Story",
            path: "exercises/lesson22/06_async_spans/src/lib.rs",
            story: "Two couriers share one road. The log mixes up who did what. Untangle it.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`span.enter()` across `.await` leaks the span to other tasks on the thread.",
                "Wrap the body in an async block and attach the span: \
                 `async move { ... }.instrument(info_span!(\"courier\", id)).await`",
                "Or put `#[tracing::instrument]` on `deliver` and remove the manual span.",
            ],
            quiz: Some(Quiz {
                question: "How do you attach a span to a future correctly?",
                options: &[
                    "span.enter() before awaiting",
                    "fut.instrument(span) or #[instrument]",
                    "spawn it on a new thread",
                    "spans don't work with async",
                ],
                answer: 1,
                explain: "Instrumented futures enter the span only while being polled, so \
                          other tasks never see it.",
            }),
        },
        Exercise {
            id: "l22_07_boss",
            lesson: 22,
            code: "BOSS",
            title: "The Silent Saboteur",
            path: "exercises/lesson22/07_boss/src/lib.rs",
            story: "Somewhere in the dungeon code, a saboteur hides in the silence. Light \
                    every corridor with a log line.",
            check: Check::CargoTest,
            xp: 400,
            hints: &[
                "`#[instrument(skip_all, fields(hero = %hero))]` on run_dungeon.",
                "Per room: `for (i, kind) in rooms.iter().enumerate() { let n = i + 1; \
                 let span = info_span!(\"room\", n, kind = %kind); let _g = span.enter(); ... }`",
                "Trap: `hp = hp.saturating_sub(30); warn!(hp, \"trap sprung\"); if hp == 0 \
                 { error!(\"hero fell\"); return Err(format!(\"{hero} fell in room {n}\")); }`",
                "After the loop, OUTSIDE the room span: `info!(gold, \"run complete\"); Ok(gold)`.",
            ],
            quiz: Some(Quiz {
                question: "What belongs in logs?",
                options: &[
                    "passwords, to debug logins",
                    "IDs, counts, states and errors, never secrets",
                    "everything, always",
                    "only errors",
                ],
                answer: 1,
                explain: "Log facts that explain what happened. Secrets and personal data stay \
                          out, which is what skip() is for.",
            }),
        },
    ],
};
