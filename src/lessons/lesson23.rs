use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 23,
    title: "The Raven Post: HTTP clients with reqwest",
    notes: "starters/lesson23/README.md",
    boss_trophy: ("boss_l23", "Hydra of Many Endpoints"),
    exercises: &[
        Exercise {
            id: "l23_01_get",
            lesson: 23,
            code: "23-1",
            title: "A Letter from Afar",
            path: "exercises/lesson23/01_get/src/lib.rs",
            story: "The Raven Post carries messages between distant towers. Your first \
                    letter: ask a tower for its message of the day.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "\"expected `String`, found `impl Future`\": reading the body is async too: `.text().await?`.",
                "`let client = reqwest::Client::new();` then \
                 `client.get(format!(\"{base}/motd\")).send().await?.text().await?` (and the same for /news).",
                "`Ok(format!(\"{motd} | {news}\"))`",
            ],
            quiz: Some(Quiz {
                question: "Why reuse one reqwest::Client instead of calling reqwest::get each time?",
                options: &[
                    "reqwest::get doesn't work twice",
                    "A Client keeps connections open and shares settings",
                    "Clients are required for GET",
                    "It's only a style choice",
                ],
                answer: 1,
                explain: "A Client holds a connection pool. Reusing it saves a new TCP (and TLS) \
                          handshake per request.",
            }),
        },
        Exercise {
            id: "l23_02_json",
            lesson: 23,
            code: "23-2",
            title: "Typed Replies",
            path: "exercises/lesson23/02_json/src/lib.rs",
            story: "The towers reply in JSON. Read it straight into proper Rust types.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`.json()` needs the type to implement Deserialize: `#[derive(Debug, PartialEq, Deserialize)]`.",
                "`let heroes: Vec<Hero> = client.get(format!(\"{base}/party\")).send().await?.json().await?;`",
                "`Ok(heroes.into_iter().filter(|h| h.level >= min_level).map(|h| h.name).collect())`",
            ],
            quiz: Some(Quiz {
                question: "The JSON has an extra field \"class\" your struct doesn't have. Result?",
                options: &[
                    "a decoding error",
                    "it's ignored",
                    "a panic",
                    "it's stored in a map",
                ],
                answer: 1,
                explain: "serde ignores unknown fields by default, so APIs can add fields \
                          without breaking you.",
            }),
        },
        Exercise {
            id: "l23_03_query",
            lesson: 23,
            code: "23-3",
            title: "The Right Questions",
            path: "exercises/lesson23/03_query/src/lib.rs",
            story: "The registry tower only answers questions with the guild seal attached, \
                    asked in exactly the right form.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`client.get(format!(\"{base}/heroes\")).query(&[(\"class\", class), (\"min_level\", &min_level.to_string())])`",
                "Add `.header(\"x-guild-token\", token)` before `.send().await?.text().await`.",
            ],
            quiz: Some(Quiz {
                question: "Why use `.query(&params)` instead of `format!(\"{url}?class={c}\")`?",
                options: &[
                    "It's shorter",
                    "It URL-encodes values (spaces, &, = ...) correctly",
                    "format! doesn't work with URLs",
                    "Servers reject format! URLs",
                ],
                answer: 1,
                explain: "A value like \"dark elf & co\" would break a hand-built URL. .query() \
                          escapes it.",
            }),
        },
        Exercise {
            id: "l23_04_post",
            lesson: 23,
            code: "23-4",
            title: "Send a Raven",
            path: "exercises/lesson23/04_post/src/lib.rs",
            story: "A new hero wants to join the registry. Send the form, and keep the id \
                    the clerk writes back.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`client.post(format!(\"{base}/heroes\")).json(hero).send().await?`",
                "`let created: Created = ... .json().await?; Ok(created.id)`",
            ],
            quiz: Some(Quiz {
                question: "What does `.json(&value)` on a request builder do?",
                options: &[
                    "Parses the response",
                    "Serializes value as the body and sets content-type: application/json",
                    "Validates JSON",
                    "Sends a GET with JSON query",
                ],
                answer: 1,
                explain: "On the request it serializes the body. On the response, `.json()` \
                          deserializes it.",
            }),
        },
        Exercise {
            id: "l23_05_status",
            lesson: 23,
            code: "23-5",
            title: "Read the Seal",
            path: "exercises/lesson23/05_status/src/lib.rs",
            story: "A raven came back. That doesn't mean the answer was yes. Check the seal \
                    before you read the letter.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`client.get(url).send().await?.error_for_status()?.text().await`",
                "find_hero: `let resp = client.get(...).send().await?; if resp.status() == StatusCode::NOT_FOUND \
                 { return Ok(None); }`",
                "Then `Ok(Some(resp.error_for_status()?.json().await?))`",
            ],
            quiz: Some(Quiz {
                question: "A server answers 500. What does `client.get(url).send().await` return?",
                options: &["Err", "Ok(Response) with status 500", "a panic", "Ok(None)"],
                answer: 1,
                explain: "HTTP errors are valid responses. Check `status()` or call \
                          `error_for_status()`.",
            }),
        },
        Exercise {
            id: "l23_06_retry",
            lesson: 23,
            code: "23-6",
            title: "Patience and Persistence",
            path: "exercises/lesson23/06_retry/src/lib.rs",
            story: "Some towers are slow, some are busy. A good courier knows when to wait, \
                    when to try again, and when to give up.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`reqwest::Client::builder().timeout(Duration::from_millis(300)).build().unwrap()`",
                "`for attempt in 1..=attempts { let resp = client.get(url).send().await?; \
                 if resp.status() == StatusCode::SERVICE_UNAVAILABLE && attempt < attempts { \
                 sleep(Duration::from_millis(50 * attempt as u64)).await; continue; } \
                 return resp.error_for_status()?.text().await; }`",
                "After the loop: `unreachable!()` (attempts is at least 1).",
            ],
            quiz: Some(Quiz {
                question: "Which request is generally SAFE to retry automatically?",
                options: &[
                    "POST /payments",
                    "GET /heroes/1",
                    "POST /orders",
                    "any request, always",
                ],
                answer: 1,
                explain: "GET is idempotent: repeating it changes nothing. Retrying a POST may \
                          do the action twice.",
            }),
        },
        Exercise {
            id: "l23_07_boss",
            lesson: 23,
            code: "BOSS",
            title: "The Registry Hydra",
            path: "exercises/lesson23/07_boss/src/lib.rs",
            story: "The Guild Registry has an endpoint for everything, and each one bites \
                    differently. Tame them all with one solid client.",
            check: Check::CargoTest,
            xp: 400,
            hints: &[
                "`pub struct Registry { client: reqwest::Client, base: String }` and in new: \
                 `reqwest::Client::builder().timeout(Duration::from_secs(2)).build()?`",
                "list: `let mut req = self.client.get(format!(\"{}/heroes\", self.base)); if let Some(c) = class \
                 { req = req.query(&[(\"class\", c)]); }` then `.send().await?.error_for_status()?.json().await?`",
                "create: `.post(..).json(&serde_json::json!({\"name\": name, \"class\": class}))`, \
                 then `if resp.status() == StatusCode::CONFLICT { bail!(\"hero {name} already exists\"); }`",
                "strongest: `let all = self.list(None).await.context(\"listing heroes\")?; \
                 Ok(all.into_iter().max_by_key(|h| h.level))`",
            ],
            quiz: Some(Quiz {
                question: "What makes an API client library pleasant to use?",
                options: &[
                    "Returning raw Response objects",
                    "Typed data, clear errors with context, and sensible timeouts",
                    "Retrying every request forever",
                    "Panicking on 404",
                ],
                answer: 1,
                explain: "Callers want Hero values and errors that explain what failed, not HTTP \
                          details.",
            }),
        },
    ],
};
