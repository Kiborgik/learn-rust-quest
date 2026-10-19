use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 26,
    title: "Capstone: the Guild Board (CLI + REST service)",
    notes: "starters/lesson26/README.md",
    boss_trophy: ("boss_l26", "Founder of the Guild Hall"),
    exercises: &[
        Exercise {
            id: "l26_01_domain",
            lesson: 26,
            code: "26-1",
            title: "The Charter",
            path: "exercises/lesson26/01_domain/src/domain.rs",
            story: "The guild asks you to build its quest board, start to finish. First, write \
                    down the rules: what a quest is, and what may never happen to one.",
            check: Check::CargoTest,
            xp: 200,
            hints: &[
                "validate: `let title = self.title.trim(); if title.is_empty() { return Err(BoardError::EmptyTitle); }`",
                "`if !(1..=MAX_REWARD).contains(&self.reward) { return Err(BoardError::BadReward(self.reward)); }` \
                 then `Ok(NewQuest { title: title.to_string(), reward: self.reward })`",
                "finish: `if self.done { return Err(BoardError::AlreadyDone(self.id)); } self.done = true; Ok(self.reward)`",
                "Summary::of: start from `Summary::default()` and add each quest to the done or open side.",
            ],
            quiz: Some(Quiz {
                question: "Why keep the domain free of database and HTTP code?",
                options: &[
                    "It compiles faster",
                    "The rules can be tested alone and reused by every layer",
                    "serde doesn't work with sqlx",
                    "Rust requires it",
                ],
                answer: 1,
                explain: "Pure rules are easy to test and don't care if the caller is an API, a CLI \
                          or a test.",
            }),
        },
        Exercise {
            id: "l26_02_store",
            lesson: 26,
            code: "26-2",
            title: "The Ledger Vault",
            path: "exercises/lesson26/02_store/src/store.rs",
            story: "Rules on paper are nice, but quests must survive the night. Give the board \
                    a vault that stores them and still obeys the charter.",
            check: Check::CargoTest,
            xp: 200,
            hints: &[
                "add: `let new = new.validate()?;` then INSERT with binds and use `last_insert_rowid()` as the id.",
                "get: `query_as::<_, QuestRow>(..).fetch_optional(&self.pool).await?` then \
                 `Ok(row.ok_or(BoardError::NotFound(id))?.into())`",
                "list: `WHERE ? IS NULL OR done = ? ORDER BY id` with `.bind(done).bind(done)`, then \
                 `rows.into_iter().map(Quest::from).collect()`",
                "finish: in a transaction: read the row, `quest.finish()?`, \
                 `UPDATE quests SET done = TRUE WHERE id = ?`, `tx.commit()`.",
            ],
            quiz: Some(Quiz {
                question: "Why does the store read into a private QuestRow instead of deriving FromRow on Quest?",
                options: &[
                    "FromRow can't be derived on pub structs",
                    "It keeps sqlx out of the domain; a schema change stays in store.rs",
                    "It's faster",
                    "Quest has no Default",
                ],
                answer: 1,
                explain: "The domain doesn't depend on the database crate. Only the store knows how \
                          rows look.",
            }),
        },
        Exercise {
            id: "l26_03_api",
            lesson: 26,
            code: "26-3",
            title: "The Front Desk",
            path: "exercises/lesson26/03_api/src/api.rs",
            story: "Adventurers can't walk into the vault. Open a front desk: a REST API that \
                    takes requests, asks the vault, and answers in clear terms.",
            check: Check::CargoTest,
            xp: 200,
            hints: &[
                "`let status = match &self { AppError::Board(BoardError::EmptyTitle | BoardError::BadReward(_)) \
                 => StatusCode::UNPROCESSABLE_ENTITY, ... };`",
                "Message: `match &self { AppError::Db(_) => \"internal error\".to_string(), other => other.to_string() }`, \
                 then `(status, Json(json!({ \"error\": message }))).into_response()`.",
                "Handlers are one-liners: `async fn get_one(State(store): State<Store>, Path(id): Path<i64>) \
                 -> Result<Json<Quest>, AppError> { Ok(Json(store.get(id).await?)) }`",
                "add returns `Result<(StatusCode, Json<Quest>), AppError>`; summary is \
                 `Json(Summary::of(&store.list(None).await?))`. End with `.with_state(store)`.",
            ],
            quiz: Some(Quiz {
                question: "A valid request asks to finish a quest that's already done. Best status?",
                options: &["400 Bad Request", "404 Not Found", "409 Conflict", "500"],
                answer: 2,
                explain: "The request is fine, the quest's state isn't. 409 tells the client that \
                          repeating it won't help.",
            }),
        },
        Exercise {
            id: "l26_04_client",
            lesson: 26,
            code: "26-4",
            title: "The Courier",
            path: "exercises/lesson26/04_client/src/client.rs",
            story: "The desk is open, but nobody wants to write raw HTTP letters. Train a \
                    courier who knows every route and brings back real answers.",
            check: Check::CargoTest,
            xp: 200,
            hints: &[
                "`pub struct GuildClient { http: reqwest::Client, base: String }` and in new: \
                 `reqwest::Client::builder().timeout(Duration::from_secs(5)).build().unwrap()`.",
                "Every method: build the request, `.send().await?`, then `check(resp).await?.json().await?` \
                 wrapped in `Ok(...)`.",
                "add: `.post(..).json(&NewQuest { title: title.to_string(), reward })`; finish is \
                 `POST /quests/{id}/finish`.",
                "list: only add `.query(&[(\"done\", d)])` when `done` is `Some(d)`.",
            ],
            quiz: Some(Quiz {
                question: "Client and server both use domain::Quest. What's the main win?",
                options: &[
                    "Smaller binaries",
                    "Change the type once and both sides agree; the compiler finds the mismatches",
                    "JSON becomes faster",
                    "No need for serde",
                ],
                answer: 1,
                explain: "Shared types turn many API mismatches into compile errors.",
            }),
        },
        Exercise {
            id: "l26_05_cli",
            lesson: 26,
            code: "26-5",
            title: "The Town Crier",
            path: "exercises/lesson26/05_cli/src/cli.rs",
            story: "Heroes live in terminals. Give them a crier who takes short commands and \
                    shouts back short, clear lines.",
            check: Check::CargoTest,
            xp: 200,
            hints: &[
                "`Add { title: String, #[arg(long, short)] reward: i64 }` and `Finish { id: i64 }`.",
                "run: `let client = GuildClient::new(&cli.url); match cli.command { ... }`",
                "list: turn the flags into `Option<bool>` with `match (done, open) { (true, _) => Some(true), \
                 (_, true) => Some(false), _ => None }`.",
                "`writeln!(out, \"#{} [{}] {} ({} gold)\", q.id, if q.done { 'x' } else { ' ' }, q.title, q.reward)?;`",
            ],
            quiz: Some(Quiz {
                question: "Why does `run` write to `&mut impl Write` instead of using println!?",
                options: &[
                    "println! is slow",
                    "Tests can pass a Vec<u8> and check exactly what was printed",
                    "println! doesn't work in async",
                    "clap requires it",
                ],
                answer: 1,
                explain: "The same code prints to stdout in main and into a buffer in tests.",
            }),
        },
        Exercise {
            id: "l26_06_boss",
            lesson: 26,
            code: "BOSS",
            title: "The Guild Hall Opens",
            path: "exercises/lesson26/06_boss/src/server.rs",
            story: "Everything is built. Tonight the hall opens its doors for real: read the \
                    charter's config, light the lamps, and close up cleanly at dawn.",
            check: Check::CargoTest,
            xp: 500,
            hints: &[
                "Config: add `#[serde(default, deny_unknown_fields)]`; from_toml is `toml::from_str(text)`.",
                "`pub struct Running { pub url: String, stop: oneshot::Sender<()>, task: JoinHandle<std::io::Result<()>> }`",
                "start: `Store::open(&config.database).await.context(\"opening the database\")?`, \
                 `TcpListener::bind(&config.addr).await.with_context(|| format!(\"binding {}\", config.addr))?`, \
                 url from `listener.local_addr()?`.",
                "`let (stop, rx) = oneshot::channel::<()>(); let task = tokio::spawn(async move { \
                 axum::serve(listener, router(store)).with_graceful_shutdown(async { rx.await.ok(); }).await });` \
                 and stop: `let _ = self.stop.send(()); self.task.await??; Ok(())`",
            ],
            quiz: Some(Quiz {
                question: "What does a graceful shutdown do that killing the process doesn't?",
                options: &[
                    "Nothing, it's the same",
                    "Stops taking new requests and lets running ones finish",
                    "Restarts the server",
                    "Deletes the database",
                ],
                answer: 1,
                explain: "Requests in flight complete and the database is left in a clean state.",
            }),
        },
    ],
};
