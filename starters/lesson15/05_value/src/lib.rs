// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 15-5 · Shapeless Data                            175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Sometimes you don't have (or want) a struct for the data. Then use
//   `serde_json::Value`, a JSON tree you can poke at:
//
//     use serde_json::{json, Value};
//
//     let v: Value = serde_json::from_str(text)?;
//     v["party"][0]["name"]          // indexing never panics: missing → Null
//     v["party"][0]["name"].as_str() // Option<&str>
//     v["gold"].as_u64()             // Option<u64>
//     v["party"].as_array()          // Option<&Vec<Value>>
//
//     let event = json!({            // build JSON with a macro
//         "event": "level_up",
//         "level": level,            // any serializable Rust value works
//         "tags": ["fast", "brave"],
//     });
//     event.to_string()              // {"event":"level_up",...}
//
//   Typed structs are safer and faster. Value is for exploring, for
//   pass-through data, and for small one-off scripts.
//
// 🎯 YOUR TASK
//
//   Implement the three functions. The JSON shape is in the tests.

use serde_json::{Value, json};

/// Name of the first hero in "party", if any.
pub fn leader(save: &Value) -> Option<&str> {
    todo!()
}

/// Sum of every hero's "gold". Heroes without gold count as 0.
pub fn total_gold(save: &Value) -> u64 {
    todo!()
}

/// {"event": kind, "xp": xp}
pub fn event(kind: &str, xp: u32) -> Value {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn save() -> Value {
        serde_json::from_str(
            r#"{ "party": [
                    { "name": "Ferris", "gold": 30 },
                    { "name": "Corro" },
                    { "name": "Lumi", "gold": 12 }
                ] }"#,
        )
        .unwrap()
    }

    #[test]
    fn leading() {
        assert_eq!(leader(&save()), Some("Ferris"));
        assert_eq!(leader(&json!({ "party": [] })), None);
        assert_eq!(leader(&json!({})), None);
    }

    #[test]
    fn gold() {
        assert_eq!(total_gold(&save()), 42);
        assert_eq!(total_gold(&json!({})), 0);
    }

    #[test]
    fn events() {
        assert_eq!(event("level_up", 50).to_string(), r#"{"event":"level_up","xp":50}"#);
    }
}
