// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Silent Saboteur                           400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   Someone sabotages the dungeon runs, and nobody knows where, because
//   the dungeon code logs nothing. Instrument it so the saboteur has
//   nowhere to hide.
//
//   `run_dungeon(hero, rooms)` walks through rooms in order:
//     • "trap"     → hp -= 30 (saturating)
//     • "potion"   → hp += 20 (max 100)
//     • "treasure" → gold += 50
//     • anything else is an unknown room: stop with Err("unknown room: X")
//   Stop early with Err("{hero} fell in room {n}") if hp hits 0.
//   Rooms are numbered from 1. The hero starts with 100 hp, 0 gold.
//   Success: Ok(gold).
//
//   ⚔️  HEAD 1: the logic above (tests check the results)
//
//   ⚔️  HEAD 2: the whole run is inside a span `run_dungeon{hero=Ferris}`
//        Tip: `#[instrument(skip_all, fields(hero = %hero))]` skips the
//        automatic argument fields and records `hero` with Display.
//
//   ⚔️  HEAD 3: every room is a nested span `room{n=1 kind=trap}`
//
//   ⚔️  HEAD 4: events, each with fields:
//        trap      → warn  "trap sprung"     with field hp (after damage)
//        potion    → info  "potion found"    with field hp
//        treasure  → info  "treasure found"  with field gold
//        unknown   → error "unknown room"
//        hp 0      → error "hero fell"
//
//   ⚔️  HEAD 5: success → info "run complete" with field gold (logged in
//        the run span, outside any room)

pub mod capture;

use tracing::{error, info, info_span, instrument, warn};

pub fn run_dungeon(hero: &str, rooms: &[&str]) -> Result<u32, String> {
    todo!()
}

// ─── The Saboteur's tests: don't change these! ───────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::capture;

    #[test]
    fn head_1_logic() {
        assert_eq!(run_dungeon("Ferris", &["treasure", "trap", "potion", "treasure"]), Ok(100));
        assert_eq!(
            run_dungeon("Ferris", &["trap", "trap", "trap", "trap", "treasure"]),
            Err("Ferris fell in room 4".into())
        );
        assert_eq!(run_dungeon("Ferris", &["treasure", "lava"]), Err("unknown room: lava".into()));
    }

    #[test]
    fn head_2_to_5_logs() {
        let log = capture(|| {
            let _ = run_dungeon("Ferris", &["trap", "potion", "treasure"]);
        });
        let lines: Vec<&str> = log.lines().collect();
        assert_eq!(lines.len(), 4, "{log}");
        assert!(lines[0].contains("WARN"), "{log}");
        assert!(lines[0].contains("run_dungeon{hero=Ferris}:room{n=1 kind=trap}: trap sprung hp=70"), "{log}");
        assert!(lines[1].contains("room{n=2 kind=potion}: potion found hp=90"), "{log}");
        assert!(lines[2].contains("room{n=3 kind=treasure}: treasure found gold=50"), "{log}");
        assert!(lines[3].contains("run_dungeon{hero=Ferris}: run complete gold=50"), "{log}");
        assert!(!lines[3].contains("room{"), "{log}");
    }

    #[test]
    fn errors_are_logged() {
        let log = capture(|| {
            let _ = run_dungeon("Ferris", &["trap", "trap", "trap", "trap"]);
            let _ = run_dungeon("Ferris", &["lava"]);
        });
        assert!(log.contains("ERROR"), "{log}");
        assert!(log.contains("room{n=4 kind=trap}: hero fell"), "{log}");
        assert!(log.contains("room{n=1 kind=lava}: unknown room"), "{log}");
    }
}
