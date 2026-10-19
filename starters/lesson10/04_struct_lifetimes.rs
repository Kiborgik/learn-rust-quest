// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 10-4 · Bookmarks                                 150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A struct can HOLD references, but then it needs a lifetime
//   parameter: the struct must not outlive the data it points to.
//
//     struct Quote<'a> {
//         text: &'a str,           // borrowed from somewhere else
//     }
//
//     impl<'a> Quote<'a> {
//         fn text(&self) -> &'a str {   // returns a ref to the ORIGINAL text,
//             self.text                 // not to the Quote itself
//         }
//     }
//
//     let novel = String::from("Call me Ferris. Some years ago...");
//     let q = Quote { text: &novel[..15] };
//     // drop(novel);  ✘ can't drop the novel while `q` borrows it
//
//   Why? It's "zero-copy": a parser can hand out pieces of the input
//   without allocating new Strings.
//
//   `'_` means "some lifetime, let the compiler figure it out":
//     fn make(text: &str) -> Quote<'_>   // borrows from `text`
//
// 🎯 YOUR TASK
//
//   1. Give `Bookmark` a lifetime parameter.
//   2. Fix the impl block (it needs `impl<'a> Bookmark<'a>`).
//   3. Implement `first_sentence(text: &str) -> Bookmark<'_>`: a bookmark
//      of everything up to and including the first '.' (the whole text if
//      there is no '.').
//      Tip: `text.find('.')` returns `Option<usize>`.

struct Bookmark {
    page: u32,
    passage: &str,
}

impl Bookmark {
    fn passage(&self) -> &str {
        self.passage
    }

    fn word_count(&self) -> usize {
        self.passage.split_whitespace().count()
    }
}

fn first_sentence(text: &str) -> Bookmark<'_> {
    todo!()
}

fn main() {
    let book = String::from("It was a dark night. The crab walked on.");
    let mark = first_sentence(&book);
    println!("p.{}: {}", mark.page, mark.passage());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bookmarks() {
        let book = String::from("It was a dark night. The crab walked on.");
        let mark = Bookmark { page: 7, passage: &book[21..] };
        assert_eq!(mark.passage(), "The crab walked on.");
        assert_eq!(mark.word_count(), 4);
    }

    #[test]
    fn sentences() {
        let book = String::from("It was a dark night. The crab walked on.");
        let mark = first_sentence(&book);
        assert_eq!(mark.passage(), "It was a dark night.");
        assert_eq!(mark.page, 1);
        assert_eq!(first_sentence("no dot here").passage(), "no dot here");
    }
}
