// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 29-5 · Speaking Their Language                   175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Now the other way: Rust code that C (or Python, C#, Go...) can call.
//
//     #[unsafe(no_mangle)]               // keep the name "hero_heal" as is
//     pub extern "C" fn hero_heal(hero: *mut Hero, amount: u32) -> bool { ... }
//
//   `no_mangle` is an UNSAFE attribute in edition 2024: two functions with
//   the same exported name would clash at link time, and you vouch they
//   don't.
//
//   Structs shared with C need C's memory layout:
//
//     #[repr(C)]
//     pub struct Hero { pub id: u32, pub level: u8, pub hp: u32 }
//
//   Without it Rust may reorder fields to save space, and C would read
//   garbage. With repr(C) the fields stay in order, padded like C does.
//
//   C pointers may be NULL. `ptr.as_mut()` (unsafe) turns a raw pointer
//   into `Option<&mut T>`: None for null.
//
//   Static C strings: `c"text"` is a `&CStr` literal, zero byte included.
//   `c"text".as_ptr()` lives for the whole program, so it's safe to hand
//   out.
//
//   (To build a real C library: `crate-type = ["cdylib"]` in Cargo.toml,
//   and `cbindgen` writes the C header.)
//
// 🎯 YOUR TASK
//
//   1. It doesn't compile in edition 2024. Fix the attributes.
//   2. Give `Hero` C's layout.
//   3. `hero_heal`: false for a null pointer; else add `amount` to hp,
//      capped at MAX_HP, and return true.
//   4. `hero_title`: c"novice" below level 10, c"veteran" below 50,
//      c"legend" from 50.

use std::ffi::c_char;

pub const MAX_HP: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hero {
    pub id: u32,
    pub level: u8,
    pub hp: u32,
}

#[no_mangle]
pub extern "C" fn hero_heal(hero: *mut Hero, amount: u32) -> bool {
    todo!()
}

#[no_mangle]
pub extern "C" fn hero_title(level: u8) -> *const c_char {
    todo!()
}

fn main() {
    let mut h = Hero { id: 1, level: 12, hp: 40 };
    hero_heal(&mut h, 30);
    println!("{h:?}");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use std::mem::{offset_of, size_of};

    #[test]
    fn c_layout() {
        assert_eq!(offset_of!(Hero, id), 0);
        assert_eq!(offset_of!(Hero, level), 4);
        assert_eq!(offset_of!(Hero, hp), 8);
        assert_eq!(size_of::<Hero>(), 12);
    }

    #[test]
    fn healing() {
        let mut h = Hero { id: 1, level: 12, hp: 40 };
        assert!(hero_heal(&mut h, 30));
        assert_eq!(h.hp, 70);
        assert!(hero_heal(&mut h, 500));
        assert_eq!(h.hp, MAX_HP);
        assert!(!hero_heal(std::ptr::null_mut(), 10));
    }

    #[test]
    fn titles() {
        let title = |level| unsafe { CStr::from_ptr(hero_title(level)) }.to_str().unwrap();
        assert_eq!(title(1), "novice");
        assert_eq!(title(10), "veteran");
        assert_eq!(title(49), "veteran");
        assert_eq!(title(50), "legend");
    }
}
