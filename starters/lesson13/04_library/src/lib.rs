// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 13-4 · The Library Crate                         150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A package can hold a LIBRARY crate and a BINARY crate side by side:
//
//     04_library/
//     ├── src/lib.rs        library: reusable code, the public API
//     ├── src/main.rs       binary: a thin `main` that uses the library
//     ├── src/catalog.rs    a module of the library
//     └── tests/books.rs    INTEGRATION tests: use the library like an
//                           outside user would
//
//   The binary and the integration tests reach the library by its crate
//   name: the package name, with `-` turned into `_`:
//
//     use l13_04_library::catalog::find;
//
//   They only see what lib.rs makes public. Putting the logic in lib.rs
//   and keeping main.rs tiny is the usual Rust layout: it makes the code
//   testable.
//
//   `cargo test` runs unit tests (inside src/), integration tests
//   (tests/*.rs) and doc tests (next quest), all at once.
//
// 🎯 YOUR TASK
//
//   1. The integration tests in `tests/books.rs` can't see the catalog.
//      Fix that here in lib.rs.
//   2. Implement `find` in `src/catalog.rs`.

mod catalog;
