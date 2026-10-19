// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 16-4 · Carve It Once                             175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Compiling a regex is slow (microseconds to milliseconds). Doing it on
//   every call is a classic performance bug. Build it once, globally.
//
//   A `static` must be built at compile time, so this does NOT work:
//
//     static RE: Regex = Regex::new(r"\d+").unwrap();
//     // ✘ E0015: cannot call non-const associated function in statics
//
//   `std::sync::LazyLock` runs the closure the first time it's used, then
//   keeps the result forever. It's thread-safe and in std (no crate):
//
//     use std::sync::LazyLock;
//
//     static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());
//
//     fn digits(s: &str) -> bool {
//         RE.is_match(s)               // derefs to &Regex
//     }
//
//   (Older code uses the `once_cell` or `lazy_static` crates for this.)
//
// 🎯 YOUR TASK
//
//   1. Make the `CALL` static compile using LazyLock.
//   2. Implement `count_calls` with it.

use regex::Regex;

/// A spell call: a word followed by "!", like "fireball!".
static CALL: Regex = Regex::new(r"\b[a-z]+!").unwrap();

pub fn is_call(word: &str) -> bool {
    CALL.is_match(word)
}

/// How many spell calls are in the text.
pub fn count_calls(text: &str) -> usize {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calls() {
        assert!(is_call("fireball!"));
        assert!(!is_call("fireball"));
        assert_eq!(count_calls("shield! then heal! and wait. zap!"), 3);
        assert_eq!(count_calls("quiet"), 0);
    }
}
