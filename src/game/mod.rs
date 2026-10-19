//! The game loop: pick the next quest, play it, pay out, repeat.

mod commands;
mod finish;
mod play;
mod quiz;
pub mod reset;

use crate::exercises;
use crate::progress::Progress;
use crate::view;
use std::io::{self, BufRead};
use std::sync::mpsc::{self, Receiver};
use std::thread;

pub struct Game {
    progress: Progress,
    input: Receiver<String>,
    first_run: bool,
    session: Session,
}

/// What happened since the game was started, for the goodbye screen.
#[derive(Default)]
struct Session {
    start_xp: u32,
    start_done: usize,
    /// Quests passed in a row without a hint.
    combo: u32,
}

impl Session {
    fn starting_from(p: &Progress) -> Session {
        Session {
            start_xp: p.xp,
            start_done: p.done.len(),
            combo: 0,
        }
    }
}

/// What the player wants after a quest screen.
enum Flow {
    Next,
    Quit,
}

impl Game {
    pub fn new() -> Self {
        // Read stdin on a background thread so the main loop can watch
        // for file changes and keyboard commands at the same time.
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            for line in io::stdin().lock().lines() {
                let Ok(line) = line else { break };
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
        let first_run = !Progress::exists();
        let mut progress = Progress::load();
        progress.touch_day();
        progress.save();
        let session = Session::starting_from(&progress);
        Game {
            progress,
            input: rx,
            first_run,
            session,
        }
    }

    pub fn run(&mut self) {
        if self.first_run && !self.welcome() {
            return;
        }
        // A streak achievement can unlock just by starting the game today.
        self.check_achievements();
        loop {
            let next = exercises::all().find(|ex| !self.progress.done.contains(ex.id));
            let Some(ex) = next else {
                self.victory();
                return;
            };
            if let Flow::Quit = self.play(ex) {
                let p = &self.progress;
                let quests = p.done.len().saturating_sub(self.session.start_done);
                let xp = p.xp.saturating_sub(self.session.start_xp);
                view::goodbye(quests, xp, p.streak);
                return;
            }
        }
    }

    /// First-run screen. Returns false if input was closed.
    fn welcome(&mut self) -> bool {
        view::welcome();
        commands::print_commands();
        view::welcome_tips();
        self.input.recv().is_ok()
    }

    fn victory(&mut self) {
        crate::ui::clear();
        view::header(&self.progress);
        view::victory();
        view::map(&self.progress);
        view::next_steps();
    }
}
