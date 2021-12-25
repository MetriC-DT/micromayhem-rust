use core::fmt;

use BlockType::*;


type BlockTypeResult<T> = Result<T, InvalidBlockTypeError>;


/// Width of a block in pixels
pub const BLOCK_WIDTH: f32 = 30.0;

/// Height of a block in pixels
pub const BLOCK_HEIGHT: f32 = 20.0;

trait Block {
    fn get_friction(&self) -> BlockTypeResult<f32>;
    fn get_stickiness(&self) -> BlockTypeResult<f32>;
}

pub enum BlockType {
    GrassBlock,

    IceBlock,

    MudBlock,

    // required to count number of enum elements.
    // should never use to represent an actual block type
    Total
}

/// number of different types of blocks
pub const BLOCK_TYPES_COUNT: usize = BlockType::Total as usize;

/// called whenever an invalid block is used (e.g. Total).
#[derive(Debug, Clone)]
pub struct InvalidBlockTypeError;

impl fmt::Display for InvalidBlockTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot use block {:?}", self)
    }
}


impl Block for BlockType {

    /// Obtains the coefficient of friction for each type.
    ///
    /// Used for "Slippery" feeling on blocks.
    fn get_friction(&self) -> BlockTypeResult<f32> {
        match self {
            GrassBlock => Ok(1.0),
            IceBlock => Ok(0.5),
            MudPlank => Ok(1.0),

            Total => Err(InvalidBlockTypeError)
        }
    }

    /// Obtains the stickiness for each type.
    ///
    /// Stickiness directly modifies the velocity of 
    /// the player on the block.
    fn get_stickiness(&self) -> BlockTypeResult<f32> {
        match self {
            GrassBlock => todo!(),
            IceBlock => todo!(),
            MudBlock => todo!(),
            Total => Err(InvalidBlockTypeError)
        }
    }
}
