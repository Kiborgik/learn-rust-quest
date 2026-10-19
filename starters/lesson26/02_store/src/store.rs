// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 26-2 · The Ledger Vault                          200 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The store keeps quests in SQLite and speaks in DOMAIN types. Callers
//   never see rows or SQL.
//
//   Keep sqlx out of the domain: read into a private row type, then
//   convert. If the table changes, only this file changes.
//
//     #[derive(FromRow)]
//     struct QuestRow { id: i64, title: String, reward: i64, done: bool }
//
//     impl From<QuestRow> for Quest { ... }
//
//     let quest: Quest = row.into();
//
//   Let the domain decide the rules, and the store just saves:
//
//     let new = new.validate()?;     // BoardError → AppError via #[from]
//
//   Read, check, write? Do it in a transaction so nobody can change the
//   row in between:
//
//     let mut tx = self.pool.begin().await?;
//     ... read with &mut *tx, call quest.finish()?, UPDATE with &mut *tx ...
//     tx.commit().await?;
//
//   An optional filter in one query: `WHERE ? IS NULL OR done = ?`, then
//   bind the Option twice.
//
// 🎯 YOUR TASK
//
//   Write `add`, `get`, `list` and `finish`. Errors are AppError::Board
//   for rule problems (NotFound, AlreadyDone, ...) and AppError::Db for
//   database ones. `list` is ordered by id.

use crate::domain::{BoardError, NewQuest, Quest};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{FromRow, SqlitePool};

/// Every error the app can hit below the HTTP layer.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Board(#[from] BoardError),
    #[error("database: {0}")]
    Db(#[from] sqlx::Error),
}

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS quests (
    id     INTEGER PRIMARY KEY,
    title  TEXT NOT NULL,
    reward INTEGER NOT NULL,
    done   BOOLEAN NOT NULL DEFAULT FALSE
)";

#[derive(FromRow)]
struct QuestRow {
    id: i64,
    title: String,
    reward: i64,
    done: bool,
}

impl From<QuestRow> for Quest {
    fn from(r: QuestRow) -> Quest {
        Quest { id: r.id, title: r.title, reward: r.reward, done: r.done }
    }
}

#[derive(Clone)]
pub struct Store {
    pool: SqlitePool,
}

impl Store {
    /// "sqlite::memory:" for tests, "sqlite://board.db?mode=rwc" for a file.
    pub async fn open(url: &str) -> Result<Store, AppError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .idle_timeout(None)
            .max_lifetime(None)
            .connect(url)
            .await?;
        sqlx::query(SCHEMA).execute(&pool).await?;
        Ok(Store { pool })
    }

    pub async fn add(&self, new: NewQuest) -> Result<Quest, AppError> {
        todo!()
    }

    pub async fn get(&self, id: i64) -> Result<Quest, AppError> {
        todo!()
    }

    pub async fn list(&self, done: Option<bool>) -> Result<Vec<Quest>, AppError> {
        todo!()
    }

    pub async fn finish(&self, id: i64) -> Result<Quest, AppError> {
        todo!()
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    async fn store() -> Store {
        Store::open("sqlite::memory:").await.unwrap()
    }

    fn new(title: &str, reward: i64) -> NewQuest {
        NewQuest { title: title.into(), reward }
    }

    fn titles(quests: Vec<Quest>) -> Vec<String> {
        quests.into_iter().map(|q| q.title).collect()
    }

    #[tokio::test]
    async fn adding_and_getting() {
        let s = store().await;
        let q = s.add(new("  Slay the slime ", 50)).await.unwrap();
        assert_eq!(q, Quest { id: 1, title: "Slay the slime".into(), reward: 50, done: false });
        assert_eq!(s.get(1).await.unwrap(), q);
        assert!(matches!(s.get(9).await, Err(AppError::Board(BoardError::NotFound(9)))));
    }

    #[tokio::test]
    async fn the_rules_still_apply() {
        let s = store().await;
        assert!(matches!(s.add(new("", 5)).await, Err(AppError::Board(BoardError::EmptyTitle))));
        assert!(matches!(s.add(new("x", -5)).await, Err(AppError::Board(BoardError::BadReward(-5)))));
        assert!(s.list(None).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn listing_and_finishing() {
        let s = store().await;
        for (title, reward) in [("a", 10), ("b", 20), ("c", 30)] {
            s.add(new(title, reward)).await.unwrap();
        }
        assert!(s.finish(2).await.unwrap().done);
        assert!(s.get(2).await.unwrap().done, "finish must be saved");
        assert!(matches!(s.finish(2).await, Err(AppError::Board(BoardError::AlreadyDone(2)))));
        assert!(matches!(s.finish(9).await, Err(AppError::Board(BoardError::NotFound(9)))));
        assert_eq!(titles(s.list(None).await.unwrap()), ["a", "b", "c"]);
        assert_eq!(titles(s.list(Some(true)).await.unwrap()), ["b"]);
        assert_eq!(titles(s.list(Some(false)).await.unwrap()), ["a", "c"]);
    }
}
