//! The `guildboard` command. Thin on purpose: parse, run, print.
//! Try it once the boss is beaten and a server is running.

use clap::Parser;
use l26_05_cli::cli::{Cli, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run(Cli::parse(), &mut std::io::stdout()).await
}
