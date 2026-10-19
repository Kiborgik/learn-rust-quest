// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Tribunal                                  400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The guild vault is on trial. Four judges have examined it, each with
//   their own tool, and each found a crime. Read every verdict (the test
//   output) and fix the vault. The tests are right; the code is not.
//
//   ⚔️  HEAD 1 · the Table Judge (rstest): interest is paid per FULL
//        week. Day 6 is not a week.
//
//   ⚔️  HEAD 2 · the Chaos Judge (proptest): huge deposits make the vault
//        panic. Gold must never be lost, whatever the amount.
//
//   ⚔️  HEAD 3 · the Puppet Judge (mockall): the herald should announce
//        only deposits of BIG_DEPOSIT gold or more, as
//        "{hero} stored {gold} gold!".
//
//   ⚔️  HEAD 4 · the Painted Judge (insta): the statement lists deposits
//        by day, and by hero name within a day.

pub const BIG_DEPOSIT: u32 = 500;

pub trait Clock {
    /// Days since the vault opened.
    fn today(&self) -> u32;
}

#[cfg_attr(test, mockall::automock)]
pub trait Herald {
    fn announce(&mut self, message: &str);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deposit {
    pub hero: String,
    pub gold: u32,
    pub day: u32,
}

/// Full weeks between two days.
pub fn full_weeks(from: u32, to: u32) -> u32 {
    (to - from + 6) / 7
}

pub struct Vault<C: Clock, H: Herald> {
    clock: C,
    herald: H,
    deposits: Vec<Deposit>,
}

impl<C: Clock, H: Herald> Vault<C, H> {
    pub fn new(clock: C, herald: H) -> Self {
        Vault { clock, herald, deposits: Vec::new() }
    }

    pub fn deposit(&mut self, hero: &str, gold: u32) -> Result<(), String> {
        if gold == 0 {
            return Err("nothing to deposit".to_string());
        }
        let day = self.clock.today();
        self.deposits.push(Deposit { hero: hero.to_string(), gold, day });
        self.herald.announce(&format!("{hero} stored {gold} gold!"));
        Ok(())
    }

    /// Everything the hero stored, plus 1% interest per full week, per
    /// deposit (rounded down).
    pub fn worth(&self, hero: &str) -> u64 {
        let today = self.clock.today();
        self.deposits
            .iter()
            .filter(|d| d.hero == hero)
            .map(|d| {
                let weeks = full_weeks(d.day, today);
                (d.gold + d.gold * weeks / 100) as u64
            })
            .sum()
    }

    pub fn statement(&self) -> String {
        let mut sorted: Vec<&Deposit> = self.deposits.iter().collect();
        sorted.sort_by(|a, b| a.hero.cmp(&b.hero));
        let mut lines: Vec<String> =
            sorted.iter().map(|d| format!("day {:>3}  {:<8}{:>6}", d.day, d.hero, d.gold)).collect();
        let total: u64 = self.deposits.iter().map(|d| d.gold as u64).sum();
        lines.push(format!("{:<17}{:>6}", "total", total));
        lines.join("\n")
    }
}

// ─── The Tribunal's tests: don't change these! ───────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rstest::rstest;
    use std::cell::Cell;
    use std::rc::Rc;

    #[derive(Clone, Default)]
    struct FakeClock(Rc<Cell<u32>>);

    impl FakeClock {
        fn set(&self, day: u32) {
            self.0.set(day);
        }
    }

    impl Clock for FakeClock {
        fn today(&self) -> u32 {
            self.0.get()
        }
    }

    fn quiet() -> MockHerald {
        let mut h = MockHerald::new();
        h.expect_announce().return_const(());
        h
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(0, 6, 0)]
    #[case(0, 7, 1)]
    #[case(3, 16, 1)]
    #[case(3, 17, 2)]
    #[case(10, 80, 10)]
    fn head_1_full_weeks(#[case] from: u32, #[case] to: u32, #[case] weeks: u32) {
        assert_eq!(full_weeks(from, to), weeks, "full_weeks({from}, {to})");
    }

    #[test]
    fn head_1_interest() {
        let clock = FakeClock::default();
        let mut v = Vault::new(clock.clone(), quiet());
        v.deposit("Ferris", 1000).unwrap();
        clock.set(13);
        assert_eq!(v.worth("Ferris"), 1010);
        clock.set(14);
        assert_eq!(v.worth("Ferris"), 1020);
    }

    proptest! {
        #[test]
        fn head_2_no_gold_is_lost(gold in 1u32..=u32::MAX, days in 0u32..10_000) {
            let clock = FakeClock::default();
            let mut v = Vault::new(clock.clone(), quiet());
            v.deposit("Ferris", gold).unwrap();
            clock.set(days);
            let now = v.worth("Ferris");
            prop_assert!(now >= gold as u64, "worth {} < deposit {}", now, gold);
            clock.set(days + 7);
            prop_assert!(v.worth("Ferris") >= now, "worth went down over time");
        }
    }

    #[test]
    fn head_3_the_herald() {
        let mut herald = MockHerald::new();
        herald.expect_announce().withf(|m: &str| m == "Lumi stored 700 gold!").times(1).return_const(());
        let mut v = Vault::new(FakeClock::default(), herald);
        v.deposit("Ferris", 20).unwrap();
        v.deposit("Lumi", 700).unwrap();
        v.deposit("Corro", 499).unwrap();
        assert_eq!(v.deposit("Zed", 0), Err("nothing to deposit".to_string()));
    }

    #[test]
    fn head_4_statement() {
        let clock = FakeClock::default();
        let mut v = Vault::new(clock.clone(), quiet());
        v.deposit("Zed", 20).unwrap();
        clock.set(2);
        v.deposit("Ferris", 50).unwrap();
        v.deposit("Ada", 700).unwrap();
        clock.set(9);
        insta::assert_snapshot!(v.statement(), @r"
        day   0  Zed         20
        day   2  Ada        700
        day   2  Ferris      50
        total               770
        ");
    }
}
