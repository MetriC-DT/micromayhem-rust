use core::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use crate::block;
use bincode::deserialize_from;
use bincode::serialize_into;
use glam::const_vec2;
use glam::Vec2;
use serde::{Serialize , Deserialize};

/// Type alias to represent all positions occupied by the 8x16
/// grid of blocks of all types. Used internally.
type MapBitsList = [u128; block::BLOCK_TYPES_COUNT];

/// Bits used to construct a map.
#[derive(Debug, Serialize, Deserialize)]
pub struct MapBits(MapBitsList);
impl From<MapBitsList> for MapBits {
    fn from(lst: MapBitsList) -> Self {
        MapBits(lst)
    }
}

/// represents the data obtained for a block of certain type on a Map
pub type BlocksResult = Result<u128, block::InvalidBlockTypeError>;

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
    mapbits: MapBits,

    /// gravity vector.
    gravity: [f32; 2],
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let occupied = self.get_all_occupied();
        write!(f, "{}\nGravity={:?}",
               Map::blocks_result_string(&occupied),
               self.gravity)
    }
}

impl Map {
    /// Constructs a new map
    pub fn new(mapbits: MapBits, gravity: Vec2) -> Result<Map, String> {
        let mapbits = Map::verify_mapbits(mapbits)?;
        Ok(Map { mapbits, gravity: gravity.to_array() })
    }

    /// creates a default map. Used only for testing.
    pub fn default(mapbits: MapBits) -> Result<Map, String> {
        Map::new(mapbits, GRAVITY_DEFAULT)
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

    /// verifies if mapbits can form a legal map.
    ///
    /// A legal map is defined as a map with no overlapping blocks.
    fn verify_mapbits(mapbits: MapBits) -> Result<MapBits, &'static str> {
        let MapBits(bits) = mapbits;
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
                Ok(MapBits(bits))
            }
            else {
                Err(INVALID_MAP)
            }
        } else {
            Ok(MapBits(bits))
        }
    }

    /// obtains the gravity associated with the map.
    pub fn get_gravity(&self) -> Vec2 {
        self.gravity.into()
    }

    /// obtains the locations that are occupied by blocks of specified type
    pub fn get_blocks_of_type(&self, blocktype: block::BlockType) -> BlocksResult {
        let blockindex = blocktype as usize;
        let MapBits(mapbits) = self.mapbits;
        let result = if blockindex < block::BLOCK_TYPES_COUNT {
            Ok(mapbits[blockindex])
        }
        else {
            Err(block::InvalidBlockTypeError)
        };

        result
    }

    /// Obtains the locations that are occupied by blocks of any type.
    pub fn get_all_occupied(&self) -> BlocksResult {
        let MapBits(mapbits) = self.mapbits;
        let x = mapbits.iter().fold(0, |acc, x| {acc | x});
        Ok(x)
    }

    /// Gets the string representation of a BlocksResult
    fn blocks_result_string(b: &BlocksResult) -> String {
        let mut string_rep = String::new();
        for i in 0..MAP_HEIGHT {
            let mask: u128 = ROWMASK << i;
            let row = (b.as_ref().unwrap() & mask) >> i;

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

        string_rep
    }
}
