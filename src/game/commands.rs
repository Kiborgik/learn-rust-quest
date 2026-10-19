//! In-game commands as one table. Parsing, the prompt line, `?` and the
//! welcome screen all read `COMMANDS`, so a new command is one entry.

use crate::exercises::Exercise;
use crate::ui::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Command {
    Check,
    Hint,
    Map,
    Quit,
    Help,
    Restore,
    Reset,
}

pub struct CommandInfo {
    pub cmd: Command,
    /// What the player may type (lowercase). "" is a bare Enter.
    pub keys: &'static [&'static str],
    pub label: &'static str,
    pub help: &'static str,
    /// One word for the prompt line, or "" to show it only in `?`.
    pub short: &'static str,
}

const fn command(
    cmd: Command,
    keys: &'static [&'static str],
    label: &'static str,
    short: &'static str,
    help: &'static str,
) -> CommandInfo {
    CommandInfo {
        cmd,
        keys,
        label,
        help,
        short,
    }
}

pub const COMMANDS: &[CommandInfo] = &[
    command(
        Command::Check,
        &["", "c", "check"],
        "Enter",
        "check",
        "check again now (saving the file does it too)",
    ),
    command(
        Command::Hint,
        &["h", "hint"],
        "h",
        "hint",
        "show a hint (this quest gives half XP)",
    ),
    command(
        Command::Map,
        &["m", "map"],
        "m",
        "map",
        "map, achievements, streak",
    ),
    command(
        Command::Quit,
        &["q", "quit", "exit"],
        "q",
        "quit",
        "save and quit",
    ),
    command(Command::Help, &["?", "help"], "?", "help", "list commands"),
    command(
        Command::Restore,
        &["restore"],
        "restore",
        "",
        "put this quest's files back to the start",
    ),
    command(
        Command::Reset,
        &["reset"],
        "reset",
        "",
        "wipe progress and restore every file",
    ),
];

impl Command {
    pub fn parse(input: &str) -> Option<Command> {
        let input = input.trim().to_lowercase();
        COMMANDS
            .iter()
            .find(|c| c.keys.contains(&input.as_str()))
            .map(|c| c.cmd)
    }
}

/// The full list, for `?` and the welcome screen.
pub fn print_commands() {
    for c in COMMANDS {
        println!("  {}  {}", bold(format!("{:<7}", c.label)), c.help);
    }
}

/// The line under every check:
/// `Enter check · h hint (3) · m map · q quit · ? help`
pub fn prompt(ex: &Exercise, hints_shown: usize) {
    rule();
    let hints_left = ex.hints.len() - hints_shown;
    let parts: Vec<String> = COMMANDS
        .iter()
        .filter(|c| !c.short.is_empty())
        .map(|c| match c.cmd {
            Command::Hint if hints_left == 0 => dim("no hints"),
            Command::Hint => format!(
                "{} {} {}",
                bold(c.label),
                c.short,
                dim(format!("({hints_left})"))
            ),
            _ => format!("{} {}", bold(c.label), c.short),
        })
        .collect();
    println!("  {}", parts.join(&dim(" · ")));
    print!("  > ");
    flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_commands() {
        assert_eq!(Command::parse(""), Some(Command::Check));
        assert_eq!(Command::parse("  H "), Some(Command::Hint));
        assert_eq!(Command::parse("exit"), Some(Command::Quit));
        assert_eq!(Command::parse("dance"), None);
    }

    #[test]
    fn keys_are_unique() {
        let mut keys: Vec<&str> = COMMANDS
            .iter()
            .flat_map(|c| c.keys.iter().copied())
            .collect();
        let n = keys.len();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), n, "a key is used by two commands");
    }
}
