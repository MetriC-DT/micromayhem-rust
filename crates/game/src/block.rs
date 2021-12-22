use BlockType::*;

/// Width of a block
pub const BLOCK_WIDTH: u32 = 30;

/// Height of a block
pub const BLOCK_HEIGHT: u32 = 30;

trait Block {
    fn get_friction(&self) -> f32;
    fn get_stickiness(&self) -> f32;
    fn is_solid(&self) -> bool;
}

pub enum BlockType {
    WoodPlank,
    WoodBlock,

    IcePlank,
    IceBlock,

    SlimePlank,
    SlimeBlock,
}

impl Block for BlockType {

    /// Obtains the coefficient of friction for each type.
    ///
    /// Used for "Slippery" feeling on blocks.
    fn get_friction(&self) -> f32 {
        match self {
            WoodPlank => 1.0,
            WoodBlock => 1.0,

            IcePlank => todo!(),
            IceBlock => todo!(),

            SlimePlank => todo!(),
            SlimeBlock => todo!(),
        }
    }

    /// Obtains the stickiness for each type.
    ///
    /// Stickiness directly modifies the velocity of 
    /// the player on the block.
    fn get_stickiness(&self) -> f32 {
        match self {
            WoodPlank => todo!(),
            WoodBlock => todo!(),
            IcePlank => todo!(),
            IceBlock => todo!(),
            SlimePlank => todo!(),
            SlimeBlock => todo!(),
        }
    }

    /// Determines whether a player can phase through
    /// a block. Players cannot phase through solid blocks
    fn is_solid(&self) -> bool {
        match self {
            WoodPlank => false,
            WoodBlock => true,

            IcePlank => false,
            IceBlock => true,

            SlimePlank => false,
            SlimeBlock => true,
        }
    }
}
