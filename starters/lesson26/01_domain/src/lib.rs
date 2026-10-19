//! guildboard: the guild's quest board.
//!
//! Over this lesson it grows layer by layer:
//!   domain  (rules, no IO)      ← this quest
//!   store   (SQLite with sqlx)
//!   api     (HTTP with axum)
//!   client  (HTTP with reqwest)
//!   cli     (commands with clap)
//!   server  (config, start, stop)

pub mod domain;
