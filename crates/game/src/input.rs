use std::fmt;

/// Represents the inputs a person can input to control the player of the arena.

pub enum Input {
    Left,
    Right,
    Up,
    Down,
    Shoot,
    Bomb,
    Throw,
}

#[derive(Debug, Clone, Copy)]
pub struct InputMask(u8);

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

impl fmt::Display for InputMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl From<u8> for InputMask {
    fn from(inputdata: u8) -> Self {
        Self(inputdata)
    }
}

impl From<InputMask> for u8 {
    fn from(inputmask: InputMask) -> Self {
        inputmask.0
    }
}
