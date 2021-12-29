use core::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use crate::block;
use crate::block::BLOCK_HEIGHT;
use crate::block::BlockType;
use glam::const_vec2;
use glam::Vec2;
use bincode::deserialize_from;
use bincode::serialize_into;
use serde::{Serialize , Deserialize};
use strum::EnumCount;
use strum::IntoEnumIterator;


/// Error message for invalid map.
///
/// TODO: refactor into actual enum of different errors.
const INVALID_MAP: &str = "Invalid Map";

/// width of a map in blocks
pub const HORIZONTAL_BLOCKS: usize = 16;

/// height of a map in blocks
pub const VERTICAL_BLOCKS: usize = 8;

/// bitmask for getting an entire row.
const ROWMASK: i128 = {
    let mut bitmask = 0;
    let mut i = 0;

    // for loops not allowed inside a const.
    while i < HORIZONTAL_BLOCKS {
        bitmask |= 1 << (i * VERTICAL_BLOCKS);
        i += 1;
    }

    bitmask
};

/// bitmask for getting an entire column.
const COLMASK: i128 = (1 << VERTICAL_BLOCKS) - 1;

/// default gravity limit (positive orientation is downwards).
pub(crate) const GRAVITY_DEFAULT: Vec2 = const_vec2!([0.0, 3000.0]);

/// horizontal padding of map in number of blocks
/// This is the region around where player is considered to be alive.
pub(crate) const HORIZONTAL_PADDING: f32 = 200.0;

/// vertical padding of map in number of blocks.
/// This is the region around where player is considered to be alive.
pub(crate) const VERTICAL_PADDING: f32 = 200.0;

/// vertical spacing in numbers of vertical blocks of spacing
pub(crate) const VERTICAL_BLOCK_SPACING: f32 = 100.0;

/// Type alias to represent all positions occupied by the 8x16
/// grid of blocks of all types. Used internally.
pub(crate) type MapBlocksList = [i128; BlockType::COUNT];


/// Bits used to construct a map.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MapBlocks(MapBlocksList);

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
pub(crate) struct MapBits(i128);

/// MapBits represented as a string.
impl fmt::Display for MapBits {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bits = self.0;

        let mut string_rep = String::new();
        for i in 0..VERTICAL_BLOCKS {
            let mask: i128 = ROWMASK << i;
            let row = (bits & mask) >> i;

            for j in 0..HORIZONTAL_BLOCKS {
                let mask2: i128 = 1 << (j * VERTICAL_BLOCKS);
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
    /// of block is represented as a `i128` type (the MapBits struct), 
    /// with 8 bits for each column for a total of 16 total columns.
    ///
    /// The types of blocks are batched into one MapBlocks array for convenience.
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
               occupied,
               self.gravity)
    }
}

impl Default for Map {
    /// creates a default map. Used only for testing.
    ///
    /// alternates between all of the blocks of all types on each row,
    /// covering all possible block locations. e.g. row 1 is all grass,
    /// row 2 is all ice, etc...
    fn default() -> Map {
        let mut data: [i128; BlockType::COUNT] = [0; BlockType::COUNT];
        for i in 0..VERTICAL_BLOCKS {
            data[i % BlockType::COUNT] |= ROWMASK << i;
        }

        let mapblocks: MapBlocks = data.into();
        Map::new(mapblocks, GRAVITY_DEFAULT).unwrap()
    }
}

impl Map {
    /// Constructs a new map
    pub(crate) fn new(mapblocks: MapBlocks, gravity: Vec2) -> Result<Map, String> {
        assert!(VERTICAL_BLOCK_SPACING > BLOCK_HEIGHT);

        let mapblocks = Map::verify_mapblocks(mapblocks)?;
        Ok(Map { mapblocks, gravity: gravity.to_array() })
    }

    pub(crate) fn from_mapblocks(mapblocks: MapBlocks) -> Result<Map, String> {
        Map::new(mapblocks, GRAVITY_DEFAULT)
    }

    /// Constructs a new map from data saved in a file.
    pub(crate) fn read_from_file(filename: &str) -> Result<Map, String> {
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

    pub(crate) fn write_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
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
            let overlaps = bits.into_iter().fold(-1, |acc, x| {
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
    pub(crate) fn get_gravity(&self) -> Vec2 {
        self.gravity.into()
    }

    /// obtains the locations that are occupied by blocks of specified type
    pub(crate) fn get_bits_of_type(&self, blocktype: block::BlockType) -> MapBits {
        let blockindex = blocktype as usize;
        let MapBlocks(mapblocks) = self.mapblocks;

        // it will always be the case that mapblockslist will have a greater
        // length than blockindex. Therefore, we can just call directly with no checks
        MapBits(mapblocks[blockindex])
    }

    /// Obtains the locations that are occupied by blocks of any type.
    pub(crate) fn get_all_occupied(&self) -> MapBits {
        let MapBlocks(mapblocks) = self.mapblocks;
        let x = mapblocks.iter().fold(0, |acc, x| {acc | x});
        MapBits(x)
    }

    /// returns a copy of the map as blocktypes array.
    pub(crate) fn to_blocktypes(&self) -> [Option<BlockType>; VERTICAL_BLOCKS * HORIZONTAL_BLOCKS] {
        let mut blockrects = [None; VERTICAL_BLOCKS * HORIZONTAL_BLOCKS];
        for blocktype in BlockType::iter() {
            let MapBits(bits) = self.get_bits_of_type(blocktype);
            for i in 0..VERTICAL_BLOCKS * HORIZONTAL_BLOCKS {
                if (bits & (1 << i)) != 0 {
                    blockrects[i] = Some(blocktype);
                }
            }
        }
        blockrects
    }

    /// returns the index of the first row containing a block
    /// below the point defined by row and col. if there is no block, then returns
    /// a number greater than VERTICAL_BLOCKS.
    pub(crate) fn first_row_below(&self, row: usize, col: usize) -> Option<usize> {
        let rows_below_mask = (-1) ^ ((1 << row) - 1);
        let mask = (COLMASK & rows_below_mask) << (col * VERTICAL_BLOCKS);
        let occupied_bits = self.get_all_occupied().0;

        let column: i128 = (occupied_bits & mask) >> (col * VERTICAL_BLOCKS);
        let row = i128::trailing_zeros(column) as usize;

        if row >= VERTICAL_BLOCKS {
            None
        } else {
            Some(row)
        }
    }
}
