//! Width-aware layout: wrapping, separators, bars, screen clearing.

use super::{Style, dim, green, plain, term};
use std::io::{self, Write};

/// Columns to lay text out in: the terminal width, kept between 40 and 72
/// so lines stay short enough to read and never scroll sideways.
pub fn cols() -> usize {
    term::width()
        .or_else(|| std::env::var("COLUMNS").ok()?.parse().ok())
        .unwrap_or(72)
        .clamp(40, 72)
}

pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = vec![String::new()];
    for word in text.split_whitespace() {
        let cur = lines.last_mut().unwrap();
        if !cur.is_empty() && cur.chars().count() + word.chars().count() + 1 > width {
            lines.push(word.to_string());
        } else {
            if !cur.is_empty() {
                cur.push(' ');
            }
            cur.push_str(word);
        }
    }
    lines
}

/// Word-wraps `text` to fit after `indent` spaces, and prints it with `style`.
pub fn print_wrapped(indent: usize, text: &str, style: Style) {
    let pad = " ".repeat(indent);
    for line in wrap(text, cols().saturating_sub(indent + 1)) {
        println!("{pad}{}", style(line));
    }
}

/// A separator line. In plain mode just a blank line: screen readers would
/// read out every box character.
pub fn rule() {
    if plain() {
        println!();
    } else {
        println!("{}", dim("─".repeat(cols())));
    }
}

/// `██████░░░░░░` style bar in green; `[######------]` in plain mode.
pub fn bar(current: u32, total: u32, width: usize) -> String {
    bar_with(current, total, width, green)
}

/// Like `bar`, with the filled part in `color`.
pub fn bar_with(current: u32, total: u32, width: usize, color: Style) -> String {
    let filled = if total == 0 {
        width
    } else {
        ((current as usize * width) / total as usize).min(width)
    };
    if plain() {
        return format!("[{}{}]", "#".repeat(filled), "-".repeat(width - filled));
    }
    format!(
        "{}{}",
        color("█".repeat(filled)),
        dim("░".repeat(width - filled))
    )
}

pub fn clear() {
    if plain() {
        println!("\n");
        return;
    }
    print!("\x1b[2J\x1b[H");
    flush();
}

pub fn flush() {
    let _ = io::stdout().flush();
}
