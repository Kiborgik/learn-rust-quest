//! guildboard: the guild's quest board.
//!
//!   domain  (rules, no IO)      done
//!   store   (SQLite with sqlx)  done
//!   api     (HTTP with axum)    ← this quest
//!   client  (HTTP with reqwest)
//!   cli     (commands with clap)
//!   server  (config, start, stop)

pub mod api;
pub mod domain;
pub mod store;

#[cfg(test)]
mod testkit;
