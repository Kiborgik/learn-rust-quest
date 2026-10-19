// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 15-4 · Many Shapes, One Scroll                   175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Enums have several JSON shapes. You pick one with attributes:
//
//     enum Spell { Fireball { power: u32 }, Shield }
//
//     // default, "externally tagged":
//     {"Fireball":{"power":40}}      "Shield"
//
//     #[serde(tag = "type")]                       // "internally tagged"
//     {"type":"Fireball","power":40}  {"type":"Shield"}
//
//     #[serde(tag = "type", content = "data")]     // "adjacently tagged"
//     {"type":"Fireball","data":{"power":40}}
//
//     #[serde(untagged)]                           // no tag, tries each variant
//     {"power":40}
//
//   Combine with rename_all for the tag values:
//
//     #[serde(tag = "type", rename_all = "snake_case")]
//     {"type":"fireball","power":40}
//
//   Internally tagged is what most web APIs use.
//
// 🎯 YOUR TASK
//
//   The game server sends spells like the tests show. Add the attributes
//   to `Spell` so both directions match.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Spell {
    Fireball { power: u32 },
    IceLance { power: u32, slow: bool },
    Shield,
}

pub fn encode(spells: &[Spell]) -> String {
    serde_json::to_string(spells).unwrap()
}

pub fn decode(json: &str) -> Vec<Spell> {
    serde_json::from_str(json).unwrap()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    const WIRE: &str = r#"[{"type":"fireball","power":40},{"type":"ice_lance","power":10,"slow":true},{"type":"shield"}]"#;

    fn spells() -> Vec<Spell> {
        vec![
            Spell::Fireball { power: 40 },
            Spell::IceLance { power: 10, slow: true },
            Spell::Shield,
        ]
    }

    #[test]
    fn encoding() {
        assert_eq!(encode(&spells()), WIRE);
    }

    #[test]
    fn decoding() {
        assert_eq!(decode(WIRE), spells());
    }
}
