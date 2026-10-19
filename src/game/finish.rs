//! Passing a quest: XP, bonuses, quiz, achievements, rank ups.

use super::commands::Command;
use super::{Flow, Game};
use crate::exercises::Exercise;
use crate::rewards::{self, achievements, earned, rank_index};
use crate::runner::test_counts;
use crate::ui::*;
use crate::view;

/// What happened on the quest's way to passing.
pub(super) struct Win<'a> {
    pub output: &'a str,
    pub warnings: &'a str,
    /// Passed on the first check of this session, no hint shown.
    pub first_try: bool,
    /// The boss's HP before the winning check, for the drain animation.
    pub boss_hp: Option<(u32, u32)>,
}

impl Game {
    pub(super) fn finish(&mut self, ex: &Exercise, win: Win) -> Flow {
        let rank_before = rank_index(self.progress.xp);
        let used_hint = self.progress.hints_used.contains(ex.id);
        let reward = ex.reward(used_hint);
        let first_try = win.first_try && !used_hint;
        self.session.combo = if used_hint { 0 } else { self.session.combo + 1 };

        // (applies, label, XP). Add new bonuses here.
        let combo_xp = rewards::combo_xp(self.session.combo);
        let daily_xp = if self.progress.claim_daily_bonus() {
            rewards::daily_xp(self.progress.streak)
        } else {
            0
        };
        let bonuses = [
            (first_try, "🎯 first try".to_string(), 0),
            (
                combo_xp > 0,
                format!("🔥 combo x{}", self.session.combo),
                combo_xp,
            ),
            (daily_xp > 0, "☀ daily bonus".to_string(), daily_xp),
        ];
        let bonus_xp: u32 = bonuses.iter().filter(|b| b.0).map(|b| b.2).sum();

        self.progress.xp += reward + bonus_xp;
        self.progress.done.insert(ex.id.to_string());
        if first_try {
            self.progress.first_try.insert(ex.id.to_string());
        }
        self.progress.save();

        println!();
        view::victory_shout(ex, win.boss_hp, self.progress.done.len());
        count_up("  ", reward);
        if used_hint {
            print!("{}", dim(" (half: hint used)"));
        }
        println!();
        view::bonus_lines(&bonuses);
        show_result(ex, win.output, win.warnings);

        if let Some(quiz) = &ex.quiz
            && !self.progress.quizzes_answered.contains(ex.id)
            && !self.quiz(ex, quiz)
        {
            return Flow::Quit;
        }

        self.check_achievements();
        let rank_after = rank_index(self.progress.xp);
        if rank_after > rank_before {
            view::rank_up(rank_after);
        }
        if ex.is_boss() {
            view::region_cleared(ex.lesson, &self.progress);
        }

        println!("\n  {}", bold(green("Press Enter for the next quest")));
        println!("  {}", dim("(or q to quit)"));
        match self.input.recv() {
            Ok(s) if Command::parse(&s) == Some(Command::Quit) => Flow::Quit,
            Ok(_) => Flow::Next,
            Err(_) => Flow::Quit,
        }
    }

    /// Unlocks every achievement whose rule is now met, and pays it out.
    pub(super) fn check_achievements(&mut self) {
        for a in achievements() {
            if self.progress.achievements.contains(&a.id) || !earned(&self.progress, a.rule) {
                continue;
            }
            self.progress.achievements.insert(a.id.clone());
            self.progress.xp += a.xp;
            view::achievement_unlocked(&a);
        }
        self.progress.save();
    }
}

/// Test quests: how many tests pass. Output quests: what the program
/// printed. Then any compiler warnings.
fn show_result(ex: &Exercise, output: &str, warnings: &str) {
    if ex.check.is_test() {
        if let Some((passed, _)) = test_counts(output) {
            println!("  {}", green(format!("✔ all {passed} tests pass")));
        }
    } else if !output.trim().is_empty() {
        println!("\n  {}", dim("Program output:"));
        for line in output.trim().lines() {
            println!("  {} {line}", dim("│"));
        }
    }
    if !warnings.trim().is_empty() {
        println!();
        print_wrapped(
            2,
            "The compiler still has advice (warnings) for you:",
            yellow,
        );
        println!("{}", show(warnings.trim_end()));
    }
}
