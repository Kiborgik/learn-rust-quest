//! Tiny terminal toolkit: colors, layout, small animations. Plain `std`.
//!
//! Plain mode (no colors, no animation, no screen clearing, no escape codes)
//! is on when `NO_COLOR` is set (https://no-color.org), `TERM=dumb`, the
//! game runs with `--plain`, or the output isn't a terminal (unless
//! `CLICOLOR_FORCE=1`). That keeps it readable for screen readers, logs and
//! pipes.

mod effects;
mod layout;
mod style;
mod term;

pub use effects::*;
pub use layout::*;
pub use style::*;

use std::io::{self, IsTerminal};
use std::sync::OnceLock;

static PLAIN: OnceLock<bool> = OnceLock::new();

/// Decides plain mode once, at startup. `forced` is the `--plain` flag.
pub fn init(forced: bool) {
    let set = |name: &str| std::env::var_os(name).is_some_and(|v| !v.is_empty() && v != "0");
    let plain = forced
        || set("NO_COLOR")
        || std::env::var("TERM").is_ok_and(|t| t == "dumb")
        || (!io::stdout().is_terminal() && !set("CLICOLOR_FORCE"));
    let _ = PLAIN.set(plain);
    if !plain {
        term::enable_ansi();
    }
}

pub fn plain() -> bool {
    *PLAIN.get().unwrap_or(&false)
}
