// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 9-5 · The Delta                                  150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   CONSUMERS run the iterator and produce a final result:
//
//     let v = vec![3, 8, 1, 9];
//
//     v.iter().sum::<u32>()                   // 21
//     v.iter().count()                        // 4
//     v.iter().max()  /  .min()               // Some(&9) / Some(&1)
//     v.iter().any(|&x| x > 8)                // true
//     v.iter().all(|&x| x > 0)                // true
//     v.iter().find(|&&x| x > 5)              // Some(&8): first match
//     v.iter().position(|&x| x == 1)          // Some(2): its index
//     v.iter().fold(0, |acc, x| acc + x)      // general "reduce"
//
//   Choosing by a key:
//
//     heroes.iter().max_by_key(|h| h.level)   // Option<&Hero>
//     heroes.iter().min_by_key(|h| h.hp)
//
//   Map and filter in one go:
//
//     ["1", "x", "3"].iter().filter_map(|s| s.parse::<u32>().ok())  // 1, 3
//     [[1, 2], [3, 4]].iter().flat_map(|row| row.iter())            // 1, 2, 3, 4
//
//   Closure patterns like `|&x|` destructure the reference right in the
//   parameter: `|&x| x > 8` is the same as `|x| *x > 8`.
//
// 🎯 YOUR TASK
//
//   Implement each function as one iterator chain.

#[derive(Debug, PartialEq)]
struct Fish {
    name: &'static str,
    weight: u32,
}

fn catch() -> Vec<Fish> {
    vec![
        Fish { name: "Trout", weight: 3 },
        Fish { name: "Pike", weight: 8 },
        Fish { name: "Carp", weight: 5 },
        Fish { name: "Minnow", weight: 1 },
    ]
}

/// Total weight of all fish.
fn total_weight(fish: &[Fish]) -> u32 {
    todo!()
}

/// The name of the heaviest fish (None if there are none).
fn heaviest(fish: &[Fish]) -> Option<&'static str> {
    todo!()
}

/// Is there any fish heavier than `limit`?
fn any_over(fish: &[Fish], limit: u32) -> bool {
    todo!()
}

/// The index of the first fish with this name.
fn index_of(fish: &[Fish], name: &str) -> Option<usize> {
    todo!()
}

/// Sum of all the numbers in the log that parse correctly.
fn valid_total(log: &[&str]) -> u32 {
    todo!()
}

/// Product of all the numbers, using `fold`.
fn product(nums: &[u64]) -> u64 {
    todo!()
}

/// All the characters of all the words, as one String
/// (["ab", "cd"] → "abcd"). Use `flat_map` and `.chars()`.
fn all_chars(words: &[&str]) -> String {
    todo!()
}

fn main() {
    println!("{}", total_weight(&catch()));
    println!("{:?}", heaviest(&catch()));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weights() {
        assert_eq!(total_weight(&catch()), 17);
        assert_eq!(total_weight(&[]), 0);
    }

    #[test]
    fn heavy() {
        assert_eq!(heaviest(&catch()), Some("Pike"));
        assert_eq!(heaviest(&[]), None);
    }

    #[test]
    fn over() {
        assert!(any_over(&catch(), 7));
        assert!(!any_over(&catch(), 8));
    }

    #[test]
    fn indexes() {
        assert_eq!(index_of(&catch(), "Carp"), Some(2));
        assert_eq!(index_of(&catch(), "Shark"), None);
    }

    #[test]
    fn valid() {
        assert_eq!(valid_total(&["4", "oops", "6", "-1", "10"]), 20);
    }

    #[test]
    fn products() {
        assert_eq!(product(&[2, 3, 4]), 24);
        assert_eq!(product(&[]), 1);
    }

    #[test]
    fn chars() {
        assert_eq!(all_chars(&["ab", "cd", "", "e"]), "abcde");
    }
}
