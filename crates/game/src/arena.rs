use crate::map::{self, Map, MapBits, PADDING_WIDTH_DEFAULT, PADDING_HEIGHT_DEFAULT, GRAVITY_DEFAULT};

#[derive(Debug)]
pub struct Arena {
    map: map::Map,
}

impl Arena {
    pub fn new(map: Map) -> Result<Arena, &'static str> {
        Ok(Arena { map })
    }

    pub fn default() -> Result<Arena, &'static str> {
        let mapbits: MapBits = [0, 0, 0, 0, 0, 0].into();
        let map: Map = Map::new(mapbits, PADDING_WIDTH_DEFAULT, PADDING_HEIGHT_DEFAULT, GRAVITY_DEFAULT.into())?;
        Arena::new(map)
    }

    pub fn from_file(filename: &str) -> Result<Arena, &'static str> {
        let map = Map::from_file(filename)?;
        Arena::new(map)
    }
}
