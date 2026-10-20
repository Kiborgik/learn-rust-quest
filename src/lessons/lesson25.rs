use crate::exercises::{Check, Exercise, Lesson, Quiz};

pub const LESSON: Lesson = Lesson {
    number: 25,
    title: "The Archive: databases with sqlx",
    notes: "starters/lesson25/README.md",
    boss_trophy: ("boss_l25", "Keeper of the Archive"),
    exercises: &[
        Exercise {
            id: "l25_01_connect",
            lesson: 25,
            code: "25-1",
            title: "The Archive Opens",
            path: "exercises/lesson25/01_connect/src/lib.rs",
            story: "The guild's paper records are a mess. The new archive speaks SQL, and \
                    the first clerk already let a prankster's name wreck a page.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`sqlx::query(\"INSERT INTO heroes (name, level) VALUES (?, ?)\").bind(name).bind(level).execute(pool).await?;`",
                "count: `sqlx::query_scalar(\"SELECT COUNT(*) FROM heroes\").fetch_one(pool).await`",
            ],
            quiz: Some(Quiz {
                question: "Why use `?` placeholders with `.bind()` instead of format!?",
                options: &[
                    "It's faster to type",
                    "Values travel apart from the SQL, so quotes can't break it or inject SQL",
                    "format! can't build strings with quotes",
                    "SQLite requires it",
                ],
                answer: 1,
                explain: "A bound value is never parsed as SQL. That stops both broken queries and \
                          SQL injection.",
            }),
        },
        Exercise {
            id: "l25_02_query_as",
            lesson: 25,
            code: "25-2",
            title: "Rows into Structs",
            path: "exercises/lesson25/02_query_as/src/lib.rs",
            story: "Rows of numbers and strings are hard to work with. The archive can hand \
                    you real Hero values instead.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "find: swap `.fetch_one(pool).await?` for `.fetch_optional(pool).await` and return it directly.",
                "strong: `sqlx::query_as::<_, Hero>(\"SELECT id, name, level FROM heroes WHERE level >= ? \
                 ORDER BY level DESC\").bind(min_level).fetch_all(pool).await`",
            ],
            quiz: Some(Quiz {
                question: "A query may return zero rows or one row. Which fetch fits?",
                options: &["fetch_one", "fetch_all", "fetch_optional", "execute"],
                answer: 2,
                explain: "fetch_optional gives Ok(None) for no row. fetch_one would be an error.",
            }),
        },
        Exercise {
            id: "l25_03_changes",
            lesson: 25,
            code: "25-3",
            title: "Ink and Eraser",
            path: "exercises/lesson25/03_changes/src/lib.rs",
            story: "One hero earned a level and the clerk promoted the entire guild. Time for \
                    a more careful pen.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "create: `.execute(pool).await?.last_insert_rowid()` after an INSERT with `level` 1.",
                "level_up needs `WHERE id = ?` and `.bind(id)`.",
                "retire: `DELETE FROM heroes WHERE id = ?`, then `Ok(done.rows_affected() == 1)`.",
            ],
            quiz: Some(Quiz {
                question: "UPDATE ... WHERE id = 99 finds no such row. What happens?",
                options: &[
                    "An error",
                    "Ok, with rows_affected() == 0",
                    "A new row is created",
                    "A panic",
                ],
                answer: 1,
                explain: "Changing zero rows is not an error in SQL. Check rows_affected() when it \
                          matters.",
            }),
        },
        Exercise {
            id: "l25_04_transactions",
            lesson: 25,
            code: "25-4",
            title: "All or Nothing",
            path: "exercises/lesson25/04_transactions/src/lib.rs",
            story: "The guild bank lost track of its gold. Half-finished transfers have been \
                    minting coins out of thin air.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "`let mut tx = pool.begin().await?;` and run both queries with `.execute(&mut *tx)`.",
                "Check `rows_affected() == 0` after BOTH updates and `bail!` if so. The dropped tx rolls back.",
                "End with `tx.commit().await?; Ok(())`.",
            ],
            quiz: Some(Quiz {
                question: "A function returns early with `?` before calling `tx.commit()`. What happens to its changes?",
                options: &[
                    "They're saved",
                    "They're rolled back when tx is dropped",
                    "The database locks forever",
                    "It doesn't compile",
                ],
                answer: 1,
                explain: "Transaction's Drop rolls back. Only commit() makes the changes real.",
            }),
        },
        Exercise {
            id: "l25_05_migrations",
            lesson: 25,
            code: "25-5",
            title: "The Chronicle",
            path: "exercises/lesson25/05_migrations/src/lib.rs",
            story: "Every change to the archive's shelves must be written in the Chronicle, \
                    in order, so any new archive can be built the same way.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "setup: `sqlx::migrate!().run(pool).await`",
                "Create the file `migrations/0003_items.sql` next to `0002_hero_level.sql`.",
                "`CREATE TABLE items (id INTEGER PRIMARY KEY, hero_id INTEGER NOT NULL REFERENCES heroes(id), \
                 name TEXT NOT NULL);`",
            ],
            quiz: Some(Quiz {
                question: "A migration already ran in production and you need another column. What do you do?",
                options: &[
                    "Edit the old migration file",
                    "Add a new migration file",
                    "Drop the database",
                    "Change the table by hand",
                ],
                answer: 1,
                explain: "Migrations are history. Old ones never change; sqlx even checks their \
                          checksums.",
            }),
        },
        Exercise {
            id: "l25_06_joins",
            lesson: 25,
            code: "25-6",
            title: "The Census",
            path: "exercises/lesson25/06_joins/src/lib.rs",
            story: "The king wants numbers: who belongs where, and which guild is strongest. \
                    One wanderer belongs nowhere, and the census clerk writes him down wrong.",
            check: Check::CargoTest,
            xp: 175,
            hints: &[
                "The guild name may be NULL: return `Vec<(String, Option<String>)>`. That's all for part 1.",
                "report: `SELECT g.name AS guild, COUNT(*) AS members, SUM(h.level) AS total_level \
                 FROM guilds g JOIN heroes h ON h.guild_id = g.id GROUP BY g.id ORDER BY total_level DESC`",
                "`sqlx::query_as::<_, GuildReport>(...).fetch_all(pool).await`",
            ],
            quiz: Some(Quiz {
                question: "A column can be NULL. What Rust type should you read it into?",
                options: &["String", "Option<String>", "Box<String>", "Result<String>"],
                answer: 1,
                explain: "NULL maps to None. A plain String can't tell NULL from \"\", and on \
                          Postgres it fails with \"unexpected null\".",
            }),
        },
        Exercise {
            id: "l25_07_boss",
            lesson: 25,
            code: "BOSS",
            title: "The Archive Keeper",
            path: "exercises/lesson25/07_boss/src/lib.rs",
            story: "The Keeper lets no bad record in and no good record out of place. Build \
                    the store that guards the guild's whole history.",
            check: Check::CargoTest,
            xp: 400,
            hints: &[
                "`pub struct Store { pool: SqlitePool }`. open: the pool from earlier quests, then \
                 `sqlx::raw_sql(SCHEMA).execute(&pool).await?`.",
                "add_hero: `match sqlx::query(..).execute(&self.pool).await { Ok(done) => Ok(done.last_insert_rowid()), \
                 Err(sqlx::Error::Database(e)) if e.is_unique_violation() => Err(StoreError::Duplicate(name.into())), \
                 Err(e) => Err(e.into()) }`",
                "Reuse the insert in add_party: make it a fn taking `impl sqlx::SqliteExecutor<'_>` \
                 (quest 25-4), so it accepts `&self.pool` and `&mut *tx`.",
                "leaderboard: `SELECT h.name, COUNT(i.id) FROM heroes h LEFT JOIN items i ON i.hero_id = h.id \
                 GROUP BY h.id ORDER BY COUNT(i.id) DESC, h.name`",
            ],
            quiz: Some(Quiz {
                question: "Why map sqlx errors to StoreError::Duplicate / NotFound in the store?",
                options: &[
                    "sqlx errors can't be printed",
                    "Callers can match on what went wrong without knowing SQL or SQLite codes",
                    "It's faster",
                    "thiserror requires it",
                ],
                answer: 1,
                explain: "The store hides its storage. A web handler can turn Duplicate into 409 \
                          without parsing database messages.",
            }),
        },
    ],
};
