// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Compiler Dragon                           250 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The dragon has six heads. Each head can only be slain by one
//   perfectly-typed function. No starter code, just the tests below.
//   Read the tests: they show the exact names, types and expected values.
//
//   Everything here uses only this lesson: arithmetic, casts, arrays,
//   tuples and functions. No `if`, no loops (those come in lesson 2).
//   Each function body is a single expression.
//
//   ⚔️  HEAD 1: `xp_to_next_level(level: u32) -> u32`
//        A hero needs level × level × 100 XP to reach the next level.
//          level 1 → 100,  level 3 → 900
//
//   ⚔️  HEAD 2: `is_boss_level(level: u32) -> bool`
//        Every 5th level is a boss level (5, 10, 15, ...).
//        Remember: comparisons like `a == b` already ARE a `bool`.
//        `%` gives the remainder of a division:  12 % 5 == 2
//
//   ⚔️  HEAD 3: `fire_breath_fahrenheit(celsius: f64) -> f64`
//        Convert °C to °F:  F = C × 1.8 + 32
//        Careful: floats and integers don't mix in Rust!
//
//   ⚔️  HEAD 4: `hp_percent(hp: u32, max_hp: u32) -> u8`
//        How full is the health bar, in whole percent (rounded DOWN)?
//          (50, 200) → 25,  (1, 3) → 33
//        Order matters with integer division: think about when to × 100.
//
//   ⚔️  HEAD 5: `critical_hit(base: u8, multiplier: u8) -> u8`
//        base × multiplier, but a u8 can't hold more than 255:
//        cap it there instead of overflowing.  (200, 2) → 255
//        Quest 1-5 showed a method that stops at the max for you.
//
//   ⚔️  HEAD 6: `average_damage(hits: [u32; 4]) -> f64`
//        The average of four hits, WITH the fraction:  [1, 2, 3, 4] → 2.5
//        Read each hit by index (`hits[0]`...), add, then convert.
//
//   💎 BONUS HEAD: `dodge(pos: (i32, i32)) -> (i32, i32)`
//        Dodging swaps your coordinates: (2, 7) → (7, 2)
//
//   Write your functions below. Run the quest after each one and watch the
//   compiler errors shrink as the dragon's heads fall!

// Your functions go here ↓

fn main() {
    println!("The dragon roars. Run the tests to fight it!");
}

// ─── The Dragon's tests: don't change these! ─────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_1_xp_curve() {
        assert_eq!(xp_to_next_level(1), 100);
        assert_eq!(xp_to_next_level(3), 900);
        assert_eq!(xp_to_next_level(10), 10_000);
    }

    #[test]
    fn head_2_boss_levels() {
        assert!(is_boss_level(5));
        assert!(is_boss_level(20));
        assert!(!is_boss_level(7)); // `!` means "not"
        assert!(!is_boss_level(1));
    }

    #[test]
    fn head_3_fire_breath() {
        assert_eq!(fire_breath_fahrenheit(0.0), 32.0);
        assert_eq!(fire_breath_fahrenheit(100.0), 212.0);
        assert_eq!(fire_breath_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn head_4_health_bar() {
        assert_eq!(hp_percent(50, 200), 25);
        assert_eq!(hp_percent(1, 3), 33);
        assert_eq!(hp_percent(200, 200), 100);
        assert_eq!(hp_percent(0, 80), 0);
    }

    #[test]
    fn head_5_critical() {
        assert_eq!(critical_hit(50, 2), 100);
        assert_eq!(critical_hit(200, 2), 255);
        assert_eq!(critical_hit(255, 255), 255);
    }

    #[test]
    fn head_6_average() {
        assert_eq!(average_damage([1, 2, 3, 4]), 2.5);
        assert_eq!(average_damage([10, 0, 5, 5]), 5.0);
        assert_eq!(average_damage([0, 0, 0, 1]), 0.25);
    }

    #[test]
    fn bonus_head_dodge() {
        assert_eq!(dodge((2, 7)), (7, 2));
        assert_eq!(dodge((-1, 0)), (0, -1));
    }
}
