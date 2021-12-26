use crate::{map::Map, player::Player, block::{BlockRect, BlockType, BLOCK_HEIGHT, BLOCK_WIDTH}};
use crate::map::VERTICAL_PADDING;
use crate::map::VERTICAL_BLOCK_SPACING;
use crate::map::VERTICAL_BLOCKS;
use crate::map::HORIZONTAL_BLOCKS;
use crate::map::HORIZONTAL_PADDING;
use strum::IntoEnumIterator;

/// total width in pixels
/// (number of blocks horizontally + padding on both sides)
pub const ARENA_WIDTH: f32 = BLOCK_WIDTH * ((HORIZONTAL_BLOCKS as f32) + 2.0 * (HORIZONTAL_PADDING as f32));

/// total height in pixels
/// (number of blocks vertically + padding on both sides)
pub const ARENA_HEIGHT: f32 = BLOCK_HEIGHT * ((VERTICAL_BLOCKS * VERTICAL_BLOCK_SPACING) as f32 + 2.0 * (VERTICAL_PADDING as f32));


/// represents the entire world of the game (entire map + players).
#[derive(Debug)]
pub struct Arena {
    map: Map,
    pub player: Player,
    pub blockrects: Vec<Vec<BlockRect>>
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new(Map::default(), Player::default())
    }
}

impl Arena {
    pub fn new(map: Map, player: Player) -> Self {
        let blockrects = BlockType::iter()
            .map(|blocktype| {map.get_bits_of_type(blocktype).to_block_rects(blocktype)})
            .collect();

        println!("{}", map);
        Self { map, player, blockrects }
    }

    pub fn update(dt: f32) {
        todo!();
    }
}
