// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 29-4 · The Old Tongue Calls Back                 175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   C libraries often call YOUR code back: "here's a function pointer,
//   I'll call it when I need you". C's `qsort` sorts any array, and asks
//   a comparator you give it which of two items comes first:
//
//     unsafe extern "C" {
//         fn qsort(
//             base: *mut c_void,                 // the array
//             count: usize,                      // how many items
//             size: usize,                       // bytes per item
//             compare: extern "C" fn(*const c_void, *const c_void) -> c_int,
//         );
//     }
//
//   The comparator must use the C calling convention: `extern "C" fn`.
//   It gets untyped pointers (`*const c_void`). Cast them back to what
//   you know they are, then read:
//
//     extern "C" fn by_value(a: *const c_void, b: *const c_void) -> c_int {
//         // SAFETY: qsort only passes pointers into our [u8] array.
//         let (a, b) = unsafe { (*(a as *const u8), *(b as *const u8)) };
//         a.cmp(&b) as c_int      // Ordering → -1, 0, 1
//     }
//
//   ⚠️ A panic must never cross into C. Rust aborts the whole program if
//   it tries. Keep callbacks simple.
//
//   (In real Rust you'd call `slice.sort()`. This is about the pattern,
//   which you'll meet in C libraries with callbacks and event handlers.)
//
// 🎯 YOUR TASK
//
//   1. `desc` should sort i32s from big to small. Write the comparator.
//   2. Write `sort_heroes`: by level (low to high), then by id, using
//      qsort with your own comparator `by_level_then_id`.

use std::ffi::{c_int, c_void};
use std::mem::size_of;

unsafe extern "C" {
    fn qsort(base: *mut c_void, count: usize, size: usize, compare: extern "C" fn(*const c_void, *const c_void) -> c_int);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hero {
    pub id: u32,
    pub level: u32,
}

extern "C" fn desc(a: *const c_void, b: *const c_void) -> c_int {
    0
}

pub fn sort_desc(values: &mut [i32]) {
    // SAFETY: the pointer, count and item size describe `values` exactly,
    // and `desc` reads i32s.
    unsafe { qsort(values.as_mut_ptr().cast(), values.len(), size_of::<i32>(), desc) }
}

pub fn sort_heroes(heroes: &mut [Hero]) {
    todo!()
}

fn main() {
    let mut v = [3, 9, 1];
    sort_desc(&mut v);
    println!("{v:?}");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let mut v = [3, -9, 12, 0, 7, 7];
        sort_desc(&mut v);
        assert_eq!(v, [12, 7, 7, 3, 0, -9]);
    }

    #[test]
    fn heroes() {
        let h = |id, level| Hero { id, level };
        let mut party = [h(4, 30), h(2, 12), h(9, 12), h(1, 50), h(3, 1)];
        sort_heroes(&mut party);
        assert_eq!(party, [h(3, 1), h(2, 12), h(9, 12), h(4, 30), h(1, 50)]);
    }
}
