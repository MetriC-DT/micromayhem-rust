use core::fmt;
use crate::block;

/// Type alias to represent all positions occupied by the 8x16
/// grid of blocks of all types. Used internally.
type AllBlocksList = [u128; block::BLOCK_TYPES_COUNT];

/// Type alias for result that returns a map struct
type MapResult = Result<Map, InvalidMapError>;

/// Container for the type alias BlocksList
#[derive(Debug)]
pub struct AllBlocks(AllBlocksList);
impl From<AllBlocksList> for AllBlocks {
    fn from(lst: AllBlocksList) -> Self {
        AllBlocks(lst)
    }
}


/// represents the data obtained for a block of certain type on a Map
pub type BlocksResult = Result<u128, block::InvalidBlockTypeError>;


/// Error when map is unable to be constructed.
pub struct InvalidMapError;
impl fmt::Display for InvalidMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map is invalid")
    }
}

/// bitmask for getting an entire row.
const ROWMASK: u128 = 1334440654591915542993625911497130241;

/// width of a map in blocks
const MAP_WIDTH: usize = 16;

/// height of a map in blocks
const MAP_HEIGHT: usize = 8;


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
    allblocks: AllBlocks
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let occupied = self.get_all_occupied();
        write!(f, "{}", blocks_result_string(&occupied))
    }
}

/// Gets the string representation of a BlocksResult
pub fn blocks_result_string(b: &BlocksResult) -> String {
    let mut string_rep = String::new();
    for i in 0..MAP_HEIGHT {
        let mask: u128 = ROWMASK << i;
        let row = (b.as_ref().unwrap() & mask) >> i;
        for j in (0..MAP_WIDTH * MAP_HEIGHT).step_by(MAP_HEIGHT) {
            if (1 << j) & row != 0 {
                string_rep += "_";
            }
            else {
                string_rep += " ";
            }
        }
        string_rep += "\n";
    }

    return string_rep;
}

impl Map {
    /// Constructs a new Map from an allblocks array.
    ///
    /// Returns Error if map has overlapping blocks.
    pub fn from_bits(allblocks: AllBlocks) -> MapResult {
        let AllBlocks(blocks) = allblocks;

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
                Ok(Map {allblocks})
            }
            else {
                Err(InvalidMapError)
            }
        } else {
            Ok(Map {allblocks})
        }
    }

    /// Constructs a new map from data saved in a file.
    pub fn from_file(filename: &str) -> MapResult {
        todo!();
    }

    /// obtains the locations that are occupied by blocks of specified type
    pub fn get_blocks_of_type(&self, blocktype: block::BlockType) -> BlocksResult {
        let blockindex = blocktype as usize;
        let AllBlocks(allblocks) = self.allblocks;
        let result = if blockindex < block::BLOCK_TYPES_COUNT {
            Ok(allblocks[blockindex])
        }
        else {
            Err(block::InvalidBlockTypeError)
        };

        result
    }

    /// Obtains the locations that are occupied by blocks of any type.
    pub fn get_all_occupied(&self) -> BlocksResult {
        let AllBlocks(allblocks) = self.allblocks;
        let x = allblocks.iter().fold(0, |acc, x| {acc | x});
        Ok(x)
    }
}
