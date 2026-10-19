//! The board server. Runs until Ctrl+C.
//!
//!   cargo run --bin guildboard-server              (defaults, in-memory)
//!   cargo run --bin guildboard-server -- board.toml

use anyhow::Context;
use l26_06_boss::server::{Config, start};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = match std::env::args().nth(1) {
        Some(path) => {
            let text = std::fs::read_to_string(&path).with_context(|| format!("reading {path}"))?;
            Config::from_toml(&text).with_context(|| format!("parsing {path}"))?
        }
        None => Config::default(),
    };
    let running = start(&config).await?;
    println!("📜 quest board open at {} (Ctrl+C to close)", running.url);
    tokio::signal::ctrl_c().await?;
    running.stop().await?;
    println!("board closed, see you tomorrow");
    Ok(())
}
