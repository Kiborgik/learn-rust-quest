// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 14-6 · Chains of Cause                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   An error can carry the error that caused it. With thiserror, mark the
//   field with `#[source]` (or name it `source`, or use `#[from]`):
//
//     #[derive(Debug, Error)]
//     #[error("quest {name} failed")]            // structs work too
//     pub struct QuestFailed {
//         pub name: String,
//         #[source]
//         pub cause: PotionError,
//     }
//
//   Walking the chain by hand uses `source()` in a loop:
//
//     let mut current: Option<&dyn Error> = Some(&err);
//     while let Some(e) = current {
//         println!("{e}");
//         current = e.source();
//     }
//
//   anyhow does the same with `err.chain()` and `{:#}`. Libraries expose
//   the chain; applications decide how to print it.
//
// 🎯 YOUR TASK
//
//   1. `QuestFailed` doesn't report its cause as the source. Fix that.
//   2. Implement `chain_messages`: every message, outermost first.

use std::error::Error as StdError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("potion {0} is empty")]
pub struct PotionError(pub String);

#[derive(Debug, Error)]
#[error("quest {name} failed")]
pub struct QuestFailed {
    pub name: String,
    pub cause: PotionError,
}

pub fn chain_messages(err: &(dyn StdError + 'static)) -> Vec<String> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn failed() -> QuestFailed {
        QuestFailed {
            name: "rescue".into(),
            cause: PotionError("heal".into()),
        }
    }

    #[test]
    fn chain() {
        assert_eq!(chain_messages(&failed()), vec!["quest rescue failed", "potion heal is empty"]);
        assert_eq!(chain_messages(&PotionError("x".into())), vec!["potion x is empty"]);
    }

    #[test]
    fn anyhow_sees_the_chain() {
        let err = anyhow::Error::from(failed());
        assert_eq!(format!("{err:#}"), "quest rescue failed: potion heal is empty");
        assert_eq!(err.chain().count(), 2);
    }
}
