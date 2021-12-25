use crate::{map::Map, player::Player, block::{BLOCK_TYPES_COUNT, BlockRect, BlockType}};
use num_traits::FromPrimitive;

#[derive(Debug)]
pub struct Arena {
    map: Map,
    player: Player,
    blockrects: Vec<BlockRect>
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new(Map::default(), Player::default())
    }
}

impl Arena {
    pub fn new(map: Map, player: Player) -> Self {
        let mut blockrects: Vec<BlockRect> = Vec::new();

        for i in 0..BLOCK_TYPES_COUNT {
            let blocktype: BlockType = match FromPrimitive::from_usize(i) {
                Some(b) => b,
                _ => continue
            };

            let mapbits = map.get_bits_of_type(blocktype).unwrap();
            blockrects.append(&mut mapbits.to_block_rects(blocktype));
        }

        Self { map, player, blockrects }
    }

    pub fn update(dt: f32) {
        todo!();
    }
}
