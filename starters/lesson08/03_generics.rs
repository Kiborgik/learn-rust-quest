// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 8-3 · One Spell, Many Targets                    125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   GENERICS: write code once, use it with many types.
//
//     fn first<T>(items: &[T]) -> &T {   // T = any type
//         &items[0]
//     }
//     first(&[1, 2]);  first(&["a", "b"]);
//
//   Generic structs and impls:
//
//     struct Pair<T> { a: T, b: T }
//
//     impl<T> Pair<T> {                   // for EVERY T
//         fn swap(self) -> Pair<T> { Pair { a: self.b, b: self.a } }
//     }
//     impl Pair<f64> {                    // ONLY for Pair<f64>
//         fn sum(&self) -> f64 { self.a + self.b }
//     }
//
//   To DO anything with a T (compare, add, print), you must say what T
//   can do with a TRAIT BOUND:
//
//     fn biggest<T: PartialOrd>(a: T, b: T) -> T {
//         if a > b { a } else { b }      // `>` needs PartialOrd
//     }
//
//   Rust generates a specialised copy for each type you use
//   ("monomorphisation", like C++ templates), so generics have zero
//   runtime cost.
//
// 🎯 YOUR TASK
//
//   1. Fix `largest`: it doesn't know T can be compared, or copied out
//      of the slice. (Bound: `T: PartialOrd + Copy`)
//   2. Make `Chest` generic: `Chest<T>` holding a `T` called `item`,
//      with `new(item: T)` and `open(self) -> T`.
//   3. Add `fn is_heavy(&self) -> bool` ONLY for `Chest<u32>`: true when
//      the item is over 100.

fn largest<T>(items: &[T]) -> T {
    let mut best = items[0];
    for &x in items {
        if x > best {
            best = x;
        }
    }
    best
}

struct Chest {
    item: String,
}

impl Chest {
    fn new(item: String) -> Self {
        Chest { item }
    }
    fn open(self) -> String {
        self.item
    }
}

fn main() {
    println!("{}", largest(&[3, 9, 2]));
    println!("{}", Chest::new(String::from("gem")).open());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_of_all() {
        assert_eq!(largest(&[3, 9, 2]), 9);
        assert_eq!(largest(&[1.5, -2.0]), 1.5);
        assert_eq!(largest(&['x', 'b', 'z']), 'z');
    }

    #[test]
    fn chests_hold_anything() {
        assert_eq!(Chest::new(String::from("gem")).open(), "gem");
        assert_eq!(Chest::new(42u32).open(), 42);
        assert_eq!(Chest::new((1, 'a')).open(), (1, 'a'));
    }

    #[test]
    fn heavy_chests() {
        assert!(Chest::new(500u32).is_heavy());
        assert!(!Chest::new(5u32).is_heavy());
    }
}
