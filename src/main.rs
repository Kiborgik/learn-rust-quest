//! 🦀 RUST QUEST: learn Rust by fixing small programs and earning XP.
//!
//!   cargo run                play (auto-checks every time you save an exercise)
//!   cargo run -- map         show your progress
//!   cargo run -- reset       start over: wipe progress and restore every exercise file
//!   cargo run -- --plain     no colors or animation (also: NO_COLOR=1)
//!
//! Layout: `exercises` + `lessons` are the catalogue, `rewards` the XP and
//! achievement rules, `game` the loop, `view` the screens, `ui` the
//! terminal toolkit, `runner` compiles and checks, `workspace` manages the
//! player's files, `progress` the save file.

mod exercises;
mod game;
mod lessons;
mod progress;
mod rewards;
mod runner;
mod ui;
mod versions;
mod view;
mod workspace;

use game::reset::{CANCELLED, full_reset, is_yes, print_reset_warning, report_reset};
use progress::Progress;
use std::{env, io};
use ui::{dim, green, red};

/// `cargo run -- <args>` usage, shown by `help`.
const CLI_HELP: &[(&str, &str)] = &[
    ("cargo run", "play"),
    ("cargo run -- map", "show your progress"),
    (
        "cargo run -- reset",
        "start over: wipe progress and restore all exercise files",
    ),
    (
        "",
        "(asks first; add --yes to skip; old files are backed up)",
    ),
    (
        "cargo run -- --plain",
        "no colors, animation or screen clearing (or set NO_COLOR=1)",
    ),
];

fn main() {
    let (flags, args): (Vec<String>, Vec<String>) =
        env::args().skip(1).partition(|a| a == "--plain");
    ui::init(!flags.is_empty());
    let synced = match workspace::init() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} {e}", red("Could not set up the exercises/ folder:"));
            return;
        }
    };
    if synced.created > 0 {
        let msg = format!("Set up {} exercise file(s) in exercises/.", synced.created);
        println!("{}", dim(msg));
    }
    match args.first().map(String::as_str) {
        Some("reset") => reset(args.get(1).is_some_and(|a| a == "--yes" || a == "-y")),
        Some("map" | "list") => {
            view::quests_updated(&synced.updated);
            let p = Progress::load();
            view::header(&p);
            view::map(&p);
        }
        Some("help" | "--help" | "-h") => {
            for (usage, what) in CLI_HELP {
                println!("{usage:<22} {what}");
            }
            println!("\nBackups go to {}/.", workspace::BACKUP_DIR);
        }
        _ => game::Game::new().run(&synced.updated),
    }
}

fn reset(skip_confirm: bool) {
    let confirmed = skip_confirm || {
        print_reset_warning(&Progress::load());
        let mut answer = String::new();
        let _ = io::stdin().read_line(&mut answer);
        is_yes(&answer)
    };
    if confirmed {
        report_reset(full_reset());
    } else {
        println!("{}", green(CANCELLED));
    }
}
