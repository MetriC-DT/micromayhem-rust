use core::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use crate::block;
use crate::block::BLOCK_TYPES_COUNT;
use crate::block::BlockType;
use crate::block::InvalidBlockTypeError;
use glam::const_vec2;
use glam::Vec2;
use bincode::deserialize_from;
use bincode::serialize_into;
use serde::{Serialize , Deserialize};

/// Type alias to represent all positions occupied by the 8x16
/// grid of blocks of all types. Used internally.
type MapBlocksList = [u128; block::BLOCK_TYPES_COUNT];

/// Bits used to construct a map.
#[derive(Debug, Serialize, Deserialize)]
pub struct MapBlocks(MapBlocksList);

impl From<MapBlocksList> for MapBlocks {
    fn from(lst: MapBlocksList) -> Self {
        MapBlocks(lst)
    }
}

/// represents the data obtained for a block of certain type on a Map
///
/// LSB is leftmost, and runs column-wise. Bitwise representation is as follows:
/// 0 8  ... 120
/// 1 9  ... 121
/// 2 10 ... 122
/// 3 11 ... 123
/// 4 12 ... 124
/// 5 13 ... 125
/// 6 14 ... 126
/// 7 15 ... 127
pub struct MapBits(u128);

/// MapBits represented as a string.
impl fmt::Display for MapBits {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let MapBits(bits) = self;

        let mut string_rep = String::new();
        for i in 0..MAP_HEIGHT {
            let mask: u128 = ROWMASK << i;
            let row = (bits & mask) >> i;

            for j in 0..MAP_WIDTH {
                let mask2: u128 = 1 << (j * MAP_HEIGHT);
                if row & mask2 != 0 {
                    string_rep += "1";
                } else {
                    string_rep += "0";
                }
            }
            string_rep += "\n";
        }

        write!(f, "{}", string_rep)
    }
}

/// Error message for invalid map.
///
/// TODO: refactor into actual enum of different errors.
const INVALID_MAP: &str = "Invalid Map";

/// width of a map in blocks
pub const MAP_WIDTH: usize = 16;

/// height of a map in blocks
pub const MAP_HEIGHT: usize = 8;

/// bitmask for getting an entire row.
const ROWMASK: u128 = 1334440654591915542993625911497130241;

/// bitmask for getting an entire column.
const COLMASK: u128 = 1 << MAP_HEIGHT - 1;

pub const GRAVITY_DEFAULT: Vec2 = const_vec2!([0.0, -10.0]);

/// horizontal padding of map in number of blocks
/// This is the region around where player is considered to be alive.
pub const PADDING_WIDTH: usize = 3;

/// vertical padding of map in number of blocks.
/// This is the region around where player is considered to be alive.
pub const PADDING_HEIGHT: usize = 3;

/// Represents a map object, which contains the locations
/// of all the types of blocks, as well as the surrounding padding
/// and gravity.
///
/// A Map is represented by the locations of all the blocks, 
/// in an 8x16 array. The surrounding padding is part of the arena.
/// Look at the arena module for more details.
#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    /// an array containing information about all the blocks. Each type
    /// of block is represented as a `u128` type, with 8 bits for each column
    /// for a total of 16 total columns.
    ///
    /// The types of blocks are batched into one array for convenience.
    /// In order to access the information for a specific block type,
    /// call the `get_blocks_of_type(BlockType)` method.
    mapblocks: MapBlocks,

    /// gravity vector.
    gravity: [f32; 2],
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let occupied = self.get_all_occupied();
        write!(f, "{}\nGravity={:?}",
               occupied.to_string(),
               self.gravity)
    }
}

impl Default for Map {

    /// creates a default map. Used only for testing.
    fn default() -> Map {
        let mut data: [u128; BLOCK_TYPES_COUNT] = [0; BLOCK_TYPES_COUNT];
        data[BlockType::GrassBlock as usize] = u128::MAX;
        let mapblocks: MapBlocks = data.into();

        Map::new(mapblocks, GRAVITY_DEFAULT).unwrap()
    }

}

impl Map {
    /// Constructs a new map
    pub fn new(mapblocks: MapBlocks, gravity: Vec2) -> Result<Map, String> {
        let mapblocks = Map::verify_mapblocks(mapblocks)?;
        Ok(Map { mapblocks, gravity: gravity.to_array() })
    }

    pub fn from_mapblocks(mapblocks: MapBlocks) -> Result<Map, String> {
        Map::new(mapblocks, GRAVITY_DEFAULT)
    }

    /// Constructs a new map from data saved in a file.
    pub fn read_from_file(filename: &str) -> Result<Map, String> {
        match File::open(filename) {
            Ok(f) => {
                let reader = BufReader::new(f);
                match deserialize_from::<_, Map>(reader) {
                    Ok(map) => Ok(map),
                    Err(e) => Err(e.to_string())
                }
            },
            Err(e) => Err(e.to_string())
        }
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        let mut f = BufWriter::new(File::create(filename)?);
        match serialize_into(&mut f, &self) {
            Ok(_) => Ok(()),
            Err(_) => todo!(),
        }
    }

    /// verifies if mapblocks can form a legal map.
    ///
    /// A legal map is defined as a map with no overlapping blocks.
    fn verify_mapblocks(mapblocks: MapBlocks) -> Result<MapBlocks, &'static str> {
        let MapBlocks(bits) = mapblocks;
        let nonzerocount = bits.into_iter().filter(|&x| x != 0).count();
        if nonzerocount > 1 {
            let overlaps = bits.into_iter().fold(u128::MAX, |acc, x| {
                if x == 0 {
                    acc
                } else {
                    acc & x
                }
            });
        
            if overlaps == 0 {
                Ok(MapBlocks(bits))
            }
            else {
                Err(INVALID_MAP)
            }
        } else {
            Ok(MapBlocks(bits))
        }
    }

    /// obtains the gravity associated with the map.
    pub fn get_gravity(&self) -> Vec2 {
        self.gravity.into()
    }

    /// obtains the locations that are occupied by blocks of specified type
    pub fn get_bits_of_type(&self, blocktype: block::BlockType) -> Result<MapBits, InvalidBlockTypeError> {
        let blockindex = blocktype as usize;
        let MapBlocks(mapblocks) = self.mapblocks;
        let result = if blockindex < block::BLOCK_TYPES_COUNT {
            Ok(MapBits(mapblocks[blockindex]))
        }
        else {
            Err(block::InvalidBlockTypeError)
        };

        result
    }

    /// Obtains the locations that are occupied by blocks of any type.
    pub fn get_all_occupied(&self) -> MapBits {
        let MapBlocks(mapblocks) = self.mapblocks;
        let x = mapblocks.iter().fold(0, |acc, x| {acc | x});
        MapBits(x)
    }
}
