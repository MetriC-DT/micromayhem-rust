/// A list of all the weapons and their various properties
///
/// README: An alternative approach is to have all of the weapon properties in a 
/// JSON object and just read all that in at run time. Might be more readable than this.
use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};
use WeaponType::*;
use BulletType::*;

/// Type of the weapon
#[derive(Debug, Clone, EnumCount, Copy)]
pub enum WeaponType {
    BasicPistol,
}

/// Type of the bullets fired from various weapons.
/// README: maybe implement bullet "attacks" types so we can have different implementations
/// (e.g. exploding missile).
#[derive(Debug, Clone, EnumCount, Copy, FromRepr)]
pub enum BulletType {
    Pistol,
    Rifle,
    Sniper,
}

/// bullet mass properties.
pub(crate) const DEFAULT_BULLET_MASSES: [f32; BulletType::COUNT] = {
    let mut bullet_masses = [0.0; BulletType::COUNT];

    bullet_masses[Pistol as usize] = 20.0;
    bullet_masses[Rifle as usize] = 20.0;
    bullet_masses[Sniper as usize] = 200.0;
    bullet_masses
};


/// number of initial bullets of each gun
pub(crate) const DEFAULT_BULLET_COUNTS: [u8; WeaponType::COUNT] = {
    let mut bulletcounts: [u8; WeaponType::COUNT] = [0; WeaponType::COUNT];

    bulletcounts[BasicPistol as usize] = 8;
    bulletcounts
};

/// masses of each gun
pub(crate) const DEFAULT_MASSES: [f32; WeaponType::COUNT] = {
    let mut masses: [f32; WeaponType::COUNT] = [0.0; WeaponType::COUNT];

    masses[BasicPistol as usize] = 5.0;
    masses
};

/// reloading times of each gun in milliseconds
pub(crate) const RELOAD_TIMES: [u128; WeaponType::COUNT] = {
    let mut times: [u128; WeaponType::COUNT] = [0; WeaponType::COUNT];

    times[BasicPistol as usize] = 1000;
    times
};

/// attacking times of each gun (time between consecutive bullet shots)
/// in milliseconds
pub(crate) const ATTACK_TIMES: [u128; WeaponType::COUNT] = {
    let mut times: [u128; WeaponType::COUNT] = [0; WeaponType::COUNT];

    times[BasicPistol as usize] = 500;
    times
};

/// types of bullets that a specific gun uses
pub(crate) const BULLET_TYPES: [BulletType; WeaponType::COUNT] = {
    let mut types: [BulletType; WeaponType::COUNT] = [Pistol; WeaponType::COUNT];

    types[BasicPistol as usize] = Pistol;
    types
};


/// speed of the bullet that a specific can can shoot out.
pub(crate) const BULLET_SPEEDS: [f32; WeaponType::COUNT] = {
    let mut bullet_speeds = [0.0; WeaponType::COUNT];

    bullet_speeds[BasicPistol as usize] = 1000.0;
    bullet_speeds
};

