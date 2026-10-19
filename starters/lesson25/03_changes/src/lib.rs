// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 25-3 · Ink and Eraser                            175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `execute` returns a result that tells you what happened:
//
//     let done = sqlx::query("UPDATE heroes SET level = ? WHERE id = ?")
//         .bind(5).bind(id)
//         .execute(&pool).await?;
//
//     done.rows_affected()      // how many rows changed (0 = no such id)
//     done.last_insert_rowid()  // SQLite: the id of the row just inserted
//
//   Or ask the database to hand back what it wrote (SQLite 3.35+,
//   Postgres):
//
//     let id: i64 = sqlx::query_scalar("INSERT INTO heroes (name) VALUES (?) RETURNING id")
//         .bind(name).fetch_one(&pool).await?;
//
//   ⚠️ An UPDATE or DELETE without WHERE hits EVERY row. The database
//   won't ask "are you sure?".
//
// 🎯 YOUR TASK
//
//   1. `create`: insert a hero at level 1 and return the new id.
//   2. `level_up` levels up the whole guild. Only hero `id` should grow.
//      Return false if there's no such hero.
//   3. `retire`: delete hero `id`; true if a row was deleted.

use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

/// Given: an in-memory database with an empty heroes table.
pub async fn open() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::query("CREATE TABLE heroes (id INTEGER PRIMARY KEY, name TEXT NOT NULL, level INTEGER NOT NULL)")
        .execute(&pool)
        .await
        .unwrap();
    pool
}

pub async fn create(pool: &SqlitePool, name: &str) -> Result<i64, sqlx::Error> {
    todo!()
}

pub async fn level_up(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let done = sqlx::query("UPDATE heroes SET level = level + 1").execute(pool).await?;
    Ok(done.rows_affected() > 0)
}

pub async fn retire(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    async fn levels(pool: &SqlitePool) -> Vec<(String, i64)> {
        sqlx::query_as("SELECT name, level FROM heroes ORDER BY id").fetch_all(pool).await.unwrap()
    }

    #[tokio::test]
    async fn creating() {
        let pool = open().await;
        assert_eq!(create(&pool, "Ferris").await.unwrap(), 1);
        assert_eq!(create(&pool, "Lumi").await.unwrap(), 2);
        assert_eq!(levels(&pool).await, [("Ferris".into(), 1), ("Lumi".into(), 1)]);
    }

    #[tokio::test]
    async fn leveling() {
        let pool = open().await;
        let ferris = create(&pool, "Ferris").await.unwrap();
        create(&pool, "Lumi").await.unwrap();
        assert!(level_up(&pool, ferris).await.unwrap());
        assert!(level_up(&pool, ferris).await.unwrap());
        assert!(!level_up(&pool, 99).await.unwrap());
        assert_eq!(levels(&pool).await, [("Ferris".into(), 3), ("Lumi".into(), 1)]);
    }

    #[tokio::test]
    async fn retiring() {
        let pool = open().await;
        let ferris = create(&pool, "Ferris").await.unwrap();
        create(&pool, "Lumi").await.unwrap();
        assert!(retire(&pool, ferris).await.unwrap());
        assert!(!retire(&pool, ferris).await.unwrap());
        assert_eq!(levels(&pool).await, [("Lumi".into(), 1)]);
    }
}
