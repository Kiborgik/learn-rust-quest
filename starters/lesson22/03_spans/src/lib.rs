// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 22-3 · Chapters of the Story                     175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   An EVENT is a moment. A SPAN is a period of time with a name and
//   fields, like "handling request 42" or "quest: rescue". Events that
//   happen inside a span are tagged with it:
//
//     use tracing::{info, info_span};
//
//     let span = info_span!("quest", name = %quest_name);
//     let _guard = span.enter();           // inside the span until _guard drops
//     info!("started");                    // quest{name=rescue}: started
//     info!("done");                       // quest{name=rescue}: done
//
//   Spans nest: entering a span inside another shows both,
//   outer:inner: message.
//
//   Name the guard `_guard`, not `_`: `let _ = span.enter();` drops the
//   guard immediately, so you'd leave the span right away.
//
// 🎯 YOUR TASK
//
//   Wrap the work in `run_quest` in a span named "quest" with a `name`
//   field (Display), and each step in a nested span "step" with field `n`.
//   Expected lines look like:
//     INFO quest{name=rescue}: started
//     INFO quest{name=rescue}:step{n=1}: working

pub mod capture;

use tracing::{info, info_span};

pub fn run_quest(name: &str, steps: u32) {
    info!("started");
    for n in 1..=steps {
        info!("working");
    }
    info!("finished");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::capture;

    #[test]
    fn spans() {
        let log = capture(|| run_quest("rescue", 2));
        let lines: Vec<&str> = log.lines().collect();
        assert_eq!(lines.len(), 4, "{log}");
        assert!(lines[0].ends_with("quest{name=rescue}: started"), "{log}");
        assert!(lines[1].ends_with("quest{name=rescue}:step{n=1}: working"), "{log}");
        assert!(lines[2].ends_with("quest{name=rescue}:step{n=2}: working"), "{log}");
        assert!(lines[3].ends_with("quest{name=rescue}: finished"), "{log}");
    }
}
