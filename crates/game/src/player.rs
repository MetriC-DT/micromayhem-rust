use crate::weapon::{BasicGun, Weapon};
use glam::Vec2;

#[derive(Debug)]
pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub name: String,
    pub width: f32,
    pub height: f32,
    default_weapon: Box<dyn Weapon>,
    current_weapon: Box<dyn Weapon>,
    team: usize,
    damage_multiplier: f32,
    lives: usize,
}

impl Player {
    pub fn get_new_default_weapon(&self) -> Box<dyn Weapon> {
        self.default_weapon.clone_weapon()
    }
}

impl Default for Player {
    fn default() -> Self {
        let default_weapon = Box::new(BasicGun{});
        let current_weapon = default_weapon.clone_weapon();

        Player {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            name: String::new(),
            width: 10.0,
            height: 10.0,
            default_weapon,
            current_weapon,
            team: 0,
            damage_multiplier: 0.0,
            lives: 3
        }
    }
}
