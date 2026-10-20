// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 15-1 · Into the Scroll                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   serde is Rust's (de)serialization framework. You derive two traits,
//   and a FORMAT crate (serde_json, toml, ...) does the rest:
//
//     use serde::{Serialize, Deserialize};
//
//     #[derive(Serialize, Deserialize)]
//     struct Hero { name: String, level: u32 }
//
//     let json = serde_json::to_string(&hero)?;          // {"name":"Ferris","level":3}
//     let pretty = serde_json::to_string_pretty(&hero)?; // indented, multi-line
//     let back: Hero = serde_json::from_str(&json)?;
//
//   • Fields come out in declaration order.
//   • Every field's type must be serializable too: numbers, String, bool,
//     Vec, Option, HashMap, and your own derived types all are.
//   • The derive generates code at compile time: no reflection, no
//     runtime cost for figuring out the shape.
//
//   Cargo.toml needs the derive feature:
//     serde = { version = "1", features = ["derive"] }
//
//   The tests write JSON as RAW strings: `r#"..."#`. Inside, `"` and
//   `\` are plain characters, so `r#"{"a":1}"#` is the text {"a":1}.
//
// 🎯 YOUR TASK
//
//   1. Make `Hero` and `Item` serializable.
//   2. Implement `to_pretty` with `serde_json::to_string_pretty`.

use serde::Serialize;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub weight: u32,
}

#[derive(Debug)]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub alive: bool,
    pub items: Vec<Item>,
}

pub fn to_json(hero: &Hero) -> String {
    serde_json::to_string(hero).unwrap()
}

pub fn to_pretty(hero: &Hero) -> String {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn ferris() -> Hero {
        Hero {
            name: "Ferris".into(),
            level: 3,
            alive: true,
            items: vec![Item { name: "rope".into(), weight: 2 }],
        }
    }

    #[test]
    fn compact() {
        assert_eq!(
            to_json(&ferris()),
            r#"{"name":"Ferris","level":3,"alive":true,"items":[{"name":"rope","weight":2}]}"#
        );
    }

    #[test]
    fn pretty() {
        let p = to_pretty(&ferris());
        assert!(p.starts_with("{\n  \"name\": \"Ferris\",\n  \"level\": 3,"));
        assert_eq!(p.lines().count(), 11);
    }
}
