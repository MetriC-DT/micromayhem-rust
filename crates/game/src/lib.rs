use glam::{Vec2, const_vec2};
use map::{VERTICAL_BLOCKS, HORIZONTAL_BLOCKS};

pub mod map;
pub mod block;
pub mod arena;
pub mod player;
pub mod weapon;
pub mod input;
pub mod weaponscatalog;

#[cfg(test)]
mod unittests;

// public constants
pub const PLAYER_WIDTH: f32 = 32.0;
pub const PLAYER_HEIGHT: f32 = 32.0;
pub const PLAYER_MASS: f32 = 50.0;
pub const BLOCK_WIDTH: f32 = 100.0;
pub const BLOCK_HEIGHT: f32 = 32.0;

// to be multiplied out
pub const PLAYER_SPEED_CAP: f32 = 400.0;

/// default gravity limit (positive orientation is downwards).
pub const GRAVITY_DEFAULT: Vec2 = const_vec2!([0.0, 2000.0]);

/// acceleration for jumping.
pub const JUMP_ACCEL: Vec2 = const_vec2!([0.0, -75000.0]);

/// friction in air to allow for minor control of character while not touching ground.
pub const AIR_FRICTION: f32 = 20000.0;

/// horizontal padding of map in number of blocks
/// This is the region around where player is considered to be alive.
pub const HORIZONTAL_PADDING: f32 = 200.0;

/// vertical padding of map in number of blocks.
/// This is the region around where player is considered to be alive.
pub const VERTICAL_PADDING: f32 = 200.0;

/// vertical spacing between rows of blocks. NEEDS TO BE GREATER THAN BLOCK_HEIGHT
pub const VERTICAL_BLOCK_SPACING: f32 = 100.0;

/// total width in pixels
/// (number of blocks horizontally + padding on both sides)
pub const ARENA_WIDTH: f32 = 2.0 * HORIZONTAL_PADDING + BLOCK_WIDTH * HORIZONTAL_BLOCKS as f32;

/// total height in pixels
/// (number of blocks vertically + padding above and below)
pub const ARENA_HEIGHT: f32 = 2.0 * VERTICAL_PADDING + VERTICAL_BLOCK_SPACING * VERTICAL_BLOCKS as f32;

/// threshold for determining if a float should be rounded to zero.
pub const ERROR_THRESHOLD: f32 = 1e-6;
