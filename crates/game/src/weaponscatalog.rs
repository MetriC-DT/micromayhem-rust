/// A list of all the weapons and their various properties
use strum::EnumCount;
use strum_macros::EnumCount;
use crate::weapon::Weapon;
use WeaponType::*;

/// Type of the weapon
#[derive(Debug, Clone, EnumCount, Copy)]
pub enum WeaponType {
    BasicPistol,
}

/// Type of the bullets fired from various weapons.
#[derive(Debug, Clone, EnumCount, Copy)]
pub enum BulletType {
    Pistol,
    Rifle,
    Sniper,
    Missile,
}


/// number of initial bullets of each gun
pub(crate) const DEFAULT_BULLET_COUNTS: [u8; WeaponType::COUNT] = {
    let mut bulletcounts: [u8; WeaponType::COUNT] = [0; WeaponType::COUNT];

    bulletcounts[BasicPistol as usize] = 8;
    bulletcounts
};

/// masses of each gun
pub(crate) const DEFAULT_MASSES: [f32; WeaponType::COUNT] = {
    let mut masses: [f32; WeaponType::COUNT] = [0.0; WeaponType::COUNT];

    masses[BasicPistol as usize] = 10.0;
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


/// handling of each attack using various attack implementations
pub(crate) const ATTACK_FUNCTIONS: [fn(&mut Weapon); WeaponType::COUNT] = {
    // for some reason, we don't need a default initialization of array here?
    let mut funcs = [GUN_ATTACK; WeaponType::COUNT];

    funcs[BasicPistol as usize] = GUN_ATTACK;
    funcs
};

// attack implementations
/// if no bullets left, throw the gun. Otherwise, subtract one and create
const GUN_ATTACK: fn(&mut Weapon) = |weapon: &mut Weapon| {
    if weapon.bullets > 0 {
        weapon.bullets -= 1;
    } else {
        weapon.throw();
    }
};
