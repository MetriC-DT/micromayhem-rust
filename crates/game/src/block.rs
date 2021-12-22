use core::fmt;

use BlockType::*;


type BlockTypeResult<T> = Result<T, InvalidBlockTypeError>;


/// Width of a block
pub const BLOCK_WIDTH: u32 = 30;

/// Height of a block
pub const BLOCK_HEIGHT: u32 = 30;

trait Block {
    fn get_friction(&self) -> BlockTypeResult<f32>;
    fn get_stickiness(&self) -> BlockTypeResult<f32>;
    fn is_solid(&self) -> BlockTypeResult<bool>;
}

pub enum BlockType {
    WoodPlank,
    WoodBlock,

    IcePlank,
    IceBlock,

    SlimePlank,
    SlimeBlock,

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
            WoodPlank | WoodBlock => Ok(1.0),

            IcePlank | IceBlock => Ok(0.5),

            SlimeBlock | SlimePlank => Ok(1.0),

            Total => Err(InvalidBlockTypeError)
        }
    }

    /// Obtains the stickiness for each type.
    ///
    /// Stickiness directly modifies the velocity of 
    /// the player on the block.
    fn get_stickiness(&self) -> BlockTypeResult<f32> {
        match self {
            WoodPlank => todo!(),
            WoodBlock => todo!(),

            IcePlank => todo!(),
            IceBlock => todo!(),

            SlimePlank => todo!(),
            SlimeBlock => todo!(),

            Total => Err(InvalidBlockTypeError)
        }
    }


    /// Determines whether a player can phase through
    /// a block. Players cannot phase through solid blocks
    fn is_solid(&self) -> BlockTypeResult<bool> {
        match self {
            WoodPlank => Ok(false),
            WoodBlock => Ok(true),

            IcePlank => Ok(false),
            IceBlock => Ok(true),

            SlimePlank => Ok(false),
            SlimeBlock => Ok(true),

            Total => Err(InvalidBlockTypeError)
        }
    }
}
