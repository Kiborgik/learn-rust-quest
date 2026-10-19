// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 15-3 · Fine Print                                175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   serde attributes shape the format without changing your Rust code:
//
//     #[derive(Serialize, Deserialize, Default)]
//     #[serde(rename_all = "camelCase")]     // music_volume ↔ "musicVolume"
//     #[serde(default)]                      // missing fields come from Default
//     struct Settings {
//         music_volume: u8,
//         #[serde(skip_serializing_if = "Option::is_none")]
//         nickname: Option<String>,          // left out when None
//         #[serde(skip)]
//         cache: Vec<u8>,                    // never written or read
//     }
//
//   `#[serde(default)]` on the STRUCT uses `impl Default for Settings`.
//   On a single FIELD it uses that field type's default (0, false, "").
//
//   rename_all options: "camelCase", "snake_case", "kebab-case",
//   "SCREAMING_SNAKE_CASE", "lowercase", "UPPERCASE", "PascalCase".
//
// 🎯 YOUR TASK
//
//   The settings file is written by a JavaScript app: camelCase keys, and
//   it leaves out anything the player never changed. Make the tests pass
//   with attributes only. Don't touch the functions or `impl Default`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub music_volume: u8,
    pub show_hints: bool,
    pub nickname: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { music_volume: 7, show_hints: true, nickname: None }
    }
}

pub fn load(json: &str) -> Settings {
    serde_json::from_str(json).unwrap()
}

pub fn save(s: &Settings) -> String {
    serde_json::to_string(s).unwrap()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_fields_use_defaults() {
        assert_eq!(load("{}"), Settings::default());
        assert_eq!(
            load(r#"{"musicVolume": 2}"#),
            Settings { music_volume: 2, show_hints: true, nickname: None }
        );
    }

    #[test]
    fn camel_case_and_skip_none() {
        let s = Settings { music_volume: 5, show_hints: false, nickname: None };
        assert_eq!(save(&s), r#"{"musicVolume":5,"showHints":false}"#);
        let s = Settings { nickname: Some("Crabby".into()), ..Settings::default() };
        assert_eq!(save(&s), r#"{"musicVolume":7,"showHints":true,"nickname":"Crabby"}"#);
    }
}
