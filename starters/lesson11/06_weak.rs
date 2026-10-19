// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 11-6 · Weak Links                                150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Two Rcs pointing at each other form a CYCLE: their counts never
//   reach 0, so neither is ever freed (a memory leak, though not UB).
//
//   `Weak<T>` is a NON-OWNING pointer: it doesn't keep the value alive.
//
//     use std::rc::{Rc, Weak};
//     let strong = Rc::new(5);
//     let weak: Weak<i32> = Rc::downgrade(&strong);
//     weak.upgrade();             // Some(Rc<i32>) while the value exists
//     drop(strong);
//     weak.upgrade();             // None: the value is gone
//
//   Classic pattern, a tree:
//     • parent → children:  Rc   (the parent OWNS its children)
//     • child → parent:     Weak (a child doesn't own its parent)
//
//     struct Node {
//         name: String,
//         parent: RefCell<Weak<Node>>,       // can be set later
//         children: RefCell<Vec<Rc<Node>>>,
//     }
//
//   `Weak::new()` creates an empty Weak that never upgrades.
//   `Rc::strong_count` / `Rc::weak_count` show both counters.
//
// 🎯 YOUR TASK
//
//   1. `parent` is currently an `Rc`, which creates a cycle. The test
//      `no_cycles` checks the counts. Change it to `Weak`.
//   2. Fix `attach` to match (`Rc::downgrade`).
//   3. Implement `parent_name`: the parent's name, or None.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Room {
    name: String,
    parent: RefCell<Option<Rc<Room>>>,
    children: RefCell<Vec<Rc<Room>>>,
}

impl Room {
    fn new(name: &str) -> Rc<Room> {
        Rc::new(Room {
            name: name.to_string(),
            parent: RefCell::new(None),
            children: RefCell::new(Vec::new()),
        })
    }
}

fn attach(parent: &Rc<Room>, child: &Rc<Room>) {
    *child.parent.borrow_mut() = Some(Rc::clone(parent));
    parent.children.borrow_mut().push(Rc::clone(child));
}

fn parent_name(room: &Rc<Room>) -> Option<String> {
    todo!()
}

fn main() {
    let tower = Room::new("Tower");
    let attic = Room::new("Attic");
    attach(&tower, &attic);
    println!("{:?}", parent_name(&attic));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parents() {
        let tower = Room::new("Tower");
        let attic = Room::new("Attic");
        attach(&tower, &attic);
        assert_eq!(parent_name(&attic), Some(String::from("Tower")));
        assert_eq!(parent_name(&tower), None);
    }

    #[test]
    fn no_cycles() {
        let tower = Room::new("Tower");
        let attic = Room::new("Attic");
        attach(&tower, &attic);
        assert_eq!(Rc::strong_count(&tower), 1); // only `tower` itself owns it
        assert_eq!(Rc::weak_count(&tower), 1);   // the attic points back weakly
        assert_eq!(Rc::strong_count(&attic), 2); // `attic` + the tower's children
    }

    #[test]
    fn orphaned() {
        let attic = Room::new("Attic");
        {
            let tower = Room::new("Tower");
            attach(&tower, &attic);
        } // tower dropped
        assert_eq!(parent_name(&attic), None);
    }
}
