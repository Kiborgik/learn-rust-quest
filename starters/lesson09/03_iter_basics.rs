// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 9-3 · Downstream                                 150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   An ITERATOR is anything with a `next()` method returning Option:
//
//     let v = vec![10, 20];
//     let mut it = v.iter();
//     it.next();     // Some(&10)
//     it.next();     // Some(&20)
//     it.next();     // None, exhausted
//
//   Three ways to get one from a collection (the ownership rules again):
//
//     v.iter()         // yields &T      (borrow)
//     v.iter_mut()     // yields &mut T  (borrow mutably)
//     v.into_iter()    // yields T       (consume the Vec)
//
//   `for x in &v` is just sugar for `for x in v.iter()`.
//
//   Iterators are LAZY: adapters like `map` do NOTHING until something
//   consumes the iterator (a `for` loop, `sum`, `collect`...):
//
//     v.iter().map(|x| println!("{x}"));      // prints NOTHING!
//                                             // (warning: unused `Map` that must be used)
//     let total: i32 = v.iter().sum();        // consuming: runs it
//     let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
//
//   `sum` and `collect` usually need a type annotation, because they
//   can produce many different types.
//
// 🎯 YOUR TASK
//
//   1. `first_two`: return the first two items using `next()`.
//   2. Fix `total`: it needs to know what type to sum into.
//   3. Fix `boost_all`: it should modify the Vec in place, but nothing
//      happens! (Look at the warning.) Use `iter_mut()` and a `for` loop.
//   4. `into_names`: consume the Vec of (name, level) tuples and return
//      just the names, using `into_iter()` and a `for` loop.

fn first_two(items: &[u32]) -> (Option<&u32>, Option<&u32>) {
    let mut it = items.iter();
    todo!()
}

fn total(items: &[u32]) -> u32 {
    let sum = items.iter().sum();
    sum.min(9999) // the bard can't count any higher
}

fn boost_all(stats: &mut Vec<u32>) {
    stats.iter().map(|s| s + 10);
}

fn into_names(party: Vec<(String, u32)>) -> Vec<String> {
    todo!()
}

fn main() {
    println!("{:?}", first_two(&[7, 8, 9]));
    println!("{}", total(&[1, 2, 3]));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn firsts() {
        assert_eq!(first_two(&[7, 8, 9]), (Some(&7), Some(&8)));
        assert_eq!(first_two(&[7]), (Some(&7), None));
    }

    #[test]
    fn totals() {
        assert_eq!(total(&[1, 2, 3]), 6);
    }

    #[test]
    fn boosting() {
        let mut s = vec![1, 2];
        boost_all(&mut s);
        assert_eq!(s, vec![11, 12]);
    }

    #[test]
    fn names() {
        let party = vec![(String::from("Ferris"), 3), (String::from("Corro"), 5)];
        assert_eq!(into_names(party), vec!["Ferris", "Corro"]);
    }
}
