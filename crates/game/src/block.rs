use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter};
use BlockType::*;

/// Width of a block in pixels
pub const BLOCK_WIDTH: f32 = 128.0;

/// Height of a block in pixels
pub const BLOCK_HEIGHT: f32 = 20.0;


/// types of blocks
#[derive(Debug, Clone, Copy, EnumIter, EnumCount)]
pub enum BlockType {
    GrassBlock,
    IceBlock,
}

/// coefficient of frictions for various blocks
///
/// Accessing the frictions of a particular block is:
/// ```
/// use game::block::BlockType;
/// use game::block::BLOCK_FRICTIONS;
/// assert_eq!(BLOCK_FRICTIONS[BlockType::GrassBlock as usize], 1.0);
/// ```
pub const BLOCK_FRICTIONS: [f32; BlockType::COUNT] = {
    let mut frictions = [0.0; BlockType::COUNT];

    frictions[GrassBlock as usize] = 1.0;
    frictions[IceBlock as usize] = 0.5;
    frictions
};

/// block as a rectangle.
#[derive(Debug)]
pub struct BlockRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub blocktype: BlockType,
}
