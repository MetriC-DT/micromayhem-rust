pub mod map;
pub mod block;
pub mod arena;
pub mod player;
pub mod weapon;


#[macro_use]
extern crate num_derive;
extern crate num_traits;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

#[cfg(test)]
mod test_map;

#[cfg(test)]
mod test_block;

#[cfg(test)]
mod test_player;

#[cfg(test)]
mod test_weapon;
