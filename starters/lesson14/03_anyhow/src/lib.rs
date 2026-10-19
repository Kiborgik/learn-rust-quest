// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 14-3 · Any Error Will Do                         175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Applications usually don't care WHICH error happened, only that it
//   gets reported well. `anyhow::Error` holds any error type:
//
//     use anyhow::{Result, anyhow};    // Result<T> = Result<T, anyhow::Error>
//
//     fn total(text: &str) -> Result<u32> {
//         let n: u32 = text.parse()?;          // ParseIntError: converts
//         let m: u32 = "7".parse()?;           // any std error converts
//         Ok(n + m)
//     }
//
//     fn find(name: &str) -> Result<u32> {
//         Err(anyhow!("no hero named {name}"))  // an ad-hoc error message
//     }
//
//     fn main() -> anyhow::Result<()> { ... }  // pretty error report on exit
//
//   It's `Box<dyn Error>` from lesson 7, but nicer: messages, context
//   (next quest), backtraces.
//
//   Rule of thumb:  library → thiserror,  application → anyhow.
//
// 🎯 YOUR TASK
//
//   1. `total_gold` can't use `?` with a `String` error. Switch it to
//      `anyhow::Result`.
//   2. Implement `hero_level` with `anyhow!` for the missing-hero case.

use anyhow::anyhow;

/// Sums one number per line.
pub fn total_gold(lines: &str) -> Result<u32, String> {
    let mut total = 0;
    for line in lines.lines() {
        let gold: u32 = line.trim().parse()?;
        total += gold;
    }
    Ok(total)
}

/// Level of a hero from a "name:level" roster, one hero per line.
/// Unknown hero: an error "no hero named {name}".
/// Tip: `line.split_once(':')`, then parse the level with `?`.
pub fn hero_level(roster: &str, name: &str) -> anyhow::Result<u32> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold() {
        assert_eq!(total_gold("1\n2\n 3").unwrap(), 6);
        let err = total_gold("1\nlots").unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string");
    }

    #[test]
    fn levels() {
        let roster = "Ferris:12\nCorro:7";
        assert_eq!(hero_level(roster, "Corro").unwrap(), 7);
        assert_eq!(
            hero_level(roster, "Bob").unwrap_err().to_string(),
            "no hero named Bob"
        );
        assert!(hero_level("Ferris:twelve", "Ferris").is_err());
    }
}
