// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 5-6 · Guarded Gates                              125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   MATCH GUARDS add an extra `if` condition to an arm:
//
//     match hp {
//         0 => "dead",
//         x if x < 20 => "critical",     // binds x, then checks the guard
//         _ => "fine",
//     }
//
//   `@` BINDINGS test a pattern AND keep the value:
//
//     match age {
//         n @ 13..=19 => format!("teen ({n})"),
//         n => format!("{n}"),
//     }
//
//   TUPLE PATTERNS match several values at once:
//
//     match (is_day, has_torch) {
//         (true, _) => "can see",
//         (false, true) => "can see a bit",
//         (false, false) => "darkness",
//     }
//
//   STRUCT PATTERNS with `..` to ignore the other fields:
//
//     match hero {
//         Hero { hp: 0, .. } => "fallen",
//         Hero { level, .. } if *level > 10 => "elite",
//         _ => "ok",
//     }
//
// 🎯 YOUR TASK
//
//   1. `classify`: negative → "negative", 0 → "zero", other even numbers
//      → "even", odd → "odd". Use guards.
//   2. Fix `level_badge` (it can't find `l`).
//   3. Fix `judge`: a natural 20 must ALWAYS be "critical".
//   4. `status`: hp 0 → "fallen", otherwise mana 0 → "drained",
//      otherwise level >= 10 → "elite", otherwise "ready".

struct Hero {
    name: String,
    hp: u32,
    mana: u32,
    level: u32,
}

fn classify(n: i32) -> &'static str {
    todo!()
}

fn level_badge(level: u32) -> String {
    match level {
        0 => String::from("Unranked"),
        1..=9 => format!("Rookie {l}"),
        10..=49 => format!("Veteran {l}"),
        l => format!("Master {l}"),
    }
}

fn judge(roll: u32, lucky: bool) -> &'static str {
    match (roll, lucky) {
        (r, _) if r >= 15 => "hit",
        (20, _) => "critical",
        (r, true) if r >= 10 => "lucky hit",
        (1, false) => "fumble",
        _ => "miss",
    }
}

fn status(hero: &Hero) -> &'static str {
    todo!()
}

fn main() {
    println!("{}", classify(-3));
    println!("{}", level_badge(12));
    println!("{}", judge(20, false));
    let h = Hero { name: String::from("Ferris"), hp: 10, mana: 0, level: 3 };
    println!("{} is {}", h.name, status(&h));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifying() {
        assert_eq!(classify(-3), "negative");
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(4), "even");
        assert_eq!(classify(7), "odd");
    }

    #[test]
    fn badges() {
        assert_eq!(level_badge(0), "Unranked");
        assert_eq!(level_badge(5), "Rookie 5");
        assert_eq!(level_badge(10), "Veteran 10");
        assert_eq!(level_badge(99), "Master 99");
    }

    #[test]
    fn judging() {
        assert_eq!(judge(20, false), "critical");
        assert_eq!(judge(20, true), "critical");
        assert_eq!(judge(16, false), "hit");
        assert_eq!(judge(12, true), "lucky hit");
        assert_eq!(judge(12, false), "miss");
        assert_eq!(judge(1, false), "fumble");
        assert_eq!(judge(1, true), "miss");
    }

    fn hero(hp: u32, mana: u32, level: u32) -> Hero {
        Hero { name: String::from("T"), hp, mana, level }
    }

    #[test]
    fn statuses() {
        assert_eq!(status(&hero(0, 0, 20)), "fallen");
        assert_eq!(status(&hero(5, 0, 20)), "drained");
        assert_eq!(status(&hero(5, 5, 20)), "elite");
        assert_eq!(status(&hero(5, 5, 2)), "ready");
    }
}
