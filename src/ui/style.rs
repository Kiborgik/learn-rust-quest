//! Colors and escape codes. Every function returns plain text in plain mode.

use super::plain;
use std::fmt::Display;

/// A style function: text in, styled text out.
pub type Style = fn(String) -> String;

pub fn paint(code: &str, s: impl Display) -> String {
    if plain() {
        s.to_string()
    } else {
        format!("\x1b[{code}m{s}\x1b[0m")
    }
}

pub fn bold(s: impl Display) -> String {
    paint("1", s)
}
pub fn dim(s: impl Display) -> String {
    paint("2", s)
}
pub fn red(s: impl Display) -> String {
    paint("31", s)
}
pub fn green(s: impl Display) -> String {
    paint("32", s)
}
pub fn yellow(s: impl Display) -> String {
    paint("33", s)
}
pub fn blue(s: impl Display) -> String {
    paint("34", s)
}
pub fn magenta(s: impl Display) -> String {
    paint("35", s)
}
pub fn cyan(s: impl Display) -> String {
    paint("36", s)
}

/// No style, for places that take a style function.
pub fn as_is(s: String) -> String {
    s
}

/// Clickable link to a local file (OSC 8). Terminals without support just
/// show `text`.
pub fn file_link(path: &std::path::Path, text: impl Display) -> String {
    if plain() {
        return text.to_string();
    }
    let url = path.to_string_lossy().replace('\\', "/");
    let url = url.trim_start_matches("//?/");
    let sep = if url.starts_with('/') { "" } else { "/" };
    format!("\x1b]8;;file://{sep}{url}\x1b\\{text}\x1b]8;;\x1b\\")
}

/// Text for the screen: as is, or without escape codes in plain mode.
/// Use it for text that comes with its own colors (compiler output).
pub fn show(s: &str) -> String {
    if plain() {
        strip_ansi(s)
    } else {
        s.to_string()
    }
}

/// Removes ANSI color codes and OSC 8 links.
pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\x1b' {
            out.push(c);
            continue;
        }
        match chars.next() {
            // CSI: ESC [ ... final letter
            Some('[') => {
                for c in chars.by_ref() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
            // OSC: ESC ] ... ended by BEL or ESC \
            Some(']') => {
                while let Some(c) = chars.next() {
                    if c == '\x07' || (c == '\x1b' && chars.next_if_eq(&'\\').is_some()) {
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_colors_and_links() {
        let s = "\x1b[1;31merror\x1b[0m: \x1b]8;;file:///a.rs\x1b\\a.rs\x1b]8;;\x1b\\ ok";
        assert_eq!(strip_ansi(s), "error: a.rs ok");
        assert_eq!(strip_ansi("plain ✔"), "plain ✔");
    }
}
