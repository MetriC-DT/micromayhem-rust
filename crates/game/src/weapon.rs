use crate::weaponscatalog::WeaponType;
use core::fmt::Debug;
use glam::Vec2;

use crate::weaponscatalog::{DEFAULT_BULLET_COUNTS, DEFAULT_MASSES, ATTACK_FUNCTIONS};

/// contains the various implementations for all the weapons and bullets
/// in the game.

/// The weapon "superstruct" as a workaround for rust
/// not having trait fields.
///
/// direction assumes a unit vector.
#[derive(Debug, Clone)]
pub struct Weapon {
    position: Vec2,
    pub(crate) bullets: u8,
    discarded: bool,
    direction: f32,
    weapontype: WeaponType,
    mass: f32,
}

/// The bullet "superstruct" as a workaround for rust
/// not having trait fields.
pub struct Bullet {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
}

impl Weapon {
    pub fn new(position: Vec2, weapontype: WeaponType, direction: f32) -> Self {
        let i = weapontype as usize;
        let bullets = DEFAULT_BULLET_COUNTS[i];
        let mass = DEFAULT_MASSES[i];
        let discarded = false;

        Self { position, bullets, mass, discarded, weapontype,  direction }
    }

    pub fn attack(&mut self) {
        let i = self.weapontype as usize;
        ATTACK_FUNCTIONS[i](self);
    }

    pub fn throw(&mut self) {
        self.discarded = true;
    }
}
