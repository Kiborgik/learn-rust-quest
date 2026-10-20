// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 27-2 · Many Voices                               175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   REPETITION matches a pattern many times, like `vec![1, 2, 3]`:
//
//     $( ... ),*     zero or more, separated by commas
//     $( ... ),+     one or more
//     $( ... )?      zero or one
//     $(,)?          an optional trailing comma
//
//   In the output, `$( ... )*` repeats once per match:
//
//     macro_rules! shout_all {
//         ($($word:expr),* $(,)?) => {
//             vec![$( $word.to_uppercase() ),*]
//         };
//     }
//     shout_all!("hi", "yo",)   // vec!["HI", "YO"]
//
//   Rules are tried top to bottom; the first one that matches wins. So a
//   macro can have a special case for one argument and a general one for
//   more:
//
//     ($x:expr) => { ... };
//     ($x:expr, $($rest:expr),+) => { ... };
//
//   A rule may also use the macro itself on the rest, passing the
//   repetition on: `shout_all!($($rest),+)`. (Quest 27-5 shows more.)
//
// 🎯 YOUR TASK
//
//   1. `hashmap!` breaks on a trailing comma. Allow one.
//   2. `max_of!` takes one value. Make it take any number (1 or more).
//      It must work for floats too, so no `.max()` on an iterator.

use std::collections::HashMap;

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {{
        let mut map = HashMap::new();
        $( map.insert($key, $value); )*
        map
    }};
}

macro_rules! max_of {
    ($x:expr) => {
        $x
    };
}

fn main() {
    let prices = hashmap! { "sword" => 50, "shield" => 30 };
    println!("{prices:?}, best: {}", max_of!(3));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps() {
        let prices = hashmap! {
            "sword" => 50,
            "shield" => 30,
            "potion" => 5,
        };
        assert_eq!(prices.len(), 3);
        assert_eq!(prices["shield"], 30);
        let empty: HashMap<&str, u32> = hashmap! {};
        assert!(empty.is_empty());
    }

    #[test]
    fn maxes() {
        assert_eq!(max_of!(7), 7);
        assert_eq!(max_of!(3, 9, 2), 9);
        assert_eq!(max_of!(1, 2, 3, 4, 5, 4, 3), 5);
        assert_eq!(max_of!(0.5, 2.5, 1.0), 2.5);
    }
}
