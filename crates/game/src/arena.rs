use crate::JUMP_ACCEL;
use crate::AIR_FRICTION;
use crate::ARENA_HEIGHT;
use crate::HORIZONTAL_PADDING;
use crate::VERTICAL_BLOCK_SPACING;
use crate::VERTICAL_PADDING;
use crate::block;
use crate::map::Map;
use crate::block::BlockType;
use crate::block::BlockRect;
use crate::BLOCK_WIDTH;
use crate::BLOCK_HEIGHT;
use crate::map::VERTICAL_BLOCKS;
use crate::map::HORIZONTAL_BLOCKS;
use crate::player::Input;
use crate::player::{Player, InputMask};
use glam::Vec2;


/// represents the entire world of the game (entire map + players).
#[derive(Debug)]
pub struct Arena {
    map: Map,
    pub blocks: [Option<BlockType>; VERTICAL_BLOCKS * HORIZONTAL_BLOCKS],
    pub player: Player,
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new(Map::default(), Player::default())
    }
}

impl Arena {
    pub fn new(map: Map, player: Player) -> Self {
        let blocks = map.to_blocktypes();
        Self { map, player, blocks }
    }

    /// obtains the block type at the specified row and column, or None if it doesn't exist.
    pub fn get_blocktype_at(&self, row: usize, col: usize) -> Option<BlockType> {
        self.blocks[col * VERTICAL_BLOCKS + row]
    }

    /// returns the top corner x and y coordinates of a block at row and col.
    pub fn get_block_position_at(&self, row: usize, col: usize) -> Vec2 {
        let y = Arena::get_block_row_position(row);
        let x = Arena::get_block_col_position(col);
        Vec2::new(x, y)
    }

    /// obtains the position of the row as f32.
    pub fn get_block_row_position(row: usize) -> f32 {
        VERTICAL_PADDING + VERTICAL_BLOCK_SPACING * row as f32
    }

    /// obtains the position of the col as f32.
    pub fn get_block_col_position(col: usize) -> f32 {
        HORIZONTAL_PADDING + BLOCK_WIDTH * col as f32
    }

    /// returns an iterable over the valid blocks.
    pub fn get_blocks_iter(&self) -> impl Iterator<Item=BlockRect> + '_ {
        let mut index = 0;

        return self.blocks.iter()
            .filter_map(move |blocktypeoption: &Option<BlockType>| {
                let (r, c) = (index % VERTICAL_BLOCKS, index / VERTICAL_BLOCKS);
                index += 1;

                let x: f32 = HORIZONTAL_PADDING + BLOCK_WIDTH * c as f32;
                let y: f32 = VERTICAL_PADDING + r as f32 * VERTICAL_BLOCK_SPACING;
                let w: f32 = BLOCK_WIDTH;
                let h: f32 = BLOCK_HEIGHT;

                if let Some(blocktype) = *blocktypeoption {
                    Some(BlockRect {x, y, w, h, blocktype})
                } else {
                    None
                }
            });
    }

    /// changes a point in the arena to the nearest row and column as represented by the map. If
    /// the point is within padding, then returns None.
    ///
    /// Because this function is meant for assisting in calculating the location of the first block
    /// below a player's current position, it is more useful to count all positions as "belonging to the
    /// row below yourself" rather than just being contained within the spacing of the row above.
    ///
    /// TODO - write test cases for this function. Tentatively, this works for now.
    fn to_row_col(point: Vec2) -> Option<(usize, usize)> {
        let mut col = (point.x - HORIZONTAL_PADDING) / BLOCK_WIDTH;

        // 0.0625 is just a correcting constant to make the player "higher" than it is supposed to
        // be. this makes sure we are not neglecting counting if he is standing exactly on a block.
        let mut row = (point.y - VERTICAL_PADDING - 0.0625) / VERTICAL_BLOCK_SPACING;

        col = col.floor();
        row = row.floor();

        // added 1 because I want anything below the platform to be registered as being part of the
        // row below it.
        let r = i32::max(1 + row as i32, 0);
        let c = col as i32;

        // manual checking should not be necessary because it is highly unlikely I will ever set
        // the number of vertical blocks above 8 and horizontal blocks as 16
        let toplimitvert: i32 = VERTICAL_BLOCKS as i32;
        let toplimithorz: i32 = HORIZONTAL_BLOCKS as i32;

        // since r and c are both guaranteed to be >= 0, we can just return it as a usize.
        if 0 <= r && r < toplimitvert && 0 <= c && c < toplimithorz {
            Some((r as usize, c as usize))
        } else {
            None
        }
    }

    /// Simulates the arena when delta time `dt` has passed.
    pub fn update(&mut self, dt: f32, input: &InputMask) {
        // total mass obtains mass of player + weapon.
        let total_mass = self.player.get_total_mass();
        let player_mass = self.player.mass;
        let player_bottom = self.player.position + Vec2::new(0.0, self.player.height);
        let left_grid_position = Arena::to_row_col(player_bottom);
        let right_grid_position = Arena::to_row_col(player_bottom + Vec2::new(self.player.width, 0.0));

        // TODO: calculates the acceleration experienced by the player, with all variables and
        // inputs accounted for.
        //
        // Considers forces from:
        // weight + gun recoil + block friction + block normal + bullet hit + WASD inputs.
        let weight = self.map.get_gravity() * total_mass;
        let mut lowest_block_y: f32 = ARENA_HEIGHT + self.player.height;
        let mut block_friction = Vec2::ZERO;
        let mut run_friction = Vec2::new(AIR_FRICTION, 0.0);
        let mut block_normal = Vec2::ZERO;
        let mut gun_recoil = Vec2::ZERO;
        let mut bullet_hit = Vec2::ZERO;
        let mut jump = Vec2::ZERO;
        let mut run: Vec2;
        let mut drop_input: bool = false;
        let has_left = input.has_mask(Input::Left) as u8 as f32 * -1.0;
        let has_right = input.has_mask(Input::Right) as u8 as f32;
        let has_jump = input.has_mask(Input::Up) as u8 as f32;

        let first_rowcol_below_opt = self.find_first_rowcol_below(&left_grid_position, &right_grid_position);

        if let Some((row, col)) = first_rowcol_below_opt {
            lowest_block_y = Arena::get_block_row_position(row);
            let standing_on_block = player_bottom.y == lowest_block_y;

            if standing_on_block {
                // sets player's velocity y component to zero.
                self.player.velocity.y = 0.0;

                // obtains the normal force
                block_normal = -weight;

                // obtains the frictional force.
                let velocity_x_unit: f32 = normalize_float(self.player.velocity.x);

                // if player's velocity is normalized to be 0, then we can directly set it to
                // prevent floating point rounding errors.
                self.player.velocity.x = self.player.velocity.x * velocity_x_unit.abs();

                // we are already on a block, so the blocktype should not be None
                let blocktype = self.get_blocktype_at(row, col).expect("BlockType should not be None");

                let coeff_friction = block::get_block_friction(blocktype);
                block_friction = -coeff_friction * block_normal.length() * Vec2::new(velocity_x_unit, 0.0);
                run_friction = coeff_friction * block_normal.length() * Vec2::X;

                // accelerations from player inputs
                jump = player_mass * JUMP_ACCEL * has_jump;

                // can only drop down if we are standing on block, and not on the lowest platform.
                drop_input = input.has_mask(Input::Down) && row != VERTICAL_BLOCKS - 1;
            }
        }

        // Disallows any acceleration input that is in the same direction as the player's
        // velocity if the player's velocity is already above its speed_cap.
        //
        // Therefore, if they are already at the max speed, then just keep their run
        // acceleration the same magnitude as the friction force.
        //
        // TODO: A better solution might employ correcting the run force by calculating its difference
        // against the maximum allowed acceleration to reach the speed cap.
        let multiplier = 2.0;
        let direction = has_left + has_right;
        run = multiplier * run_friction * direction;
        if (run.x * self.player.velocity.x > 0.0) && (self.player.velocity.x.abs() >= self.player.speed_cap) {
            run = Vec2::ZERO;
        }

        // gets player shooting bullet recoil.
        // Since the recoil should punish a player less than a knockback, the force exerted by
        // recoil will be a fraction of the impulse over time rather than the entire dp/dt.
        if input.has_mask(Input::Shoot) && self.player.current_weapon.attack() {
            println!("attack called");
        }

        let total_force = weight + gun_recoil + block_friction + block_normal + bullet_hit + jump + run;

        // updates the player after calculating the applied forces above.
        self.player.update(dt, lowest_block_y, total_force, drop_input);

        // TODO: Obtains the location of all the other players.
    }

    /// returns the first (row, col) that has a block below the current player. If no such block exists,
    /// then returns None for the first argument. Bool represents whether the left or right edge
    /// of the player found the first block.
    fn find_first_rowcol_below(&self,
                            left_grid_position: &Option<(usize, usize)>,
                            right_grid_position: &Option<(usize, usize)>) -> Option<(usize, usize)> {

        let l_rowbelow_option: Option<usize>;
        let r_rowbelow_option: Option<usize>;
        let mut leftcol: usize = 0;
        let mut rightcol: usize = 0;

        if let Some((l_row, l_col)) = left_grid_position {
            l_rowbelow_option = self.map.first_row_below(*l_row, *l_col);
            leftcol = *l_col;
        } else {
            l_rowbelow_option = None;
        }

        if let Some((r_row, r_col)) = right_grid_position {
            r_rowbelow_option = self.map.first_row_below(*r_row, *r_col);
            rightcol = *r_col;
        } else {
            r_rowbelow_option = None;
        }

        // checks whether the first row column pair is from the left edge or right. Chooses
        // whichever one is smaller.
        match (l_rowbelow_option, r_rowbelow_option) {
            (None, None) => None,

            (None, Some(row)) => Some((row, rightcol)),

            (Some(row), None) => Some((row, leftcol)),

            (Some(l_row), Some(r_row)) => {
                if r_row < l_row {
                    Some((r_row, rightcol))
                } else {
                    Some((l_row, leftcol))
                }
            }
        }
    }
}

/// returns 1.0 if num is greater than the threshold and positive, -1.0 if num absolute value is
/// greater than threshold and negative, and 0.0 if its magnitude is less than the threshold.
fn normalize_float(num: f32) -> f32 {
    let threshold = 1.0;
    if f32::abs(num) <= threshold {
        0.0
    } else if num > 0.0 {
        1.0
    } else {
        -1.0
    }
}
