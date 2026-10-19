// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Borrow Checker Golem                       250 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Golem guarding the Bridge of Ownership is made of pure borrow
//   checker. Every function you write must have EXACTLY the right
//   signature: borrow when you only read, borrow mutably when you
//   change, take ownership only when you consume.
//   Read the tests carefully: how each function is CALLED tells you
//   what its parameters must be.
//
//   ⚔️  HEAD 1: `count_char`: how many times a char appears in a text.
//        Tip: `for c in text.chars() { ... }`
//
//   ⚔️  HEAD 2: `add_title`: append " the Brave" to a name, in place.
//
//   ⚔️  HEAD 3: `double_all`: double every number in a slice, in place.
//        Tip: `for i in 0..nums.len() { nums[i] *= 2; }`
//
//   ⚔️  HEAD 4: `take_and_shout`: CONSUME a String and return it
//        uppercased with "!" at the end.
//
//   ⚔️  HEAD 5: `longest_len`: length of the longest word in a slice
//        of `&str`.
//
//   💎 BONUS: `split_loot`: sum of the first half and the second half
//        of a slice (first half = the first `len / 2` items).
//
//   Your functions go here ↓



fn main() {
    println!("The Golem's eyes glow. Run the tests to fight it!");
}

// ─── The Golem's tests: don't change these! ──────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_1_count() {
        assert_eq!(count_char("banana", 'a'), 3);
        let owned = String::from("crabcake");
        assert_eq!(count_char(&owned, 'c'), 2);
        assert_eq!(owned, "crabcake");
    }

    #[test]
    fn head_2_title() {
        let mut name = String::from("Ferris");
        add_title(&mut name);
        assert_eq!(name, "Ferris the Brave");
    }

    #[test]
    fn head_3_double() {
        let mut nums = [1, 2, 3];
        double_all(&mut nums);
        assert_eq!(nums, [2, 4, 6]);
    }

    #[test]
    fn head_4_shout() {
        let s = String::from("roar");
        assert_eq!(take_and_shout(s), "ROAR!");
    }

    #[test]
    fn head_5_longest() {
        assert_eq!(longest_len(&["a", "abc", "ab"]), 3);
        assert_eq!(longest_len(&[]), 0);
    }

    #[test]
    fn bonus_split() {
        assert_eq!(split_loot(&[1, 2, 3, 4]), (3, 7));
        assert_eq!(split_loot(&[1, 2, 3]), (1, 5));
    }
}
