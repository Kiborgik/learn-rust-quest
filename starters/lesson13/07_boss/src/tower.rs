use crate::notation::{ParseError, Roll, parse};
use rand::Rng;

#[derive(Debug)]
pub struct Tower {
    pub rolls: Vec<Roll>,
}

impl Tower {
    /// Comma-separated rolls, for example "2d6+3, d20".
    /// The first bad roll's error is returned.
    pub fn parse(text: &str) -> Result<Tower, ParseError> {
        todo!()
    }

    /// The sum of every roll's maximum.
    pub fn max_total(&self) -> i32 {
        todo!()
    }

    /// Rolls each entry once, in order.
    pub fn roll_all(&self, rng: &mut impl Rng) -> Vec<i32> {
        todo!()
    }
}
