// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Guildmaster's Trial                        250 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   To become a full guild member you must design your own adventurer
//   record from scratch. The Guildmaster tests every method personally.
//
//   Write a struct `Adventurer` with PUBLIC-looking fields (tests read
//   them directly): name: String, level: u32, hp: u32, max_hp: u32, xp: u32
//   It needs Debug, Clone and PartialEq.
//
//   ⚔️  HEAD 1: `Adventurer::new(name: &str) -> Self`
//        level 1, hp 100, max_hp 100, xp 0
//
//   ⚔️  HEAD 2: `take_damage(&mut self, amount: u32)` and `is_alive(&self)`
//        hp never goes below 0 (careful, u32 can't be negative!
//        `a.saturating_sub(b)` stops at 0 instead of crashing.)
//
//   ⚔️  HEAD 3: `heal(&mut self, amount: u32)`
//        hp never goes above max_hp (`.min()` is handy: 5.min(3) == 3)
//
//   ⚔️  HEAD 4: `gain_xp(&mut self, amount: u32)`
//        Reaching the next level costs `level * 100` xp (leftover xp is
//        kept). Each level-up: level += 1, max_hp += 10, hp = max_hp.
//        A big xp gain can trigger SEVERAL level-ups: use a loop!
//
//   ⚔️  HEAD 5: `title(&self) -> String`
//        level 1..=4 → "Ferris the Novice", 5..=9 → "Ferris the Veteran",
//        10+ → "Ferris the Legend"   (using the adventurer's own name)
//
//   Your code goes here ↓



fn main() {
    println!("The Guildmaster awaits. Run the tests to begin the trial!");
}

// ─── The Guildmaster's tests: don't change these! ────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_1_new() {
        let a = Adventurer::new("Ferris");
        assert_eq!(a.name, "Ferris");
        assert_eq!((a.level, a.hp, a.max_hp, a.xp), (1, 100, 100, 0));
    }

    #[test]
    fn head_2_damage() {
        let mut a = Adventurer::new("Ferris");
        a.take_damage(30);
        assert_eq!(a.hp, 70);
        assert!(a.is_alive());
        a.take_damage(500);
        assert_eq!(a.hp, 0);
        assert!(!a.is_alive());
    }

    #[test]
    fn head_3_heal() {
        let mut a = Adventurer::new("Ferris");
        a.take_damage(50);
        a.heal(20);
        assert_eq!(a.hp, 70);
        a.heal(1000);
        assert_eq!(a.hp, 100);
    }

    #[test]
    fn head_4_level_up() {
        let mut a = Adventurer::new("Ferris");
        a.take_damage(60);
        a.gain_xp(250); // 100 for level 2, 150 left (level 3 needs 200)
        assert_eq!((a.level, a.xp, a.max_hp, a.hp), (2, 150, 110, 110));
        a.gain_xp(50);
        assert_eq!((a.level, a.xp, a.max_hp), (3, 0, 120));
        a.gain_xp(700); // 300 → level 4, 400 → level 5
        assert_eq!((a.level, a.xp), (5, 0));
    }

    #[test]
    fn head_5_titles() {
        let mut a = Adventurer::new("Ferris");
        assert_eq!(a.title(), "Ferris the Novice");
        a.level = 7;
        assert_eq!(a.title(), "Ferris the Veteran");
        a.level = 12;
        assert_eq!(a.title(), "Ferris the Legend");
    }

    #[test]
    fn bonus_snapshot() {
        let mut a = Adventurer::new("Ferris");
        let snapshot = a.clone();
        assert_eq!(snapshot, a);
        a.take_damage(1);
        assert_ne!(snapshot, a);
    }
}
