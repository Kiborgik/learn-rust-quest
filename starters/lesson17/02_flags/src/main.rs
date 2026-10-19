// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 17-2 · Flags and Banners                         175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The field TYPE decides how an argument behaves:
//
//     #[derive(Parser)]
//     struct Cli {
//         #[arg(short, long)]
//         loud: bool,               // a flag: present → true      --loud
//         #[arg(long)]
//         title: Option<String>,    // optional value               --title Sir
//         #[arg(short = 'g', long = "tag")]
//         tags: Vec<String>,        // repeatable                   -g a --tag b
//         #[arg(long, default_value = "hall")]
//         room: String,             // value with a default
//     }
//
//   ⚠️ Without `#[arg(...)]` a field is POSITIONAL. A positional `bool`
//   makes no sense to clap, so it refuses (a panic when building the
//   parser, which tests catch).
//
//   clap derives its checks from your types: wrong numbers, missing
//   values and unknown flags all become clear error messages.
//
// 🎯 YOUR TASK
//
//   `--loud`/`-l` must be a flag, and `--tag` must be repeatable (with
//   the short form `-g`). Fix the attributes.

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    name: String,
    loud: bool,
    #[arg(long)]
    title: Option<String>,
    tags: Vec<String>,
}

fn banner(cli: &Cli) -> String {
    let mut line = match &cli.title {
        Some(t) => format!("{t} {}", cli.name),
        None => cli.name.clone(),
    };
    if !cli.tags.is_empty() {
        line.push_str(&format!(" [{}]", cli.tags.join(", ")));
    }
    if cli.loud { line.to_uppercase() + "!" } else { line }
}

fn main() {
    println!("{}", banner(&Cli::parse()));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn run(args: &[&str]) -> String {
        banner(&Cli::try_parse_from(args).unwrap())
    }

    #[test]
    fn plain() {
        assert_eq!(run(&["banner", "Ferris"]), "Ferris");
        assert_eq!(run(&["banner", "Ferris", "--title", "Sir"]), "Sir Ferris");
    }

    #[test]
    fn flags_and_tags() {
        assert_eq!(run(&["banner", "Ferris", "-l"]), "FERRIS!");
        assert_eq!(
            run(&["banner", "Ferris", "--tag", "brave", "-g", "crab"]),
            "Ferris [brave, crab]"
        );
        assert_eq!(run(&["banner", "--loud", "Ferris", "-g", "x"]), "FERRIS [X]!");
    }
}
