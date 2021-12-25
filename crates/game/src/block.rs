use core::fmt;
use BlockType::*;
use num_derive::{FromPrimitive, ToPrimitive};

type BlockTypeResult<T> = Result<T, InvalidBlockTypeError>;


/// Width of a block in pixels
pub const BLOCK_WIDTH: f32 = 30.0;

/// Height of a block in pixels
pub const BLOCK_HEIGHT: f32 = 20.0;


trait Block {
    fn get_friction(&self) -> BlockTypeResult<f32>;
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum BlockType {
    GrassBlock,
    IceBlock,

    // required to count number of enum elements.
    // should never use to represent an actual block type
    Total
}

/// number of different types of blocks
pub const BLOCK_TYPES_COUNT: usize = BlockType::Total as usize;

/// called whenever an invalid block is used (e.g. Total).
#[derive(Debug)]
pub struct InvalidBlockTypeError;

impl fmt::Display for InvalidBlockTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot use block {:?}", self)
    }
}

impl Block for BlockType {
    /// Obtains the coefficient of friction for each type.
    fn get_friction(&self) -> BlockTypeResult<f32> {
        match self {
            GrassBlock => Ok(1.0),
            IceBlock => Ok(0.5),
            _ => Err(InvalidBlockTypeError)
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
