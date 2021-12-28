use crate::weaponscatalog::{WeaponType, RELOAD_TIMES, ATTACK_TIMES, ATTACK_FUNCTIONS};
use core::fmt::Debug;
use std::time::SystemTime;
use glam::Vec2;
use crate::weaponscatalog::{DEFAULT_BULLET_COUNTS, DEFAULT_MASSES};

/// contains the various implementations for all the weapons and bullets
/// in the game.

/// The weapon "superstruct" as a workaround for rust
/// not having trait fields.
///
/// direction assumes a unit vector.
/// Velocity does not matter until the weapon is discarded.
#[derive(Debug, Clone)]
pub struct Weapon {
    pub(crate) bullets: u8,
    position: Vec2,
    velocity: Vec2,
    discarded: bool,
    direction: f32,
    weapontype: WeaponType,
    pub(crate) mass: f32,
    last_attack_time: u128,
    reload_started_time: u128,
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
        let velocity = Vec2::ZERO;
        let i = weapontype as usize;
        let bullets = DEFAULT_BULLET_COUNTS[i];
        let mass = DEFAULT_MASSES[i];
        let discarded = false;
        let curr_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Unable to get current time!")
            .as_millis();

        let last_attack_time = curr_time;
        let reload_started_time = curr_time;

        Self {
            position,
            velocity,
            bullets,
            mass,
            discarded,
            weapontype,
            direction,
            last_attack_time,
            reload_started_time,
        }
    }

    /// calls the specific attack function for a weapon of `self.weapontype` only if the
    /// attack has passed the attack cooldown timer, and the gun already reloaded.
    pub fn attack(&mut self) {
        let i = self.weapontype as usize;

        let currtime = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Unable to get current time!")
            .as_millis();

        let reloaded_check = currtime - self.reload_started_time > RELOAD_TIMES[i];
        let attack_cooldown_check = currtime - self.last_attack_time > ATTACK_TIMES[i];

        if reloaded_check && attack_cooldown_check {
            self.last_attack_time = currtime;
            ATTACK_FUNCTIONS[i](self);
        }
    }

    /// throws the weapon. Starts reloading.
    pub fn throw(&mut self) {
        self.discarded = true;
    }
}
