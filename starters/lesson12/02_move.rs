// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 12-2 · Packing Supplies                          150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A spawned thread can outlive the function that started it, so its
//   closure must not BORROW local variables:
//
//     let name = String::from("Ferris");
//     thread::spawn(|| println!("{name}"));
//     // ✘ E0373: closure may outlive the current function, but it
//     //   borrows `name`
//
//   Give the thread OWNERSHIP with `move`:
//
//     thread::spawn(move || println!("{name}"));   // ✔ name moved in
//     // println!("{name}");                         ✘ moved away
//
//   Need the data in several threads? Give each one its own copy:
//
//     for i in 0..3 {
//         let supplies = supplies.clone();          // a copy per thread
//         thread::spawn(move || use(supplies));
//     }
//
//   (Copying big data is wasteful: `Arc` shares it instead (quest 12-5).)
//
// 🎯 YOUR TASK
//
//   1. Fix `pack_and_count`: the thread must own the backpack.
//   2. Fix `three_scouts`: each scout thread needs its OWN copy of the
//      map, and the function still uses `map` afterwards.

use std::thread;

fn pack_and_count() -> usize {
    let backpack = vec!["rope", "torch", "bread"];
    let handle = thread::spawn(|| backpack.len());
    handle.join().unwrap()
}

/// Each scout returns "{map} #{i}", for i in 0..3. Results in order.
fn three_scouts(map: String) -> (Vec<String>, String) {
    let mut handles = Vec::new();
    for i in 0..3 {
        handles.push(thread::spawn(move || format!("{map} #{i}")));
    }
    let reports = handles.into_iter().map(|h| h.join().unwrap()).collect();
    (reports, map)
}

fn main() {
    println!("{}", pack_and_count());
    println!("{:?}", three_scouts(String::from("Hive map")));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packing() {
        assert_eq!(pack_and_count(), 3);
    }

    #[test]
    fn scouting() {
        let (reports, map) = three_scouts(String::from("Hive map"));
        assert_eq!(reports, vec!["Hive map #0", "Hive map #1", "Hive map #2"]);
        assert_eq!(map, "Hive map");
    }
}
