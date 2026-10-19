// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 28-4 · Stand-ins                                 175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Code that reads the clock, rolls dice, or calls a server is hard to
//   test: you can't wait two days for a test. The fix is a design change,
//   not a test trick: pass the dependency IN, behind a trait.
//
//     pub trait Clock { fn now(&self) -> u64; }
//
//     pub struct Inn<C: Clock> { clock: C, ... }
//     impl<C: Clock> Inn<C> {
//         pub fn new(clock: C) -> Self { ... }
//     }
//
//     Inn::new(SystemClock)        // the real program
//     Inn::new(fake_clock.clone()) // tests: a FAKE you control
//
//   A fake is just another impl of the trait, written by hand. No crate
//   needed. Generics cost nothing at run time; `Box<dyn Clock>` also
//   works if you prefer.
//
//   Same idea for randomness: take `&mut impl rand::Rng` and pass a
//   seeded `StdRng` in tests, so every run rolls the same numbers.
//
// 🎯 YOUR TASK
//
//   The inn reads the system clock directly, so the tests can't control
//   time. Make Inn generic over `Clock`, built with `Inn::new(clock)`.
//   (Keep the pricing rules as they are.)

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub const DAY: u64 = 24 * 60 * 60;
pub const PRICE_PER_NIGHT: u64 = 10;

pub trait Clock {
    /// Seconds since some fixed start.
    fn now(&self) -> u64;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}

pub struct Inn {
    guests: HashMap<String, u64>,
}

impl Inn {
    pub fn new() -> Inn {
        Inn { guests: HashMap::new() }
    }

    pub fn check_in(&mut self, hero: &str) {
        self.guests.insert(hero.to_string(), SystemClock.now());
    }

    /// The bill: every started day counts as a night, at least one night.
    pub fn check_out(&mut self, hero: &str) -> Option<u64> {
        let since = self.guests.remove(hero)?;
        let stayed = SystemClock.now() - since;
        let nights = stayed.div_ceil(DAY).max(1);
        Some(nights * PRICE_PER_NIGHT)
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// A clock the test moves by hand. Clones share the same time.
    #[derive(Clone, Default)]
    struct FakeClock(Rc<Cell<u64>>);

    impl FakeClock {
        fn advance(&self, secs: u64) {
            self.0.set(self.0.get() + secs);
        }
    }

    impl Clock for FakeClock {
        fn now(&self) -> u64 {
            self.0.get()
        }
    }

    #[test]
    fn short_stay() {
        let clock = FakeClock::default();
        let mut inn = Inn::new(clock.clone());
        inn.check_in("Ferris");
        clock.advance(3600);
        assert_eq!(inn.check_out("Ferris"), Some(10));
        assert_eq!(inn.check_out("Ferris"), None, "already left");
    }

    #[test]
    fn long_stay() {
        let clock = FakeClock::default();
        let mut inn = Inn::new(clock.clone());
        inn.check_in("Lumi");
        clock.advance(2 * DAY);
        inn.check_in("Corro");
        clock.advance(1);
        assert_eq!(inn.check_out("Lumi"), Some(30));
        assert_eq!(inn.check_out("Corro"), Some(10));
    }

    #[test]
    fn real_clock_still_works() {
        let mut inn = Inn::new(SystemClock);
        inn.check_in("Ferris");
        assert_eq!(inn.check_out("Ferris"), Some(10));
    }
}
