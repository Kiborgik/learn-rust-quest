// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 7-3 · The ? Rune                                 125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Matching every Result by hand gets tedious:
//
//     let a = match "3".parse::<u32>() {
//         Ok(v) => v,
//         Err(e) => return Err(e),     // pass the error up to the caller
//     };
//
//   The `?` operator does exactly that, in one character:
//
//     fn add_strs(a: &str, b: &str) -> Result<u32, std::num::ParseIntError> {
//         let x = a.parse::<u32>()?;   // Ok → unwrap it; Err → return early
//         let y = b.parse::<u32>()?;
//         Ok(x + y)
//     }
//
//   `?` also works on `Option` inside a function that returns `Option`:
//
//     fn first_char_upper(s: &str) -> Option<char> {
//         let c = s.chars().next()?;   // None → return None
//         Some(c.to_ascii_uppercase())
//     }
//
//   ⚠️ `?` only works in a function whose return type can carry the
//   error: Result for Result, Option for Option.
//
//   `ParseIntError` is the error type of parsing integers. Its full path
//   is `std::num::ParseIntError`, so we `use` it below.
//
// 🎯 YOUR TASK
//
//   1. `total_damage("12,30")` → Ok(42): parse two numbers separated
//      by a comma. Rewrite it with `?` (or just make it compile).
//   2. `stat_sum("str=5 dex=7")`: the stats are "key=value" pairs
//      separated by spaces. Return the sum of all values, or the first
//      parse error. Use `?` inside a loop!
//   3. `last_word_len`: the length of the last word, or None for empty
//      text. Use `?` on an Option.

use std::num::ParseIntError;

fn total_damage(input: &str) -> Result<u32, ParseIntError> {
    let (a, b) = input.split_once(',').unwrap();
    let a: u32 = a.trim().parse();
    let b: u32 = b.trim().parse();
    Ok(a + b)
}

fn stat_sum(stats: &str) -> Result<u32, ParseIntError> {
    // Tip: for pair in stats.split_whitespace() { ... }
    //      `pair.split_once('=')` gives Some(("str", "5"))
    todo!()
}

fn last_word_len(text: &str) -> Option<usize> {
    // Tip: `text.split_whitespace().last()` returns Option<&str>
    todo!()
}

fn main() {
    println!("{:?}", total_damage("12,30"));
    println!("{:?}", stat_sum("str=5 dex=7"));
    println!("{:?}", last_word_len("go west"));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage() {
        assert_eq!(total_damage("12,30"), Ok(42));
        assert_eq!(total_damage(" 1 , 2 "), Ok(3));
        assert!(total_damage("1,x").is_err());
    }

    #[test]
    fn stats() {
        assert_eq!(stat_sum("str=5 dex=7"), Ok(12));
        assert_eq!(stat_sum("str=5 dex=7 int=10"), Ok(22));
        assert_eq!(stat_sum(""), Ok(0));
        let err = stat_sum("str=5 dex=lots").unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string");
    }

    #[test]
    fn last_words() {
        assert_eq!(last_word_len("go west"), Some(4));
        assert_eq!(last_word_len("   "), None);
    }
}
