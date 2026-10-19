// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 25-1 · The Archive Opens                         175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `sqlx` talks to Postgres, MySQL and SQLite, async. This lesson uses
//   SQLite in memory: a real SQL database with no server and no files.
//
//     let pool = SqlitePoolOptions::new()
//         .max_connections(1)
//         .connect("sqlite::memory:").await?;
//
//   ⚠️ Each connection to "sqlite::memory:" is its OWN empty database.
//   With one connection in the pool, everyone sees the same data. (A
//   file, "sqlite://guild.db", or Postgres doesn't have this problem.)
//
//   Run SQL with `query`, and pass values with `?` placeholders + `bind`:
//
//     sqlx::query("INSERT INTO heroes (name, level) VALUES (?, ?)")
//         .bind(name)
//         .bind(level)
//         .execute(&pool).await?;
//
//     let n: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM heroes")
//         .fetch_one(&pool).await?;
//
//   NEVER paste values into SQL with format!. A name like O'Brien breaks
//   the query, and a nasty one can run its own SQL (SQL injection).
//   Bound values are sent apart from the SQL, so they're always safe.
//
//   SQLite INTEGER is i64 in Rust. TEXT is String / &str.
//
// 🎯 YOUR TASK
//
//   1. `add_hero` builds SQL with format!. Use placeholders and bind.
//   2. Write `count`: the number of heroes.

use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

/// Given: an in-memory database pool.
pub async fn open() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect("sqlite::memory:")
        .await
        .unwrap()
}

pub async fn setup(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("CREATE TABLE heroes (id INTEGER PRIMARY KEY, name TEXT NOT NULL, level INTEGER NOT NULL)")
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn add_hero(pool: &SqlitePool, name: &str, level: i64) -> Result<(), sqlx::Error> {
    let sql = format!("INSERT INTO heroes (name, level) VALUES ('{name}', {level})");
    sqlx::query(&sql).execute(pool).await?;
    Ok(())
}

pub async fn count(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn the_archive() {
        let pool = open().await;
        setup(&pool).await.unwrap();
        assert_eq!(count(&pool).await.unwrap(), 0);
        for (name, level) in [("Ferris", 12), ("O'Brien", 3), ("Bobby'); DROP TABLE heroes; --", 1)] {
            add_hero(&pool, name, level).await.unwrap();
        }
        assert_eq!(count(&pool).await.unwrap(), 3);
        let names: Vec<String> = sqlx::query_scalar("SELECT name FROM heroes ORDER BY id").fetch_all(&pool).await.unwrap();
        assert_eq!(names, ["Ferris", "O'Brien", "Bobby'); DROP TABLE heroes; --"]);
    }
}
