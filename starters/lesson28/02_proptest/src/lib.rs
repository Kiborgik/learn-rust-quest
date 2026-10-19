// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 28-2 · Trial by Chaos                            175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Example tests check the cases you thought of. PROPERTY tests check
//   rules that must hold for ALL inputs, and `proptest` throws hundreds
//   of random inputs at them:
//
//     proptest! {
//         #[test]
//         fn reverse_twice(v in prop::collection::vec(any::<u8>(), 0..50)) {
//             let mut w = v.clone();
//             w.reverse();
//             w.reverse();
//             prop_assert_eq!(w, v);
//         }
//     }
//
//   `x in 0u32..100` picks values from a range; `any::<i64>()` from all
//   of them. When a case fails, proptest SHRINKS it: it looks for the
//   smallest input that still fails, so you get "total = 1, n = 2"
//   instead of "total = 83712, n = 41".
//
//   Failing inputs are saved in proptest-regressions/ and tried first on
//   the next run.
//
//   Good properties: round trips (decode(encode(x)) == x), totals that
//   must not change, results that must stay in a range, "never panics".
//
// 🎯 YOUR TASK
//
//   The properties found bugs in both functions. Read the shrunk inputs
//   in the test output and fix the code.

/// Split `total` gold among `n` heroes (n >= 1) as evenly as possible.
/// No coin is lost; the first heroes get the extra coins.
pub fn split(total: u32, n: u32) -> Vec<u32> {
    let share = total / n;
    vec![share; n as usize]
}

/// Any level clamped into 1..=99.
pub fn clamp_level(level: i64) -> u8 {
    (level.max(1) as u8).min(99)
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn an_example() {
        assert_eq!(split(10, 3), [4, 3, 3]);
    }

    proptest! {
        #[test]
        fn no_coin_is_lost(total in 0u32..100_000, n in 1u32..50) {
            let shares = split(total, n);
            prop_assert_eq!(shares.len(), n as usize);
            prop_assert_eq!(shares.iter().sum::<u32>(), total);
        }

        #[test]
        fn shares_are_fair(total in 0u32..100_000, n in 1u32..50) {
            let shares = split(total, n);
            let min = *shares.iter().min().unwrap();
            let max = *shares.iter().max().unwrap();
            prop_assert!(max - min <= 1, "shares differ by more than 1: {:?}", shares);
            prop_assert!(shares.windows(2).all(|w| w[0] >= w[1]), "extra coins go first: {:?}", shares);
        }

        #[test]
        fn levels_stay_in_range(level in any::<i64>()) {
            let l = clamp_level(level);
            prop_assert!((1..=99).contains(&l), "clamp_level({}) = {}", level, l);
        }
    }
}
