//! guildboard: the guild's quest board.
//!
//!   domain  (rules, no IO)       done
//!   store   (SQLite with sqlx)   done
//!   api     (HTTP with axum)     done
//!   client  (HTTP with reqwest)  ← this quest
//!   cli     (commands with clap)
//!   server  (config, start, stop)

pub mod api;
pub mod client;
pub mod domain;
pub mod store;

#[cfg(test)]
mod testkit;
