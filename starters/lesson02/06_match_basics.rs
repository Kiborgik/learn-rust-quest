// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 2-6 · The Sorting Stone                          100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `match` is `switch` on steroids:
//
//     let n = 7;
//     let size = match n {
//         0 => "none",                // a literal
//         1 | 2 => "a couple",        // several values with `|`
//         3..=9 => "a few",           // an inclusive range
//         _ => "lots",                // `_` = anything else
//     };
//
//   • `match` is an expression, so every arm must produce the same type.
//   • No fall-through and no `break` needed.
//   • Arms are checked top to bottom; the first match wins.
//   • It must be EXHAUSTIVE: every possible value has to be covered.
//     For integers that practically always means a `_` arm at the end.
//   • `a..b` excludes b, `a..=b` includes it, in patterns too!
//
//   (`&'static str` below just means "a text literal baked into the
//   program". Full story in lesson 10. For now, read it as "text".)
//
// 🎯 YOUR TASK
//
//   1. Make `rarity` compile and pass its test:
//        1 → "cursed", 2..=14 → "common", 15..=19 → "rare",
//        20 → "legendary", anything else → "invalid"
//   2. Implement `coins_for(enemy_level)` with a `match`:
//        0 → 0, 1..=5 → 10, 6..=10 → 25, anything higher → 100

fn rarity(roll: u32) -> &'static str {
    match roll {
        1 => "cursed",
        2..=14 => "common",
        15..19 => "rare",
        20 => "legendary",
    }
}

fn coins_for(enemy_level: u32) -> u32 {
    todo!()
}

fn main() {
    for roll in [1, 7, 19, 20, 42] {
        println!("Rolled {roll}: {}", rarity(roll));
    }
    println!("A level 8 goblin drops {} coins", coins_for(8));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rarities() {
        assert_eq!(rarity(1), "cursed");
        assert_eq!(rarity(2), "common");
        assert_eq!(rarity(14), "common");
        assert_eq!(rarity(15), "rare");
        assert_eq!(rarity(19), "rare");
        assert_eq!(rarity(20), "legendary");
        assert_eq!(rarity(0), "invalid");
        assert_eq!(rarity(21), "invalid");
    }

    #[test]
    fn coins() {
        assert_eq!(coins_for(0), 0);
        assert_eq!(coins_for(1), 10);
        assert_eq!(coins_for(5), 10);
        assert_eq!(coins_for(6), 25);
        assert_eq!(coins_for(10), 25);
        assert_eq!(coins_for(11), 100);
        assert_eq!(coins_for(999), 100);
    }
}
