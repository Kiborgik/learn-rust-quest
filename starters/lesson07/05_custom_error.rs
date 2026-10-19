// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 7-5 · Name Your Foe                              125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `String` errors are easy, but callers can't tell the failure kinds
//   apart. A custom error ENUM makes every failure explicit:
//
//     #[derive(Debug, PartialEq)]
//     enum DoorError {
//         Locked,
//         TooHeavy { weight: u32 },
//     }
//
//   To print it nicely with `{}`, implement the `Display` trait. Treat
//   this as a recipe for now; lesson 8 explains traits in depth:
//
//     use std::fmt;
//
//     impl fmt::Display for DoorError {
//         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             match self {
//                 DoorError::Locked => write!(f, "the door is locked"),
//                 DoorError::TooHeavy { weight } => write!(f, "too heavy ({weight} kg)"),
//             }
//         }
//     }
//
//   And to make it a "real" error that works with other libraries,
//   implement the standard Error trait (no methods required!):
//
//     impl std::error::Error for DoorError {}
//
//   `write!(f, ...)` is just like `format!`, but it writes into `f`.
//
// 🎯 YOUR TASK
//
//   1. Give `CastError` a `Display` impl with these messages:
//        NotEnoughMana { needed, have } → "need {needed} mana, have {have}"
//        UnknownSpell(name)             → "unknown spell: {name}"
//        Silenced                       → "you are silenced"
//   2. Implement `std::error::Error` for it.
//   3. Fix `cast`: it's missing the NotEnoughMana check.
//      Costs: "fireball" 30, "heal" 10, anything else is unknown.

use std::fmt;

#[derive(Debug, PartialEq)]
enum CastError {
    NotEnoughMana { needed: u32, have: u32 },
    UnknownSpell(String),
    Silenced,
}

// TODO: impl fmt::Display for CastError

// TODO: impl std::error::Error for CastError

fn cast(spell: &str, mana: u32, silenced: bool) -> Result<u32, CastError> {
    if silenced {
        return Err(CastError::Silenced);
    }
    let cost = match spell {
        "fireball" => 30,
        "heal" => 10,
        other => return Err(CastError::UnknownSpell(other.to_string())),
    };
    Ok(mana - cost)
}

fn main() {
    match cast("fireball", 12, false) {
        Ok(left) => println!("Cast! {left} mana left"),
        Err(e) => println!("Failed: {e}"),
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casting() {
        assert_eq!(cast("heal", 50, false), Ok(40));
        assert_eq!(cast("heal", 50, true), Err(CastError::Silenced));
        assert_eq!(
            cast("fireball", 12, false),
            Err(CastError::NotEnoughMana { needed: 30, have: 12 })
        );
        assert_eq!(
            cast("teleport", 99, false),
            Err(CastError::UnknownSpell(String::from("teleport")))
        );
    }

    #[test]
    fn messages() {
        let e = CastError::NotEnoughMana { needed: 30, have: 12 };
        assert_eq!(e.to_string(), "need 30 mana, have 12");
        assert_eq!(CastError::UnknownSpell("zap".into()).to_string(), "unknown spell: zap");
        assert_eq!(CastError::Silenced.to_string(), "you are silenced");
    }

    #[test]
    fn is_a_real_error() {
        let boxed: Box<dyn std::error::Error> = Box::new(CastError::Silenced);
        assert_eq!(boxed.to_string(), "you are silenced");
    }
}
