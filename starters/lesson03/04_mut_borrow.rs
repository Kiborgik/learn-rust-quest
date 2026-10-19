// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 3-4 · The One-Writer Rule                        100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   To change a borrowed value, borrow it MUTABLY with `&mut`:
//
//     fn sharpen(blade: &mut String) {
//         blade.push_str(" +1");
//     }
//     fn heal(hp: &mut u32) {
//         *hp += 10;            // `*` dereferences: "the value behind hp"
//     }
//
//     let mut sword = String::from("Sword");   // the owner must be `mut` too
//     sharpen(&mut sword);
//
//   ⚖️  THE BORROWING RULES: at any moment you can have EITHER:
//       • any number of shared borrows   `&T`      (many readers), OR
//       • exactly one mutable borrow     `&mut T`  (one writer)
//     ...but never both at once.
//
//     let r1 = &mut sword;
//     let r2 = &mut sword;      // ✘ ERROR: second mutable borrow
//     r1.push_str("!");
//
//   A borrow lasts until its LAST USE, not until the end of the block
//   (that's called "non-lexical lifetimes"). So this is fine:
//
//     let r1 = &mut sword;
//     r1.push_str("!");         // last use of r1: borrow over
//     let r2 = &mut sword;      // ✔
//
//   This rule rules out data races and "iterator invalidation" bugs
//   at compile time.
//
// 🎯 YOUR TASK
//
//   Three functions, three borrowing bugs. Fix them WITHOUT `.clone()`:
//   reorder lines or change types instead.

fn add_gold(purse: &u32, amount: u32) {
    *purse += amount;
}

fn forge() -> String {
    let mut blade = String::from("Blade");
    let first = &mut blade;
    let second = &mut blade;
    first.push_str(" +1");
    second.push_str(" +2");
    blade
}

fn polish() -> usize {
    let mut armor = String::from("Armor");
    let before = &armor;
    armor.push_str(" (shiny)");
    println!("Before polishing: {before}");
    armor.len()
}

fn main() {
    let mut purse = 10;
    add_gold(&mut purse, 5);
    println!("Purse: {purse}");
    println!("Forged: {}", forge());
    println!("Polished length: {}", polish());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold() {
        let mut purse = 10;
        add_gold(&mut purse, 5);
        add_gold(&mut purse, 20);
        assert_eq!(purse, 35);
    }

    #[test]
    fn forging() {
        assert_eq!(forge(), "Blade +1 +2");
    }

    #[test]
    fn polishing() {
        assert_eq!(polish(), 13);
    }
}
