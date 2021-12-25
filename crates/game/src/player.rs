use crate::weapon::{BasicGun, Weapon};
use glam::Vec2;

#[derive(Debug)]
pub struct Player {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    name: String,
    default_weapon: Box<dyn Weapon>,
    current_weapon: Box<dyn Weapon>,
    team: usize,
    damage_multiplier: f32,
    lives: usize,
}

impl Player {
    pub fn new(position: Vec2,
               velocity: Vec2,
               acceleration: Vec2,
               name: String,
               default_weapon: Box<dyn Weapon>,
               current_weapon: Box<dyn Weapon>,
               team: usize,
               damage_multiplier: f32,
               lives: usize) -> Self {

        Self {
            position,
            velocity,
            acceleration,
            name,
            default_weapon,
            current_weapon,
            team,
            damage_multiplier,
            lives
        }
    }

    pub fn get_new_default_weapon(&self) -> Box<dyn Weapon> {
        self.default_weapon.clone_weapon()
    }
}

impl Default for Player {
    fn default() -> Self {
        let default_weapon = Box::new(BasicGun{});
        let current_weapon = default_weapon.clone_weapon();

        Player::new(
            Vec2::ZERO,
            Vec2::ZERO,
            Vec2::ZERO,
            String::new(),
            default_weapon,
            current_weapon,
            0,
            0.0,
            3
        )
    }
}
