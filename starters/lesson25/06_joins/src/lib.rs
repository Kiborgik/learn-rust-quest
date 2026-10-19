// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 25-6 · The Census                                175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   SQL NULL becomes `Option` in Rust. A column that can be NULL (or a
//   LEFT JOIN that may find nothing) must be read into an Option:
//
//     // heroes without a guild get NULL for g.name
//     sqlx::query_as::<_, (String, Option<String>)>(
//         "SELECT h.name, g.name FROM heroes h LEFT JOIN guilds g ON g.id = h.guild_id",
//     )
//
//   Read a NULL into a plain String and you lose the difference between
//   "no guild" and a guild called "". (SQLite is loose and may hand you
//   an empty value; Postgres fails with "unexpected null".)
//
//   Tuples work with query_as too, handy for small ad-hoc queries. For
//   computed columns in a struct, name them with AS so FromRow can find
//   them:
//
//     #[derive(FromRow)]
//     struct Stat { class: String, heroes: i64 }
//
//     "SELECT class, COUNT(*) AS heroes FROM heroes GROUP BY class"
//
// 🎯 YOUR TASK
//
//   1. `roster` hides that one hero has no guild. Return an Option.
//   2. Write `report`: for each guild that has members, its name, number
//      of members and total level, highest total first.

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, PartialEq, FromRow)]
pub struct GuildReport {
    pub guild: String,
    pub members: i64,
    pub total_level: i64,
}

/// Given: two guilds and four heroes, one of them without a guild.
pub async fn seeded() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::raw_sql(
        "CREATE TABLE guilds (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
         CREATE TABLE heroes (id INTEGER PRIMARY KEY, name TEXT NOT NULL, level INTEGER NOT NULL,
                              guild_id INTEGER REFERENCES guilds(id));
         INSERT INTO guilds (id, name) VALUES (1, 'Iron Crabs'), (2, 'Moon Owls'), (3, 'Empty Hall');
         INSERT INTO heroes (name, level, guild_id) VALUES
             ('Ferris', 12, 1), ('Corro', 7, 1), ('Lumi', 30, 2), ('Wanderer', 5, NULL);",
    )
    .execute(&pool)
    .await
    .unwrap();
    pool
}

/// Every hero (by id) with their guild's name.
pub async fn roster(pool: &SqlitePool) -> Result<Vec<(String, String)>, sqlx::Error> {
    sqlx::query_as("SELECT h.name, g.name FROM heroes h LEFT JOIN guilds g ON g.id = h.guild_id ORDER BY h.id")
        .fetch_all(pool)
        .await
}

pub async fn report(pool: &SqlitePool) -> Result<Vec<GuildReport>, sqlx::Error> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn the_roster() {
        let pool = seeded().await;
        let r = roster(&pool).await.unwrap();
        assert_eq!(r.len(), 4);
        assert_eq!(format!("{:?}", r[0]), r#"("Ferris", Some("Iron Crabs"))"#);
        assert_eq!(format!("{:?}", r[3]), r#"("Wanderer", None)"#);
    }

    #[tokio::test]
    async fn the_report() {
        let pool = seeded().await;
        let r = report(&pool).await.unwrap();
        let row = |guild: &str, members, total_level| GuildReport { guild: guild.into(), members, total_level };
        assert_eq!(r, [row("Moon Owls", 1, 30), row("Iron Crabs", 2, 19)]);
    }
}
