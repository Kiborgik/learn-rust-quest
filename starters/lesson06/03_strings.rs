// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 6-3 · The Scribe's Workshop                      125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A `String` is a Vec of bytes that are guaranteed to be valid UTF-8.
//
//     let mut s = String::new();
//     s.push_str("Ferris");        // append &str
//     s.push('!');                 // append ONE char
//     let t = format!("{s} {}", 1);
//
//     "héllo".len();               // 6  ← BYTES, not characters!
//     "héllo".chars().count();     // 5  ← characters
//
//     let s = "crab";
//     s[0];                        // ✘ ERROR: can't index a String
//     s.chars().next();            // Some('c') ✔
//     s.chars().rev();             // characters in reverse order
//
//     for word in "a b  c".split_whitespace() { }   // "a", "b", "c"
//     for part in "a,b,c".split(',') { }            // "a", "b", "c"
//
//     let mut out = String::new();     // build text char by char
//     for c in "abc".chars() { out.push(c); }
//
//     "Hi".to_uppercase();   "Hi".to_lowercase();   "  x ".trim();
//     'q'.to_ascii_uppercase();    // 'Q' (one char)
//     "dragon".contains("rag");   "dragon".starts_with("dr");
//     "a-b".replace("-", "+");    // "a+b"
//
//   Why no `s[i]`? With UTF-8 one character can be 1–4 bytes, so "the
//   i-th character" isn't a constant-time operation. Rust makes you say
//   whether you want bytes or chars.
//
// 🎯 YOUR TASK
//
//   1. Fix `initial` (first character, or '?' for an empty string).
//   2. Fix `rune_count` (count characters, not bytes).
//   3. Implement `reverse`.
//   4. Implement `acronym`: "portable network graphics" → "PNG".

fn initial(name: &str) -> char {
    name[0]
}

fn rune_count(s: &str) -> usize {
    s.len()
}

fn reverse(s: &str) -> String {
    todo!()
}

fn acronym(phrase: &str) -> String {
    todo!()
}

fn main() {
    println!("{}", initial("Ferris"));
    println!("{}", rune_count("🦀 crab"));
    println!("{}", reverse("stressed"));
    println!("{}", acronym("rust is fun"));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initials() {
        assert_eq!(initial("Ferris"), 'F');
        assert_eq!(initial("Émile"), 'É');
        assert_eq!(initial(""), '?');
    }

    #[test]
    fn counting() {
        assert_eq!(rune_count("crab"), 4);
        assert_eq!(rune_count("🦀 crab"), 6);
        assert_eq!(rune_count("héllo"), 5);
    }

    #[test]
    fn reversing() {
        assert_eq!(reverse("stressed"), "desserts");
        assert_eq!(reverse("añb"), "bña");
    }

    #[test]
    fn acronyms() {
        assert_eq!(acronym("portable network graphics"), "PNG");
        assert_eq!(acronym("  rust   is fun "), "RIF");
    }
}
