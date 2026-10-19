// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 30-4 · Borrow or Forge                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Some functions USUALLY return their input unchanged, and only
//   sometimes need to build something new. Returning `String` always
//   allocates, even when nothing changed. `Cow` ("clone on write") lets
//   you return either:
//
//     use std::borrow::Cow;
//
//     fn no_spaces(s: &str) -> Cow<'_, str> {
//         if s.contains(' ') {
//             Cow::Owned(s.replace(' ', "_"))   // had to build a new one
//         } else {
//             Cow::Borrowed(s)                  // free: just lend it back
//         }
//     }
//
//   A Cow<str> derefs to &str, so callers use it like any string. Call
//   `.into_owned()` if you need a String.
//
//   Note: a borrowed result may be a PART of the input. `s.trim()` is a
//   slice of `s`, so `Cow::Borrowed(s.trim())` is still free.
//
// 🎯 YOUR TASK
//
//   1. `clean_name` always allocates. Borrow when the trimmed name is
//      already lowercase.
//   2. Write `escape` for the guild website: & → &amp;  < → &lt;
//      > → &gt;. Borrow when there's nothing to escape.

use std::borrow::Cow;

/// Trimmed and lowercase.
pub fn clean_name(name: &str) -> Cow<'_, str> {
    Cow::Owned(name.trim().to_lowercase())
}

pub fn escape(text: &str) -> Cow<'_, str> {
    todo!()
}

fn main() {
    println!("{} {}", clean_name("  Ferris "), escape("<b>hi</b>"));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert!(matches!(clean_name("ferris"), Cow::Borrowed("ferris")));
        assert!(matches!(clean_name("  lumi \n"), Cow::Borrowed("lumi")));
        let c = clean_name("  Ferris ");
        assert!(matches!(c, Cow::Owned(_)));
        assert_eq!(c, "ferris");
    }

    #[test]
    fn escaping() {
        assert!(matches!(escape("plain text"), Cow::Borrowed("plain text")));
        let e = escape("<b>Gold & Glory</b>");
        assert!(matches!(e, Cow::Owned(_)));
        assert_eq!(e, "&lt;b&gt;Gold &amp; Glory&lt;/b&gt;");
    }
}
