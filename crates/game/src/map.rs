/// Represents a map object, which contains the locations
/// of all the types of blocks.
///
/// A Map is represented by the locations of all the blocks, 
/// in an 8x16 array, and surrounded by padding.
pub struct Map {
    woodblocks: u128,
    woodplanks: u128,
    iceblocks: u128,
    iceplanks: u128,
    slimeblocks: u128,
    slimeplanks: u128,
}

/// represents the width around the map (in numbers of blocks)
/// where the player is considered still alive.
pub const PADDING_WIDTH: usize = 3;


/// represents the height around the map (in numbers of blocks)
/// where the player is considered still alive.
pub const PADDING_HEIGHT: usize = 3;


impl Map {
    /// Constructs a new Map.
    ///
    /// Returns Error if map has overlapping blocks, or is completely empty.
    pub fn from_bits(woodblocks: u128,
                     woodplanks: u128,
                     iceblocks: u128,
                     iceplanks: u128,
                     slimeblocks: u128,
                     slimeplanks: u128) -> Result<Map, &'static str> {

        let data = [woodblocks, woodplanks, iceblocks, iceplanks, slimeblocks, slimeplanks];

        let overlaps = data.into_iter().fold(u128::MAX, |acc, x| {
            if x == 0 {
                acc
            } else {
                acc & x
            }
        });
        
        if overlaps == 0 {
            Ok(Map {
                woodblocks,
                woodplanks,
                iceblocks,
                iceplanks,
                slimeblocks,
                slimeplanks
            })
        }
        else {
            Err("Cannot have overlapping blocks")
        }
    }

    pub fn from_file() -> Result<Map, &'static str> {
        todo!();
    }
}
