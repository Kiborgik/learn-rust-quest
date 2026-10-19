// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 22-1 · The Watchtower Log                        175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `println!` is fine for a toy. Real programs LOG: every message has a
//   level, can be filtered, and goes wherever you configure (terminal,
//   file, a log service). `tracing` is the standard crate for this.
//
//     use tracing::{trace, debug, info, warn, error};
//
//     info!("chest opened");                 // levels, quiet → loud:
//     warn!("the chest was empty");          //   trace < debug < info < warn < error
//     error!("it was a mimic!");
//
//   The macros only CREATE events. A SUBSCRIBER decides what happens to
//   them. Usually main sets one up once:
//
//     tracing_subscriber::fmt::init();       // pretty text to stderr
//
//   With no subscriber, events go nowhere and cost almost nothing.
//   (The tests use a subscriber that writes into a buffer: see capture.rs.)
//
// 🎯 YOUR TASK
//
//   `open_chest` prints with println!, so nothing is logged. Turn the
//   prints into log events with the right levels:
//     gold == 0      → warn   "the chest was empty"
//     gold > 1000    → error  "it was a mimic!"
//     otherwise      → info   "chest opened"
//   Keep the return values.

pub mod capture;

pub fn open_chest(gold: u32) -> u32 {
    if gold == 0 {
        println!("the chest was empty");
        0
    } else if gold > 1000 {
        println!("it was a mimic!");
        0
    } else {
        println!("chest opened");
        gold
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::capture;

    #[test]
    fn levels() {
        let log = capture(|| {
            assert_eq!(open_chest(50), 50);
            assert_eq!(open_chest(0), 0);
            assert_eq!(open_chest(5000), 0);
        });
        let lines: Vec<&str> = log.lines().collect();
        assert_eq!(lines.len(), 3, "{log}");
        assert!(lines[0].contains("INFO") && lines[0].contains("chest opened"), "{log}");
        assert!(lines[1].contains("WARN") && lines[1].contains("the chest was empty"), "{log}");
        assert!(lines[2].contains("ERROR") && lines[2].contains("it was a mimic!"), "{log}");
    }
}
