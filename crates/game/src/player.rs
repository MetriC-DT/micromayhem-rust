use std::time::SystemTime;

use crate::{weaponscatalog::WeaponType, PLAYER_WIDTH, PLAYER_HEIGHT, PLAYER_MASS, PLAYER_SPEED_CAP, ARENA_WIDTH, JUMP_ACCEL, JUMP_COOLDOWN, weapon::{Weapon, WeaponStatus, Bullet}};
use glam::Vec2;

/// Since the display grid has increasing y for going lower on screen,
/// the convention will be downward y direction is positive.
///
/// Distinguish direction from the direction of the velocity vector:
/// The `direction` is the direction the player is facing (left=-1.0 or right=+1.0)
/// Velocity is the player's velocity vector, and can be opposite the player's 
/// facing direction (e.g. when player gets shot from the front).
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
    pub mass: f32,
    pub speed_cap: f32,
    pub jumps_count: u8,
    pub jumps_left: u8,
    last_jump_time: u128,
    default_weapontype: WeaponType,
    current_weapon: Weapon,
    team: u8,
    damage_multiplier: f32,
    lives: u8,
}

impl Player {
    /// Updates the position and velocities of the player.
    ///
    /// `max_y` is the maximum y unit that the player can drop down to. This is used
    /// when accounting for ground interrupting the player's fall.
    /// 
    /// assumes the tick rate is high enough such that the player will always fall
    /// on the block that is right below. Unsure if this is a valid assumption or not.
    /// If this is an invalid assumption, the alternative method is to solve for 
    /// whether the parabola arced by the player's motion will intersect with a line segment
    /// formed by the platforms that the player will cross. However, this is more computationally
    /// expensive to compute.
    ///
    /// `drop_input` detects whether a valid drop input command was pushed (e.g. only when on block).
    pub fn update(&mut self, dt: f32, max_y: f32, force: Vec2, drop_input: bool, direction: f32) {

        // sets the player's direction based on input left or right. if no input, then just keep
        // current direction facing.
        if direction != 0.0 {
            self.direction = direction;
        }

        // acceleration is force divided by (mass of player + weapon)
        self.acceleration = force / self.get_total_mass();

        // dx = vt + 1/2 at^2
        let mut new_position = self.position + self.velocity * dt + 0.5 * self.acceleration * dt * dt;

        // make edits to player's new_position based on obstacles between the original and final
        // destinations.
        //
        // if the player's decent is interrupted, we need to recalculate the x coordinate
        // (solving delta_t from the new y coordinate, and plugging in for delta_x)
        // for now, I am going to assume negligible difference between the newly calculated
        // x coordinate and the actual physical x coordinate.
        //
        // drop_input puts in a 1 pixel offset so players can "phase" through the block if a down
        // input is sent successfully.
        let drop_height = drop_input as u8 as f32;
        new_position.y = f32::min(max_y - self.height, new_position.y) + drop_height;

        self.velocity = (new_position - self.position) / dt;
        self.position = new_position;

        // updates the gun that the player is holding.
        self.current_weapon.set_position(self.position);
        self.current_weapon.set_direction(self.direction);
    }

    /// obtains the total mass of the player (player + current weapon).
    pub(crate) fn get_total_mass(&self) -> f32 {
        self.mass + self.current_weapon.get_mass()
    }

    /// let jump_force be a function of the number of jumps left, so
    /// subsequent midair jumps are weaker compared to a ground jump.
    ///
    /// if jump was unsuccessful (cooldown active, or no more jumps left),
    /// then return the zero vector for the jump force. Automatically docks
    /// one from the `jumps_left` variable if possible.
    pub(crate) fn jump_force_and_decrement(&mut self) -> Vec2 {
        let curr_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Unable to get current time!")
            .as_millis();

        let still_has_jumps = self.jumps_left > 0;
        let time_since_last_jump = curr_time - self.last_jump_time;

        if still_has_jumps && time_since_last_jump > JUMP_COOLDOWN {
            // TODO: figure out a good function to use so double jumping results in the 
            // same final position regardless of when the player inputted the 2nd jump input.
            // let fraction: f32 = self.jumps_left as f32 / self.jumps_count as f32;
            self.last_jump_time = curr_time;
            self.jumps_left -= 1;

            let multiplier: f32 = 1.0;
            return multiplier * self.mass * JUMP_ACCEL;
        } else {
            return Vec2::ZERO;
        }
    }

    /// attacks with the current weapon.
    pub(crate) fn attack(&mut self) -> WeaponStatus {
        let status_after_attack = self.current_weapon.attack();

        // if weapon is empty, discard on an attack command.
        if status_after_attack == WeaponStatus::Empty {
            self.throw_current_weapon();
        }

        status_after_attack
    }

    pub(crate) fn create_new_bullet(&self, id: u16) -> Bullet {
        let position_x = self.position.x + PLAYER_WIDTH / 2.0;
        let position_y = self.position.y + PLAYER_HEIGHT / 2.0;
        let position = Vec2::new(position_x, position_y);
        let velocity = self.current_weapon.get_bullet_speed() * Vec2::X * self.direction;
        let bullettype = self.current_weapon.get_bullet_type();
        let team = self.team;

        Bullet::new(position, velocity, bullettype, team, id)
    }

    /// throws the current weapon away and create a new weapon from the player's default.
    pub(crate) fn throw_current_weapon(&mut self) {
        // TODO: discard velocity should be different from player's velocity.
        self.current_weapon.discard(self.velocity);
        self.current_weapon = Weapon::new(self.position, self.default_weapontype, self.direction);
    }

    pub(crate) fn get_bullet_momentum(&self) -> Vec2 {
        self.current_weapon.get_bullet_momentum()
    }
}

impl Default for Player {
    fn default() -> Self {
        let midmap = (ARENA_WIDTH - PLAYER_WIDTH) / 2.0;
        let default_position = Vec2::new(midmap, -PLAYER_HEIGHT);
        let default_direction = 1.0;
        let default_weapontype = WeaponType::BasicPistol;
        let current_weapon = Weapon::new(default_position, default_weapontype, default_direction);
        let curr_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Unable to get current time!")
            .as_millis();

        Player {
            position: default_position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            name: String::from("default"),
            speed_cap: PLAYER_SPEED_CAP,
            jumps_left: 0,
            jumps_count: 2,
            last_jump_time: curr_time,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
            direction: 1.0,
            default_weapontype,
            current_weapon,
            team: 0,
            damage_multiplier: 0.0,
            lives: 5,
            mass: PLAYER_MASS,
        }
    }
}
