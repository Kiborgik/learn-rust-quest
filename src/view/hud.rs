//! The always-on parts of the screen: header, quest card, quest pips and
//! the test meter.

use crate::exercises::{self, Exercise, Lesson};
use crate::progress::Progress;
use crate::rewards::{RANKS, rank_index};
use crate::runner::{self, test_counts};
use crate::ui::*;
use crate::workspace;

/// Rank, XP, quests and streak, then the bar to the next rank.
pub fn header(p: &Progress) {
    let i = rank_index(p.xp);
    let rank = &RANKS[i];
    let total = exercises::all().count();
    let streak = if p.streak > 1 {
        format!(" · {}", yellow(format!("🔥{}", p.streak)))
    } else {
        String::new()
    };
    println!();
    println!(
        "  {} · {} · {} · {}{streak}",
        bold(cyan("🦀 RUST QUEST")),
        bold(rank.name),
        yellow(format!("{} XP", p.xp)),
        dim(format!("{}/{total}", p.done.len()))
    );
    match RANKS.get(i + 1) {
        Some(n) => println!(
            "  {} {}",
            bar(p.xp - rank.xp, n.xp - rank.xp, 20),
            dim(format!("{} XP to {}", n.xp - p.xp, n.name))
        ),
        None => println!("  {}", bold(yellow("★ MAX RANK ★"))),
    }
    rule();
}

/// One pip per quest: done, current, still to do. The boss is a diamond.
pub fn pips(lesson: &Lesson, p: &Progress, current: Option<&str>) -> String {
    lesson
        .exercises
        .iter()
        .map(|e| {
            let state = if p.done.contains(e.id) {
                0
            } else if Some(e.id) == current {
                1
            } else {
                2
            };
            // [done, current, to do], fancy and plain.
            let (fancy, ascii) = if e.is_boss() {
                (["◆", "◆", "◇"], ["#", ">", "B"])
            } else {
                (["●", "◉", "○"], ["#", ">", "-"])
            };
            if plain() {
                return ascii[state].to_string();
            }
            let todo: Style = if e.is_boss() { red } else { dim };
            let styles: [Style; 3] = [green, yellow, todo];
            styles[state](fancy[state].to_string())
        })
        .collect()
}

/// Region and pips, the quest, its story, the file to open, any hints.
pub fn quest_card(ex: &Exercise, p: &Progress, hints_shown: usize) {
    let lesson = exercises::lesson(ex.lesson);
    if let Some(lesson) = lesson {
        println!(
            "  {} {}  {}",
            dim(format!("{} ·", lesson.number)),
            lesson.region(),
            pips(lesson, p, Some(ex.id))
        );
        println!();
    }
    let label = if ex.is_boss() {
        bold(red("👹 BOSS"))
    } else {
        bold(cyan(format!("Quest {}", ex.code)))
    };
    println!(
        "  {label}  {}  {}",
        bold(ex.title),
        yellow(format!("⭐{} XP", ex.xp))
    );
    print_wrapped(2, ex.story, dim);
    println!();
    let file = runner::root().join(ex.path);
    println!("  📜 {}", file_link(&file, bold(blue(ex.path))));
    let notes = lesson.map_or(String::new(), |l| {
        format!(
            " · {}",
            file_link(&runner::root().join(l.notes), "📖 notes")
        )
    });
    println!("  {}", dim(format!("fix it, save, it's checked{notes}")));
    if workspace::has_update(ex) {
        println!();
        print_wrapped(
            2,
            "⬆ This quest was improved since you started it. Type restore \
             for the new version (yours is backed up first).",
            cyan,
        );
    }
    for (i, hint) in ex.hints.iter().take(hints_shown).enumerate() {
        println!();
        println!("  {}", yellow(format!("💡 Hint {}", i + 1)));
        print_wrapped(5, hint, as_is);
    }
    rule();
}

/// After failing tests: a boss HP bar, or a "tests passing" meter.
/// Returns the boss's HP as (heads left, total).
pub fn test_meter(ex: &Exercise, output: &str) -> Option<(u32, u32)> {
    let (passed, failed) = test_counts(output)?;
    let total = passed + failed;
    if ex.is_boss() {
        println!(
            "\n  {} {} {}",
            bold(red("👹 Boss HP")),
            bar_with(failed, total, 20, red),
            dim(format!("{failed}/{total} heads left"))
        );
        Some((failed, total))
    } else {
        println!(
            "\n  {} {} {}",
            bold("Tests"),
            bar(passed, total, 20),
            dim(format!("{passed}/{total} passing"))
        );
        None
    }
}
