use core::fmt;
use std::fmt::write;
use crate::block;

/// Type alias to represent all positions occupied by the 8x16
/// grid of blocks of all types. Used internally.
type AllBlocksList = [u128; block::BLOCK_TYPES_COUNT];

/// Type alias for result that returns a map struct
type MapResult = Result<Map, BlocksOverlapError>;

/// Container for the type alias BlocksList
pub struct AllBlocks(AllBlocksList);
impl From<AllBlocksList> for AllBlocks {
    fn from(lst: AllBlocksList) -> Self {
        AllBlocks(lst)
    }
}


/// represents the data obtained for a block of certain type on a Map
pub struct BlocksResult(Result<u128, block::InvalidBlockTypeError>);


/// Error for when blocks overlap
pub struct BlocksOverlapError;
impl fmt::Display for BlocksOverlapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot have overlapping blocks")
    }
}


/// Represents a map object, which contains the locations
/// of all the types of blocks.
///
/// A Map is represented by the locations of all the blocks, 
/// in an 8x16 array. The surrounding padding is part of the arena.
/// Look at the arena module for more details.
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


impl Map {
    /// Constructs a new Map from an allblocks array.
    ///
    /// Returns Error if map has overlapping blocks, or is completely empty.
    pub fn from_bits(allblocks: AllBlocks) -> MapResult {
        let AllBlocks(blocks) = allblocks;
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
            Err(BlocksOverlapError)
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

        BlocksResult(result)
    }

    /// Obtains the locations that are occupied by blocks of any type.
    pub fn get_all_occupied(&self) -> BlocksResult {
        let AllBlocks(allblocks) = self.allblocks;
        let x = allblocks.iter().fold(0, |acc, x| {acc | x});
        BlocksResult(Ok(x))
    }
}
