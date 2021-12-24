use crate::weapon::Weapon;
use glam::Vec2;

#[derive(Debug)]
pub struct Player {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    name: String,
    default_weapon: Weapon,
    current_weapon: Weapon,
    team: usize,
    damage_multiplier: f32,
    lives: usize,
}
