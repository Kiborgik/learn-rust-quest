//! Store: quests in SQLite. (Given: you wrote this in 26-2.)

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

const SELECT: &str = "SELECT id, title, reward, done FROM quests";

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
        let new = new.validate()?;
        let id = sqlx::query("INSERT INTO quests (title, reward) VALUES (?, ?)")
            .bind(&new.title)
            .bind(new.reward)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();
        Ok(Quest { id, title: new.title, reward: new.reward, done: false })
    }

    pub async fn get(&self, id: i64) -> Result<Quest, AppError> {
        let row = sqlx::query_as::<_, QuestRow>(&format!("{SELECT} WHERE id = ?"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.ok_or(BoardError::NotFound(id))?.into())
    }

    pub async fn list(&self, done: Option<bool>) -> Result<Vec<Quest>, AppError> {
        let rows = sqlx::query_as::<_, QuestRow>(&format!("{SELECT} WHERE ? IS NULL OR done = ? ORDER BY id"))
            .bind(done)
            .bind(done)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(Quest::from).collect())
    }

    pub async fn finish(&self, id: i64) -> Result<Quest, AppError> {
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query_as::<_, QuestRow>(&format!("{SELECT} WHERE id = ?"))
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?;
        let mut quest: Quest = row.ok_or(BoardError::NotFound(id))?.into();
        quest.finish()?;
        sqlx::query("UPDATE quests SET done = TRUE WHERE id = ?").bind(id).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(quest)
    }
}
