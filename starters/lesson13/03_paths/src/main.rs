// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 13-3 · Nested Rooms                              150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Modules can nest, and paths say how to reach an item:
//
//     crate::a::b::item     // absolute: from the crate root
//     super::item           // relative: the parent module
//     self::child::item     // relative: a child of this module
//
//   `use` makes a shorter name, in the current module only:
//
//     use crate::workshop::forge;
//     forge::heat();
//
//   `pub use` RE-EXPORTS an item, so callers get a shorter public path.
//   Libraries do this a lot:
//
//     pub mod workshop {
//         pub mod forge { pub fn heat() -> u32 { 9 } }
//         pub use forge::heat;     // now `workshop::heat()` works too
//     }
//
//   (Modules can live in files, like last quest, or inline like here.
//   The rules are the same.)
//
// 🎯 YOUR TASK
//
//   1. `heat_report` can't find `forge`. Fix the path.
//   2. The tests want to call `workshop::heat()` directly. Re-export it.

mod workshop {
    const BASE_HEAT: u32 = 500;

    pub mod forge {
        pub fn heat() -> u32 {
            super::BASE_HEAT * 2
        }
    }

    pub mod storage {
        pub fn heat_report() -> String {
            format!("forge at {}", forge::heat())
        }
    }
}

use workshop::storage;

fn main() {
    println!("{}", storage::heat_report());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use crate::workshop;

    #[test]
    fn report() {
        assert_eq!(workshop::storage::heat_report(), "forge at 1000");
    }

    #[test]
    fn re_export() {
        assert_eq!(workshop::heat(), 1000);
    }
}
