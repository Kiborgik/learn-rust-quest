// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 3-5 · Scrolls and Inscriptions                   100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Rust has two main text types:
//
//   ┌──────────┬──────────────────────────────┬─────────────────────────┐
//   │ String   │ OWNED, growable, on the heap │ like Java StringBuilder │
//   │ &str     │ a BORROWED view of some text │ read-only "string slice"│
//   └──────────┴──────────────────────────────┴─────────────────────────┘
//
//     let lit: &str = "hello";               // literals are &str
//     let owned: String = String::from("hi"); // &str → String
//     let owned2: String = "hi".to_string();  // same thing
//     let view: &str = &owned;                // String → &str (cheap!)
//
//     let mut s = String::from("Sir");
//     s.push_str(" Ferris");                  // append a &str
//     let t = s + "!";                        // String + &str → String
//                                             // (note: `s` is MOVED here)
//     let u = format!("{t} {}", 42);          // the easiest way to build text
//
//   ⭐ RULE OF THUMB: function PARAMETERS should take `&str`, not
//      `&String`. A `&str` parameter accepts BOTH literals and `&String`
//      (Rust converts `&String` → `&str` for you: "deref coercion").
//      Return a `String` when you create new text.
//
//     "a" + "b"     // ✘ ERROR: can't add two &str. One side must own.
//
// 🎯 YOUR TASK
//
//   Fix the three functions so the tests pass.

fn shout(name: &String) -> String {
    name.to_uppercase() + "!"
}

fn make_title(name: &str) -> String {
    "Sir " + name
}

fn new_scroll() -> String {
    "Blank scroll"
}

fn main() {
    println!("{}", shout("ferris"));
    println!("{}", make_title("Ferris"));
    println!("{}", new_scroll());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shouting() {
        assert_eq!(shout("ferris"), "FERRIS!");
        let owned = String::from("crab");
        assert_eq!(shout(&owned), "CRAB!");
    }

    #[test]
    fn titles() {
        assert_eq!(make_title("Ferris"), "Sir Ferris");
    }

    #[test]
    fn scrolls() {
        let mut s = new_scroll();
        s.push_str(" (signed)");
        assert_eq!(s, "Blank scroll (signed)");
    }
}
