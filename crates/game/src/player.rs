use crate::weapon::{BasicGun, Weapon};
use glam::Vec2;


/// Since the display grid has increasing y for going lower on screen,
/// the convention will be downward y direction is positive.
#[derive(Debug)]
pub struct Player {
    /// the top left corner of the player.
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub direction: f32,
    default_weapon: Box<dyn Weapon>,
    current_weapon: Box<dyn Weapon>,
    team: usize,
    damage_multiplier: f32,
    lives: usize,
    mass: f32,
}

impl Player {
    pub fn get_new_default_weapon(&self) -> Box<dyn Weapon> {
        self.default_weapon.clone_weapon()
    }

    /// Updates the position and velocities of the player.
    ///
    /// max_y is the maximum y unit that the player can move. This is used
    /// when accounting for ground interrupting the player's fall.
    pub fn update(&mut self, dt: f32, max_y: f32) {
        self.position += self.velocity * dt + 0.5 * self.acceleration * dt * dt;
        self.position.y = f32::min(max_y, self.position.y);
        self.velocity += self.acceleration * dt;
    }

    /// used for movement, jump, and firing recoil inputs.
    pub fn set_acceleration(&mut self, acceleration: Vec2) {
        self.acceleration = acceleration;
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
            direction: 1.0,
            default_weapon,
            current_weapon,
            team: 0,
            damage_multiplier: 0.0,
            lives: 3,
            mass: 1.0
        }
    }
}
