//! The library's book catalog.

#[derive(Debug, PartialEq)]
pub struct Book {
    pub title: &'static str,
    pub pages: u32,
}

pub const BOOKS: &[Book] = &[
    Book { title: "The Crab Codex", pages: 120 },
    Book { title: "Borrowing for Beginners", pages: 88 },
    Book { title: "The Lifetime Lexicon", pages: 300 },
];

/// The first book whose title starts with `prefix` (case-sensitive).
/// Tip: `BOOKS.iter().find(|b| ...)` and `str::starts_with`.
pub fn find(prefix: &str) -> Option<&'static Book> {
    todo!()
}
