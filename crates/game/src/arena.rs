use glam::Vec2;

use crate::{map::{self, Map, MapBits}, player::Player, block::{BLOCK_TYPES_COUNT, BlockType}};

#[derive(Debug)]
pub struct Arena {
    map: map::Map,
    players: Vec<Player>,
    gravity: Vec2,
}

impl Arena {
    pub fn new(map: Map, players: Vec<Player>) -> Result<Arena, String> {
        let gravity = map.get_gravity();
        Ok(Arena { map, players, gravity })
    }

    pub fn default() -> Result<Arena, String> {
        let mut data: [u128; BLOCK_TYPES_COUNT] = [0; BLOCK_TYPES_COUNT];
        data[BlockType::GrassPlank as usize] = u128::MAX;
        let mapbits: MapBits = data.into();
        let map: Map = Map::default(mapbits)?;
        let players = Vec::new();
        Arena::new(map, players)
    }

    pub fn from_file(filename: &str) -> Result<Arena, String> {
        let map = Map::read_from_file(filename)?;
        let players = Vec::new();
        Arena::new(map, players)
    }
}
