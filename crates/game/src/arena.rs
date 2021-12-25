use crate::{map::Map, player::Player, block::{BlockRect, BlockType}};
use strum::IntoEnumIterator;

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

    pub fn update(dt: f32) {
        todo!();
    }
}
