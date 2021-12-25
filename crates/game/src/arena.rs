use crate::{map::{Map, HORIZONTAL_BLOCKS, HORIZONTAL_PADDING, VERTICAL_PADDING, VERTICAL_BLOCKS}, player::Player, block::{BLOCK_HEIGHT, BLOCK_WIDTH}};

/// represents the entire world of the game (entire map + players).

// total width is (number of blocks horizontally + padding on both sides)
pub const ARENA_WIDTH: f32 = BLOCK_WIDTH * ((HORIZONTAL_BLOCKS as f32) + 2.0 * (HORIZONTAL_PADDING as f32));

// total height is (number of blocks vertically + padding on both sides)
pub const ARENA_HEIGHT: f32 = BLOCK_HEIGHT * ((VERTICAL_BLOCKS as f32) + 2.0 * (VERTICAL_PADDING as f32));

#[derive(Debug)]
pub struct Arena {
    map: Map,
    player: Player,
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new(Map::default(), Player::default())
    }
}

impl Arena {
    pub fn new(map: Map, player: Player) -> Self {
        Self { map, player }
    }

    pub fn update(dt: f32) {
        todo!();
    }
}
