//! Big moments: welcome, entering a region, bosses, rewards, rank ups,
//! victory. The flavor text is data at the top, so it's easy to extend.

use crate::exercises::{self, Exercise, LESSONS};
use crate::progress::Progress;
use crate::rewards::{Achievement, RANKS};
use crate::ui::*;

/// Shown when a build fails, picked by attempt number so it varies.
pub const COMPILE_LINES: &[&str] = &[
    "The compiler blocks your path!",
    "The compiler raises its shield.",
    "Not yet. The compiler has notes.",
    "Read the error closely. The fix is often in it.",
    "A wild compile error appears!",
];

/// Shown when it builds but the check fails.
pub const FAIL_LINES: &[&str] = &[
    "So close! It compiles, but...",
    "It runs! It just isn't right yet.",
    "Almost. Check the details.",
    "The code lives, but the quest isn't done.",
];

/// Victory shouts, picked by how many quests are done so they vary.
const PASS_LINES: &[&str] = &[
    "★ QUEST COMPLETE ★",
    "★ NAILED IT ★",
    "★ THE CRAB APPROVES ★",
    "★ CLEAN BUILD, CLEAN WIN ★",
    "★ THE COMPILER BOWS ★",
    "★ ANOTHER ONE FIXED ★",
];

const WELCOME: &str = "Each quest is a small Rust program that doesn't work yet. \
    Read the comments at the top, fix the code, save. The game checks it on every save.";

const WELCOME_TIPS: &[&str] = &[
    "Every region ends with a boss. Its HP drops as its tests pass.",
    "Bonus XP: quizzes, quests in a row without hints (combo), and your first quest each day.",
];

const CRAB: &str = r"
        _~^~^~_
    \) /  o o  \ (/
      '_   -   _'
      / '-----' \";

const BOSS_ART: &str = r#"
        /(.-""-.)\
    |\  \/      \/  /|
    | \ / =.  .= \ / |
    \( \   o\/o   / )/
     \_, '-/  \-' ,_/
       /   \__/   \
       \ \__/\__/ /
        \ \|--|/ /"#;

/// ASCII art, skipped in plain mode (screen readers would spell it out).
fn art(text: &str, style: Style) {
    if !plain() {
        println!("{}", style(text.to_string()));
    }
}

/// First-run screen (the caller prints the command list after it).
pub fn welcome() {
    clear();
    println!();
    art(CRAB, |s| bold(cyan(s)));
    println!("  {}", bold("Welcome to Rust Quest!"));
    println!();
    print_wrapped(2, WELCOME, as_is);
    println!();
}

pub fn welcome_tips() {
    println!();
    for tip in WELCOME_TIPS {
        print_wrapped(2, tip, dim);
    }
    if !plain() {
        print_wrapped(
            2,
            "Colors hard to read? Run `cargo run -- --plain` or set NO_COLOR=1.",
            dim,
        );
    }
    println!("\n  {}", bold(green("Press Enter to start your quest.")));
}

/// Above the quest card on its first check: a banner when you enter a new
/// region, and the boss itself before a boss fight.
pub fn entrance(ex: &Exercise, p: &Progress) {
    let Some(lesson) = exercises::lesson(ex.lesson) else {
        return;
    };
    if lesson.exercises.iter().all(|e| !p.done.contains(e.id)) {
        let xp: u32 = lesson.exercises.iter().map(|e| e.xp).sum();
        let boss = lesson.boss().map_or("?", |e| e.title);
        println!(
            "  {}",
            bold(magenta(format!("🗺  NEW REGION · {}", lesson.region())))
        );
        print_wrapped(2, lesson.topic(), dim);
        let facts = format!("{} quests · {xp} XP · boss: {boss}", lesson.exercises.len());
        print_wrapped(2, &facts, dim);
        rule();
    }
    if ex.is_boss() {
        art(BOSS_ART, red);
        println!("  {}", bold(red("⚠ A BOSS BLOCKS THE ROAD ⚠")));
        rule();
    }
}

/// The shout when a quest passes: a boss's fall, or a victory line.
pub fn victory_shout(ex: &Exercise, boss_hp: Option<(u32, u32)>, done: usize) {
    if !ex.is_boss() {
        println!("  {}", bold(green(PASS_LINES[done % PASS_LINES.len()])));
        return;
    }
    if let Some((left, total)) = boss_hp {
        drain_bar("  👹 Boss HP ", left, total, red);
    }
    rule();
    art(CRAB, |s| bold(magenta(s)));
    let fallen = format!("⚔ {} HAS FALLEN! ⚔", ex.title.to_uppercase());
    print_wrapped(2, &fallen, |s| bold(magenta(s)));
    rule();
}

/// One line per bonus that applies: `🔥 combo x3 +10 XP`.
pub fn bonus_lines(bonuses: &[(bool, String, u32)]) {
    for (_, label, xp) in bonuses.iter().filter(|b| b.0) {
        let xp = if *xp > 0 {
            yellow(format!(" +{xp} XP"))
        } else {
            String::new()
        };
        println!("  {}{xp}", bold(cyan(label)));
    }
}

pub fn achievement_unlocked(a: &Achievement) {
    println!();
    println!(
        "  {} {}  {}",
        bold(yellow("🏆 ACHIEVEMENT")),
        bold(&a.name),
        yellow(format!("+{} XP", a.xp))
    );
    print_wrapped(5, &a.desc, dim);
}

pub fn rank_up(index: usize) {
    println!();
    rule();
    println!("  {}", bold(magenta("▲ ▲ ▲   RANK UP   ▲ ▲ ▲")));
    println!("  You are now a {}", bold(cyan(RANKS[index].name)));
    if let Some(next) = RANKS.get(index + 1) {
        println!(
            "  {}",
            dim(format!("Next: {} at {} XP", next.name, next.xp))
        );
    }
    rule();
}

/// After a boss: region cleared, overall progress, the next region.
pub fn region_cleared(number: u32, p: &Progress) {
    let Some(pos) = LESSONS.iter().position(|l| l.number == number) else {
        return;
    };
    let total = exercises::all().count();
    println!();
    println!(
        "  {} {}",
        bold(green("✔ REGION CLEARED:")),
        bold(LESSONS[pos].region())
    );
    println!(
        "  {} {}",
        bar(p.done.len() as u32, total as u32, 20),
        dim(format!("{}/{total} quests", p.done.len()))
    );
    if let Some(next) = LESSONS.get(pos + 1) {
        println!("  {} {}", dim("Next region:"), bold(next.region()));
        print_wrapped(2, next.topic(), dim);
    }
}

/// The very end, above the final map.
pub fn victory() {
    art(CRAB, |s| bold(magenta(s)));
    println!("  {}", bold(magenta("🎉 EVERY QUEST COMPLETE 🎉")));
    println!();
}

pub fn next_steps() {
    println!();
    print_wrapped(
        2,
        "Next: build something real with `cargo new`. Ideas to explore: procedural \
         macros (syn, quote), embedded and no_std, WebAssembly, or games with bevy.",
        as_is,
    );
    println!();
}

pub fn goodbye(quests: usize, xp: u32, streak: u32) {
    println!();
    if quests > 0 {
        println!(
            "  {} {} {}",
            bold("This session:"),
            green(format!("{quests} quest(s)")),
            yellow(format!("+{xp} XP"))
        );
    }
    if streak > 1 {
        let line = format!("🔥 {streak}-day streak. Come back tomorrow to keep it!");
        print_wrapped(2, &line, yellow);
    }
    println!(
        "\n  {} Progress saved. {} to continue.\n",
        cyan("🦀"),
        bold("cargo run")
    );
}
