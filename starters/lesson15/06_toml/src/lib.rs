// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 15-6 · The Config Tome                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The same derives work with every serde format. Swap the crate, keep
//   the types. TOML is the usual choice for config files (Cargo.toml is
//   TOML):
//
//     title = "Rust Quest"
//     difficulty = "hard"
//
//     [keys]             ← a table becomes a nested struct
//     hint = "h"
//
//     [[party]]          ← an array of tables becomes a Vec
//     name = "Ferris"
//
//     let cfg: Config = toml::from_str(text)?;
//     let text = toml::to_string(&cfg)?;
//
//   A unit-only enum maps to a string. Use rename_all so "hard" matches
//   a `Hard` variant.
//
// 🎯 YOUR TASK
//
//   `load` fails on the sample config. Fix the types (attributes and one
//   missing field), not the config text or `load`.

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Deserialize)]
pub struct Keys {
    pub hint: char,
    pub quit: char,
}

#[derive(Debug, Deserialize)]
pub struct Member {
    pub name: String,
    pub level: u32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    pub difficulty: Difficulty,
    pub keys: Keys,
}

pub fn load(text: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(text)
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
title = "Rust Quest"
difficulty = "hard"

[keys]
hint = "h"
quit = "q"

[[party]]
name = "Ferris"
level = 12

[[party]]
name = "Corro"
level = 7
"#;

    #[test]
    fn loads_sample() {
        let cfg = load(SAMPLE).unwrap();
        assert_eq!(cfg.title, "Rust Quest");
        assert_eq!(cfg.difficulty, Difficulty::Hard);
        assert_eq!(cfg.keys.hint, 'h');
        assert_eq!(cfg.party.len(), 2);
        assert_eq!(cfg.party[1].name, "Corro");
        assert_eq!(cfg.party[0].level, 12);
    }

    #[test]
    fn missing_title_is_an_error() {
        let err = load("difficulty = \"easy\"\n[keys]\nhint=\"h\"\nquit=\"q\"\n").unwrap_err();
        assert!(err.to_string().contains("missing field `title`"), "{err}");
    }
}
