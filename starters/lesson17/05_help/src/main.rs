// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 17-5 · Read the Manual                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `#[command(...)]` on the struct sets up the program itself:
//
//     /// Roll dice like a hero          ← becomes `about` (first line)
//     #[derive(Parser)]
//     #[command(version, about)]         // --version from Cargo.toml, about from the doc comment
//     struct Cli { ... }
//
//   Or spell it out: `#[command(version, about = "Roll dice like a hero")]`.
//
//   `--help` and `--version` are "errors" in try_parse: the parse stops
//   and returns a clap::Error with a kind you can check:
//
//     let err = Cli::try_parse_from(["dice", "--help"]).unwrap_err();
//     err.kind()          // ErrorKind::DisplayHelp
//     err.to_string()     // the help text
//
//   With `Cli::parse()` clap prints it and exits with code 0, as users
//   expect.
//
// 🎯 YOUR TASK
//
//   The tool has no version and no description. Add both, so the tests
//   pass. The version must come from Cargo.toml (2.3.0), not be typed in.

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    /// Dice to roll, like 2d6
    dice: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Rolling {}", cli.dice);
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;

    #[test]
    fn help() {
        let err = Cli::try_parse_from(["dice", "--help"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::DisplayHelp);
        let text = err.to_string();
        assert!(text.contains("Roll dice like a hero"), "{text}");
        assert!(text.contains("Dice to roll, like 2d6"), "{text}");
    }

    #[test]
    fn version() {
        let err = Cli::try_parse_from(["dice", "--version"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::DisplayVersion);
        assert!(err.to_string().contains("2.3.0"));
    }
}
