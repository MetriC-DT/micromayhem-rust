use crate::weapon::Weapon;
use glam::f32::Vec2;

pub struct Player {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    name: String,
    default_weapon: Weapon,
    current_weapon: Weapon,
    team: usize,
}
