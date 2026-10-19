use crate::dice::Die;
use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Roll {
    pub count: u32,
    pub die: Die,
    pub bonus: i32,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Empty,
    MissingD,
    BadNumber(String),
    BadDie(u32),
}

/// Parses "2d6+3", "d20", "1d4-1" and so on.
pub fn parse(text: &str) -> Result<Roll, ParseError> {
    todo!()
}

impl Roll {
    /// The lowest possible total (every die shows 1).
    pub fn min(&self) -> i32 {
        todo!()
    }

    /// The highest possible total (every die shows its max).
    pub fn max(&self) -> i32 {
        todo!()
    }

    /// Rolls every die and adds the bonus.
    pub fn roll(&self, rng: &mut impl Rng) -> i32 {
        todo!()
    }
}

impl fmt::Display for Roll {
    /// "2d6+3", "1d20", "1d4-1" (no bonus part when it is 0).
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}
