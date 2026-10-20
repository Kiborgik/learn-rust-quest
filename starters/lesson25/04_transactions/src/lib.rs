// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 25-4 · All or Nothing                            175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Some changes only make sense together. Moving gold means taking it
//   from one hero AND giving it to another. If the second step fails,
//   the first must be undone. That's a TRANSACTION:
//
//     let mut tx = pool.begin().await?;
//
//     sqlx::query("...").bind(..).execute(&mut *tx).await?;   // note &mut *tx
//     sqlx::query("...").bind(..).execute(&mut *tx).await?;
//
//     tx.commit().await?;      // make it all real
//
//   If `tx` is dropped without `commit` (an early `return`, a `?`
//   error, a panic), everything in it is ROLLED BACK. So `?` inside a
//   transaction is exactly what you want.
//
//   A helper that works with BOTH a pool and a transaction takes any
//   SQLite executor (`use sqlx::SqliteExecutor;`):
//
//     async fn add(db: impl SqliteExecutor<'_>, name: &str) -> ... {
//         sqlx::query("...").bind(name).execute(db).await
//     }
//     add(&pool, "Ferris").await?;     // on its own
//     add(&mut *tx, "Lumi").await?;    // inside a transaction
//
//   The table has a CHECK rule: `gold >= 0`. A change that breaks it
//   fails with a database error.
//
// 🎯 YOUR TASK
//
//   `transfer` moves `amount` gold from one hero to another. Right now a
//   failed transfer can create gold from nothing, and a transfer from a
//   missing hero "works". Make it all or nothing, and fail with an error
//   when either hero doesn't exist.

use anyhow::{Result, bail};
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

/// Given: Ferris (id 1) has 100 gold, Lumi (id 2) has 20.
pub async fn seeded() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::raw_sql(
        "CREATE TABLE heroes (id INTEGER PRIMARY KEY, name TEXT NOT NULL,
                              gold INTEGER NOT NULL CHECK (gold >= 0));
         INSERT INTO heroes (name, gold) VALUES ('Ferris', 100), ('Lumi', 20);",
    )
    .execute(&pool)
    .await
    .unwrap();
    pool
}

pub async fn transfer(pool: &SqlitePool, from: i64, to: i64, amount: i64) -> Result<()> {
    let given = sqlx::query("UPDATE heroes SET gold = gold + ? WHERE id = ?")
        .bind(amount)
        .bind(to)
        .execute(pool)
        .await?;
    if given.rows_affected() == 0 {
        bail!("no hero {to}");
    }
    sqlx::query("UPDATE heroes SET gold = gold - ? WHERE id = ?")
        .bind(amount)
        .bind(from)
        .execute(pool)
        .await?;
    Ok(())
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    async fn gold(pool: &SqlitePool) -> Vec<i64> {
        sqlx::query_scalar("SELECT gold FROM heroes ORDER BY id").fetch_all(pool).await.unwrap()
    }

    #[tokio::test]
    async fn fair_trade() {
        let pool = seeded().await;
        transfer(&pool, 1, 2, 30).await.unwrap();
        assert_eq!(gold(&pool).await, [70, 50]);
    }

    #[tokio::test]
    async fn no_gold_from_nothing() {
        let pool = seeded().await;
        assert!(transfer(&pool, 2, 1, 500).await.is_err());
        assert_eq!(gold(&pool).await, [100, 20]);
    }

    #[tokio::test]
    async fn ghosts_cant_trade() {
        let pool = seeded().await;
        assert!(transfer(&pool, 1, 9, 10).await.is_err());
        assert!(transfer(&pool, 9, 1, 10).await.is_err());
        assert_eq!(gold(&pool).await, [100, 20]);
    }
}
