//! Cli: the `guildboard` command. (Given: you wrote this in 26-5.)

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
    /// Post a new quest
    Add {
        title: String,
        /// Gold paid when it's done
        #[arg(long, short)]
        reward: i64,
    },
    /// Mark a quest as done
    Finish { id: i64 },
    /// Show totals
    Summary,
}

pub async fn run(cli: Cli, out: &mut impl Write) -> Result<()> {
    let client = GuildClient::new(&cli.url);
    match cli.command {
        Command::List { done, open } => {
            let filter = match (done, open) {
                (true, _) => Some(true),
                (_, true) => Some(false),
                _ => None,
            };
            let quests = client.list(filter).await?;
            if quests.is_empty() {
                writeln!(out, "the board is empty")?;
            }
            for q in quests {
                let mark = if q.done { 'x' } else { ' ' };
                writeln!(out, "#{} [{mark}] {} ({} gold)", q.id, q.title, q.reward)?;
            }
        }
        Command::Add { title, reward } => {
            let q = client.add(&title, reward).await?;
            writeln!(out, "posted #{}: {} ({} gold)", q.id, q.title, q.reward)?;
        }
        Command::Finish { id } => {
            let q = client.finish(id).await.with_context(|| format!("finishing quest {id}"))?;
            writeln!(out, "done #{}: {}, paid {} gold", q.id, q.title, q.reward)?;
        }
        Command::Summary => {
            let s = client.summary().await?;
            writeln!(out, "open {} ({} gold) | done {} ({} gold paid)", s.open, s.gold_waiting, s.done, s.gold_paid)?;
        }
    }
    Ok(())
}
