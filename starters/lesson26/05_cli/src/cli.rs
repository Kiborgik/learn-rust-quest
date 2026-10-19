// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 26-5 · The Town Crier                            200 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The command line is one more thin layer: parse arguments (clap,
//   lesson 17), call the client, print. main.rs is three lines.
//
//   Write output to `&mut impl Write`, not println!. main passes stdout;
//   tests pass a Vec<u8> and read what was printed:
//
//     pub async fn run(cli: Cli, out: &mut impl Write) -> anyhow::Result<()> {
//         writeln!(out, "hello")?;
//         Ok(())
//     }
//
//     let mut out = Vec::new();
//     run(cli, &mut out).await?;
//     assert_eq!(String::from_utf8(out)?, "hello\n");
//
//   Errors get context, so the user sees what was being done:
//
//     client.finish(id).await.with_context(|| format!("finishing quest {id}"))?;
//     // Error: finishing quest 9: quest 9 not found (HTTP 404)
//
// 🎯 YOUR TASK
//
//   1. Add the subcommands `add <TITLE> --reward <N>` (also `-r`) and
//      `finish <ID>`.
//   2. Write `run`. Output, one line each:
//        list     #1 [x] Slay the slime (50 gold)    ([ ] when open)
//                 or "the board is empty"
//        add      posted #1: Slay the slime (50 gold)
//        finish   done #1: Slay the slime, paid 50 gold
//        summary  open 1 (20 gold) | done 1 (50 gold paid)
//      Errors from `finish` get the context "finishing quest <id>".

use crate::client::GuildClient;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::io::Write;

#[derive(Parser, Debug)]
#[command(name = "guildboard", about = "The guild's quest board")]
pub struct Cli {
    /// Board server URL
    #[arg(long, default_value = "http://127.0.0.1:3000")]
    pub url: String,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Command {
    /// List quests
    List {
        /// Only finished quests
        #[arg(long, conflicts_with = "open")]
        done: bool,
        /// Only open quests
        #[arg(long)]
        open: bool,
    },
    /// Show totals
    Summary,
}

pub async fn run(cli: Cli, out: &mut impl Write) -> Result<()> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::spawn_board;

    async fn cmd(url: &str, args: &[&str]) -> Result<String> {
        let cli = Cli::try_parse_from(["guildboard", "--url", url].iter().chain(args))?;
        let mut out = Vec::new();
        run(cli, &mut out).await?;
        Ok(String::from_utf8(out)?)
    }

    #[test]
    fn parsing() {
        let cli = Cli::try_parse_from(["guildboard", "summary"]).unwrap();
        assert_eq!(cli.url, "http://127.0.0.1:3000");
        assert!(Cli::try_parse_from(["guildboard", "list", "--done", "--open"]).is_err());
        assert!(Cli::try_parse_from(["guildboard", "add", "No reward"]).is_err());
        assert!(Cli::try_parse_from(["guildboard", "finish", "seven"]).is_err());
    }

    #[tokio::test]
    async fn a_day_at_the_board() {
        let url = spawn_board().await;
        assert_eq!(cmd(&url, &["list"]).await.unwrap(), "the board is empty\n");
        assert_eq!(
            cmd(&url, &["add", "Slay the slime", "--reward", "50"]).await.unwrap(),
            "posted #1: Slay the slime (50 gold)\n"
        );
        cmd(&url, &["add", "Find the cat", "-r", "20"]).await.unwrap();
        assert_eq!(cmd(&url, &["finish", "1"]).await.unwrap(), "done #1: Slay the slime, paid 50 gold\n");
        assert_eq!(
            cmd(&url, &["list"]).await.unwrap(),
            "#1 [x] Slay the slime (50 gold)\n#2 [ ] Find the cat (20 gold)\n"
        );
        assert_eq!(cmd(&url, &["list", "--open"]).await.unwrap(), "#2 [ ] Find the cat (20 gold)\n");
        assert_eq!(cmd(&url, &["list", "--done"]).await.unwrap(), "#1 [x] Slay the slime (50 gold)\n");
        assert_eq!(cmd(&url, &["summary"]).await.unwrap(), "open 1 (20 gold) | done 1 (50 gold paid)\n");
    }

    #[tokio::test]
    async fn helpful_errors() {
        let url = spawn_board().await;
        let err = cmd(&url, &["finish", "9"]).await.unwrap_err();
        assert_eq!(format!("{err:#}"), "finishing quest 9: quest 9 not found (HTTP 404)");
    }
}
