use crate::map::{self, Map, AllBlocks};

#[derive(Debug)]
pub struct Arena {
    map: map::Map,
    padding_width: usize,
    padding_height: usize,
}

impl Arena {
    pub fn new() -> Arena {
        let allblocks: AllBlocks = [0, 0, 0, 0, 0, 0].into();

        match Map::from_bits(allblocks) {
            Ok(map) => Arena { map, padding_width: 3, padding_height: 3 },
            Err(e) => panic!("{}", e),
        }
    }
}
