//! The `guildboard` command. Thin on purpose: parse, run, print.

use clap::Parser;
use l26_06_boss::cli::{Cli, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run(Cli::parse(), &mut std::io::stdout()).await
}
