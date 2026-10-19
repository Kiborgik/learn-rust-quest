// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 16-3 · Rune Patterns                             175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     use regex::Regex;
//
//     let re = Regex::new(r"(\d+)d(\d+)").unwrap();   // r"..." = raw string,
//                                                     // no "\\d" escaping
//     re.is_match("roll 2d6")                          // true
//     re.find("roll 2d6").map(|m| m.as_str())          // Some("2d6")
//     for m in re.find_iter("1d4 and 3d8") { }         // every match
//
//     let caps = re.captures("roll 12d20").unwrap();   // groups:
//     &caps[0]   // "12d20"   (whole match)
//     &caps[1]   // "12"
//     &caps[2]   // "20"
//
//   Quick reference:  \d digit  \w word char  \s space  + one or more
//   * zero or more  ? optional  ^ start  $ end  [abc] one of  (..) group
//
//   Rust's regex crate guarantees linear time: no catastrophic
//   backtracking, even on hostile input. The price: no backreferences or
//   lookaround.
//
// 🎯 YOUR TASK
//
//   The dice pattern is too strict: it only works for single-digit dice.
//   Fix the PATTERN (not the code around it), then implement `all_dice`.

use regex::Regex;

/// (count, sides) of the first dice roll in the text, like "2d6".
pub fn first_dice(text: &str) -> Option<(u32, u32)> {
    let re = Regex::new(r"(\d)d(\d)").unwrap();
    let caps = re.captures(text)?;
    Some((caps[1].parse().ok()?, caps[2].parse().ok()?))
}

/// Every dice roll in the text as written, in order.  "1d4 and 3d8" → ["1d4", "3d8"]
pub fn all_dice(text: &str) -> Vec<String> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(first_dice("roll 2d6 now"), Some((2, 6)));
        assert_eq!(first_dice("then 12d20!"), Some((12, 20)));
        assert_eq!(first_dice("no dice"), None);
    }

    #[test]
    fn all() {
        assert_eq!(all_dice("1d4 and 3d8, then 10d100"), vec!["1d4", "3d8", "10d100"]);
        assert!(all_dice("nothing").is_empty());
    }
}
