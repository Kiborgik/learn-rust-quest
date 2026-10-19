// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 10-3 · Unwritten Rules                           150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Why didn't you need lifetimes before? The compiler fills them in
//   using three ELISION RULES:
//
//   1. Each reference parameter gets its own lifetime.
//        fn f(a: &str, b: &str)  →  fn f<'a, 'b>(a: &'a str, b: &'b str)
//
//   2. If there is exactly ONE input lifetime, the output gets it.
//        fn first(s: &str) -> &str   →  output borrows from `s` ✔
//
//   3. If one parameter is `&self` / `&mut self`, the output gets
//      self's lifetime.
//        fn name(&self, prefix: &str) -> &str   →  borrows from self ✔
//
//   If the rules can't decide, you must annotate (last quest).
//   Rule 3 can also be WRONG for what you mean:
//
//     impl Book {
//         fn find(&self, word: &str) -> &str { word }  // ✘ returns `word`, but
//     }                                                //   rule 3 says "from self"
//
//   Fix: say that the output comes from `word`:
//         fn find<'w>(&self, word: &'w str) -> &'w str { word }
//
// 🎯 YOUR TASK
//
//   Three functions. For each one, decide: does it compile without
//   annotations (then leave it alone), or does it need one?
//   Fix the ones that don't compile.

struct Library {
    motto: String,
}

impl Library {
    /// The library's motto.
    fn motto(&self) -> &str {
        &self.motto
    }

    /// Returns the request back if the motto mentions it, else the motto.
    fn echo(&self, request: &str) -> &str {
        if self.motto.contains(request) { request } else { &self.motto }
    }
}

/// The last word of the text.
fn last_word(text: &str) -> &str {
    text.split_whitespace().last().unwrap_or("")
}

/// Whichever text contains the keyword ("" if neither does).
fn which_has(a: &str, b: &str, keyword: &str) -> &str {
    if a.contains(keyword) {
        a
    } else if b.contains(keyword) {
        b
    } else {
        ""
    }
}

fn main() {
    let lib = Library { motto: String::from("knowledge is power") };
    println!("{}", lib.motto());
    println!("{}", lib.echo("power"));
    println!("{}", last_word("the final page"));
    println!("{}", which_has("cats", "dogs", "og"));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library() {
        let lib = Library { motto: String::from("knowledge is power") };
        assert_eq!(lib.motto(), "knowledge is power");
        assert_eq!(lib.echo("power"), "power");
        assert_eq!(lib.echo("gold"), "knowledge is power");
    }

    #[test]
    fn words() {
        assert_eq!(last_word("the final page"), "page");
        assert_eq!(which_has("cats", "dogs", "og"), "dogs");
        assert_eq!(which_has("cats", "dogs", "zz"), "");
    }

    #[test]
    fn keyword_can_die_early() {
        let a = String::from("red dragon");
        let found;
        {
            let key = String::from("drag");
            found = which_has(&a, "blue", &key);
        }
        assert_eq!(found, "red dragon");
    }
}
