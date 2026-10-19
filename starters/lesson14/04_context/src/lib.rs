// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 14-4 · Breadcrumbs                               175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   "invalid digit found in string" ... in WHICH file? WHICH field?
//   `.context()` wraps an error with a message about what you were doing:
//
//     use anyhow::{Context, Result};
//
//     let n: u32 = text.parse().context("bad level in save")?;
//     let line = lines.next().context("save is empty")?;  // works on Option too
//     let cfg = load(path).with_context(|| format!("loading {path}"))?;
//
//   `with_context` takes a closure, so the message is only built when
//   there's actually an error.
//
//   Printing shows the chain, outermost first:
//
//     format!("{err}")      // "loading hero.sav"           (just the top)
//     format!("{err:#}")    // "loading hero.sav: bad level in save: invalid digit found in string"
//     format!("{err:?}")    // multi-line report with "Caused by:" (used by main)
//
// 🎯 YOUR TASK
//
//   1. `read_level` gives a bare parse error. Add the context
//      "bad level in save".
//   2. Implement `load_hero`: call `read_level` and add the context
//      "loading {name}" with `with_context`.

use anyhow::{Context, Result};

/// Finds the "level=N" line in a save and returns N.
pub fn read_level(save: &str) -> Result<u32> {
    let line = save
        .lines()
        .find(|l| l.starts_with("level="))
        .context("no level line")?;
    let level: u32 = line["level=".len()..].trim().parse()?;
    Ok(level)
}

pub fn load_hero(name: &str, save: &str) -> Result<u32> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_save() {
        assert_eq!(read_level("name=Ferris\nlevel=12").unwrap(), 12);
        assert_eq!(load_hero("Ferris", "level=3").unwrap(), 3);
    }

    #[test]
    fn missing_line() {
        assert_eq!(format!("{:#}", read_level("name=Ferris").unwrap_err()), "no level line");
    }

    #[test]
    fn bad_level_has_context() {
        let err = read_level("level=abc").unwrap_err();
        assert_eq!(format!("{err}"), "bad level in save");
        assert_eq!(format!("{err:#}"), "bad level in save: invalid digit found in string");
    }

    #[test]
    fn hero_context() {
        let err = load_hero("Ferris", "level=abc").unwrap_err();
        assert_eq!(
            format!("{err:#}"),
            "loading Ferris: bad level in save: invalid digit found in string"
        );
    }
}
