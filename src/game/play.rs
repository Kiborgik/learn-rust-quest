//! Playing one quest: draw, check, then wait for a save or a command.

use super::commands::{Command, print_commands, prompt};
use super::finish::Win;
use super::reset::{CANCELLED, ask_yes, full_reset, is_yes, print_reset_warning, report_reset};
use super::{Flow, Game, Session};
use crate::exercises::Exercise;
use crate::progress::Progress;
use crate::runner::{self, Outcome};
use crate::ui::*;
use crate::{view, workspace};
use std::path::Path;
use std::sync::mpsc::RecvTimeoutError;
use std::thread;
use std::time::{Duration, SystemTime};

/// How often to look for a saved file while waiting for input.
const POLL: Duration = Duration::from_millis(400);
/// Give the editor a moment to finish writing before checking.
const SETTLE: Duration = Duration::from_millis(150);

/// The state of one quest attempt.
struct Attempt {
    hints_shown: usize,
    checks: usize,
    /// The boss's HP after the last check: (heads left, total).
    boss_hp: Option<(u32, u32)>,
    last_seen: Option<SystemTime>,
}

/// What to do after a command.
enum Next {
    /// Draw the screen and check again.
    Recheck,
    /// Keep waiting for input.
    Wait,
    /// Leave this quest.
    Leave(Flow),
}

impl Game {
    pub(super) fn play(&mut self, ex: &Exercise) -> Flow {
        let path = runner::root().join(ex.path);
        let mut at = Attempt {
            hints_shown: 0,
            checks: 0,
            boss_hp: None,
            last_seen: mtime(&path),
        };
        loop {
            if let Some(flow) = self.check_once(ex, &mut at) {
                return flow;
            }
            loop {
                match self.input.recv_timeout(POLL) {
                    Ok(line) => match self.command(ex, &line, &mut at, &path) {
                        Next::Recheck => break,
                        Next::Wait => {}
                        Next::Leave(flow) => return flow,
                    },
                    Err(RecvTimeoutError::Timeout) => {
                        if mtime(&path) != at.last_seen {
                            thread::sleep(SETTLE);
                            at.last_seen = mtime(&path);
                            break;
                        }
                    }
                    Err(RecvTimeoutError::Disconnected) => return Flow::Quit,
                }
            }
        }
    }

    /// Draws the quest screen and checks the file once. Some(flow) if the
    /// quest was passed.
    fn check_once(&mut self, ex: &Exercise, at: &mut Attempt) -> Option<Flow> {
        clear();
        view::header(&self.progress);
        at.checks += 1;
        if at.checks == 1 {
            view::entrance(ex, &self.progress);
        }
        view::quest_card(ex, &self.progress, at.hints_shown);
        let doing = if ex.check.is_cargo() {
            "Building"
        } else {
            "Checking"
        };
        println!("  {}", dim(format!("⚙ {doing}... try #{}", at.checks)));
        if ex.check.is_cargo() && at.checks == 1 {
            println!(
                "  {}",
                dim("  (a quest's first build downloads its crates)")
            );
        }
        flush();

        let pick = |lines: &[&'static str]| lines[(at.checks - 1) % lines.len()];
        match runner::check(ex) {
            Outcome::Pass { output, warnings } => {
                let win = Win {
                    output: &output,
                    warnings: &warnings,
                    first_try: at.checks == 1 && at.hints_shown == 0,
                    boss_hp: at.boss_hp,
                };
                return Some(self.finish(ex, win));
            }
            Outcome::CompileError(err) => {
                println!(
                    "\n  {} {}\n",
                    red("✘"),
                    bold(red(pick(view::COMPILE_LINES)))
                );
                println!("{}", show(err.trim_end()));
            }
            Outcome::Fail(msg) => {
                println!(
                    "\n  {} {}\n",
                    yellow("✘"),
                    bold(yellow(pick(view::FAIL_LINES)))
                );
                println!("{}", show(msg.trim_end()));
                at.boss_hp = view::test_meter(ex, &msg);
            }
        }
        if at.checks >= 4 && at.hints_shown == 0 {
            println!();
            print_wrapped(
                2,
                "Stuck? Type h for a hint, or open the lesson notes.",
                cyan,
            );
        }
        prompt(ex, at.hints_shown);
        None
    }

    fn command(&mut self, ex: &Exercise, line: &str, at: &mut Attempt, path: &Path) -> Next {
        let Some(cmd) = Command::parse(line) else {
            println!(
                "{} Unknown command `{}`. Type ? for the list.",
                yellow("?"),
                line.trim()
            );
            prompt(ex, at.hints_shown);
            return Next::Wait;
        };
        match cmd {
            Command::Check => Next::Recheck,
            Command::Quit => Next::Leave(Flow::Quit),
            Command::Hint => {
                if at.hints_shown < ex.hints.len() {
                    at.hints_shown += 1;
                    if self.progress.hints_used.insert(ex.id.to_string()) {
                        self.progress.save();
                    }
                }
                Next::Recheck
            }
            Command::Map => {
                clear();
                view::header(&self.progress);
                view::map(&self.progress);
                println!("\n  {}", dim("Press Enter to go back."));
                match self.input.recv() {
                    Ok(_) => Next::Recheck,
                    Err(_) => Next::Leave(Flow::Quit),
                }
            }
            Command::Help => {
                println!();
                print_commands();
                prompt(ex, at.hints_shown);
                Next::Wait
            }
            Command::Restore => self.restore(ex, at, path),
            Command::Reset => self.reset(ex, at),
        }
    }

    fn restore(&mut self, ex: &Exercise, at: &mut Attempt, path: &Path) -> Next {
        println!();
        println!(
            "  {} this quest's files to the start?",
            bold(yellow("↺ RESTORE"))
        );
        let backup = format!(
            "Your version is backed up to {}/ first.",
            workspace::BACKUP_DIR
        );
        print_wrapped(2, &backup, dim);
        ask_yes();
        let Ok(answer) = self.input.recv() else {
            return Next::Leave(Flow::Quit);
        };
        if !is_yes(&answer) {
            println!("  {}", green(CANCELLED));
        } else {
            match workspace::restore(ex) {
                Ok(_) => {
                    at.last_seen = mtime(path);
                    return Next::Recheck;
                }
                Err(e) => println!("  {} {e}", red("Restore failed:")),
            }
        }
        prompt(ex, at.hints_shown);
        Next::Wait
    }

    fn reset(&mut self, ex: &Exercise, at: &Attempt) -> Next {
        print_reset_warning(&self.progress);
        let Ok(answer) = self.input.recv() else {
            return Next::Leave(Flow::Quit);
        };
        if !is_yes(&answer) {
            println!("  {}", green(CANCELLED));
        } else if report_reset(full_reset()) {
            self.progress = Progress::default();
            self.session = Session::default();
            println!("\n  {}", dim("Press Enter to start from the first quest."));
            let _ = self.input.recv();
            return Next::Leave(Flow::Next);
        }
        prompt(ex, at.hints_shown);
        Next::Wait
    }
}

fn mtime(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}
