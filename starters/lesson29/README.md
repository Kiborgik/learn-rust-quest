# Lesson 29 · The Forbidden Archive 🗝️

> `unsafe`, raw pointers, FFI with C, statics, `MaybeUninit`.

Most Rust programmers write very little unsafe code. You still need to
read it (it's in std and many crates), and know when it's the right tool.

## The five powers

`unsafe` allows: dereferencing raw pointers, calling `unsafe fn`, using
`static mut`, implementing `unsafe trait`s, reading `union` fields.
Everything else is still checked.

## The rules of thumb

1. Keep unsafe blocks small, inside a safe function that can't be misused.
2. Write `/// # Safety` on every `unsafe fn`: what must the caller promise?
3. Write `// SAFETY:` on every `unsafe {}`: why is the promise kept here?
4. Run `cargo +nightly miri test` on unsafe code. It catches most UB.

## FFI cheat sheet

| Want | Write |
|---|---|
| declare C functions | `unsafe extern "C" { fn f(x: c_int) -> c_int; }` |
| Rust string → C | `let c = CString::new(s)?; f(c.as_ptr())` (keep `c` alive) |
| C string → Rust | `unsafe { CStr::from_ptr(p) }.to_str()?` |
| static C string | `c"text".as_ptr()` |
| callback for C | `extern "C" fn cb(..) -> c_int` (must not panic) |
| export to C | `#[unsafe(no_mangle)] pub extern "C" fn f(..)` |
| shared struct | `#[repr(C)]` |
| nullable pointer | `unsafe { p.as_mut() }` → `Option<&mut T>` |
| build C code | `cc` crate in build.rs; `bindgen` / `cbindgen` for bindings |

## Edition 2024 changes

- `extern` blocks must be `unsafe extern`.
- `no_mangle` and `export_name` are written `#[unsafe(...)]`.
- References to `static mut` are an error. Use atomics, `Mutex`, `LazyLock`.
- Inside an `unsafe fn`, unsafe steps still need their own `unsafe {}`.

## Coming from other languages

| Elsewhere | Rust |
|---|---|
| C pointers | `*const T` / `*mut T`, only unsafe to read |
| C# `unsafe`, `fixed`, P/Invoke | `unsafe`, `extern "C"` |
| Python ctypes / cffi | `extern "C"` (and PyO3 for real extensions) |
| JNI | `jni` crate, same `extern "C"` ideas |

## 🎮 Controls

| Key | Action |
|---|---|
| *save the file* | auto re-check |
| `Enter` | re-check now |
| `h` | reveal a hint (that quest gives half XP) |
| `m` | quest map & achievements |
| `q` | save & quit |
| `restore` | put this quest's files back to the start (asks first) |
| `reset` | start over: wipe progress and restore all files (asks first) |
