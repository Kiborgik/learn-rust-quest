// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 29-1 · Two Hands, One Blade                      175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `unsafe` doesn't turn the borrow checker off. It unlocks five extra
//   powers, and YOU promise to use them correctly:
//
//     1. dereference a raw pointer (*const T, *mut T)
//     2. call an `unsafe fn`
//     3. access a `static mut`
//     4. implement an `unsafe trait` (Send, Sync)
//     5. access fields of a `union`
//
//   Raw pointers are like C pointers: no lifetimes, may be null or
//   dangling. Making one is safe; READING through it is unsafe:
//
//     let x = 5;
//     let p = &x as *const i32;          // safe
//     let y = unsafe { *p };             // unsafe: you vouch p is valid
//
//   Why would you? Some correct code can't be proven by the borrow
//   checker. Two &mut to DIFFERENT halves of one slice is fine, but the
//   compiler only sees "two &mut into `values`". The standard library
//   solves it once, with unsafe inside a SAFE function:
//
//     let ptr = values.as_mut_ptr();
//     unsafe { std::slice::from_raw_parts_mut(ptr.add(mid), len - mid) }
//
//   Write a `// SAFETY:` comment on every unsafe block: why is this OK?
//   The checks (like `assert!(mid <= len)`) go BEFORE it.
//
// 🎯 YOUR TASK
//
//   1. Make `split_at_mut` compile, using raw pointers. (It's in std as
//      `slice::split_at_mut`; write your own to see how.)
//   2. Write `first_two_mut` with it: mutable refs to items 0 and 1, or
//      None if there are fewer than two. No unsafe needed there.

use std::slice;

pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len, "mid {mid} is past the end ({len})");
    (&mut values[..mid], &mut values[mid..])
}

pub fn first_two_mut(values: &mut [i32]) -> Option<(&mut i32, &mut i32)> {
    todo!()
}

fn main() {
    let mut hp = [10, 20, 30, 40];
    let (left, right) = split_at_mut(&mut hp, 2);
    left[0] += right[1];
    println!("{hp:?}");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitting() {
        let mut v = [1, 2, 3, 4, 5];
        let (a, b) = split_at_mut(&mut v, 2);
        assert_eq!((&*a, &*b), (&[1, 2][..], &[3, 4, 5][..]));
        a[0] = 10;
        b[0] = 30;
        assert_eq!(v, [10, 2, 30, 4, 5]);
        let (a, b) = split_at_mut(&mut v, 5);
        assert_eq!((a.len(), b.len()), (5, 0));
    }

    #[test]
    #[should_panic]
    fn past_the_end() {
        split_at_mut(&mut [1, 2], 3);
    }

    #[test]
    fn first_two() {
        let mut v = [1, 2, 3];
        if let Some((a, b)) = first_two_mut(&mut v) {
            std::mem::swap(a, b);
        }
        assert_eq!(v, [2, 1, 3]);
        assert!(first_two_mut(&mut [7]).is_none());
    }
}
