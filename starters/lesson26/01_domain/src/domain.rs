// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 26-1 · The Charter                               200 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 THE CAPSTONE
//
//   This lesson builds one real program, "guildboard": a quest board with
//   a database, a REST API, a client and a CLI. Each quest adds a layer.
//   The layers you already wrote come with the next quest, solved.
//
//   Start in the middle: the DOMAIN. Plain types and rules, no database,
//   no HTTP, no async. Everything else calls into it, and because it does
//   no IO it's trivial to test.
//
//     fn validate(self) -> Result<NewQuest, BoardError>   // check input once,
//                                                         // at the edge
//     fn finish(&mut self) -> Result<i64, BoardError>     // state change + rule
//
//   One error enum (thiserror, lesson 14) names every way the rules can
//   say no. Later layers turn it into HTTP codes and CLI messages.
//
// 🎯 YOUR TASK
//
//   1. `NewQuest::validate`: trim the title. Empty → EmptyTitle. Reward
//      outside 1..=MAX_REWARD → BadReward(reward).
//   2. `Quest::finish`: AlreadyDone(id) if done, else mark it done and
//      return the reward.
//   3. `Summary::of`: count open and done quests, and add up the gold
//      paid (done) and the gold waiting (open).

use serde::{Deserialize, Serialize};

pub const MAX_REWARD: i64 = 1000;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quest {
    pub id: i64,
    pub title: String,
    pub reward: i64,
    pub done: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewQuest {
    pub title: String,
    pub reward: i64,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    pub open: usize,
    pub done: usize,
    pub gold_paid: i64,
    pub gold_waiting: i64,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum BoardError {
    #[error("title can't be empty")]
    EmptyTitle,
    #[error("reward must be 1..={MAX_REWARD} gold, got {0}")]
    BadReward(i64),
    #[error("quest {0} not found")]
    NotFound(i64),
    #[error("quest {0} is already done")]
    AlreadyDone(i64),
}

impl NewQuest {
    pub fn validate(self) -> Result<NewQuest, BoardError> {
        todo!()
    }
}

impl Quest {
    pub fn finish(&mut self) -> Result<i64, BoardError> {
        todo!()
    }
}

impl Summary {
    pub fn of(quests: &[Quest]) -> Summary {
        todo!()
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn new(title: &str, reward: i64) -> NewQuest {
        NewQuest { title: title.into(), reward }
    }

    fn quest(id: i64, reward: i64, done: bool) -> Quest {
        Quest { id, title: format!("quest {id}"), reward, done }
    }

    #[test]
    fn validating() {
        assert_eq!(new("  Slay the slime ", 50).validate(), Ok(new("Slay the slime", 50)));
        assert_eq!(new("   ", 50).validate(), Err(BoardError::EmptyTitle));
        assert_eq!(new("Free work", 0).validate(), Err(BoardError::BadReward(0)));
        assert_eq!(new("Too rich", 5000).validate(), Err(BoardError::BadReward(5000)));
        assert!(new("Max", MAX_REWARD).validate().is_ok());
    }

    #[test]
    fn messages() {
        assert_eq!(BoardError::BadReward(0).to_string(), "reward must be 1..=1000 gold, got 0");
    }

    #[test]
    fn finishing() {
        let mut q = quest(3, 40, false);
        assert_eq!(q.finish(), Ok(40));
        assert!(q.done);
        assert_eq!(q.finish(), Err(BoardError::AlreadyDone(3)));
    }

    #[test]
    fn summary() {
        let quests = [quest(1, 10, true), quest(2, 20, false), quest(3, 30, true), quest(4, 5, false)];
        let s = Summary::of(&quests);
        assert_eq!(s, Summary { open: 2, done: 2, gold_paid: 40, gold_waiting: 25 });
        assert_eq!(Summary::of(&[]), Summary::default());
    }

    #[test]
    fn json_shape() {
        let json = serde_json::to_value(quest(1, 10, false)).unwrap();
        assert_eq!(json, serde_json::json!({"id": 1, "title": "quest 1", "reward": 10, "done": false}));
    }
}
