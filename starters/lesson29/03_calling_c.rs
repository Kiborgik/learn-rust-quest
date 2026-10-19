// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 29-3 · Words from the Old Tongue                 175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   FFI (foreign function interface) lets Rust call C. Declare the C
//   functions in an `unsafe extern "C"` block (edition 2024 requires the
//   `unsafe`: you vouch the signatures are right):
//
//     use std::ffi::{c_char, c_int};
//
//     unsafe extern "C" {
//         fn abs(x: c_int) -> c_int;            // from the C library
//         fn strlen(s: *const c_char) -> usize;
//     }
//
//   Calling them is unsafe. Rust can't check what C does.
//
//   C strings end with a 0 byte; Rust strings don't. Convert:
//
//     let c = CString::new("hello")?;   // adds the 0; fails if the text has a 0 inside
//     unsafe { strlen(c.as_ptr()) }     // `c` must stay alive while C uses the pointer!
//
//   ⚠️ `CString::new(s).unwrap().as_ptr()` in one line is a classic bug:
//   the CString is dropped at the end of the statement and the pointer
//   dangles. Keep it in a variable.
//
//   The other way: `CStr::from_ptr(p)` reads a C string you were given.
//
//   This quest uses functions from the C runtime, which every Rust
//   program already links. For your own C code, the `cc` crate compiles
//   it in a build.rs, and `bindgen` writes the extern block for you.
//
// 🎯 YOUR TASK
//
//   1. It doesn't compile in edition 2024. Fix the extern block.
//   2. Write the safe wrappers `c_abs` and `c_strlen`. c_strlen returns
//      None for text with a 0 byte inside (C would stop there).

use std::ffi::{CString, c_char, c_int};

extern "C" {
    fn abs(x: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

pub fn c_abs(x: i32) -> i32 {
    todo!()
}

pub fn c_strlen(text: &str) -> Option<usize> {
    todo!()
}

fn main() {
    println!("{} {:?}", c_abs(-7), c_strlen("hello"));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute() {
        assert_eq!(c_abs(-42), 42);
        assert_eq!(c_abs(7), 7);
    }

    #[test]
    fn lengths() {
        assert_eq!(c_strlen("hello"), Some(5));
        assert_eq!(c_strlen(""), Some(0));
        assert_eq!(c_strlen("crab 🦀"), Some(9), "strlen counts bytes");
        assert_eq!(c_strlen("bad\0text"), None);
    }
}
