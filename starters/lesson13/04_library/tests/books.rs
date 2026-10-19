// ─── Integration tests: don't change these! ──────────────────────────────────
// This file is compiled as its own crate and can only use the library's
// public API.

use l13_04_library::catalog::{BOOKS, find};

#[test]
fn finds_by_prefix() {
    assert_eq!(find("The Crab").unwrap().pages, 120);
    assert_eq!(find("Borrow").unwrap().title, "Borrowing for Beginners");
}

#[test]
fn first_match_wins() {
    assert_eq!(find("The").unwrap().title, "The Crab Codex");
}

#[test]
fn missing() {
    assert_eq!(find("Unsafe"), None);
    assert_eq!(find("the crab"), None);
}

#[test]
fn catalog_size() {
    assert_eq!(BOOKS.len(), 3);
}
