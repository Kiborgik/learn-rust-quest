//! Everything the player can earn, as data: XP numbers, ranks and
//! achievements, plus the rules that decide when they're earned.
//! Tune the game's balance here.

use crate::exercises::{self, Exercise, LESSONS};
use crate::progress::Progress;

// ── XP ───────────────────────────────────────────────────────────────────────

/// XP for answering a quest's bonus quiz right.
pub const QUIZ_XP: u32 = 30;
/// Bonus XP per combo step after the first quest, capped.
pub const COMBO_STEP_XP: u32 = 5;
pub const COMBO_MAX_XP: u32 = 25;
/// First quest of the day: base bonus, plus this much per streak day, capped.
pub const DAILY_XP: u32 = 20;
pub const DAILY_STREAK_XP: u32 = 5;
pub const DAILY_MAX_XP: u32 = 50;

pub fn combo_xp(combo: u32) -> u32 {
    (combo.saturating_sub(1) * COMBO_STEP_XP).min(COMBO_MAX_XP)
}

pub fn daily_xp(streak: u32) -> u32 {
    (DAILY_XP + DAILY_STREAK_XP * streak.saturating_sub(1)).min(DAILY_MAX_XP)
}

// ── Ranks ────────────────────────────────────────────────────────────────────

pub struct Rank {
    pub xp: u32,
    pub name: &'static str,
}

const fn rank(xp: u32, name: &'static str) -> Rank {
    Rank { xp, name }
}

pub const RANKS: &[Rank] = &[
    rank(0, "Crab Hatchling"),
    rank(200, "Tiny Crab"),
    rank(450, "Shell Apprentice"),
    rank(800, "Loop Wrangler"),
    rank(1300, "Borrow Squire"),
    rank(2000, "Ferris Knight"),
    rank(3000, "Pattern Ranger"),
    rank(4200, "Error Tamer"),
    rank(5600, "Trait Mage"),
    rank(7200, "Iterator Sage"),
    rank(9000, "Lifetime Lord"),
    rank(11000, "Pointer Artificer"),
    rank(13000, "Fearless Concurrent"),
    rank(15000, "Crate Smith"),
    rank(18000, "Data Alchemist"),
    rank(21000, "Command Herald"),
    rank(24500, "Thread Weaver"),
    rank(28000, "Async Voyager"),
    rank(31500, "Web Artisan"),
    rank(35000, "Guild Founder"),
    rank(38000, "Spellwright"),
    rank(41000, "Rustacean Legend"),
];

/// Index into `RANKS` for this much XP.
pub fn rank_index(xp: u32) -> usize {
    RANKS.iter().rposition(|r| xp >= r.xp).unwrap_or(0)
}

// ── Achievements ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub enum Rule {
    /// Complete any quest.
    FirstQuest,
    /// Beat the boss of lesson N.
    Boss(u32),
    /// Finish lesson N without revealing any hint.
    NoHints(u32),
    /// Answer every quiz of lesson N correctly.
    AllQuizzes(u32),
    /// Finish lessons 1..=N.
    LessonsDone(u32),
    /// Play N days in a row.
    Streak(u32),
    /// Pass N quests on the first check, without a hint.
    FirstTry(u32),
}

pub struct Achievement {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub xp: u32,
    pub rule: Rule,
    /// `Some(n)` = shown under lesson n on the map, `None` = global.
    pub lesson: Option<u32>,
}

/// Global achievements: (id, name, description, XP, rule).
/// `LessonsDone(0)` means "every lesson".
const MILESTONES: &[(&str, &str, &str, u32, Rule)] = &[
    (
        "first_blood",
        "First Blood",
        "Complete your first quest",
        25,
        Rule::FirstQuest,
    ),
    (
        "streak_3",
        "On a Roll",
        "Play 3 days in a row",
        50,
        Rule::Streak(3),
    ),
    (
        "streak_7",
        "Habit Forged",
        "Play 7 days in a row",
        150,
        Rule::Streak(7),
    ),
    (
        "first_try_10",
        "Sharpshooter",
        "Pass 10 quests on the first check, no hints",
        100,
        Rule::FirstTry(10),
    ),
    (
        "first_try_50",
        "Compiler Whisperer",
        "Pass 50 quests on the first check, no hints",
        300,
        Rule::FirstTry(50),
    ),
    (
        "halfway",
        "Halfway Hero",
        "Finish Lessons 1 to 6 (the basics)",
        150,
        Rule::LessonsDone(6),
    ),
    (
        "core_complete",
        "Core Forged",
        "Finish Lessons 1 to 12 (all of core Rust)",
        300,
        Rule::LessonsDone(12),
    ),
    (
        "capstone",
        "Ship It",
        "Finish Lessons 1 to 26, capstone included",
        400,
        Rule::LessonsDone(26),
    ),
    (
        "true_rustacean",
        "True Rustacean",
        "Finish every lesson",
        500,
        Rule::LessonsDone(0),
    ),
];

/// Every achievement: three per lesson (boss, no hints, all quizzes), then
/// the milestones.
pub fn achievements() -> Vec<Achievement> {
    let mut list = Vec::new();
    for l in LESSONS {
        let n = l.number;
        let boss = l.boss().map_or("the boss", |e| e.title);
        let per_lesson = [
            (
                l.boss_trophy.0.to_string(),
                l.boss_trophy.1,
                format!("Defeat {boss}"),
                50,
                Rule::Boss(n),
            ),
            (
                format!("no_hints_l{n}"),
                "Lone Wolf",
                format!("Finish Lesson {n} without revealing a single hint"),
                75,
                Rule::NoHints(n),
            ),
            (
                format!("big_brain_l{n}"),
                "Big Brain",
                format!("Answer every Lesson {n} quiz correctly"),
                75,
                Rule::AllQuizzes(n),
            ),
        ];
        for (id, name, desc, xp, rule) in per_lesson {
            list.push(Achievement {
                id,
                name: name.into(),
                desc,
                xp,
                rule,
                lesson: Some(n),
            });
        }
    }
    for &(id, name, desc, xp, rule) in MILESTONES {
        let rule = match rule {
            Rule::LessonsDone(0) => Rule::LessonsDone(LESSONS.len() as u32),
            r => r,
        };
        list.push(Achievement {
            id: id.into(),
            name: name.into(),
            desc: desc.into(),
            xp,
            rule,
            lesson: None,
        });
    }
    list
}

/// Has the player satisfied this achievement rule?
pub fn earned(p: &Progress, rule: Rule) -> bool {
    // Does every exercise of lesson n (matching `pick`) pass `test`?
    let lesson_all = |n: u32, pick: fn(&Exercise) -> bool, test: &dyn Fn(&Exercise) -> bool| {
        exercises::lesson(n).is_some_and(|l| l.exercises.iter().filter(|e| pick(e)).all(test))
    };
    let any = |_: &Exercise| true;
    let done = |e: &Exercise| p.done.contains(e.id);
    match rule {
        Rule::FirstQuest => !p.done.is_empty(),
        Rule::Boss(n) => lesson_all(n, Exercise::is_boss, &done),
        Rule::NoHints(n) => lesson_all(n, any, &|e| done(e) && !p.hints_used.contains(e.id)),
        Rule::AllQuizzes(n) => lesson_all(n, any, &|e| p.quizzes_passed.contains(e.id)),
        Rule::LessonsDone(n) => (1..=n).all(|n| lesson_all(n, any, &done)),
        Rule::Streak(n) => p.best_streak >= n,
        Rule::FirstTry(n) => p.first_try.len() >= n as usize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn achievement_ids_are_unique() {
        let mut ids: Vec<String> = achievements().into_iter().map(|a| a.id).collect();
        let n = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), n);
    }

    #[test]
    fn ranks_go_up() {
        assert!(RANKS.windows(2).all(|w| w[0].xp < w[1].xp));
        assert_eq!(rank_index(0), 0);
        assert_eq!(rank_index(u32::MAX), RANKS.len() - 1);
    }

    #[test]
    fn bonuses_are_capped() {
        assert_eq!(combo_xp(1), 0);
        assert_eq!(combo_xp(3), 10);
        assert_eq!(combo_xp(100), COMBO_MAX_XP);
        assert_eq!(daily_xp(1), DAILY_XP);
        assert_eq!(daily_xp(100), DAILY_MAX_XP);
    }
}
