// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 14-1 · Errors Without the Boilerplate            175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   In lesson 7 you wrote `impl Display` and `impl Error` by hand for every
//   error type. The `thiserror` crate writes them for you:
//
//     use thiserror::Error;
//
//     #[derive(Debug, Error)]
//     pub enum DoorError {
//         #[error("the door is locked")]
//         Locked,
//         #[error("too heavy ({weight} kg)")]          // named fields by name
//         TooHeavy { weight: u32 },
//         #[error("no door called {0}")]               // tuple fields by index
//         Missing(String),
//     }
//
//   `#[error("...")]` is the Display message. The format string can use
//   the variant's fields, just like `format!`.
//
//   The result is a normal error type: `.to_string()` works, `?` works,
//   and it fits in `Box<dyn std::error::Error>`. Nothing about it is
//   special at runtime; thiserror only saves typing.
//
//   Libraries use thiserror: callers get a precise enum they can `match`.
//
// 🎯 YOUR TASK
//
//   Give every variant of `ShopError` its message:
//
//     NotEnoughGold { needed, have }   "need {needed} gold, have {have}"
//     UnknownItem(name)                "unknown item: {name}"

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ShopError {
    #[error("the shop is closed")]
    Closed,
    NotEnoughGold { needed: u32, have: u32 },
    UnknownItem(String),
}

/// Buys an item and returns the gold left.
pub fn buy(item: &str, gold: u32, open: bool) -> Result<u32, ShopError> {
    if !open {
        return Err(ShopError::Closed);
    }
    let price = match item {
        "potion" => 5,
        "sword" => 40,
        other => return Err(ShopError::UnknownItem(other.to_string())),
    };
    if gold < price {
        return Err(ShopError::NotEnoughGold { needed: price, have: gold });
    }
    Ok(gold - price)
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buying() {
        assert_eq!(buy("potion", 10, true), Ok(5));
        assert_eq!(buy("potion", 10, false), Err(ShopError::Closed));
    }

    #[test]
    fn messages() {
        assert_eq!(ShopError::Closed.to_string(), "the shop is closed");
        assert_eq!(buy("sword", 12, true).unwrap_err().to_string(), "need 40 gold, have 12");
        assert_eq!(buy("dragon", 99, true).unwrap_err().to_string(), "unknown item: dragon");
    }

    #[test]
    fn is_a_real_error() {
        let boxed: Box<dyn std::error::Error> = Box::new(ShopError::Closed);
        assert_eq!(boxed.to_string(), "the shop is closed");
    }
}
