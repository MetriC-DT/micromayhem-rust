use core::fmt;
use crate::block;

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

const INVALID_MAP: &str = "Invalid Map";

/// width of a map in blocks
const MAP_WIDTH: usize = 16;

/// height of a map in blocks
const MAP_HEIGHT: usize = 8;

/// bitmask for getting an entire row.
const ROWMASK: u128 = 1334440654591915542993625911497130241;

/// bitmask for getting an entire column.
const COLMASK: u128 = 1 << MAP_HEIGHT - 1;

/// Represents a map object, which contains the locations
/// of all the types of blocks.
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
    mapbits: MapBits
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let occupied = self.get_all_occupied();
        write!(f, "{}", Map::blocks_result_string(&occupied))
    }
}

impl Map {
    /// Constructs a new Map from map bits (array).
    ///
    /// Returns Error if map has overlapping blocks.
    pub fn from_bits(mapbits: MapBits) -> Result<Map, &'static str> {
        let MapBits(blocks) = mapbits;

        let nonzerocount = blocks.into_iter().filter(|&x| x != 0).count();
        if nonzerocount > 1 {
            let overlaps = blocks.into_iter().fold(u128::MAX, |acc, x| {
                if x == 0 {
                    acc
                } else {
                    acc & x
                }
            });
        
            if overlaps == 0 {
                Ok(Map {mapbits})
            }
            else {
                Err(INVALID_MAP)
            }
        } else {
            Ok(Map {mapbits})
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
