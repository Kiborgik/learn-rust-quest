// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 16-5 · Named Runes                               175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Named groups make patterns readable: `(?<name>...)`
//
//     let re = Regex::new(r"(?<hero>\w+) hits (?<foe>\w+) for (?<dmg>\d+)").unwrap();
//     let c = re.captures("Ferris hits Slime for 12").unwrap();
//     &c["hero"]     // "Ferris"
//     &c["dmg"]      // "12"
//     c.name("foe").map(|m| m.as_str())   // Some("Slime"), no panic if absent
//
//   Replacing text:
//
//     re.replace_all(text, "$hero attacks")          // $name or ${name}
//     re.replace_all(text, |c: &regex::Captures| {   // or compute it
//         c["dmg"].to_uppercase()
//     })
//
//   `replace_all` returns a `Cow<str>`: it only allocates if something
//   was actually replaced. Use `.into_owned()` or `.to_string()` for a
//   String.
//
// 🎯 YOUR TASK
//
//   1. `parse_hit`: read a combat line into a `Hit` using named groups.
//   2. `fill`: replace every `{key}` in a template with the value from
//      `vars`. Unknown keys stay as they are.

use regex::{Captures, Regex};
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, PartialEq)]
pub struct Hit {
    pub hero: String,
    pub foe: String,
    pub damage: u32,
}

static HIT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?<hero>\w+) hits (?<foe>\w+) for (?<dmg>\d+)").unwrap());

static VAR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{(?<key>\w+)\}").unwrap());

pub fn parse_hit(line: &str) -> Option<Hit> {
    todo!()
}

/// fill("Hello {name}, you have {gold} gold", vars) → "Hello Ferris, you have 7 gold"
pub fn fill(template: &str, vars: &HashMap<&str, String>) -> String {
    todo!()
}


// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hits() {
        assert_eq!(
            parse_hit("[round 3] Ferris hits Slime for 12!"),
            Some(Hit { hero: "Ferris".into(), foe: "Slime".into(), damage: 12 })
        );
        assert_eq!(parse_hit("Ferris waves"), None);
    }

    #[test]
    fn templates() {
        let mut vars = HashMap::new();
        vars.insert("name", "Ferris".to_string());
        vars.insert("gold", "7".to_string());
        assert_eq!(
            fill("Hello {name}, you have {gold} gold. {name}!", &vars),
            "Hello Ferris, you have 7 gold. Ferris!"
        );
        assert_eq!(fill("Unknown {nope} stays", &vars), "Unknown {nope} stays");
    }
}
