//! guildboard: the guild's quest board.
//!
//!   domain  (rules, no IO)      done
//!   store   (SQLite with sqlx)  ← this quest
//!   api     (HTTP with axum)
//!   client  (HTTP with reqwest)
//!   cli     (commands with clap)
//!   server  (config, start, stop)

pub mod domain;
pub mod store;
