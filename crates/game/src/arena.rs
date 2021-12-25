use glam::Vec2;

use crate::block::{BlockType, BLOCK_TYPES_COUNT, BLOCK_WIDTH, BLOCK_HEIGHT};
use crate::player::Player;
use crate::map::{Map, MAP_WIDTH, PADDING_WIDTH, PADDING_HEIGHT, MAP_HEIGHT};

#[derive(Debug)]
pub struct Arena {
    map: Map,
    players: Vec<Player>,
    player: Player,
    pub gravity: Vec2,
    pub width: f32,
    pub height: f32,
}

impl Arena {
    pub fn new(map: Map, players: Vec<Player>, player: Player) -> Self {
        let gravity = map.get_gravity();

        // total width is (number of blocks horizontally + padding on both sides)
        let width = BLOCK_WIDTH * ((MAP_WIDTH as f32) + 2.0 * (PADDING_WIDTH as f32));
        // total height is (number of blocks vertically + padding on both sides)
        let height = BLOCK_HEIGHT * ((MAP_HEIGHT as f32) + 2.0 * (PADDING_HEIGHT as f32));
        Arena { map, players, player, gravity, width, height }
    }


    pub fn from_file(filename: &str) -> Result<Arena, String> {
        let map = Map::read_from_file(filename)?;
        let players = Vec::new();
        Ok(Arena::new(map, players, Player::default()))
    }

    /// Simulate the arena with time elapsed dt.
    pub fn update(&mut self, dt: f32) {
        // todo!();
    }
}

/// default arena for testing.
impl Default for Arena {
    fn default() -> Arena {
        let players: Vec<Player> = Vec::new();
        Arena::new(Map::default(), players, Player::default())
    }
}
