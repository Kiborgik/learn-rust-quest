// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 14-5 · Turn Back!                                175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Two macros for "stop here with an error":
//
//     use anyhow::{bail, ensure, Result};
//
//     fn open(keys: u32) -> Result<()> {
//         if keys == 0 {
//             bail!("you have no keys");          // = return Err(anyhow!(...))
//         }
//         ensure!(keys < 10, "too many keys: {keys}");   // like assert!, but
//         Ok(())                                         // returns an error
//     }
//
//   Getting the original type back: an `anyhow::Error` still knows what's
//   inside. `downcast_ref` asks "are you a ParseIntError?":
//
//     if let Some(e) = err.downcast_ref::<std::num::ParseIntError>() {
//         // handle that case specially
//     }
//
//   That's how an application can react to one specific error while still
//   using anyhow everywhere else.
//
// 🎯 YOUR TASK
//
//   Implement both functions. The tests show the exact messages.

use anyhow::{Result, bail, ensure};
use std::num::ParseIntError;

/// The dungeon needs level 5+ ("level {level} is too low") and at least one
/// key ("you have no keys"). Check the level first.
/// Success: "Ferris enters with 2 keys"
pub fn enter_dungeon(name: &str, level: u32, keys: u32) -> Result<String> {
    todo!()
}

/// Did this error start as a ParseIntError?
pub fn is_parse_error(err: &anyhow::Error) -> bool {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entering() {
        assert_eq!(enter_dungeon("Ferris", 9, 2).unwrap(), "Ferris enters with 2 keys");
        assert_eq!(enter_dungeon("Tiny", 3, 2).unwrap_err().to_string(), "level 3 is too low");
        assert_eq!(enter_dungeon("Ferris", 9, 0).unwrap_err().to_string(), "you have no keys");
        assert_eq!(enter_dungeon("Tiny", 1, 0).unwrap_err().to_string(), "level 1 is too low");
    }

    #[test]
    fn downcasting() {
        let parse: anyhow::Error = "x".parse::<u32>().unwrap_err().into();
        let other = anyhow::anyhow!("something else");
        assert!(is_parse_error(&parse));
        assert!(!is_parse_error(&other));
    }
}
