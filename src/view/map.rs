//! The world map: every region with its pips, the current one opened up,
//! then the milestones.

use super::pips;
use crate::exercises::{self, LESSONS, Lesson};
use crate::progress::Progress;
use crate::rewards::{Achievement, achievements};
use crate::runner;
use crate::ui::*;

pub fn map(p: &Progress) {
    let next = exercises::all().find(|e| !p.done.contains(e.id));
    let next_id = next.map(|e| e.id);
    let current = next.map(|e| e.lesson);
    let achievements = achievements();
    let widest = LESSONS.iter().map(|l| l.exercises.len()).max().unwrap_or(0);

    println!("  {}", bold("🗺  WORLD MAP"));
    println!();
    for lesson in LESSONS {
        let mine: Vec<&Achievement> = achievements
            .iter()
            .filter(|a| a.lesson == Some(lesson.number))
            .collect();
        let here = Some(lesson.number) == current;
        region_line(lesson, p, next_id, here, widest, &mine);
        if here {
            region_details(lesson, p, next_id, &mine);
        }
    }
    println!();
    println!("  {}", bold("🏆 MILESTONES"));
    for a in achievements.iter().filter(|a| a.lesson.is_none()) {
        achievement(p, a);
    }
}

/// `✔ 12 ●●●●●●◆ The Hive of Threads  🏆2`
fn region_line(
    lesson: &Lesson,
    p: &Progress,
    next_id: Option<&str>,
    here: bool,
    widest: usize,
    mine: &[&Achievement],
) {
    let done = lesson
        .exercises
        .iter()
        .filter(|e| p.done.contains(e.id))
        .count();
    let status = if done == lesson.exercises.len() {
        green("✔")
    } else if here {
        yellow("▶")
    } else {
        dim("·")
    };
    let won = mine
        .iter()
        .filter(|a| p.achievements.contains(&a.id))
        .count();
    let trophies = if won > 0 {
        yellow(format!("  🏆{won}"))
    } else {
        String::new()
    };
    let region = lesson.region();
    let region = if here {
        bold(region)
    } else if done == 0 {
        dim(region)
    } else {
        region.to_string()
    };
    let pad = " ".repeat(widest - lesson.exercises.len());
    println!(
        "  {status} {} {}{pad} {region}{trophies}",
        dim(format!("{:>2}", lesson.number)),
        pips(lesson, p, next_id)
    );
}

/// The opened-up current region: topic, quests with badges, notes, trophies.
fn region_details(lesson: &Lesson, p: &Progress, next_id: Option<&str>, mine: &[&Achievement]) {
    print_wrapped(7, lesson.topic(), dim);
    for e in lesson.exercises {
        let (mark, title) = if p.done.contains(e.id) {
            (green("✔"), e.title.to_string())
        } else if Some(e.id) == next_id {
            (yellow("▶"), bold(e.title))
        } else {
            (dim("·"), dim(e.title))
        };
        let badges: String = [
            (p.quizzes_passed.contains(e.id), " 🧠"),
            (p.first_try.contains(e.id), " 🎯"),
        ]
        .iter()
        .filter(|b| b.0)
        .map(|b| b.1)
        .collect();
        println!(
            "       {mark} {} {title}{}",
            dim(format!("{:<4}", e.code)),
            yellow(badges)
        );
    }
    let notes = runner::root().join(lesson.notes);
    println!("       {}", file_link(&notes, dim("📖 lesson notes")));
    for a in mine {
        achievement(p, a);
    }
    println!();
}

fn achievement(p: &Progress, a: &Achievement) {
    let (icon, style): (&str, Style) = if p.achievements.contains(&a.id) {
        ("🏆", yellow)
    } else {
        ("🔒", dim)
    };
    println!("       {icon} {}", style(a.name.clone()));
    print_wrapped(10, &a.desc, dim);
}
