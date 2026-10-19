// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Swarm of Stars                            400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The night sky is falling apart into a swarm of a million stars, and
//   the royal astronomer needs answers before dawn. One core won't do.
//   Every function must use rayon.
//
//   ⚔️  HEAD 1: `star_names(texts) -> HashMap<String, usize>`
//        Count words (split_whitespace, lowercase) over all texts.
//        fold + reduce, no locks.
//
//   ⚔️  HEAD 2: `brightest(stars) -> Option<&Star>`
//        The star with the highest brightness (the tests have no ties).
//
//   ⚔️  HEAD 3: `constellation(stars, min) -> Vec<&str>`
//        Names of stars with brightness >= min, in the original order.
//
//   ⚔️  HEAD 4: `mandel_count(size, max_iter) -> usize`
//        How many points of a size×size grid over x in [-2, 1),
//        y in [-1.5, 1.5) are in the Mandelbrot set. Point (col, row):
//            cx = -2.0 + 3.0 * col as f64 / size as f64
//            cy = -1.5 + 3.0 * row as f64 / size as f64
//        z starts at 0; repeat max_iter times: z = z² + c; it escapes
//        when |z|² > 4. A point is IN the set if it never escapes.
//        Process rows in parallel.
//
//   ⚔️  HEAD 5: `galaxy_mass(masses) -> u64`
//        Sum the masses in chunks of 4096 (par_chunks), then sum the
//        chunk totals. Must equal the plain sum.

use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Star {
    pub name: String,
    pub brightness: u32,
}

pub fn star_names(texts: &[&str]) -> HashMap<String, usize> {
    todo!()
}

pub fn brightest(stars: &[Star]) -> Option<&Star> {
    todo!()
}

pub fn constellation(stars: &[Star], min: u32) -> Vec<&str> {
    todo!()
}

pub fn mandel_count(size: usize, max_iter: u32) -> usize {
    todo!()
}

pub fn galaxy_mass(masses: &[u64]) -> u64 {
    todo!()
}

// ─── The Swarm's tests: don't change these! ──────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn stars() -> Vec<Star> {
        (0..100_000u32)
            .map(|i| Star { name: format!("s{i}"), brightness: (i * 7919) % 100_003 })
            .collect()
    }

    /// Plain sequential version, to compare against.
    fn mandel_slow(size: usize, max_iter: u32) -> usize {
        let mut count = 0;
        for row in 0..size {
            for col in 0..size {
                let cx = -2.0 + 3.0 * col as f64 / size as f64;
                let cy = -1.5 + 3.0 * row as f64 / size as f64;
                let (mut x, mut y) = (0.0f64, 0.0f64);
                let mut inside = true;
                for _ in 0..max_iter {
                    let (nx, ny) = (x * x - y * y + cx, 2.0 * x * y + cy);
                    x = nx;
                    y = ny;
                    if x * x + y * y > 4.0 {
                        inside = false;
                        break;
                    }
                }
                if inside {
                    count += 1;
                }
            }
        }
        count
    }

    #[test]
    fn head_1_names() {
        let texts = ["Vega Sirius vega", "SIRIUS rigel", "vega"];
        let c = star_names(&texts);
        assert_eq!(c["vega"], 3);
        assert_eq!(c["sirius"], 2);
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn head_2_brightest() {
        let s = stars();
        let expected = s.iter().max_by_key(|s| s.brightness).unwrap();
        assert_eq!(brightest(&s), Some(expected));
        assert_eq!(brightest(&[]), None);
    }

    #[test]
    fn head_3_constellation() {
        let s = stars();
        let picked = constellation(&s, 99_990);
        let expected: Vec<&str> = s.iter().filter(|s| s.brightness >= 99_990).map(|s| s.name.as_str()).collect();
        assert_eq!(picked, expected);
        assert!(!picked.is_empty());
    }

    #[test]
    fn head_4_mandelbrot() {
        assert_eq!(mandel_count(200, 100), mandel_slow(200, 100));
        assert_eq!(mandel_count(1, 10), mandel_slow(1, 10));
    }

    #[test]
    fn head_5_mass() {
        let masses: Vec<u64> = (0..1_000_000).collect();
        assert_eq!(galaxy_mass(&masses), masses.iter().sum::<u64>());
        assert_eq!(galaxy_mass(&[]), 0);
    }
}
