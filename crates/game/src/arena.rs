use crate::map::Map;
use crate::block::BlockType;
use crate::block::BlockRect;
use crate::block::BLOCK_WIDTH;
use crate::block::BLOCK_HEIGHT;
use crate::map::{VERTICAL_PADDING, VERTICAL_BLOCK_SPACING};
use crate::map::VERTICAL_BLOCKS;
use crate::map::HORIZONTAL_BLOCKS;
use crate::map::HORIZONTAL_PADDING;
use crate::player::{Player, InputMask};
use glam::Vec2;

/// total width in pixels
/// (number of blocks horizontally + padding on both sides)
pub const ARENA_WIDTH: f32 = 2.0 * HORIZONTAL_PADDING + BLOCK_WIDTH * HORIZONTAL_BLOCKS as f32;

/// total height in pixels
/// (number of blocks vertically + padding above and below)
pub const ARENA_HEIGHT: f32 = 2.0 * VERTICAL_PADDING + VERTICAL_BLOCK_SPACING * VERTICAL_BLOCKS as f32;


/// represents the entire world of the game (entire map + players).
#[derive(Debug)]
pub struct Arena {
    map: Map,
    pub blocks: [Option<BlockType>; VERTICAL_BLOCKS * HORIZONTAL_BLOCKS],
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
        Self { map, player, blocks: blockrects }
    }

    /// obtains the block type at the specified row and column, or None if it doesn't exist.
    pub fn get_blocktype_at(&self, row: usize, col: usize) -> Option<BlockType> {
        self.blocks[col * VERTICAL_BLOCKS + row]
    }

    /// returns the top corner x and y coordinates of a block at row and col.
    pub fn get_block_position_at(&self, row: usize, col: usize) -> Vec2 {
        let y = VERTICAL_PADDING + VERTICAL_BLOCK_SPACING * row as f32;
        let x = HORIZONTAL_PADDING + BLOCK_WIDTH * col as f32;
        Vec2::new(x, y)
    }

    /// returns an iterable over the valid blocks.
    pub fn get_blocks_iter(&self) -> impl Iterator<Item=BlockRect> + '_ {
        let mut index = 0;

        return self.blocks.iter()
            .filter_map(move |blocktypeoption: &Option<BlockType>| {
                let (r, c) = (index % VERTICAL_BLOCKS, index / VERTICAL_BLOCKS);
                index += 1;

                let x: f32 = HORIZONTAL_PADDING + BLOCK_WIDTH * c as f32;
                let y: f32 = VERTICAL_PADDING + r as f32 * VERTICAL_BLOCK_SPACING;
                let w: f32 = BLOCK_WIDTH;
                let h: f32 = BLOCK_HEIGHT;

                if let Some(blocktype) = *blocktypeoption {
                    Some(BlockRect {x, y, w, h, blocktype})
                } else {
                    None
                }
            });
    }

    /// changes a point in the arena to the nearest row and column as represented by the map. If
    /// the point is within padding, then returns None.
    fn to_row_col(point: Vec2) -> Option<(usize, usize)> {
        return Some((0, 0));
    }

    /// Simulates the arena when delta time `dt` has passed.
    pub fn update(&mut self, dt: f32, input: InputMask) {

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
