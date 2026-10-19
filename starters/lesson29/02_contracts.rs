// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 29-2 · The Contract                              175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   An `unsafe fn` has rules the compiler can't check. They're written in
//   a `# Safety` section, and every caller must follow them:
//
//     /// # Safety
//     /// `i` must be less than `v.len()`.
//     pub unsafe fn get_fast(v: &[u32], i: usize) -> u32 {
//         // SAFETY: the caller promised i < v.len().
//         unsafe { *v.get_unchecked(i) }
//     }
//
//   Calling it needs an `unsafe` block, where you show you kept the deal:
//
//     if i < v.len() {
//         // SAFETY: just checked i < v.len().
//         let x = unsafe { get_fast(v, i) };
//     }
//
//   Breaking the rules is UNDEFINED BEHAVIOR (UB): not a panic, not an
//   error, but "anything may happen", including "seems to work today".
//
//   The goal: keep unsafe small and wrap it in a SAFE function that can't
//   be misused. Then the rest of the program never needs to think about
//   it. (In edition 2024, even inside an `unsafe fn` you mark each unsafe
//   step with its own `unsafe {}` block.)
//
//   `cargo +nightly miri test` runs tests in an interpreter that catches
//   most UB. Worth knowing when you write real unsafe code.
//
// 🎯 YOUR TASK
//
//   1. `sum_first` doesn't compile. It checks the contract but forgets
//      the unsafe block. Add it, with a SAFETY comment.
//   2. Write `average_first`: the average of the first n values, rounded
//      down. None if n is 0 or past the end. Use `sum_first`.

/// Sum of the first `n` values, without bounds checks.
///
/// # Safety
/// `n` must be at most `v.len()`.
pub unsafe fn sum_first_unchecked(v: &[u32], n: usize) -> u32 {
    let mut total = 0;
    for i in 0..n {
        // SAFETY: i < n <= v.len(), as the caller promised.
        total += unsafe { *v.get_unchecked(i) };
    }
    total
}

/// Safe: None when n is past the end.
pub fn sum_first(v: &[u32], n: usize) -> Option<u32> {
    if n > v.len() {
        return None;
    }
    Some(sum_first_unchecked(v, n))
}

pub fn average_first(v: &[u32], n: usize) -> Option<u32> {
    todo!()
}

fn main() {
    println!("{:?}", sum_first(&[1, 2, 3], 2));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums() {
        let gold = [5, 10, 20, 40];
        assert_eq!(sum_first(&gold, 0), Some(0));
        assert_eq!(sum_first(&gold, 3), Some(35));
        assert_eq!(sum_first(&gold, 4), Some(75));
        assert_eq!(sum_first(&gold, 5), None);
    }

    #[test]
    fn averages() {
        let gold = [5, 10, 20, 40];
        assert_eq!(average_first(&gold, 2), Some(7));
        assert_eq!(average_first(&gold, 4), Some(18));
        assert_eq!(average_first(&gold, 0), None);
        assert_eq!(average_first(&gold, 9), None);
    }
}
