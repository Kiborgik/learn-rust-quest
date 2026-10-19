//! Confirmations and the full reset, shared by `cargo run -- reset` and
//! the in-game `reset` command.

use crate::progress::Progress;
use crate::ui::*;
use crate::workspace;
use std::io;

pub const CANCELLED: &str = "Cancelled. Nothing was changed.";

pub fn print_reset_warning(p: &Progress) {
    println!();
    println!("  {} Start the whole game over:", bold(red("⚠ RESET")));
    print_wrapped(
        4,
        &format!(
            "• delete all progress: {} XP, {} quests, {} achievements",
            p.xp,
            p.done.len(),
            p.achievements.len()
        ),
        as_is,
    );
    print_wrapped(
        4,
        "• put every exercise file back to its starting state",
        as_is,
    );
    print_wrapped(
        2,
        &format!(
            "Changed files are backed up to {}/ first.",
            workspace::BACKUP_DIR
        ),
        dim,
    );
    ask_yes();
}

pub fn ask_yes() {
    print!(
        "  Type {} to confirm, anything else to cancel: ",
        bold("yes")
    );
    flush();
}

pub fn is_yes(answer: &str) -> bool {
    answer.trim().eq_ignore_ascii_case("yes")
}

/// Restores every exercise file, then wipes progress. Files go first, so a
/// failure leaves the progress untouched.
pub fn full_reset() -> io::Result<usize> {
    let restored = workspace::restore_all()?;
    Progress::reset();
    Ok(restored)
}

/// Prints the outcome of `full_reset`; returns whether it succeeded.
pub fn report_reset(result: io::Result<usize>) -> bool {
    match result {
        Ok(n) => {
            println!(
                "  {}",
                yellow(format!("Progress wiped and {n} exercise(s) restored."))
            );
            true
        }
        Err(e) => {
            println!("  {} {e}", red("Reset failed, your progress was kept:"));
            false
        }
    }
}
