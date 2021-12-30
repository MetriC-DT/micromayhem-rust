/// Represents the inputs a person can input to control the player of the arena.

pub enum Input {
    Left,
    Right,
    Up,
    Down,
    Shoot,
    Bomb,
    Throw
}

#[derive(Debug)]
pub struct InputMask(u16);

impl InputMask {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add_mask(&mut self, input: Input) {
        self.0 |= 1 << input as usize;
    }

    pub fn has_mask(&self, input: Input) -> bool {
        (self.0 & (1 << input as usize)) != 0
    }

    pub fn remove_mask(&mut self, input: Input) {
        self.0 &= !(1 << input as usize)
    }
}

impl Default for InputMask {
    fn default() -> Self {
        Self::new()
    }
}


