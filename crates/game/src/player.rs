use crate::{weapon::Weapon, weaponscatalog::WeaponType};
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
    default_weapon: Weapon,
    current_weapon: Weapon,
    team: usize,
    damage_multiplier: f32,
    lives: usize,
    mass: f32,
}

impl Player {
    pub fn get_new_default_weapon(&self) -> Weapon {
        self.default_weapon.clone()
    }

    /// Updates the position and velocities of the player.
    ///
    /// max_y is the maximum y unit that the player can move. This is used
    /// when accounting for ground interrupting the player's fall.
    /// 
    /// assumes the tick rate is high enough such that the player will always fall
    /// on the block that is right below. Unsure if this is a valid assumption or not.
    /// If this is an invalid assumption, the alternative method is to solve for 
    /// whether the parabola arced by the player's motion will intersect with a line segment
    /// formed by the platforms that the player will cross. However, this is more computationally
    /// expensive to compute.
    pub fn update(&mut self, dt: f32, max_y: f32) {
        self.position += self.velocity * dt + 0.5 * self.acceleration * dt * dt;

        // if the player's decent is interrupted, we probably need to recalculate the x coordinate
        // (solving delta_t from the new y coordinate, and plugging in for delta_x)
        // for now, I am going to assume negligible difference between the newly calculated
        // x coordinate and the physical actual x coordinate.
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
        let default_position = Vec2::ZERO;
        let default_direction = 1.0;
        let default_weapon = Weapon::new(default_position, WeaponType::BasicPistol, default_direction);
        let current_weapon = default_weapon.clone();

        Player {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            name: String::new(),
            width: 30.0,
            height: 30.0,
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
