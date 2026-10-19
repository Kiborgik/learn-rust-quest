// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 25-5 · The Chronicle                             175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Real apps don't run CREATE TABLE by hand. They keep MIGRATIONS:
//   numbered SQL files, applied in order, each one exactly once.
//
//     migrations/
//       0001_heroes.sql
//       0002_hero_level.sql
//
//     sqlx::migrate!()          // embeds ./migrations into the program
//         .run(&pool).await?;   // applies the ones not applied yet
//
//   sqlx remembers what ran in a table called `_sqlx_migrations`, so
//   running it on every start is safe. To change the schema, ADD a new
//   file. Don't edit one that already ran: sqlx stores a checksum and
//   will refuse.
//
//   `migrate!` reads the files at COMPILE time. The `build.rs` in this
//   package tells cargo to rebuild when they change.
//   (The `sqlx-cli` tool can create files for you:
//   `sqlx migrate add items`.)
//
// 🎯 YOUR TASK
//
//   1. `setup` should run the migrations.
//   2. Inventories need a table. Add `migrations/0003_items.sql` that
//      creates `items`: id (INTEGER PRIMARY KEY), hero_id (INTEGER NOT
//      NULL, REFERENCES heroes(id)), name (TEXT NOT NULL).

use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

/// Given: an empty in-memory database.
pub async fn open() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect("sqlite::memory:")
        .await
        .unwrap()
}

pub async fn setup(pool: &SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
    todo!()
}

pub async fn add_hero(pool: &SqlitePool, name: &str) -> Result<i64, sqlx::Error> {
    let done = sqlx::query("INSERT INTO heroes (name) VALUES (?)").bind(name).execute(pool).await?;
    Ok(done.last_insert_rowid())
}

pub async fn add_item(pool: &SqlitePool, hero_id: i64, name: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO items (hero_id, name) VALUES (?, ?)")
        .bind(hero_id)
        .bind(name)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn inventory(pool: &SqlitePool, hero_id: i64) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT name FROM items WHERE hero_id = ? ORDER BY id")
        .bind(hero_id)
        .fetch_all(pool)
        .await
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    async fn applied(pool: &SqlitePool) -> i64 {
        sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations").fetch_one(pool).await.unwrap()
    }

    #[tokio::test]
    async fn migrations_run_once() {
        let pool = open().await;
        setup(&pool).await.unwrap();
        assert_eq!(applied(&pool).await, 3);
        setup(&pool).await.unwrap();
        assert_eq!(applied(&pool).await, 3);
    }

    #[tokio::test]
    async fn inventories() {
        let pool = open().await;
        setup(&pool).await.unwrap();
        let ferris = add_hero(&pool, "Ferris").await.unwrap();
        let level: i64 = sqlx::query_scalar("SELECT level FROM heroes").fetch_one(&pool).await.unwrap();
        assert_eq!(level, 1);
        add_item(&pool, ferris, "claw polish").await.unwrap();
        add_item(&pool, ferris, "tiny hat").await.unwrap();
        assert_eq!(inventory(&pool, ferris).await.unwrap(), ["claw polish", "tiny hat"]);
        assert!(add_item(&pool, 99, "ghost sword").await.is_err(), "items need a real hero");
    }
}
