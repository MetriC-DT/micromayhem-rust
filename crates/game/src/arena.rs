use crate::map::{self, Map, MapBits};

#[derive(Debug)]
pub struct Arena {
    map: map::Map,
    padding_width: usize,
    padding_height: usize,
}

impl Arena {
    pub fn new(map: Map,
               padding_width: usize,
               padding_height: usize) -> Result<Arena, &'static str> {
        Ok(Arena {
            map,
            padding_width,
            padding_height
        })
    }

    pub fn default() -> Result<Arena, &'static str> {
        let mapbits: MapBits = [0, 0, 0, 0, 0, 0].into();
        let map: Map = Map::from_bits(mapbits)?;
        Arena::new(map, 3, 3)
    }

    pub fn from_file(filename: &str) -> Result<Arena, &'static str> {
        let map = Map::from_file(filename)?;
        Arena::new(map, 3, 3)
    }
}
