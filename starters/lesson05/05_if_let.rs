// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 5-5 · Quick Glances                              125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   When you only care about ONE pattern, a full `match` is noisy.
//
//   `if let`: run code only if the pattern matches:
//
//     if let Some(key) = find_key() {
//         println!("Got {key}");
//     } else {
//         println!("No key");
//     }
//
//   `let ... else`: bind the value or bail out early. The else block
//   must leave the function (`return`, `break`, `panic!`, ...):
//
//     let Some(key) = find_key() else {
//         return 0;
//     };
//     // `key` is usable here, unwrapped
//
//   `while let`: keep looping while the pattern matches:
//
//     let mut stack = vec![1, 2, 3];       // a growable list (lesson 6)
//     while let Some(top) = stack.pop() {  // pop() returns Option<T>
//         println!("{top}");               // 3, 2, 1
//     }
//
//   `matches!`: a pattern test that returns a bool:
//
//     let is_fire = matches!(spell, Spell::Fire(_));
//     let is_big  = matches!(spell, Spell::Fire(p) if p > 10);
//
// 🎯 YOUR TASK
//
//   1. `describe_drop`: use `if let`.
//      Some(item) → "You found: {item}", None → "Nothing here"
//   2. Fix `first_even` (return -1 if there is no even number).
//   3. Fix `drain_stack`: sum everything popped off the stack.
//   4. `count_fire`: count the Fire spells using `matches!`.

enum Spell {
    Fire(u32),
    Ice(u32),
    Heal,
}

fn describe_drop(drop: Option<&str>) -> String {
    todo!()
}

fn find_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn first_even(nums: &[i32]) -> i32 {
    let Some(n) = find_even(nums);
    n
}

fn drain_stack(stack: &mut Vec<u32>) -> u32 {
    let mut total = 0;
    while let item = stack.pop() {
        total += item;
    }
    total
}

fn count_fire(spells: &[Spell]) -> usize {
    todo!()
}

fn main() {
    println!("{}", describe_drop(Some("Amulet")));
    println!("{}", first_even(&[3, 5, 8, 9]));
    let mut stack = vec![1, 2, 3];
    println!("{}", drain_stack(&mut stack));
    println!("{}", count_fire(&[Spell::Fire(3), Spell::Heal, Spell::Ice(2)]));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drops() {
        assert_eq!(describe_drop(Some("Amulet")), "You found: Amulet");
        assert_eq!(describe_drop(None), "Nothing here");
    }

    #[test]
    fn evens() {
        assert_eq!(first_even(&[3, 5, 8, 9]), 8);
        assert_eq!(first_even(&[1, 3]), -1);
    }

    #[test]
    fn draining() {
        let mut stack = vec![1, 2, 3, 4];
        assert_eq!(drain_stack(&mut stack), 10);
        assert!(stack.is_empty());
    }

    #[test]
    fn fire() {
        let spells = [Spell::Fire(3), Spell::Heal, Spell::Fire(9), Spell::Ice(2)];
        assert_eq!(count_fire(&spells), 2);
        assert_eq!(count_fire(&[]), 0);
    }
}
