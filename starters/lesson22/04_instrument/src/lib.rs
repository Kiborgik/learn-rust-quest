// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 22-4 · Instrument Everything                     175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `#[instrument]` wraps a whole function in a span named after it, with
//   the arguments as fields (recorded with Debug):
//
//     use tracing::instrument;
//
//     #[instrument]
//     fn brew(potion: &str, strength: u32) {
//         info!("bubbling");      // brew{potion="tea" strength=3}: bubbling
//     }
//
//   Options:
//     #[instrument(skip(password))]          // never log this argument
//     #[instrument(skip_all, fields(id))]    // log nothing automatically
//     #[instrument(level = "debug")]         // span level (default info)
//     #[instrument(ret)]                     // also log the return value
//     #[instrument(err)]                     // log the error if it returns Err
//
//   Secrets (passwords, tokens) must never reach the logs. `skip` them.
//
// 🎯 YOUR TASK
//
//   Instrument `brew` so its span shows `potion` and `strength` but NOT
//   the `secret_recipe`.

pub mod capture;

use tracing::{info, instrument};

pub fn brew(potion: &str, strength: u32, secret_recipe: &str) -> String {
    info!("bubbling");
    format!("{potion} x{strength} ({} steps)", secret_recipe.len())
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::capture;

    #[test]
    fn instrumented() {
        let log = capture(|| {
            assert_eq!(brew("tea", 3, "boil-steep-pour"), "tea x3 (15 steps)");
        });
        assert!(log.contains(r#"brew{potion="tea" strength=3}: bubbling"#), "{log}");
        assert!(!log.contains("boil"), "the secret leaked: {log}");
    }
}
