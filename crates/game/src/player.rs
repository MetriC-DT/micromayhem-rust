use crate::{weapon::Weapon, weaponscatalog::WeaponType, arena::ARENA_WIDTH};
use glam::{Vec2, const_vec2};

pub enum Input {
    Left,
    Right,
    Up,
    Down,
    Shoot,
    Bomb,
    Throw
}

#[derive(Debug)]
pub struct InputMask(u16);

impl InputMask {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add_mask(&mut self, input: Input) {
        self.0 |= 1 << input as usize;
    }

    pub fn has_mask(&self, input: Input) -> bool {
        (self.0 & (1 << input as usize)) != 0
    }

    pub fn remove_mask(&mut self, input: Input) {
        self.0 &= !(1 << input as usize)
    }
}

/// acceleration for jumping.
pub(crate) const JUMP_ACCEL: Vec2 = const_vec2!([0.0, -70000.0]);
pub(crate) const RUN_ACCEL: Vec2 = const_vec2!([3000.0, 0.0]);


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
    /// max_y is the maximum y unit that the player can drop down to. This is used
    /// when accounting for ground interrupting the player's fall.
    /// 
    /// assumes the tick rate is high enough such that the player will always fall
    /// on the block that is right below. Unsure if this is a valid assumption or not.
    /// If this is an invalid assumption, the alternative method is to solve for 
    /// whether the parabola arced by the player's motion will intersect with a line segment
    /// formed by the platforms that the player will cross. However, this is more computationally
    /// expensive to compute.
    pub fn update(&mut self, dt: f32, max_y: f32, force: Vec2, drop_input: bool) {
        self.acceleration = force / self.get_total_mass();

        let mut new_position = self.position + self.velocity * dt + 0.5 * self.acceleration * dt * dt;

        // make edits to player's new_position based on obstacles between the original and final
        // destinations.
        //
        // if the player's decent is interrupted, we need to recalculate the x coordinate
        // (solving delta_t from the new y coordinate, and plugging in for delta_x)
        // for now, I am going to assume negligible difference between the newly calculated
        // x coordinate and the actual physical x coordinate.
        let drop_height = drop_input as u8 as f32 * 1.0;
        new_position.y = f32::min(max_y - self.height, new_position.y) + drop_height;
        let dx = new_position - self.position;

        self.velocity = dx / dt;
        self.position = new_position;
    }

    /// obtains the total mass of the player (player + current weapon).
    pub fn get_total_mass(&self) -> f32 {
        return self.mass + self.current_weapon.mass;
    }
}

impl Default for Player {
    fn default() -> Self {
        let midmap = ARENA_WIDTH / 2.0;
        let default_position = Vec2::new(midmap, 0.0);
        let default_direction = 1.0;
        let default_weapon = Weapon::new(default_position, WeaponType::BasicPistol, default_direction);
        let current_weapon = default_weapon.clone();

        Player {
            position: default_position,
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
