// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 11-5 · The Shared Workshop                       150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Rc = many owners (read-only).  RefCell = mutation through &.
//   Together, `Rc<RefCell<T>>` = SHARED, MUTABLE state:
//
//     use std::{cell::RefCell, rc::Rc};
//
//     let chest = Rc::new(RefCell::new(Vec::new()));
//     let ferris = Rc::clone(&chest);
//     let corro = Rc::clone(&chest);
//
//     ferris.borrow_mut().push("sword");   // both see the same Vec
//     corro.borrow_mut().push("shield");
//     chest.borrow().len();                // 2
//
//   It's the closest thing Rust has to a garbage-collected mutable
//   object. Use it sparingly: it moves borrow checking to runtime.
//   Often a different design (passing `&mut`, or using indices) is
//   simpler.
//
// 🎯 YOUR TASK
//
//   Several heroes share one workshop inventory.
//   1. `Hero` must hold a SHARED, MUTABLE handle to the inventory: fix
//      the `stash` field type.
//   2. Implement `Hero::deposit` (push an item) and `Hero::take`
//      (remove and return the LAST item, if any).
//   3. Implement `total_items`.

use std::cell::RefCell;
use std::rc::Rc;

type Stash = Vec<String>;

struct Hero {
    name: String,
    stash: Stash,
}

impl Hero {
    fn new(name: &str, stash: &Rc<RefCell<Stash>>) -> Hero {
        Hero { name: name.to_string(), stash: Rc::clone(stash) }
    }

    fn deposit(&self, item: &str) {
        todo!()
    }

    fn take(&self) -> Option<String> {
        todo!()
    }
}

fn total_items(stash: &Rc<RefCell<Stash>>) -> usize {
    todo!()
}

fn main() {
    let shared = Rc::new(RefCell::new(Vec::new()));
    let ferris = Hero::new("Ferris", &shared);
    let corro = Hero::new("Corro", &shared);
    ferris.deposit("gear");
    corro.deposit("spring");
    println!("{} took {:?}", ferris.name, ferris.take());
    println!("{} items left", total_items(&shared));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workshop() {
        let shared = Rc::new(RefCell::new(Vec::new()));
        let ferris = Hero::new("Ferris", &shared);
        let corro = Hero::new("Corro", &shared);
        ferris.deposit("gear");
        corro.deposit("spring");
        ferris.deposit("cog");
        assert_eq!(total_items(&shared), 3);
        assert_eq!(corro.take(), Some(String::from("cog")));
        assert_eq!(ferris.take(), Some(String::from("spring")));
        assert_eq!(total_items(&shared), 1);
        assert_eq!(Rc::strong_count(&shared), 3);
        assert_eq!(*shared.borrow(), vec![String::from("gear")]);
    }

    #[test]
    fn empty_take() {
        let shared = Rc::new(RefCell::new(Vec::new()));
        let h = Hero::new("Lonely", &shared);
        assert_eq!(h.take(), None);
    }
}
