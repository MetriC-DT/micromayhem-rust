use crate::weaponscatalog::{ATTACK_TIMES, BULLET_TYPES, DEFAULT_BULLET_MASSES, BulletType, BULLET_SPEEDS, RELOAD_TIMES, WeaponType};
use core::fmt::Debug;
use std::time::SystemTime;
use glam::Vec2;
use crate::weaponscatalog::{DEFAULT_BULLET_COUNTS, DEFAULT_MASSES};
use WeaponStatus::*;

/// contains the various implementations for all the weapons and bullets
/// in the game.


/// The bullet "superstruct" as a workaround for rust
/// not having trait fields.
#[derive(Debug)]
pub struct Bullet {
    position: Vec2,
    velocity: Vec2,
    bullettype: BulletType,
    team: u8,
    id: u16,
}

impl Bullet {
    pub fn new(position: Vec2, velocity: Vec2, bullettype: BulletType, team: u8, id: u16) -> Self {
        Self { position, velocity, bullettype, team, id }
    }

    pub fn get_mass(&self) -> f32 {
        DEFAULT_BULLET_MASSES[self.bullettype as usize]
    }

    pub fn get_team(&self) -> u8 {
        self.team
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }

    pub fn get_bullet_type(&self) -> BulletType {
        self.bullettype
    }
}


/// The weapon "superstruct" as a workaround for rust
/// not having trait fields.
///
/// direction assumes a unit vector.
/// Velocity does not matter until the weapon is discarded.
#[derive(Debug)]
pub struct Weapon {
    pub(crate) bullets: u8,
    pub(crate) weapontype: WeaponType,
    status: WeaponStatus,
    position: Vec2,
    velocity: Vec2,
    direction: f32,
    last_attack_time: u128,
    reload_started_time: u128,
}

/// status of the weapon. Should only update when the attack function is called.
#[derive(Debug, PartialEq)]
pub enum WeaponStatus {
    Cooldown,
    Ready,
    Empty,
    Discarded,
    FireSuccess,
}

impl Weapon {
    pub fn new(position: Vec2, weapontype: WeaponType, direction: f32) -> Self {
        let velocity = Vec2::ZERO;
        let i = weapontype as usize;
        let bullets = DEFAULT_BULLET_COUNTS[i];
        let curr_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Unable to get current time!")
            .as_millis();

        let last_attack_time = curr_time;
        let reload_started_time = curr_time;
        let status = Cooldown;

        Self {
            position,
            velocity,
            status,
            bullets,
            weapontype,
            direction,
            last_attack_time,
            reload_started_time,
        }
    }

    /// calls the specific attack function for a weapon of `self.weapontype` only if the
    /// attack has passed the attack cooldown timer, and the gun already reloaded.
    ///
    /// If the attack was successfully executed, then attack returns FireSuccess, otherwise, 
    /// it will return WeaponStatus::Empty if there is no more bullets left, but the player can 
    /// attack, or WeaponStatus::Cooldown if the player cannot attack due to cooldown.
    pub(crate) fn attack(&mut self) -> WeaponStatus {
        let i = self.weapontype as usize;

        let currtime = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Unable to get current time!")
            .as_millis();

        let reloaded_check = currtime - self.reload_started_time > RELOAD_TIMES[i];
        let attack_cooldown_check = currtime - self.last_attack_time > ATTACK_TIMES[i];
        let can_attack = reloaded_check && attack_cooldown_check;

        if can_attack && self.bullets > 0 {
            self.last_attack_time = currtime;
            self.bullets -= 1;
            self.status = Cooldown;
            WeaponStatus::FireSuccess
        } else if can_attack {
            self.status = Empty;
            WeaponStatus::Empty
        } else {
            WeaponStatus::Cooldown
        }
    }

    /// obtains the mass of the weapon.
    pub(crate) fn get_mass(&self) -> f32 {
        DEFAULT_MASSES[self.weapontype as usize]
    }

    /// throws the weapon.
    pub(crate) fn discard(&mut self, velocity: Vec2) {
        self.status = Discarded;
        self.velocity = velocity;
    }

    /// sets the position of the weapon the player is holding.
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    /// sets the direction the weapon is facing.
    pub fn set_direction(&mut self, direction: f32) {
        self.direction = direction;
    }

    pub fn get_bullet_type(&self) -> BulletType {
        BULLET_TYPES[self.weapontype as usize]
    }

    pub(crate) fn get_bullet_momentum(&self) -> Vec2 {
        let bullettype = self.get_bullet_type();
        let bulletspeed = self.get_bullet_speed();
        let bulletmass = DEFAULT_BULLET_MASSES[bullettype as usize];
        self.direction * Vec2::X * bulletspeed * bulletmass
    }

    pub(crate) fn get_bullet_speed(&self) -> f32 {
        BULLET_SPEEDS[self.weapontype as usize]
    }
}
