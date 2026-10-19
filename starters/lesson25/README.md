# Lesson 25 · The Archive 📚

> Databases with `sqlx`, using SQLite in memory.

```toml
# or "postgres", "mysql"
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
tokio = { version = "1", features = ["full"] }
```

```rust
#[derive(FromRow)]
// NULL → None
struct Hero { id: i64, name: String, guild: Option<String> }

let pool = SqlitePoolOptions::new()
    .connect("sqlite://guild.db?mode=rwc")
    .await?;
sqlx::migrate!().run(&pool).await?;

let hero = sqlx::query_as::<_, Hero>(
    "SELECT id, name, guild FROM heroes WHERE id = ?",
)
.bind(id)
.fetch_optional(&pool)
.await?;

let mut tx = pool.begin().await?;
sqlx::query("UPDATE ...").bind(..).execute(&mut *tx).await?;
tx.commit().await?;                 // dropped without commit = rollback
```

## Cheat sheet

| Want | Use |
|---|---|
| run, no rows back | `query(..).execute(&pool)` → `rows_affected()`, `last_insert_rowid()` |
| one value | `query_scalar(..).fetch_one(&pool)` |
| typed rows | `query_as::<_, T>(..)` with `#[derive(FromRow)]` or a tuple |
| 0 or 1 row | `.fetch_optional` |
| several statements, no binds | `sqlx::raw_sql(..)` |
| all or nothing | `pool.begin()`, `&mut *tx`, `commit()` |
| schema changes | `migrations/NNNN_name.sql` + `sqlx::migrate!()` |
| what failed | `sqlx::Error::Database(e)`: `is_unique_violation()`, `is_foreign_key_violation()` |

Placeholders are `?` in SQLite and MySQL, `$1, $2` in Postgres.

## Compile-time checked SQL

`sqlx::query!("SELECT ...")` and `query_as!` check your SQL and types
against a real database while compiling. They need `DATABASE_URL` at build
time (or `cargo sqlx prepare` for offline builds). Great in real projects;
these quests use the plain functions so they build anywhere.

## Other options

| Crate | Style |
|---|---|
| `sqlx` | async, you write SQL |
| `diesel` | sync (async add-on), query builder, strong compile-time checks |
| `sea-orm` | async ORM on top of sqlx |
| `rusqlite` | sync, SQLite only, small |

## Coming from other languages

| Elsewhere | sqlx |
|---|---|
| JDBC / ADO.NET prepared statements | `query(..).bind(..)` |
| Go `database/sql` | very close, plus derive for rows |
| Flyway / Alembic / EF migrations | `sqlx::migrate!()` |
| ORM entities | plain structs + `FromRow` (no lazy loading, no magic) |

## 🎮 Controls

| Key | Action |
|---|---|
| *save the file* | auto re-check |
| `Enter` | re-check now |
| `h` | reveal a hint (that quest gives half XP) |
| `m` | quest map & achievements |
| `q` | save & quit |
| `restore` | put this quest's files back to the start (asks first) |
| `reset` | start over: wipe progress and restore all files (asks first) |
