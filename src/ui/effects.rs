//! Small animations. Skipped in plain mode.

use super::{Style, bar_with, dim, flush, plain, yellow};
use std::thread::sleep;
use std::time::Duration;

/// Counts a number up on one line, for a little reward feel.
pub fn count_up(prefix: &str, to: u32) {
    if plain() {
        print!("{prefix}+{to} XP");
        return;
    }
    let steps = to.clamp(1, 12);
    for i in 1..=steps {
        print!("\r{prefix}{}", yellow(format!("+{} XP", to * i / steps)));
        flush();
        sleep(Duration::from_millis(25));
    }
}

/// A bar draining from `from` to 0, like a boss losing its last HP.
pub fn drain_bar(prefix: &str, from: u32, total: u32, color: Style) {
    if plain() {
        return;
    }
    for v in (0..=from).rev() {
        print!(
            "\r{prefix}{} {}  ",
            bar_with(v, total, 20, color),
            dim(format!("{v}/{total}"))
        );
        flush();
        sleep(Duration::from_millis(140));
    }
    println!();
}
