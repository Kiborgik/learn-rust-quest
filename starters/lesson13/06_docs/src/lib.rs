// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 13-6 · Scrolls of Documentation                  150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `///` comments document the item below them. `cargo doc --open`
//   turns them into a website (that's what docs.rs shows for every crate).
//
//   Code blocks inside them are DOC TESTS: `cargo test` compiles and runs
//   them. Your examples can never go stale.
//
//     /// Adds two numbers.
//     ///
//     /// ```
//     /// use my_crate::add;
//     /// assert_eq!(add(2, 3), 5);
//     /// ```
//     pub fn add(a: i32, b: i32) -> i32 { a + b }
//
//   • Each doc test is a separate little program that uses your crate
//     from outside, so it needs `use crate_name::item;`.
//   • Doc tests only exist for library crates (lib.rs).
//   • `//!` documents the module or crate it's in (see the top of real
//     crates' lib.rs).
//
//   Also handy: `cargo clippy` (lints) and `cargo fmt` (formatting). Real
//   projects run both in CI.
//
// 🎯 YOUR TASK
//
//   1. The example for `split_loot` doesn't compile. Fix the EXAMPLE.
//   2. `title_case` doesn't do what its example says. Fix the FUNCTION.

/// Splits gold evenly between heroes. Returns (each hero's share, leftover).
///
/// ```
/// assert_eq!(split_loot(10, 3), (3, 1));
/// assert_eq!(split_loot(9, 3), (3, 0));
/// ```
pub fn split_loot(gold: u32, heroes: u32) -> (u32, u32) {
    (gold / heroes, gold % heroes)
}

/// Capitalizes the first letter of every word.
///
/// ```
/// use l13_06_docs::title_case;
/// assert_eq!(title_case("ferris the crab"), "Ferris The Crab");
/// assert_eq!(title_case("rust"), "Rust");
/// ```
pub fn title_case(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
