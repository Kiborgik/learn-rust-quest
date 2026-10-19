// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 7-6 · The Conversion Circle                      125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `?` does one more trick: before returning an error it calls
//   `From::from(err)`, converting it into YOUR function's error type,
//   as long as a `From` impl exists:
//
//     #[derive(Debug)]
//     enum LoadError {
//         BadNumber(ParseIntError),
//         Empty,
//     }
//
//     impl From<ParseIntError> for LoadError {
//         fn from(e: ParseIntError) -> Self {
//             LoadError::BadNumber(e)
//         }
//     }
//
//     fn load(s: &str) -> Result<u32, LoadError> {
//         if s.is_empty() { return Err(LoadError::Empty); }
//         let n: u32 = s.parse()?;     // ParseIntError → LoadError automatically
//         Ok(n)
//     }
//
//   THE LAZY OPTION: `Box<dyn std::error::Error>` means "any error type".
//   Every error converts into it, so `?` just works with anything.
//   Great for `main`, scripts and prototypes:
//
//     fn main() -> Result<(), Box<dyn std::error::Error>> {
//         let n: u32 = "42".parse()?;
//         Ok(())                        // main returning Err prints it and exits with code 1
//     }
//
//   Libraries use precise enums; applications often use `Box<dyn Error>`.
//
// 🎯 YOUR TASK
//
//   1. `read_save` uses `?` on a ParseIntError, but returns a SaveError.
//      Add the missing `From` impl.
//   2. `total_gold` should accept ANY error: change its return type to
//      `Result<u32, Box<dyn Error>>`. It uses `?` on two different error types!

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum SaveError {
    Corrupt(ParseIntError),
    MissingHeader,
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SaveError::Corrupt(e) => write!(f, "corrupt save: {e}"),
            SaveError::MissingHeader => write!(f, "missing header"),
        }
    }
}

impl Error for SaveError {}

// TODO: impl From<ParseIntError> for SaveError

/// A save looks like "SAVE:120" → Ok(120)
fn read_save(data: &str) -> Result<u32, SaveError> {
    let Some(number) = data.strip_prefix("SAVE:") else {
        return Err(SaveError::MissingHeader);
    };
    let level = number.parse::<u32>()?;
    Ok(level)
}

/// Read a save, then add a bonus given as text.
fn total_gold(save: &str, bonus: &str) -> Result<u32, SaveError> {
    let base = read_save(save)?;
    let extra: u32 = bonus.parse()?;
    Ok(base + extra)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", total_gold("SAVE:100", "25")?);
    Ok(())
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves() {
        assert_eq!(read_save("SAVE:120").unwrap(), 120);
        assert_eq!(read_save("120").unwrap_err().to_string(), "missing header");
        assert_eq!(
            read_save("SAVE:abc").unwrap_err().to_string(),
            "corrupt save: invalid digit found in string"
        );
    }

    #[test]
    fn gold() {
        assert_eq!(total_gold("SAVE:100", "25").unwrap(), 125);
        assert_eq!(total_gold("nope", "25").unwrap_err().to_string(), "missing header");
        assert_eq!(
            total_gold("SAVE:100", "").unwrap_err().to_string(),
            "cannot parse integer from empty string"
        );
    }
}
