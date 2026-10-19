//! guildboard: the guild's quest board.
//!
//!   domain  (rules, no IO)         done
//!   store   (SQLite with sqlx)     done
//!   api     (HTTP with axum)       done
//!   client  (HTTP with reqwest)    done
//!   cli     (commands with clap)   done
//!   server  (config, start, stop)  ← the boss
//!
//! Two programs use this library:
//!   cargo run --bin guildboard-server -- board.toml
//!   cargo run --bin guildboard -- add "Slay the slime" --reward 50

pub mod api;
pub mod cli;
pub mod client;
pub mod domain;
pub mod server;
pub mod store;
