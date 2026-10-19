// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 7-4 · Shifting Shapes                            125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Option and Result have helper methods ("combinators") that transform
//   them without a full `match`. Most take a CLOSURE: a tiny inline
//   function written `|arg| expression` (lesson 9 goes deeper):
//
//     let double = |x| x * 2;          // a closure
//     double(4);                       // 8
//
//   | method                | on Some/Ok(v)       | on None/Err(e)        |
//   |-----------------------|---------------------|-----------------------|
//   | .map(|v| ..)          | Some/Ok(f(v))       | unchanged             |
//   | .map_err(|e| ..)      | unchanged           | Err(f(e))             |
//   | .and_then(|v| ..)     | f(v), which returns an Option/Result itself |
//   | .unwrap_or(d)         | v                   | d                     |
//   | .unwrap_or_default()  | v                   | 0 / "" / empty        |
//   | .ok_or(err)           | Option → Ok(v)      | None → Err(err)       |
//   | .ok()                 | Result → Some(v)    | Err → None            |
//
//     let n: Option<u32> = Some(3);
//     n.map(|x| x + 1);                        // Some(4)
//     n.ok_or("missing");                      // Ok(3)
//     "7".parse::<u32>().ok();                 // Some(7)
//     "x".parse::<u32>().map_err(|e| e.to_string());  // Err("invalid digit...")
//
// 🎯 YOUR TASK
//
//   Implement each function in ONE expression with combinators (no
//   `match`, no `if`). The tests show what's expected.

use std::collections::HashMap;

fn inventory() -> HashMap<&'static str, u32> {
    let mut m = HashMap::new();
    m.insert("arrows", 20);
    m.insert("potions", 3);
    m
}

/// How many of an item we have, doubled. None if unknown.
fn doubled(item: &str) -> Option<u32> {
    inventory().get(item).copied() * 2
}

/// Like `doubled`, but an unknown item is an error "no {item}".
fn require(item: &str) -> Result<u32, String> {
    todo!()
}

/// Parse a number of arrows; turn the error into a friendly String "bad arrow count".
fn parse_arrows(text: &str) -> Result<u32, String> {
    todo!()
}

/// Parse a number and look up whether it is a valid quiver size (10, 20 or 30).
/// `checked` returns Some(n) for valid sizes. Chain them with `and_then`.
fn quiver_size(text: &str) -> Option<u32> {
    let checked = |n: u32| if n % 10 == 0 && n <= 30 && n > 0 { Some(n) } else { None };
    todo!()
}

/// The number of an item, or 0 when unknown.
fn count_or_zero(item: &str) -> u32 {
    todo!()
}

fn main() {
    println!("{:?}", doubled("arrows"));
    println!("{:?}", require("bombs"));
    println!("{:?}", parse_arrows("x"));
    println!("{:?}", quiver_size("20"));
    println!("{}", count_or_zero("potions"));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubling() {
        assert_eq!(doubled("arrows"), Some(40));
        assert_eq!(doubled("bombs"), None);
    }

    #[test]
    fn requiring() {
        assert_eq!(require("potions"), Ok(6));
        assert_eq!(require("bombs"), Err(String::from("no bombs")));
    }

    #[test]
    fn parsing() {
        assert_eq!(parse_arrows("12"), Ok(12));
        assert_eq!(parse_arrows("x"), Err(String::from("bad arrow count")));
    }

    #[test]
    fn quivers() {
        assert_eq!(quiver_size("20"), Some(20));
        assert_eq!(quiver_size("25"), None);
        assert_eq!(quiver_size("abc"), None);
        assert_eq!(quiver_size("40"), None);
    }

    #[test]
    fn counting() {
        assert_eq!(count_or_zero("potions"), 3);
        assert_eq!(count_or_zero("bombs"), 0);
    }
}
