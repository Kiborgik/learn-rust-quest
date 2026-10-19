// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 28-1 · Trial by Table                            175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Lesson 13 showed #[test]. Real projects add a few crates on top,
//   always as [dev-dependencies] (tests only, never shipped).
//
//   `rstest` gives TABLE tests: one function, many cases, and each case
//   is reported on its own:
//
//     #[rstest]
//     #[case(0, "zero")]
//     #[case(1, "one")]
//     #[case(7, "many")]
//     fn names(#[case] n: u32, #[case] expected: &str) {
//         assert_eq!(name(n), expected);
//     }
//
//   and FIXTURES: setup code that tests ask for by argument name:
//
//     #[fixture]
//     fn shop() -> Shop { Shop::with_items(["sword", "shield"]) }
//
//     #[rstest]
//     fn sells(shop: Shop) { ... }      // `shop` is built by the fixture
//
//   `#[values(1, 2, 3)]` on several arguments tests every combination.
//
//   Test the EDGES. Bugs live at 99/100, 0, empty, and the maximum.
//
// 🎯 YOUR TASK
//
//   1. The table finds bugs in `rank` at the boundaries. Fix `rank`.
//   2. Write the `party` fixture (below the code, above the tests):
//      Ferris with 120 xp, Lumi with 1500, Corro with 40, in that order.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Bronze,
    Silver,
    Gold,
    Legend,
}

/// 0..=99 Bronze, 100..=499 Silver, 500..=999 Gold, 1000 and up Legend.
pub fn rank(xp: u32) -> Rank {
    match xp {
        0..100 => Rank::Bronze,
        100..499 => Rank::Silver,
        500..999 => Rank::Gold,
        _ => Rank::Legend,
    }
}

pub struct Party {
    pub members: Vec<(String, u32)>,
}

impl Party {
    pub fn ranks(&self) -> Vec<Rank> {
        self.members.iter().map(|(_, xp)| rank(*xp)).collect()
    }

    pub fn strongest(&self) -> Option<&str> {
        self.members.iter().max_by_key(|(_, xp)| *xp).map(|(name, _)| name.as_str())
    }
}

// ─── Your fixtures ───────────────────────────────────────────────────────────
#[cfg(test)]
mod fixtures {
    use super::*;
    use rstest::fixture;

    #[fixture]
    pub fn party() -> Party {
        todo!()
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::party;
    use rstest::rstest;

    #[rstest]
    #[case(0, Rank::Bronze)]
    #[case(99, Rank::Bronze)]
    #[case(100, Rank::Silver)]
    #[case(499, Rank::Silver)]
    #[case(500, Rank::Gold)]
    #[case(999, Rank::Gold)]
    #[case(1000, Rank::Legend)]
    #[case(u32::MAX, Rank::Legend)]
    fn ranks(#[case] xp: u32, #[case] expected: Rank) {
        assert_eq!(rank(xp), expected, "rank({xp})");
    }

    #[rstest]
    fn party_ranks(party: Party) {
        assert_eq!(party.ranks(), [Rank::Silver, Rank::Legend, Rank::Bronze]);
    }

    #[rstest]
    fn party_leader(party: Party) {
        assert_eq!(party.strongest(), Some("Lumi"));
    }
}
