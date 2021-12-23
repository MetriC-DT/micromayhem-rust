use core::fmt;
use crate::block;
use glam::Vec2;

/// Type alias to represent all positions occupied by the 8x16
/// grid of blocks of all types. Used internally.
type MapBitsList = [u128; block::BLOCK_TYPES_COUNT];

/// Bits used to construct a map.
#[derive(Debug)]
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
const MAP_WIDTH: usize = 16;

/// height of a map in blocks
const MAP_HEIGHT: usize = 8;

/// bitmask for getting an entire row.
const ROWMASK: u128 = 1334440654591915542993625911497130241;

/// bitmask for getting an entire column.
const COLMASK: u128 = 1 << MAP_HEIGHT - 1;

pub static GRAVITY_DEFAULT: [f32; 2] = [0.0, -10.0];
pub const PADDING_WIDTH_DEFAULT: usize = 3;
pub const PADDING_HEIGHT_DEFAULT: usize = 3;

/// Represents a map object, which contains the locations
/// of all the types of blocks, as well as the surrounding padding
/// and gravity.
///
/// A Map is represented by the locations of all the blocks, 
/// in an 8x16 array. The surrounding padding is part of the arena.
/// Look at the arena module for more details.
#[derive(Debug)]
pub struct Map {
    /// an array containing information about all the blocks. Each type
    /// of block is represented as a `u128` type, with 8 bits for each column
    /// for a total of 16 total columns.
    ///
    /// The types of blocks are batched into one array for convenience.
    /// In order to access the information for a specific block type,
    /// call the `get_blocks_of_type(BlockType)` method.
    mapbits: MapBits,
    padding_width: usize,
    padding_height: usize,
    gravity: Vec2,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let occupied = self.get_all_occupied();
        write!(f, "{}\nPadding_Width={}\nPadding_Height={}\nGravity={}",
               Map::blocks_result_string(&occupied),
               self.padding_width,
               self.padding_height,
               self.gravity)
    }
}

impl Map {
    /// Constructs a new map
    pub fn new(mapbits: MapBits, padding_width: usize, padding_height: usize, gravity: Vec2) -> Result<Map, &'static str> {
        let mapbits = Map::verify_mapbits(mapbits)?;
        Ok(Map { mapbits, padding_width, padding_height, gravity })
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

    /// Constructs a new map from data saved in a file.
    pub fn from_file(filename: &str) -> Result<Map, &'static str> {
        todo!();
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

#[test]
fn map_string_representation() {
    let data = [1<<127, 1<<126, 1<<105, 1<<2, 1<<80, 1<<8].into();
    let result = Map::new(data, PADDING_WIDTH_DEFAULT, PADDING_HEIGHT_DEFAULT, GRAVITY_DEFAULT.into()).unwrap();
    let stringrep = "0100000000100000\n0000000000000100\n1000000000000000\n0000000000000000\n0000000000000000\n0000000000000000\n0000000000000001\n0000000000000001\n";
    assert_eq!(stringrep, Map::blocks_result_string(&result.get_all_occupied()));
}
