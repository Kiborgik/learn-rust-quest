// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 14-2 · The Automatic Conversion                  175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `#[from]` on a field generates `impl From<Inner> for YourError`, so `?`
//   converts the inner error for you. It also records it as the `source`
//   (the "caused by" error):
//
//     #[derive(Debug, Error)]
//     pub enum LoadError {
//         #[error("file is empty")]
//         Empty,
//         #[error("bad number: {0}")]
//         BadNumber(#[from] std::num::ParseIntError),
//     }
//
//     fn load(text: &str) -> Result<u32, LoadError> {
//         Ok(text.parse()?)            // ParseIntError → LoadError::BadNumber
//     }
//
//   In lesson 7 that was a 5-line `impl From` block. Now it's one word.
//
//   `source()` comes from the `std::error::Error` trait. It's how error
//   chains work: each error can point to the error that caused it.
//
//     use std::error::Error;
//     err.source()                     // Option<&dyn Error>
//
// 🎯 YOUR TASK
//
//   `load_level` uses `?` on a parse error, but `SaveError` can't be built
//   from one yet. Fix the enum, not the function.

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("save file is empty")]
    Empty,
    #[error("bad level: {0}")]
    BadLevel(ParseIntError),
}

/// Reads a level number from a save file's text.
pub fn load_level(text: &str) -> Result<u32, SaveError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(SaveError::Empty);
    }
    let level = text.parse::<u32>()?;
    Ok(level)
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn loading() {
        assert_eq!(load_level(" 12 ").unwrap(), 12);
        assert!(matches!(load_level("  "), Err(SaveError::Empty)));
    }

    #[test]
    fn bad_numbers() {
        let err = load_level("twelve").unwrap_err();
        assert!(matches!(err, SaveError::BadLevel(_)));
        assert_eq!(err.to_string(), "bad level: invalid digit found in string");
    }

    #[test]
    fn source_is_recorded() {
        let err = load_level("x").unwrap_err();
        let cause = err.source().expect("BadLevel should have a source");
        assert_eq!(cause.to_string(), "invalid digit found in string");
        assert!(SaveError::Empty.source().is_none());
    }
}
