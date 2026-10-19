// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 25-2 · Rows into Structs                         175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `#[derive(FromRow)]` maps columns to fields by name. `query_as` then
//   gives you typed rows:
//
//     #[derive(FromRow)]
//     struct Hero { id: i64, name: String, level: i64 }
//
//     let heroes = sqlx::query_as::<_, Hero>("SELECT id, name, level FROM heroes")
//         .fetch_all(&pool).await?;           // Vec<Hero>
//
//   Pick the fetch that matches how many rows you expect:
//
//     .fetch_all(&pool)       → Vec<T>        (0 or more)
//     .fetch_one(&pool)       → T             (no row = Err(RowNotFound))
//     .fetch_optional(&pool)  → Option<T>     (no row = Ok(None))
//
//   (sqlx also has `query!` / `query_as!` macros that check your SQL
//   against a real database at compile time. They need a DATABASE_URL
//   while building, so this lesson uses the plain functions.)
//
// 🎯 YOUR TASK
//
//   1. `find` errors when the id doesn't exist. It should return None.
//   2. Write `strong`: heroes with level >= min_level, highest level
//      first.

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, PartialEq, FromRow)]
pub struct Hero {
    pub id: i64,
    pub name: String,
    pub level: i64,
}

/// Given: an in-memory database with three heroes.
pub async fn seeded() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::raw_sql(
        "CREATE TABLE heroes (id INTEGER PRIMARY KEY, name TEXT NOT NULL, level INTEGER NOT NULL);
         INSERT INTO heroes (name, level) VALUES ('Ferris', 12), ('Corro', 3), ('Lumi', 30);",
    )
    .execute(&pool)
    .await
    .unwrap();
    pool
}

pub async fn find(pool: &SqlitePool, id: i64) -> Result<Option<Hero>, sqlx::Error> {
    let hero = sqlx::query_as::<_, Hero>("SELECT id, name, level FROM heroes WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(Some(hero))
}

pub async fn strong(pool: &SqlitePool, min_level: i64) -> Result<Vec<Hero>, sqlx::Error> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn finding() {
        let pool = seeded().await;
        assert_eq!(find(&pool, 1).await.unwrap(), Some(Hero { id: 1, name: "Ferris".into(), level: 12 }));
        assert_eq!(find(&pool, 99).await.unwrap(), None);
    }

    #[tokio::test]
    async fn the_strong_ones() {
        let pool = seeded().await;
        let names: Vec<String> = strong(&pool, 10).await.unwrap().into_iter().map(|h| h.name).collect();
        assert_eq!(names, ["Lumi", "Ferris"]);
        assert!(strong(&pool, 100).await.unwrap().is_empty());
    }
}
