// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 22-5 · Turn Down the Noise                       175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   You decide what gets through with a FILTER, usually from the RUST_LOG
//   environment variable:
//
//     RUST_LOG=warn                       only warn and error
//     RUST_LOG=info,my_app::db=debug      info everywhere, debug for one module
//     RUST_LOG=off                        nothing
//
//   In code:
//
//     use tracing_subscriber::EnvFilter;
//
//     tracing_subscriber::fmt()
//         .with_env_filter(EnvFilter::from_default_env())   // reads RUST_LOG
//         .init();
//
//     EnvFilter::new("warn,l22_05_filter::dungeon=debug")   // or fixed directives
//
//   Each event's TARGET is its module path (`l22_05_filter::dungeon`), so
//   you can turn up the detail for one part of the program only.
//
// 🎯 YOUR TASK
//
//   `logger` builds a subscriber that lets EVERYTHING through. Add an
//   EnvFilter built from `directives`. The tests try several directive
//   strings.

pub mod capture;

use capture::Buf;

pub mod town {
    pub fn visit() {
        tracing::debug!("town: buying bread");
        tracing::info!("town: visited");
        tracing::warn!("town: the bridge is out");
    }
}

pub mod dungeon {
    pub fn explore() {
        tracing::debug!("dungeon: checking for traps");
        tracing::info!("dungeon: explored");
    }
}

pub fn logger(buf: Buf, directives: &str) -> impl tracing::Subscriber + Send + Sync {
    tracing_subscriber::fmt()
        .with_writer(buf)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .with_max_level(tracing::Level::TRACE)
        .finish()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn run(directives: &str) -> String {
        let buf = Buf::default();
        tracing::subscriber::with_default(logger(buf.clone(), directives), || {
            town::visit();
            dungeon::explore();
        });
        buf.text()
    }

    #[test]
    fn only_warnings() {
        let log = run("warn");
        assert_eq!(log.lines().count(), 1, "{log}");
        assert!(log.contains("the bridge is out"));
    }

    #[test]
    fn debug_one_module() {
        let log = run("warn,l22_05_filter::dungeon=debug");
        assert!(log.contains("checking for traps"), "{log}");
        assert!(log.contains("explored"), "{log}");
        assert!(!log.contains("buying bread"), "{log}");
        assert_eq!(log.lines().count(), 3, "{log}");
    }

    #[test]
    fn off() {
        assert_eq!(run("off"), "");
    }
}
