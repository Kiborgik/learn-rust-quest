//! The forge. Decide what the outside world may touch.

const MAX_SHARPNESS: u32 = 10;

pub struct Sword {
    pub name: String,
    sharpness: u32,
}

impl Sword {
    fn new(name: &str) -> Sword {
        Sword {
            name: name.to_string(),
            sharpness: 1,
        }
    }

    pub fn sharpness(&self) -> u32 {
        self.sharpness
    }

    fn sharpen(&mut self) {
        if self.sharpness < MAX_SHARPNESS {
            self.sharpness += 1;
        }
    }
}
