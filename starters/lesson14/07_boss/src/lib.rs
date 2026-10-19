// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Config Wraith                             400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Config Wraith haunts every server: a single typo in a config file
//   and the whole realm goes dark, with a useless error message. Build a
//   config loader that fails LOUDLY and PRECISELY.
//
//   A config looks like this:
//
//       # my hero
//       name = Ferris
//       level = 12
//       gold = 300        (optional, default 0)
//
//   Blank lines and lines starting with '#' are ignored. Spaces around
//   keys and values are trimmed.
//
//   ⚔️  HEAD 1: write `ConfigError` with thiserror (variants and messages):
//        Malformed(usize)          "line {0}: expected key = value"
//        UnknownKey(String)        "unknown key: {0}"
//        MissingKey(&'static str)  "missing key: {0}"
//        BadNumber { key: String, source: ParseIntError }
//                                  "bad number for {key}"
//        (lines are numbered from 1)
//
//   ⚔️  HEAD 2: `parse_config(text) -> Result<Config, ConfigError>`
//        Check every line in order, then the missing keys in the order
//        name, level.
//
//   ⚔️  HEAD 3: `load(file, text) -> anyhow::Result<Config>`
//        Calls `parse_config`, adds the context "loading {file}", then
//        checks with `ensure!` that level is 1..=99:
//        "level {level} is out of range"
//
//   ⚔️  HEAD 4: errors keep their cause: BadNumber's source is the
//        ParseIntError, and `{:#}` on a `load` error shows the whole chain.

use anyhow::{Context, ensure};
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub name: String,
    pub level: u32,
    pub gold: u32,
}

// Your ConfigError goes here ↓


pub fn parse_config(text: &str) -> Result<Config, ConfigError> {
    todo!()
}

pub fn load(file: &str, text: &str) -> anyhow::Result<Config> {
    todo!()
}

// ─── The Wraith's tests: don't change these! ─────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const GOOD: &str = "# my hero\nname = Ferris\n\n level=12 \ngold = 300\n";

    #[test]
    fn head_1_messages() {
        assert_eq!(ConfigError::Malformed(3).to_string(), "line 3: expected key = value");
        assert_eq!(ConfigError::UnknownKey("hp".into()).to_string(), "unknown key: hp");
        assert_eq!(ConfigError::MissingKey("name").to_string(), "missing key: name");
    }

    #[test]
    fn head_2_parsing() {
        assert_eq!(
            parse_config(GOOD).unwrap(),
            Config { name: "Ferris".into(), level: 12, gold: 300 }
        );
        assert_eq!(parse_config("name=A\nlevel=1").unwrap().gold, 0);
        assert!(matches!(parse_config("name=A\noops\nlevel=1"), Err(ConfigError::Malformed(2))));
        assert!(matches!(parse_config("hp=3"), Err(ConfigError::UnknownKey(k)) if k == "hp"));
        assert!(matches!(parse_config("level=3"), Err(ConfigError::MissingKey("name"))));
        assert!(matches!(parse_config("name=A"), Err(ConfigError::MissingKey("level"))));
    }

    #[test]
    fn head_3_loading() {
        assert_eq!(load("hero.cfg", GOOD).unwrap().level, 12);
        let err = load("hero.cfg", "name=A\nlevel=0").unwrap_err();
        assert_eq!(err.to_string(), "level 0 is out of range");
        let err = load("hero.cfg", "level=5").unwrap_err();
        assert_eq!(format!("{err:#}"), "loading hero.cfg: missing key: name");
    }

    #[test]
    fn head_4_chains() {
        let err = parse_config("name=A\nlevel=ten").unwrap_err();
        assert_eq!(err.to_string(), "bad number for level");
        assert_eq!(err.source().unwrap().to_string(), "invalid digit found in string");
        let err = load("hero.cfg", "name=A\nlevel=ten").unwrap_err();
        assert_eq!(
            format!("{err:#}"),
            "loading hero.cfg: bad number for level: invalid digit found in string"
        );
    }
}
