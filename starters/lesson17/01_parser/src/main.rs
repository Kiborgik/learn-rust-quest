// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 17-1 · The Town Crier                            175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `clap` turns a struct into a command-line parser. Each field is an
//   argument; doc comments become the `--help` text:
//
//     use clap::Parser;
//
//     #[derive(Parser, Debug)]
//     struct Cli {
//         /// Who to greet               ← shown in --help
//         name: String,                 // positional:  greet Ferris
//         #[arg(short, long, default_value_t = 1)]
//         times: u32,                   // option:      -t 3  or  --times 3
//     }
//
//     fn main() {
//         let cli = Cli::parse();       // reads std::env::args, exits on error
//     }
//
//   In tests, parse a fake command line instead. It returns a Result
//   rather than exiting:
//
//     let cli = Cli::try_parse_from(["greet", "Ferris", "-t", "3"])?;
//
//   The first item is the program name, like argv[0].
//
// 🎯 YOUR TASK
//
//   Make `Cli` a clap parser. `--times` should also work as `-t` and
//   default to 1.

use clap::Parser;

#[derive(Debug)]
struct Cli {
    /// Who to announce
    name: String,
    times: u32,
}

fn announce(cli: &Cli) -> String {
    vec![format!("Hear ye! {} has arrived!", cli.name); cli.times as usize].join("\n")
}

fn main() {
    let cli = Cli::parse();
    println!("{}", announce(&cli));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let cli = Cli::try_parse_from(["crier", "Ferris"]).unwrap();
        assert_eq!(announce(&cli), "Hear ye! Ferris has arrived!");
    }

    #[test]
    fn options() {
        let short = Cli::try_parse_from(["crier", "Corro", "-t", "2"]).unwrap();
        let long = Cli::try_parse_from(["crier", "--times", "2", "Corro"]).unwrap();
        assert_eq!(announce(&short), announce(&long));
        assert_eq!(announce(&short).lines().count(), 2);
    }

    #[test]
    fn errors() {
        assert!(Cli::try_parse_from(["crier"]).is_err());
        assert!(Cli::try_parse_from(["crier", "Ferris", "-t", "many"]).is_err());
    }
}
