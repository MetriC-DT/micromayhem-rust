use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter};
use BlockType::*;

/// Width of a block in pixels
pub(crate) const BLOCK_WIDTH: f32 = 128.0;

/// Height of a block in pixels
pub(crate) const BLOCK_HEIGHT: f32 = 20.0;


/// types of blocks
#[derive(Debug, Clone, Copy, EnumIter, EnumCount)]
pub enum BlockType {
    GrassBlock,
    IceBlock,
}

/// coefficient of frictions for various blocks if you want to access the friction
/// of a specific blocktype, use the `get_block_friction(blocktype)` function.
///
/// Assumes static and kinetic friction is the same.
pub(crate) const BLOCK_FRICTIONS: [f32; BlockType::COUNT] = {
    let mut frictions = [0.0; BlockType::COUNT];

    frictions[GrassBlock as usize] = 0.7;
    frictions[IceBlock as usize] = 0.5;
    frictions
};

/// obtains the coefficient of friction of a block with type `blocktype`
pub(crate) fn get_block_friction(blocktype: BlockType) -> f32 {
    let i = blocktype as usize;
    return BLOCK_FRICTIONS[i];
}

/// represents a rectangle of the block.
pub struct BlockRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub blocktype: BlockType,
}
