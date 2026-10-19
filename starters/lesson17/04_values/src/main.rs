// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 17-4 · Only Valid Choices                        175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Let clap reject bad values before your code ever runs.
//
//   A fixed set of choices: derive `ValueEnum` on a plain enum:
//
//     #[derive(ValueEnum, Clone, Debug, PartialEq)]
//     enum Difficulty { Easy, Normal, Hard }
//
//     #[arg(long, value_enum, default_value_t = Difficulty::Normal)]
//     difficulty: Difficulty,           // --difficulty hard
//
//   `--help` lists the possible values for you.
//
//   A number range: a value parser:
//
//     #[arg(long, value_parser = clap::value_parser!(u8).range(1..=10))]
//     volume: u8,                       // --volume 11 → error
//
//   Your own check: any `fn(&str) -> Result<T, String>`:
//
//     fn even(s: &str) -> Result<u32, String> { ... }
//     #[arg(value_parser = even)]
//
// 🎯 YOUR TASK
//
//   1. `--class` must only accept warrior, mage or rogue: make `Class` a
//      ValueEnum.
//   2. `--level` must be 1 to 99.

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, PartialEq)]
enum Class {
    Warrior,
    Mage,
    Rogue,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    class: Class,
    #[arg(long, default_value_t = 1)]
    level: u8,
}

fn main() {
    let cli = Cli::parse();
    println!("A level {} {:?}", cli.level, cli.class);
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let cli = Cli::try_parse_from(["hero", "--class", "mage", "--level", "40"]).unwrap();
        assert_eq!((cli.class, cli.level), (Class::Mage, 40));
        let cli = Cli::try_parse_from(["hero", "--class", "rogue"]).unwrap();
        assert_eq!(cli.level, 1);
    }

    #[test]
    fn invalid() {
        assert!(Cli::try_parse_from(["hero", "--class", "bard"]).is_err());
        assert!(Cli::try_parse_from(["hero", "--class", "mage", "--level", "0"]).is_err());
        assert!(Cli::try_parse_from(["hero", "--class", "mage", "--level", "100"]).is_err());
    }
}
