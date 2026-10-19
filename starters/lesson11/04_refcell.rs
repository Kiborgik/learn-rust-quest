// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 11-4 · The Hidden Mechanism                      150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   INTERIOR MUTABILITY: change data through a SHARED reference `&self`,
//   with the borrow rules checked at RUNTIME instead of compile time.
//
//   `Cell<T>` for small Copy values: get / set, no references handed out:
//
//     use std::cell::Cell;
//     struct Counter { hits: Cell<u32> }
//     impl Counter {
//         fn hit(&self) { self.hits.set(self.hits.get() + 1); }   // &self!
//     }
//
//   `RefCell<T>` for anything: borrow() / borrow_mut():
//
//     use std::cell::RefCell;
//     let log = RefCell::new(Vec::new());
//     log.borrow_mut().push("tick");       // RefMut<Vec<_>>, a mutable guard
//     log.borrow().len();                  // Ref<Vec<_>>, a shared guard
//
//   The rules are the same (many readers OR one writer), but breaking
//   them PANICS at runtime:
//
//     let r = log.borrow();
//     log.borrow_mut();          // 💥 panic: already borrowed: BorrowMutError
//
//   Keep guards short-lived: use them in one expression, or drop them
//   before borrowing again.
//
// 🎯 YOUR TASK
//
//   The `Clockwork` logs every action, but its methods take `&self`
//   (many parts of the tower hold a shared reference to it).
//   1. Make `turns` a `Cell<u32>` and `log` a `RefCell<Vec<String>>`.
//   2. Fix `turn` and `history_len` to use them.
//   3. `report` panics at runtime! Find out why and fix it.

use std::cell::{Cell, RefCell};

struct Clockwork {
    turns: u32,
    log: Vec<String>,
}

impl Clockwork {
    fn new() -> Self {
        Clockwork { turns: 0, log: Vec::new() }
    }

    fn turn(&self, by: &str) {
        self.turns += 1;
        self.log.push(format!("turned by {by}"));
    }

    fn history_len(&self) -> usize {
        self.log.len()
    }

    /// Adds a summary line to the log and returns the log's length.
    fn report(&self) -> usize {
        let log = self.log.borrow();
        let summary = format!("{} entries", log.len());
        self.log.borrow_mut().push(summary);
        self.log.borrow().len()
    }
}

fn main() {
    let c = Clockwork::new();
    c.turn("Ferris");
    c.turn("Corro");
    println!("{} turns, {} log lines", c.turns.get(), c.history_len());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turning() {
        let c = Clockwork::new();
        let shared1 = &c;
        let shared2 = &c;
        shared1.turn("A");
        shared2.turn("B");
        assert_eq!(c.turns.get(), 2);
        assert_eq!(c.history_len(), 2);
        assert_eq!(c.log.borrow()[1], "turned by B");
    }

    #[test]
    fn reporting() {
        let c = Clockwork::new();
        c.turn("A");
        assert_eq!(c.report(), 2);
        assert_eq!(c.log.borrow()[1], "1 entries");
    }
}
