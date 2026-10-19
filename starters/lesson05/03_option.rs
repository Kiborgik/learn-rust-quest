// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 5-3 · Maybe a Treasure                           125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Rust has NO null. A value that might be missing is an `Option<T>`,
//   which is just an ordinary enum from the standard library:
//
//     enum Option<T> {
//         Some(T),      // there is a value
//         None,         // there isn't
//     }
//
//     fn find_key(room: u32) -> Option<String> {
//         if room == 7 { Some(String::from("Golden Key")) } else { None }
//     }
//
//     match find_key(3) {
//         Some(key) => println!("Found {key}"),
//         None => println!("Empty room"),
//     }
//
//     let n: Option<u32> = Some(5);
//     n + 1;                    // ✘ ERROR: can't add Option<u32> and integer
//     n.unwrap_or(0) + 1;       // ✔ 6  (falls back to 0 when it's None)
//
//   The type system FORCES you to handle the "missing" case before you
//   can touch the value. No NullPointerException, ever.
//   (`.unwrap()` exists too, but it crashes on None. Avoid it.)
//
// 🎯 YOUR TASK
//
//   1. Fix `find_treasure`: chest 1 → 50 gold, 2 → 120, 3 → 7, others: none.
//   2. Fix `gold_or_nothing`: the gold in a chest, or 0 if there's none.
//   3. Implement `double_treasure` with a `match`.

fn find_treasure(chest: u32) -> Option<u32> {
    match chest {
        1 => 50,
        2 => 120,
        3 => 7,
        _ => None,
    }
}

fn gold_or_nothing(chest: u32) -> u32 {
    find_treasure(chest) + 0
}

/// Like `find_treasure`, but the gold amount is doubled.
fn double_treasure(chest: u32) -> Option<u32> {
    todo!()
}

fn main() {
    for chest in 0..5 {
        println!("Chest {chest}: {:?}", find_treasure(chest));
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding() {
        assert_eq!(find_treasure(1), Some(50));
        assert_eq!(find_treasure(2), Some(120));
        assert_eq!(find_treasure(9), None);
    }

    #[test]
    fn defaults() {
        assert_eq!(gold_or_nothing(1), 50);
        assert_eq!(gold_or_nothing(9), 0);
    }

    #[test]
    fn doubling() {
        assert_eq!(double_treasure(3), Some(14));
        assert_eq!(double_treasure(0), None);
    }
}
