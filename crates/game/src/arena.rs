use glam::Vec2;

use crate::block::{BLOCK_TYPES_COUNT, BLOCK_WIDTH, BLOCK_HEIGHT};
use crate::player::Player;
use crate::map::{MapBits, Map, MAP_WIDTH, PADDING_WIDTH, PADDING_HEIGHT, MAP_HEIGHT};

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
    pub fn new(map: Map, players: Vec<Player>, player: Player) -> Result<Arena, String> {
        let gravity = map.get_gravity();

        // total width is (number of blocks horizontally + padding on both sides)
        let width = BLOCK_WIDTH * ((MAP_WIDTH as f32) + 2.0 * (PADDING_WIDTH as f32));
        // total height is (number of blocks vertically + padding on both sides)
        let height = BLOCK_HEIGHT * ((MAP_HEIGHT as f32) + 2.0 * (PADDING_HEIGHT as f32));
        Ok(Arena { map, players, player, gravity, width, height })
    }

    pub fn default() -> Result<Arena, String> {
        let mut data: [u128; BLOCK_TYPES_COUNT] = [0; BLOCK_TYPES_COUNT];
        data[0] = u128::MAX;
        let mapbits: MapBits = data.into();
        let map: Map = Map::default(mapbits)?;
        let players = Vec::new();

        Arena::new(map, players, Player::default())
    }

    pub fn from_file(filename: &str) -> Result<Arena, String> {
        let map = Map::read_from_file(filename)?;
        let players = Vec::new();
        Arena::new(map, players, Player::default())
    }

    /// Simulate the arena with time elapsed dt.
    pub fn update(&mut self, dt: f32) {
        // todo!();
    }
}
