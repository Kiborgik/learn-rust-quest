// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 17-3 · Many Commands                             175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Tools like `git` and `cargo` have subcommands. In clap, that's an enum:
//
//     use clap::{Parser, Subcommand};
//
//     #[derive(Parser)]
//     struct Cli {
//         #[arg(short, long)]
//         verbose: bool,                 // global options live here
//         #[command(subcommand)]
//         command: Command,
//     }
//
//     #[derive(Subcommand)]
//     enum Command {
//         /// Roll some dice               ← help text per subcommand
//         Roll { dice: String },         // tool roll 2d6
//         Rest {
//             #[arg(long, default_value_t = 8)]
//             hours: u32,                // tool rest --hours 4
//         },
//     }
//
//   Variant names become kebab-case commands: `LongRest` → `long-rest`.
//   Then `match cli.command { ... }`, the enums from lesson 5.
//
// 🎯 YOUR TASK
//
//   1. Wire the `Command` enum into `Cli`.
//   2. Implement `run`.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    /// Your hero's name
    #[arg(long, default_value = "Ferris")]
    hero: String,
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Attack a monster
    Attack { target: String },
    /// Rest to recover
    Rest {
        #[arg(long, default_value_t = 8)]
        hours: u32,
    },
    /// Show the hero's status
    Status,
}

/// "Ferris attacks Slime" / "Ferris rests 8 hours" / "Ferris is ready"
fn run(cli: &Cli) -> String {
    todo!()
}

fn main() {
    println!("{}", run(&Cli::parse()));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn go(args: &[&str]) -> String {
        run(&Cli::try_parse_from(args).unwrap())
    }

    #[test]
    fn commands() {
        assert_eq!(go(&["quest", "attack", "Slime"]), "Ferris attacks Slime");
        assert_eq!(go(&["quest", "rest"]), "Ferris rests 8 hours");
        assert_eq!(go(&["quest", "--hero", "Corro", "rest", "--hours", "2"]), "Corro rests 2 hours");
        assert_eq!(go(&["quest", "status"]), "Ferris is ready");
    }

    #[test]
    fn unknown_command() {
        assert!(Cli::try_parse_from(["quest", "dance"]).is_err());
        assert!(Cli::try_parse_from(["quest"]).is_err());
    }
}
