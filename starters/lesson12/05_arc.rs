// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 12-5 · Shared Honey                              150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   To share a Mutex between threads, several threads must OWN it.
//   `Rc` can't do that: its counter isn't thread-safe, so the compiler
//   refuses:
//
//     let jar = Rc::new(Mutex::new(0));
//     thread::spawn(move || ...jar...);
//     // ✘ E0277: `Rc<Mutex<i32>>` cannot be sent between threads safely
//
//   `Arc` (atomically reference counted) is the thread-safe Rc:
//
//     use std::sync::{Arc, Mutex};
//
//     let jar = Arc::new(Mutex::new(0));
//     let mut handles = vec![];
//     for _ in 0..10 {
//         let jar = Arc::clone(&jar);           // one handle per thread
//         handles.push(thread::spawn(move || {
//             *jar.lock().unwrap() += 1;
//         }));
//     }
//     for h in handles { h.join().unwrap(); }
//     *jar.lock().unwrap();                     // 10
//
//   `Send` = safe to MOVE to another thread. `Sync` = safe to SHARE
//   (&T) between threads. The compiler tracks both automatically, which
//   is why the Rc mistake above is a compile error and not a data race.
//
// 🎯 YOUR TASK
//
//   1. `count_with_bees`: `bees` threads each add `trips` drops of honey,
//      one drop at a time. Fix it: it uses `Rc`.
//   2. `collect_names`: each bee pushes "bee {i}" into a shared Vec.
//      Return the names SORTED (thread order is unpredictable!).

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn count_with_bees(bees: u32, trips: u32) -> u32 {
    let honey = Rc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..bees {
        let honey = Rc::clone(&honey);
        handles.push(thread::spawn(move || {
            for _ in 0..trips {
                *honey.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let total = *honey.lock().unwrap();
    total
}

fn collect_names(bees: u32) -> Vec<String> {
    todo!()
}

fn main() {
    println!("{}", count_with_bees(8, 1000));
    println!("{:?}", collect_names(3));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting() {
        assert_eq!(count_with_bees(8, 1000), 8000);
        assert_eq!(count_with_bees(1, 5), 5);
    }

    #[test]
    fn names() {
        assert_eq!(collect_names(3), vec!["bee 0", "bee 1", "bee 2"]);
    }
}
