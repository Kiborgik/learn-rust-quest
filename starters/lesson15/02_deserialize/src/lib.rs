// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 15-2 · Reading Old Scrolls                       175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Deserializing turns text back into your types, and checks it:
//
//     #[derive(Deserialize)]
//     struct Hero { name: String, level: u32 }
//
//     let h: Hero = serde_json::from_str(r#"{"name":"Ferris","level":3}"#)?;
//
//   • A missing field is an error ("missing field `level`").
//   • A wrong type is an error ("invalid type: string, expected u32").
//   • Extra fields in the JSON are ignored by default.
//
//   When the outside world uses different names, rename the field:
//
//     #[derive(Deserialize)]
//     struct Hero {
//         #[serde(rename = "hp")]
//         health: u32,              // reads and writes "hp"
//         #[serde(rename = "type")]
//         kind: String,             // `type` is a Rust keyword
//     }
//
//   Your Rust code keeps good names; the file format keeps its own.
//
// 🎯 YOUR TASK
//
//   The old save files use "hp" and "type". Make `parse_save` read them
//   without changing the Rust field names.

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Hero {
    pub name: String,
    pub health: u32,
    pub kind: String,
}

pub fn parse_save(json: &str) -> Result<Hero, serde_json::Error> {
    serde_json::from_str(json)
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    const OLD_SAVE: &str = r#"{ "name": "Ferris", "hp": 40, "type": "crabsmith", "legacy": true }"#;

    #[test]
    fn reads_old_saves() {
        assert_eq!(
            parse_save(OLD_SAVE).unwrap(),
            Hero { name: "Ferris".into(), health: 40, kind: "crabsmith".into() }
        );
    }

    #[test]
    fn rejects_bad_saves() {
        let err = parse_save(r#"{ "name": "Ferris", "hp": "lots", "type": "x" }"#).unwrap_err();
        assert!(err.to_string().contains("invalid type"), "{err}");
        let err = parse_save(r#"{ "name": "Ferris", "type": "x" }"#).unwrap_err();
        assert!(err.to_string().contains("missing field `hp`"), "{err}");
    }
}
