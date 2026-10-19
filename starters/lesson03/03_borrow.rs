// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 3-3 · Borrowed, Not Stolen                       100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Moving values in and out of every function would be exhausting.
//   Instead, you can BORROW a value with a reference `&`:
//
//     fn length(s: &String) -> usize {   // takes a reference (a borrow)
//         s.len()                        // methods work through references
//     }
//
//     let name = String::from("Ferris");
//     let n = length(&name);             // lend it out...
//     println!("{name} has {n} letters"); // ...still ours ✔
//
//   • `&x` creates a reference: a pointer that is GUARANTEED to be valid.
//   • A borrow doesn't own anything, so nothing is freed when it ends.
//   • Plain `&` references are read-only (mutable ones come next quest).
//   • Like in C++, but the compiler proves the reference can never
//     dangle or outlive the value.
//
// 🎯 YOUR TASK
//
//   The tests (at the bottom) want to use the spell AFTER asking about
//   it. Change the functions to BORROW instead of taking ownership, then
//   write `total_length`.

fn rune_count(spell: String) -> usize {
    spell.len()
}

fn is_fire_spell(spell: String) -> bool {
    spell.starts_with("Fire")
}

// TODO: `total_length(a, b)` borrows two Strings and returns the sum
//       of their lengths.

fn main() {
    let spell = String::from("Fireball");
    println!("{} runes", rune_count(&spell));
    println!("fire? {}", is_fire_spell(&spell));
    println!("Still in my spellbook: {spell}");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_twice() {
        let spell = String::from("Fireball");
        assert_eq!(rune_count(&spell), 8);
        assert!(is_fire_spell(&spell));
        assert_eq!(spell, "Fireball"); // still ours!
    }

    #[test]
    fn not_fire() {
        let spell = String::from("Ice Lance");
        assert!(!is_fire_spell(&spell));
    }

    #[test]
    fn total() {
        let a = String::from("Ice");
        let b = String::from("Bolt");
        assert_eq!(total_length(&a, &b), 7);
        assert_eq!(a, "Ice");
        assert_eq!(b, "Bolt");
    }
}
