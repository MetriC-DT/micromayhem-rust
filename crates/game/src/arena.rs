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

        Self { map, player, blockrects }
    }

    /// Simulates the arena when time dt has passed.
    pub fn update(&mut self, dt: f32) {
        // TODO: calculates the acceleration experienced by the player, with all inputs accounted for.
        //
        // Considers forces from:
        // gun recoil + weight + block friction + block normal + jump inputs
        self.player.set_acceleration(self.map.get_gravity());

        // TODO: find the y-location of the lowest block to plug into the second argument.
        self.player.update(dt, ARENA_HEIGHT);

        // TODO: Obtains the location of all the other players.
    }
}
