// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 22-2 · Facts, Not Sentences                      175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Put data in FIELDS, not in the message text. Tools can then search
//   and graph them ("all events where hp < 10"):
//
//     info!(hero = name, hp, "took damage");      // hero="Ferris" hp=40 took damage
//
//     info!(hp, "...")              // shorthand for hp = hp
//     info!(hero = %name, "...")    // % → record with Display:  hero=Ferris
//     info!(gear = ?items, "...")   // ? → record with Debug:    gear=["rope"]
//
//   With the text formatter, a field recorded as a plain &str shows
//   with quotes (hero="Ferris"); with `%` it shows bare (hero=Ferris).
//
//   Compare:
//     info!("Ferris took 12 damage, hp now 40");       // just a sentence
//     info!(hero = %name, damage, hp, "took damage");  // searchable facts
//
// 🎯 YOUR TASK
//
//   Rewrite the event in `take_damage` with fields. The tests expect,
//   in this order:  hero=Ferris damage=12 hp=40  and the message
//   "took damage". Use `%` for the hero's name.

pub mod capture;

use tracing::info;

pub struct Hero {
    pub name: String,
    pub hp: u32,
}

pub fn take_damage(hero: &mut Hero, damage: u32) {
    hero.hp = hero.hp.saturating_sub(damage);
    info!("{} took {} damage, hp now {}", hero.name, damage, hero.hp);
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::capture;

    #[test]
    fn structured() {
        let mut h = Hero { name: "Ferris".into(), hp: 52 };
        let log = capture(|| take_damage(&mut h, 12));
        assert_eq!(h.hp, 40);
        assert!(log.contains("took damage"), "{log}");
        assert!(log.contains("hero=Ferris damage=12 hp=40"), "{log}");
    }
}
