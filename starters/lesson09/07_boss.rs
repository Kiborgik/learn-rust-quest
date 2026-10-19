// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The River Serpent                              350 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The River Serpent's coils are made of endless streams of data. To
//   defeat it, analyse its battle log WITHOUT writing a single `for` loop:
//   iterator chains only. (Loops still compile, but where's the honour?)
//
//   ⚔️  HEAD 1: `total_damage(log: &[Hit]) -> u32`
//
//   ⚔️  HEAD 2: `damage_by_hero(log: &[Hit]) -> BTreeMap<String, u32>`
//        Tip: `fold(BTreeMap::new(), |mut map, hit| { ...; map })`
//
//   ⚔️  HEAD 3: `top_hero(log: &[Hit]) -> Option<String>`
//        The hero with the most TOTAL damage. (Use head 2!)
//        Tip: `.into_iter().max_by_key(|(_, dmg)| *dmg)`
//
//   ⚔️  HEAD 4: `crit_report(log: &[Hit]) -> Vec<String>`
//        For critical hits only, in order: "Ferris crit for 40"
//
//   ⚔️  HEAD 5: `average_damage(log: &[Hit]) -> Option<f64>`
//        None for an empty log. Convert with `as f64`.
//
//   💎 BONUS: `struct Segments { .. }`: an infinite iterator of serpent
//        segment sizes: starts at `start`, each next segment is
//        (previous * 3) % 100 + 1.
//        `Segments::new(start: u32)`  →  Segments::new(5): 5, 16, 49, 48, 45, ...

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Hit {
    hero: String,
    damage: u32,
    critical: bool,
}

fn hit(hero: &str, damage: u32, critical: bool) -> Hit {
    Hit { hero: hero.to_string(), damage, critical }
}

// Your code goes here ↓



fn main() {
    println!("The river churns. Run the tests to face the Serpent!");
}

// ─── The Serpent's tests: don't change these! ────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn log() -> Vec<Hit> {
        vec![
            hit("Ferris", 10, false),
            hit("Corro", 25, false),
            hit("Ferris", 40, true),
            hit("Corro", 5, false),
            hit("Lumi", 30, true),
        ]
    }

    #[test]
    fn head_1_total() {
        assert_eq!(total_damage(&log()), 110);
        assert_eq!(total_damage(&[]), 0);
    }

    #[test]
    fn head_2_by_hero() {
        let m = damage_by_hero(&log());
        let v: Vec<(&str, u32)> = m.iter().map(|(k, v)| (k.as_str(), *v)).collect();
        assert_eq!(v, vec![("Corro", 30), ("Ferris", 50), ("Lumi", 30)]);
    }

    #[test]
    fn head_3_top() {
        assert_eq!(top_hero(&log()), Some(String::from("Ferris")));
        assert_eq!(top_hero(&[]), None);
    }

    #[test]
    fn head_4_crits() {
        assert_eq!(crit_report(&log()), vec!["Ferris crit for 40", "Lumi crit for 30"]);
    }

    #[test]
    fn head_5_average() {
        assert_eq!(average_damage(&log()), Some(22.0));
        assert_eq!(average_damage(&[]), None);
    }

    #[test]
    fn bonus_segments() {
        let v: Vec<u32> = Segments::new(5).take(5).collect();
        assert_eq!(v, vec![5, 16, 49, 48, 45]);
        assert_eq!(Segments::new(1).skip(2).next(), Some(13)); // 1, 4, 13
    }
}
