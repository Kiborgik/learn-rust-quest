// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Guild Hall Opens                          500 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   Every layer is ready. The last step is the one every real service
//   needs: read a config, start up, and shut down cleanly. Then two real
//   programs use it (src/bin/server.rs and src/main.rs). Beat the boss,
//   then try them:
//
//     cargo run --bin guildboard-server -- board.toml
//     cargo run --bin guildboard -- add "Slay the slime" --reward 50
//     cargo run --bin guildboard -- list
//
//   ⚔️  HEAD 1: `Config` from TOML (lesson 15). Missing keys use the
//        defaults (addr "127.0.0.1:3000", database "sqlite::memory:").
//        Unknown keys are an error, so a typo can't go unnoticed.
//        (Lesson 15: `#[serde(default)]` on the struct, and the
//        struct attribute `deny_unknown_fields` for the typos.)
//
//   ⚔️  HEAD 2: `start`: open the Store, bind a TcpListener on
//        config.addr, and serve the API in a spawned task. `url` is
//        "http://" + the address it really got (port 0 = any free port).
//        `listener.local_addr()?` tells you that address.
//
//   ⚔️  HEAD 3: `stop`: graceful shutdown. Tell the server to stop
//        (a tokio oneshot channel works well, lesson 21), then wait
//        for its task. In the task:
//          axum::serve(listener, app)
//              .with_graceful_shutdown(async { rx.await.ok(); })
//              .await
//        That gives a `std::io::Result<()>`, so the task is a
//        `JoinHandle<std::io::Result<()>>`. Awaiting the handle wraps
//        it once more (Err if the task panicked): `handle.await??`.
//
//   ⚔️  HEAD 4: startup errors say what failed: context "opening the
//        database" and "binding <addr>".
//
//   ⚔️  HEAD 5: the whole story works end to end, CLI to database.

use crate::api::router;
use crate::store::Store;
use anyhow::{Context, Result};
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub addr: String,
    pub database: String,
}

impl Default for Config {
    fn default() -> Config {
        todo!()
    }
}

impl Config {
    pub fn from_toml(text: &str) -> Result<Config, toml::de::Error> {
        todo!()
    }
}

pub struct Running {
    pub url: String,
    // your fields: a way to say "stop", and the server task
}

pub async fn start(config: &Config) -> Result<Running> {
    todo!()
}

impl Running {
    pub async fn stop(self) -> Result<()> {
        todo!()
    }
}

// ─── The Guild Hall's tests: don't change these! ─────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{Cli, run};
    use crate::client::{ClientError, GuildClient};
    use clap::Parser;

    fn local() -> Config {
        Config { addr: "127.0.0.1:0".into(), ..Config::default() }
    }

    #[test]
    fn head_1_config() {
        let d = Config::default();
        assert_eq!((d.addr.as_str(), d.database.as_str()), ("127.0.0.1:3000", "sqlite::memory:"));
        assert_eq!(Config::from_toml("").unwrap(), d);
        let c = Config::from_toml("addr = \"0.0.0.0:8080\"").unwrap();
        assert_eq!((c.addr.as_str(), c.database.as_str()), ("0.0.0.0:8080", "sqlite::memory:"));
        assert!(Config::from_toml("adress = \"0.0.0.0:8080\"").is_err(), "typos must be caught");
    }

    #[tokio::test]
    async fn head_2_start() {
        let running = start(&local()).await.unwrap();
        assert!(running.url.starts_with("http://127.0.0.1:"), "{}", running.url);
        assert!(!running.url.ends_with(":0"), "url needs the real port: {}", running.url);
        let client = GuildClient::new(&running.url);
        client.add("Slay the slime", 50).await.unwrap();
        assert_eq!(client.list(None).await.unwrap().len(), 1);
        running.stop().await.unwrap();
    }

    #[tokio::test]
    async fn head_3_stop() {
        let running = start(&local()).await.unwrap();
        let url = running.url.clone();
        assert!(GuildClient::new(&url).summary().await.is_ok());
        running.stop().await.unwrap();
        let after = GuildClient::new(&url).summary().await;
        assert!(matches!(after, Err(ClientError::Http(_))), "server still answers: {after:?}");
    }

    #[tokio::test]
    async fn head_4_startup_errors() {
        let bad_db = Config { database: "sqlite://no-such-folder/board.db".into(), ..local() };
        let err = start(&bad_db).await.err().expect("bad database must fail");
        assert!(format!("{err:#}").starts_with("opening the database: "), "{err:#}");
        let bad_addr = Config { addr: "nonsense".into(), ..local() };
        let err = start(&bad_addr).await.err().expect("bad address must fail");
        assert!(format!("{err:#}").starts_with("binding nonsense: "), "{err:#}");
    }

    #[tokio::test]
    async fn head_5_the_whole_story() {
        let running = start(&local()).await.unwrap();
        let mut log = Vec::new();
        for args in [
            vec!["add", "Slay the slime", "-r", "50"],
            vec!["add", "Find the cat", "-r", "20"],
            vec!["finish", "2"],
            vec!["list", "--open"],
            vec!["summary"],
        ] {
            let cli = Cli::try_parse_from(["guildboard", "--url", running.url.as_str()].into_iter().chain(args)).unwrap();
            run(cli, &mut log).await.unwrap();
        }
        assert_eq!(
            String::from_utf8(log).unwrap(),
            "posted #1: Slay the slime (50 gold)\n\
             posted #2: Find the cat (20 gold)\n\
             done #2: Find the cat, paid 20 gold\n\
             #1 [ ] Slay the slime (50 gold)\n\
             open 1 (50 gold) | done 1 (20 gold paid)\n"
        );
        running.stop().await.unwrap();
    }
}
