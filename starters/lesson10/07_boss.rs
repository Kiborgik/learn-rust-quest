// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Librarian Lich                             350 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Librarian Lich has hoarded knowledge for a thousand years and
//   never copied a single letter. To defeat it, build a ZERO-COPY scroll
//   parser: every piece of text you hand out must borrow from the
//   original. No `String` allowed in your structs!
//
//   A scroll's first line is its title; the rest are its lines:
//       "The Crab Codex\nFirst, be kind.\nSecond, be brave."
//
//   ⚔️  HEAD 1: `struct Scroll<'a> { title: &'a str, lines: Vec<&'a str> }`
//        and `fn parse(text: &str) -> Scroll<'_>`
//        (empty text → title "" and no lines). `text.lines()` helps.
//
//   ⚔️  HEAD 2: `fn longest_line(&self) -> Option<&'a str>`
//        (the first one on a tie)
//
//   ⚔️  HEAD 3: `fn find(&self, word: &str) -> Option<&'a str>`
//        The first line containing `word`. The result must be tied to
//        the TEXT ('a), not to `word` or `self`: see the test!
//
//   ⚔️  HEAD 4: `struct Tokens<'a> { rest: &'a str }` implementing
//        `Iterator<Item = &'a str>`: yields the words of the text one by
//        one (split on spaces; skip empty pieces). Plus
//        `fn tokens(text: &str) -> Tokens<'_>`.
//        Tip: `self.rest.trim_start()`, then `.find(' ')` to split.
//
//   ⚔️  HEAD 5: `fn title_or<'a>(scroll: &Scroll<'a>, fallback: &'a str) -> &'a str`
//        Returns the title, or `fallback` when the title is empty.
//
//   Your code goes here ↓



fn main() {
    println!("Dust swirls between the shelves. Run the tests to face the Lich!");
}

// ─── The Lich's tests: don't change these! ───────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    const CODEX: &str = "The Crab Codex\nFirst, be kind.\nSecond, be brave and bold.\nThird, rest.";

    #[test]
    fn head_1_parse() {
        let s = parse(CODEX);
        assert_eq!(s.title, "The Crab Codex");
        assert_eq!(s.lines, vec!["First, be kind.", "Second, be brave and bold.", "Third, rest."]);
        let empty = parse("");
        assert_eq!(empty.title, "");
        assert!(empty.lines.is_empty());
    }

    #[test]
    fn head_2_longest() {
        assert_eq!(parse(CODEX).longest_line(), Some("Second, be brave and bold."));
        assert_eq!(parse("Only a title").longest_line(), None);
    }

    #[test]
    fn head_3_find_outlives_everything() {
        let text = String::from(CODEX);
        let found;
        {
            let scroll = parse(&text);
            let word = String::from("rest");
            found = scroll.find(&word);
        } // scroll AND word are gone here, but the text lives on
        assert_eq!(found, Some("Third, rest."));
    }

    #[test]
    fn head_4_tokens() {
        let words: Vec<&str> = tokens("  be   kind to  crabs ").collect();
        assert_eq!(words, vec!["be", "kind", "to", "crabs"]);
        assert_eq!(tokens("").count(), 0);
        assert_eq!(tokens("one").collect::<Vec<_>>(), vec!["one"]);
    }

    #[test]
    fn head_5_title_or() {
        let s = parse("\nno title here");
        assert_eq!(title_or(&s, "Untitled"), "Untitled");
        assert_eq!(title_or(&parse(CODEX), "Untitled"), "The Crab Codex");
    }
}
