// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Archivist                                 400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Archivist keeps every hero's save, from every version of the
//   game. It demands a save system that writes clean JSON, reads both
//   JSON and TOML, and never breaks an old save.
//
//   The types are below WITHOUT any serde code. Add derives and
//   attributes, then write the functions.
//
//   ⚔️  HEAD 1: `save(&Save) -> String`: pretty JSON. Items are tagged
//        like `{"kind": "potion", "heals": 20}` (snake_case kind names).
//
//   ⚔️  HEAD 2: `load_json(&str) -> Result<Save, String>`: the round trip
//        of `save`. Errors become their `to_string()`.
//
//   ⚔️  HEAD 3: old saves (version 1) have no "achievements" field. They
//        must still load, with an empty list.
//
//   ⚔️  HEAD 4: `load_any(&str)`: text starting with '{' (after trimming)
//        is JSON, anything else is TOML.
//
//   ⚔️  HEAD 5: both loaders reject saves from the future: if version is
//        greater than CURRENT_VERSION, the error is
//        "save version 3 is newer than this game (2)".
//
//   Tip: write one private `check_version(Save) -> Result<Save, String>`
//   and use it in both loaders.

pub const CURRENT_VERSION: u32 = 2;

#[derive(Debug, Clone, PartialEq)]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub hp: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Weapon { name: String, damage: u32 },
    Potion { heals: u32 },
    Gold { amount: u32 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Save {
    pub version: u32,
    pub hero: Hero,
    pub inventory: Vec<Item>,
    pub achievements: Vec<String>,
}

pub fn save(s: &Save) -> String {
    todo!()
}

pub fn load_json(text: &str) -> Result<Save, String> {
    todo!()
}

pub fn load_any(text: &str) -> Result<Save, String> {
    todo!()
}

// ─── The Archivist's tests: don't change these! ──────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> Save {
        Save {
            version: 2,
            hero: Hero { name: "Ferris".into(), level: 12, hp: 80 },
            inventory: vec![
                Item::Weapon { name: "Rusty Blade".into(), damage: 7 },
                Item::Potion { heals: 20 },
                Item::Gold { amount: 300 },
            ],
            achievements: vec!["first_blood".into()],
        }
    }

    #[test]
    fn head_1_pretty_tagged_json() {
        let text = save(&game());
        assert!(text.contains("\n  \"hero\": {"), "{text}");
        assert!(text.contains("\"kind\": \"potion\""), "{text}");
        assert!(text.contains("\"kind\": \"gold\""), "{text}");
    }

    #[test]
    fn head_2_round_trip() {
        assert_eq!(load_json(&save(&game())).unwrap(), game());
        assert!(load_json("{ not json").is_err());
    }

    #[test]
    fn head_3_old_saves() {
        let v1 = r#"{"version":1,"hero":{"name":"Old","level":1,"hp":10},"inventory":[]}"#;
        let s = load_json(v1).unwrap();
        assert_eq!(s.hero.name, "Old");
        assert!(s.achievements.is_empty());
    }

    #[test]
    fn head_4_toml() {
        let text = r#"
version = 2
achievements = []

[hero]
name = "Corro"
level = 7
hp = 50

[[inventory]]
kind = "weapon"
name = "Spear"
damage = 4
"#;
        let s = load_any(text).unwrap();
        assert_eq!(s.hero.name, "Corro");
        assert_eq!(s.inventory, vec![Item::Weapon { name: "Spear".into(), damage: 4 }]);
        assert_eq!(load_any(&save(&game())).unwrap(), game());
    }

    #[test]
    fn head_5_future_saves() {
        let mut future = game();
        future.version = 3;
        let json = save(&future);
        assert_eq!(load_json(&json).unwrap_err(), "save version 3 is newer than this game (2)");
        assert_eq!(load_any(&json).unwrap_err(), "save version 3 is newer than this game (2)");
    }
}
