// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 11-3 · Shared Blueprints                         150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Sometimes one value genuinely has SEVERAL owners. `Rc<T>`
//   ("reference counted") allows that:
//
//     use std::rc::Rc;
//
//     let map = Rc::new(String::from("Treasure Map"));
//     let a = Rc::clone(&map);       // a second owner: just bumps a counter
//     let b = Rc::clone(&map);       // a third owner (no deep copy!)
//     Rc::strong_count(&map);        // 3
//     drop(a);
//     Rc::strong_count(&map);        // 2
//     // the String is freed when the LAST owner is dropped
//
//   • `Rc::clone(&x)` is the idiomatic spelling. `x.clone()` does the
//     same thing but looks like an expensive deep copy.
//   • Rc gives SHARED access only: you can read through it, not modify.
//     (Mutation is the next quest.)
//   • Rc is single-threaded. Its thread-safe sibling is `Arc` (lesson 12).
//
// 🎯 YOUR TASK
//
//   Three adventuring parties all share ONE blueprint of the tower.
//   1. Fix `Party` so it can SHARE the blueprint instead of owning it.
//   2. Fix `assemble` so all three parties get the same blueprint.
//   3. Implement `owners(&Rc<Blueprint>) -> usize`.

use std::rc::Rc;

#[derive(Debug)]
struct Blueprint {
    floors: u32,
}

struct Party {
    name: String,
    plan: Blueprint,
}

fn assemble() -> (Rc<Blueprint>, Vec<Party>) {
    let plan = Rc::new(Blueprint { floors: 13 });
    let parties = vec![
        Party { name: String::from("Red"), plan: plan },
        Party { name: String::from("Blue"), plan: plan },
        Party { name: String::from("Gold"), plan: plan },
    ];
    (plan, parties)
}

fn owners(plan: &Rc<Blueprint>) -> usize {
    todo!()
}

fn main() {
    let (plan, parties) = assemble();
    println!("{} owners", owners(&plan));
    for p in &parties {
        println!("{} sees {} floors", p.name, p.plan.floors);
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sharing() {
        let (plan, parties) = assemble();
        assert_eq!(owners(&plan), 4);
        assert!(Rc::ptr_eq(&plan, &parties[2].plan)); // the SAME blueprint
        drop(parties);
        assert_eq!(owners(&plan), 1);
    }
}
