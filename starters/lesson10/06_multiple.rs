// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 10-6 · Tangled Timelines                         150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Using ONE lifetime for everything is often too strict. Give
//   unrelated references their OWN lifetimes:
//
//     struct Search<'t, 'q> {
//         text: &'t str,       // the big document
//         query: &'q str,      // a short-lived search term
//     }
//
//     impl<'t, 'q> Search<'t, 'q> {
//         fn first_hit(&self) -> Option<&'t str> {   // tied to TEXT only
//             self.text.split_whitespace().find(|w| w.contains(self.query))
//         }
//     }
//
//   Now the result can outlive the query.
//
//   Ask yourself: "what does my output borrow from?" Annotate exactly
//   that, and nothing more.
//
//   Sometimes the best answer is no reference at all: return an owned
//   `String` and the lifetime puzzle disappears (at the cost of an
//   allocation).
//
// 🎯 YOUR TASK
//
//   The tests create a `Finder` whose `query` dies before its result is
//   used. With one shared lifetime that can't compile.
//   1. Give `Finder` two lifetimes: `'t` for text and `'q` for query.
//   2. Make `find` return a reference tied to the TEXT only.
//   3. Implement `highlight`: return an OWNED String where every word
//      that contains the query is wrapped in [brackets], words joined
//      by single spaces.

struct Finder<'a> {
    text: &'a str,
    query: &'a str,
}

impl<'a> Finder<'a> {
    fn find(&self) -> Option<&'a str> {
        self.text.split_whitespace().find(|w| w.contains(self.query))
    }

    fn highlight(&self) -> String {
        todo!()
    }
}

fn main() {
    let text = String::from("the bold crab sails the cold sea");
    let f = Finder { text: &text, query: "old" };
    println!("{:?}", f.find());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_outlives_query() {
        let text = String::from("the bold crab sails the cold sea");
        let hit;
        {
            let query = String::from("old");
            let f = Finder { text: &text, query: &query };
            hit = f.find();
        }
        assert_eq!(hit, Some("bold"));
    }

    #[test]
    fn highlighting() {
        let text = String::from("the bold crab sails the cold sea");
        let f = Finder { text: &text, query: "old" };
        assert_eq!(f.highlight(), "the [bold] crab sails the [cold] sea");
    }
}
