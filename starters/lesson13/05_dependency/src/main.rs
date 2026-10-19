// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 13-5 · Borrowing Tools                           150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Libraries from crates.io are called CRATES. Add one under
//   `[dependencies]` in Cargo.toml:
//
//     [dependencies]
//     rand = "0.9"          // means ">=0.9.0, <0.10.0" (compatible updates)
//
//   (or run `cargo add rand`, which edits Cargo.toml for you)
//
//   The first build downloads and compiles it. The exact versions are
//   recorded in Cargo.lock, so every build uses the same code.
//
//   Using it:
//
//     use rand::Rng;                     // a trait: brings methods in scope
//
//     let mut rng = rand::rng();         // a random generator
//     let n = rng.random_range(1..=6);   // 1, 2, 3, 4, 5 or 6
//
//   For tests you want the SAME "random" numbers every run. Seed it:
//
//     use rand::{SeedableRng, rngs::StdRng};
//     let mut rng = StdRng::seed_from_u64(42);
//
//   Taking `&mut impl Rng` lets callers pass either kind.
//
//   ⚠️ The first check of this quest downloads `rand` from crates.io, so
//   it needs internet once. After that it works offline.
//
// 🎯 YOUR TASK
//
//   1. Add the `rand` crate, version "0.9", to Cargo.toml.
//   2. `roll` has a bug: a d6 never shows a 6. Fix it.

use rand::Rng;

/// Rolls one die with `sides` sides: a number from 1 to `sides`.
fn roll(rng: &mut impl Rng, sides: u32) -> u32 {
    rng.random_range(1..sides)
}

fn main() {
    let mut rng = rand::rng();
    let rolls: Vec<u32> = (0..5).map(|_| roll(&mut rng, 6)).collect();
    println!("You rolled {rolls:?}");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn every_face_shows_up() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut seen = [false; 7];
        for _ in 0..10_000 {
            let n = roll(&mut rng, 6);
            assert!((1..=6).contains(&n), "rolled {n} on a d6");
            seen[n as usize] = true;
        }
        assert_eq!(seen[1..], [true; 6], "some faces never came up");
    }

    #[test]
    fn seeded_is_repeatable() {
        let a: Vec<u32> = {
            let mut rng = StdRng::seed_from_u64(1);
            (0..20).map(|_| roll(&mut rng, 20)).collect()
        };
        let b: Vec<u32> = {
            let mut rng = StdRng::seed_from_u64(1);
            (0..20).map(|_| roll(&mut rng, 20)).collect()
        };
        assert_eq!(a, b);
    }
}
