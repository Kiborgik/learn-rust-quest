// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Archive Keeper                            400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Archive Keeper guards every hero's record and every item they
//   own. Build `Store`, the only door into the archive. Callers never
//   see SQL or sqlx errors they can't act on.
//
//   ⚔️  HEAD 1: `Store::open()`: an in-memory pool (one connection!) with
//        the SCHEMA below applied.
//
//   ⚔️  HEAD 2: `add_hero(name, class)` → the new id. A name that's taken
//        is `StoreError::Duplicate(name)`. `hero(id)` → the Hero, or
//        `StoreError::NotFound(id)`.
//        (sqlx: `Err(sqlx::Error::Database(e)) if e.is_unique_violation()`)
//
//   ⚔️  HEAD 3: `give_item(hero_id, item)`. An unknown hero is
//        `NotFound(hero_id)`. The schema has a foreign key, and sqlx turns
//        SQLite foreign keys on, so look for `e.is_foreign_key_violation()`.
//
//   ⚔️  HEAD 4: `add_party(&[(name, class)])` → the new ids. All or
//        nothing: if one name is taken, NONE of the party is added.
//
//   ⚔️  HEAD 5: `leaderboard()` → (name, item count) for EVERY hero, most
//        items first, ties by name A→Z.

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{FromRow, SqlitePool};

pub const SCHEMA: &str = "
    CREATE TABLE heroes (id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE, class TEXT NOT NULL);
    CREATE TABLE items (id INTEGER PRIMARY KEY, hero_id INTEGER NOT NULL REFERENCES heroes(id),
                        name TEXT NOT NULL);
";

#[derive(Debug, PartialEq, FromRow)]
pub struct Hero {
    pub id: i64,
    pub name: String,
    pub class: String,
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("hero {0} already exists")]
    Duplicate(String),
    #[error("hero {0} not found")]
    NotFound(i64),
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

pub struct Store {
    // your fields
}

impl Store {
    pub async fn open() -> Result<Store, StoreError> {
        todo!()
    }

    pub async fn add_hero(&self, name: &str, class: &str) -> Result<i64, StoreError> {
        todo!()
    }

    pub async fn hero(&self, id: i64) -> Result<Hero, StoreError> {
        todo!()
    }

    pub async fn give_item(&self, hero_id: i64, item: &str) -> Result<(), StoreError> {
        todo!()
    }

    pub async fn add_party(&self, party: &[(&str, &str)]) -> Result<Vec<i64>, StoreError> {
        todo!()
    }

    pub async fn leaderboard(&self) -> Result<Vec<(String, i64)>, StoreError> {
        todo!()
    }
}

// ─── The Keeper's tests: don't change these! ─────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn head_1_and_2_heroes() {
        let s = Store::open().await.unwrap();
        let id = s.add_hero("Ferris", "crab").await.unwrap();
        assert_eq!(s.hero(id).await.unwrap(), Hero { id, name: "Ferris".into(), class: "crab".into() });
        let err = s.add_hero("Ferris", "impostor").await.unwrap_err();
        assert!(matches!(&err, StoreError::Duplicate(n) if n == "Ferris"), "{err:?}");
        assert!(matches!(s.hero(99).await, Err(StoreError::NotFound(99))));
    }

    #[tokio::test]
    async fn head_3_items() {
        let s = Store::open().await.unwrap();
        let id = s.add_hero("Ferris", "crab").await.unwrap();
        s.give_item(id, "tiny hat").await.unwrap();
        let err = s.give_item(42, "cursed ring").await.unwrap_err();
        assert!(matches!(err, StoreError::NotFound(42)), "{err:?}");
    }

    #[tokio::test]
    async fn head_4_all_or_nothing() {
        let s = Store::open().await.unwrap();
        let ids = s.add_party(&[("Lumi", "mage"), ("Corro", "rogue")]).await.unwrap();
        assert_eq!(ids.len(), 2);
        assert_eq!(s.hero(ids[1]).await.unwrap().name, "Corro");
        let err = s.add_party(&[("Ada", "bard"), ("Lumi", "copycat")]).await.unwrap_err();
        assert!(matches!(&err, StoreError::Duplicate(n) if n == "Lumi"), "{err:?}");
        assert_eq!(s.leaderboard().await.unwrap().len(), 2, "Ada must not be added");
    }

    #[tokio::test]
    async fn head_5_leaderboard() {
        let s = Store::open().await.unwrap();
        let ids = s.add_party(&[("Zed", "monk"), ("Ferris", "crab"), ("Lumi", "mage"), ("Ada", "bard")]).await.unwrap();
        for item in ["hat", "claw polish"] {
            s.give_item(ids[1], item).await.unwrap();
        }
        s.give_item(ids[0], "staff").await.unwrap();
        s.give_item(ids[2], "orb").await.unwrap();
        let board = s.leaderboard().await.unwrap();
        let expected = [("Ferris", 2), ("Lumi", 1), ("Zed", 1), ("Ada", 0)].map(|(n, c)| (n.to_string(), c));
        assert_eq!(board, expected);
    }
}
