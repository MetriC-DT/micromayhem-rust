use crate::{map::Map, player::Player, block::{BlockType, BLOCK_HEIGHT, BLOCK_WIDTH}};
use crate::map::VERTICAL_PADDING;
use crate::map::VERTICAL_BLOCK_SPACING;
use crate::map::VERTICAL_BLOCKS;
use crate::map::HORIZONTAL_BLOCKS;
use crate::map::HORIZONTAL_PADDING;
use glam::Vec2;

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
    pub blockrects: [Option<BlockType>; VERTICAL_BLOCKS * HORIZONTAL_BLOCKS],
    pub player: Player,
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new(Map::default(), Player::default())
    }
}

impl Arena {
    pub fn new(map: Map, player: Player) -> Self {
        let blockrects = map.to_blocktypes();
        Self { map, player, blockrects }
    }

    /// obtains the block type at the specified row and column, or None if it doesn't exist.
    pub fn get_blocktype_at(&self, row: usize, col: usize) -> Option<BlockType> {
        self.blockrects[col * VERTICAL_BLOCKS + row]
    }

    /// Simulates the arena when delta time `dt` has passed.
    pub fn update(&mut self, dt: f32) {

        let total_mass = self.player.get_total_mass();
        let player_touching_block = true as u8 as f32;

        // TODO: calculates the acceleration experienced by the player, with all variables and
        // inputs accounted for.
        //
        // Considers forces from:
        // weight + gun recoil + block friction + block normal + WASD inputs + bullet hit.
        let weight = self.map.get_gravity() * total_mass;
        let gun_recoil = Vec2::ZERO;
        let block_friction = Vec2::ZERO;
        let block_normal = Vec2::ZERO;
        let bullet_hit = Vec2::ZERO;

        let total_force = weight + gun_recoil + block_friction + block_normal + bullet_hit;

        // TODO: find the y-location of the lowest block to plug into the second argument.
        self.player.update(dt, ARENA_HEIGHT, total_force);

        // TODO: Obtains the location of all the other players.
    }
}
