// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 17-6 · Write to Any Scroll                       175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `println!` always writes to the terminal, so tests can't see it.
//   Write to anything that implements `std::io::Write` instead:
//
//     use std::io::Write;
//
//     fn report(out: &mut impl Write, gold: u32) -> std::io::Result<()> {
//         writeln!(out, "Gold: {gold}")?;   // like println!, but to `out`
//         Ok(())
//     }
//
//     report(&mut std::io::stdout(), 7)?;    // real program: the terminal
//
//     let mut buf = Vec::new();              // in tests: a Vec<u8>
//     report(&mut buf, 7)?;
//     assert_eq!(String::from_utf8(buf)?, "Gold: 7\n");
//
//   The usual CLI shape:
//
//     fn main() -> anyhow::Result<()> {
//         let cli = Cli::parse();
//         run(cli, &mut std::io::stdout().lock())
//     }
//
//   Everything testable lives in `run`; `main` only wires things up.
//
// 🎯 YOUR TASK
//
//   `run` prints straight to the terminal, so the tests capture nothing.
//   Make it write to `out`.

use clap::Parser;
use std::io::Write;

#[derive(Parser, Debug)]
struct Cli {
    /// How many potions to count
    count: u32,
}

fn run(cli: Cli, out: &mut impl Write) -> anyhow::Result<()> {
    for i in 1..=cli.count {
        println!("Potion {i}");
    }
    println!("Total: {}", cli.count);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    run(Cli::parse(), &mut std::io::stdout().lock())
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn output(args: &[&str]) -> String {
        let mut buf = Vec::new();
        run(Cli::try_parse_from(args).unwrap(), &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn captured() {
        assert_eq!(output(&["potions", "2"]), "Potion 1\nPotion 2\nTotal: 2\n");
        assert_eq!(output(&["potions", "0"]), "Total: 0\n");
    }
}
