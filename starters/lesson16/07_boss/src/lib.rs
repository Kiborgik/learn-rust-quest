// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Chronicle Golem                           400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Chronicle Golem writes down every battle, in messy text. Turn its
//   log into a clean report with regex and itertools.
//
//   Log lines look like this (anything else is noise to skip):
//
//       [r1] Ferris hits Slime for 12
//       [r1] Corro hits Slime for 8 (crit)
//       [r2] Ferris hits Bat for 5
//       a bird sings
//
//   ⚔️  HEAD 1: `parse(log) -> Vec<Hit>`, one regex in a LazyLock.
//        Round numbers can have several digits. "(crit)" is optional.
//
//   ⚔️  HEAD 2: `damage_by_hero(&[Hit]) -> Vec<(String, u32)>`
//        Total damage per hero, highest first, ties by name A→Z.
//
//   ⚔️  HEAD 3: `damage_per_round(&[Hit]) -> Vec<(u32, u32)>`
//        (round, total damage). Hits arrive grouped by round, in order:
//        `chunk_by` fits.
//
//   ⚔️  HEAD 4: `crit_heroes(&[Hit]) -> String`
//        Heroes with at least one crit, sorted, unique, joined by ", ".
//
//   ⚔️  HEAD 5: `report(log) -> String`, three lines:
//        "rounds: 2"
//        "top: Ferris (17)"
//        "crits: Corro"
//        (empty log: "rounds: 0", "top: nobody", "crits: none")

use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub struct Hit {
    pub round: u32,
    pub hero: String,
    pub foe: String,
    pub damage: u32,
    pub crit: bool,
}

pub fn parse(log: &str) -> Vec<Hit> {
    todo!()
}

pub fn damage_by_hero(hits: &[Hit]) -> Vec<(String, u32)> {
    todo!()
}

pub fn damage_per_round(hits: &[Hit]) -> Vec<(u32, u32)> {
    todo!()
}

pub fn crit_heroes(hits: &[Hit]) -> String {
    todo!()
}

pub fn report(log: &str) -> String {
    todo!()
}

// ─── The Golem's tests: don't change these! ──────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    const LOG: &str = "\
[r1] Ferris hits Slime for 12
[r1] Corro hits Slime for 8 (crit)
a bird sings
[r2] Ferris hits Bat for 5
[r2] Lumi hits Bat for 17 (crit)
[r10] Corro hits Bat for 1 (crit)
";

    #[test]
    fn head_1_parse() {
        let hits = parse(LOG);
        assert_eq!(hits.len(), 5);
        assert_eq!(
            hits[1],
            Hit { round: 1, hero: "Corro".into(), foe: "Slime".into(), damage: 8, crit: true }
        );
        assert_eq!(hits[4].round, 10);
        assert!(!hits[0].crit);
    }

    #[test]
    fn head_2_by_hero() {
        assert_eq!(
            damage_by_hero(&parse(LOG)),
            vec![("Ferris".into(), 17), ("Lumi".into(), 17), ("Corro".into(), 9)]
        );
    }

    #[test]
    fn head_3_rounds() {
        assert_eq!(damage_per_round(&parse(LOG)), vec![(1, 20), (2, 22), (10, 1)]);
    }

    #[test]
    fn head_4_crits() {
        assert_eq!(crit_heroes(&parse(LOG)), "Corro, Lumi");
    }

    #[test]
    fn head_5_report() {
        assert_eq!(report(LOG), "rounds: 3\ntop: Ferris (17)\ncrits: Corro, Lumi");
        assert_eq!(report("nothing here"), "rounds: 0\ntop: nobody\ncrits: none");
    }
}
