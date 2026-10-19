//! Domain: the rules of the board. No IO. (Given: you wrote this in 26-1.)

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
        let title = self.title.trim();
        if title.is_empty() {
            return Err(BoardError::EmptyTitle);
        }
        if !(1..=MAX_REWARD).contains(&self.reward) {
            return Err(BoardError::BadReward(self.reward));
        }
        Ok(NewQuest { title: title.to_string(), reward: self.reward })
    }
}

impl Quest {
    pub fn finish(&mut self) -> Result<i64, BoardError> {
        if self.done {
            return Err(BoardError::AlreadyDone(self.id));
        }
        self.done = true;
        Ok(self.reward)
    }
}

impl Summary {
    pub fn of(quests: &[Quest]) -> Summary {
        let mut s = Summary::default();
        for q in quests {
            if q.done {
                s.done += 1;
                s.gold_paid += q.reward;
            } else {
                s.open += 1;
                s.gold_waiting += q.reward;
            }
        }
        s
    }
}
