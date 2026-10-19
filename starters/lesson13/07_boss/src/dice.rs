use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Die {
    sides: u32,
}

impl Die {
    /// A die needs at least 2 sides.
    pub fn new(sides: u32) -> Option<Die> {
        todo!()
    }

    pub fn sides(&self) -> u32 {
        self.sides
    }

    /// A number from 1 to `sides`.
    pub fn roll(&self, rng: &mut impl Rng) -> u32 {
        todo!()
    }
}
