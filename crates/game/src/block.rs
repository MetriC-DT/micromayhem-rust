use BlockType::*;
use strum_macros::{EnumCount, EnumIter};

/// Width of a block in pixels
pub const BLOCK_WIDTH: f32 = 128.0;

/// Height of a block in pixels
pub const BLOCK_HEIGHT: f32 = 20.0;

trait Block {
    fn get_friction(&self) -> f32;
}

#[derive(Debug, Clone, Copy, EnumIter, EnumCount)]
pub enum BlockType {
    GrassBlock,
    IceBlock,
}

impl Block for BlockType {
    /// Obtains the coefficient of friction for each type.
    fn get_friction(&self) -> f32 {
        match self {
            GrassBlock => 1.0,
            IceBlock => 0.5,
        }
    }
}

/// block as a rectangle.
#[derive(Debug)]
pub struct BlockRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub blocktype: BlockType,
}
