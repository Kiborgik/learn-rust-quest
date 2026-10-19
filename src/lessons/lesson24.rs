use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 24,
    title: "The Guild Hall: web servers with axum",
    notes: "starters/lesson24/README.md",
    boss_trophy: ("boss_l24", "Colossus of the Quest Board"),
    exercises: &[
        Exercise {
            id: "l24_01_routes",
            lesson: 24,
            code: "24-1",
            title: "Open the Gates",
            path: "exercises/lesson24/01_routes/src/lib.rs",
            story: "Last lesson you knocked on other towers. Now the guild builds its own \
                    hall, and the doors have to open for the right visitors.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "GET /health gets 405: the route is registered for POST. Use `get(health)`.",
                "`async fn not_found() -> (StatusCode, &'static str) { (StatusCode::NOT_FOUND, \"no such room\") }`",
                "Add `.fallback(not_found)` to the Router.",
            ],
            quiz: Some(Quiz {
                question: "A path exists but only for GET. A client sends DELETE. What does axum answer?",
                options: &[
                    "404 Not Found",
                    "405 Method Not Allowed",
                    "500",
                    "it runs the GET handler",
                ],
                answer: 1,
                explain: "The path matched, the method didn't: 405. 404 is for paths nothing matches.",
            }),
        },
        Exercise {
            id: "l24_02_extract",
            lesson: 24,
            code: "24-2",
            title: "Doors and Signs",
            path: "exercises/lesson24/02_extract/src/lib.rs",
            story: "Visitors ask for things by door number and by the signs they carry. Read \
                    both without writing a parser.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "The panic says captures use `{capture}`: the route is `\"/heroes/{id}\"`.",
                "`q.name` is an `Option<String>`. `q.name.as_deref().unwrap_or(\"stranger\")` gives a &str.",
                "`format!(\"Hello, {}!\", q.name.as_deref().unwrap_or(\"stranger\"))`",
            ],
            quiz: Some(Quiz {
                question: "A handler takes `Path(id): Path<u32>`. The request is /heroes/abc. What happens?",
                options: &[
                    "id is 0",
                    "the handler panics",
                    "axum answers 400 and the handler never runs",
                    "it falls back to 404",
                ],
                answer: 2,
                explain: "Extractors run first. If one fails, its rejection is the response.",
            }),
        },
        Exercise {
            id: "l24_03_json",
            lesson: 24,
            code: "24-3",
            title: "Scrolls In, Scrolls Out",
            path: "exercises/lesson24/03_json/src/lib.rs",
            story: "New heroes send their papers as JSON. The clerk must stamp them properly: \
                    created, and filed at the right shelf.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Change the return type to a tuple: `-> (StatusCode, [(header::HeaderName, &'static str); 1], Json<Hero>)`.",
                "Return `(StatusCode::CREATED, [(header::LOCATION, \"/heroes/1\")], Json(hero))`.",
                "Or keep it short: `-> impl IntoResponse` (import `axum::response::IntoResponse`).",
            ],
            quiz: Some(Quiz {
                question: "Valid JSON arrives, but a required field is missing. What does the Json extractor answer?",
                options: &["400", "415", "422 Unprocessable Entity", "500"],
                answer: 2,
                explain: "400 is for broken JSON, 415 for a missing content-type, 422 for JSON of the \
                          wrong shape.",
            }),
        },
        Exercise {
            id: "l24_04_state",
            lesson: 24,
            code: "24-4",
            title: "The Shared Ledger",
            path: "exercises/lesson24/04_state/src/lib.rs",
            story: "Every clerk in the hall writes in the same ledger. Make sure they all \
                    get it, and nobody tears a page.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "\"expected `Router`, found `Router<AppState>`\": end the chain with `.with_state(AppState::default())`.",
                "count: `state.heroes.lock().unwrap().len().to_string()`",
            ],
            quiz: Some(Quiz {
                question: "Why is AppState's data wrapped in Arc?",
                options: &[
                    "axum requires Arc for every type",
                    "State is cloned for each request; Arc makes all clones share one ledger",
                    "Arc makes it faster",
                    "Mutex doesn't work without Arc",
                ],
                answer: 1,
                explain: "Each handler gets its own clone of the state. With Arc, a clone is a new \
                          pointer to the same data.",
            }),
        },
        Exercise {
            id: "l24_05_errors",
            lesson: 24,
            code: "24-5",
            title: "Polite Refusals",
            path: "exercises/lesson24/05_errors/src/lib.rs",
            story: "When the clerk can't help, a shrug and \"something went wrong\" won't do. \
                    Say what the problem is, the same way every time.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`let (status, msg) = match self { ApiError::NotFound(id) => (StatusCode::NOT_FOUND, \
                 format!(\"hero {id} not found\")), ... };`",
                "BadLevel: `(StatusCode::BAD_REQUEST, format!(\"level {lvl} is not in 1..=100\"))`",
                "`(status, Json(json!({ \"error\": msg }))).into_response()`",
            ],
            quiz: Some(Quiz {
                question: "What lets a handler return `Result<Json<Hero>, ApiError>`?",
                options: &[
                    "ApiError implements std::error::Error",
                    "Both Json<Hero> and ApiError implement IntoResponse",
                    "axum catches panics",
                    "Result is special-cased for 500",
                ],
                answer: 1,
                explain: "Result<T, E> is IntoResponse when both sides are. Ok and Err just pick which \
                          response to send.",
            }),
        },
        Exercise {
            id: "l24_06_middleware",
            lesson: 24,
            code: "24-6",
            title: "The Gatekeeper",
            path: "exercises/lesson24/06_middleware/src/lib.rs",
            story: "The vault needs a guard, but someone posted him at the main door. Now \
                    nobody gets in at all.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "Put the guard on the vault Router only: \
                 `Router::new().route(\"/gold\", get(gold)).layer(middleware::from_fn(require_token))`.",
                "stamp: `let mut res = next.run(req).await; \
                 res.headers_mut().insert(\"x-guild\", HeaderValue::from_static(\"open\")); res`",
                "Then add `.layer(middleware::from_fn(stamp))` at the end of the main Router.",
            ],
            quiz: Some(Quiz {
                question: "`.layer(L)` on a Router wraps which routes?",
                options: &[
                    "all routes in the program",
                    "the routes added to that Router before the call",
                    "only the next route added",
                    "only routes with the same path",
                ],
                answer: 1,
                explain: "A layer wraps what's already in that Router. Routes added after it, or in \
                          other Routers, aren't wrapped.",
            }),
        },
        Exercise {
            id: "l24_07_boss",
            lesson: 24,
            code: "BOSS",
            title: "The Quest Board Colossus",
            path: "exercises/lesson24/07_boss/src/lib.rs",
            story: "The guild's paper quest board is falling apart. Put it online: a full \
                    REST API, with state, JSON and honest errors.",
            check: Check::CargoTest,
            xp: 400,
            hints: &[
                "State: `#[derive(Default)] struct Inner { quests: Vec<Quest>, last_id: u32 }` and \
                 `#[derive(Clone, Default)] pub struct Board { inner: Arc<Mutex<Inner>> }`.",
                "Routes: `.route(\"/quests\", get(list).post(create))`, \
                 `.route(\"/quests/{id}\", get(one).delete(remove))`, \
                 `.route(\"/quests/{id}/done\", patch(finish))`, then `.with_state(Board::default())`.",
                "list: `Query(f): Query<Filter>` and filter with `f.done.is_none_or(|d| q.done == d)`.",
                "create returns `Result<(StatusCode, Json<Quest>), ApiError>`; remove returns \
                 `Result<StatusCode, ApiError>` with `StatusCode::NO_CONTENT`.",
            ],
            quiz: Some(Quiz {
                question: "Which status fits a successful DELETE that returns no body?",
                options: &["200 OK", "201 Created", "204 No Content", "404 Not Found"],
                answer: 2,
                explain: "204 says: done, nothing to send back.",
            }),
        },
    ],
};
