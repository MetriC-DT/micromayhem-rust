use std::collections::HashMap;
use crate::ARENA_WIDTH;
use crate::ERROR_THRESHOLD;
use crate::AIR_FRICTION;
use crate::ARENA_HEIGHT;
use crate::HORIZONTAL_PADDING;
use crate::VERTICAL_BLOCK_SPACING;
use crate::VERTICAL_PADDING;
use crate::block;
use crate::input::InputMask;
use crate::map::Map;
use crate::block::BlockType;
use crate::block::BlockRect;
use crate::BLOCK_WIDTH;
use crate::BLOCK_HEIGHT;
use crate::map::VERTICAL_BLOCKS;
use crate::map::HORIZONTAL_BLOCKS;
use crate::input::Input;
use crate::player::Player;
use crate::weapon::Bullet;
use glam::Vec2;


/// represents the entire world of the game (entire map + players).
#[derive(Debug)]
pub struct Arena {
    map: Map,
    bullets: HashMap<u16, Bullet>,
    blocks: [Option<BlockType>; VERTICAL_BLOCKS * HORIZONTAL_BLOCKS],
    bulletcount: u16,
    players: HashMap<u8, Player>,
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new(Map::default())
    }
}

impl Arena {
    pub fn new(map: Map) -> Self {
        let blocks = map.to_blocktypes();
        let bullets = HashMap::new();
        let bulletcount = 0;
        let players = HashMap::new();
        Self { map, blocks, bullets, bulletcount, players }
    }

    pub fn get_map(&self) -> &Map {
        &self.map
    }

    /// returns the top corner x and y coordinates of a block at row and col.
    pub fn get_block_position_at(&self, row: usize, col: usize) -> Vec2 {
        let y = Arena::get_block_row_position(row);
        let x = Arena::get_block_col_position(col);
        Vec2::new(x, y)
    }

    /// adds a new player to the arena. Returns the added player.
    pub fn add_player(&mut self, player: Player, id: u8) -> &mut Player {
        self.players.insert(id, player);
        self.players.get_mut(&id).expect("Player not found. This should not happen.")
    }

    pub fn remove_player(&mut self, id: u8) {
        self.players.remove(&id);
    }

    pub fn get_player(&self, id: u8) -> Option<&Player> {
        self.players.get(&id)
    }

    pub fn get_mut_player(&mut self, id: u8) -> Option<&mut Player> {
        self.players.get_mut(&id)
    }

    pub fn get_players(&self) -> &HashMap<u8, Player> {
        &self.players
    }

    pub fn get_bullets(&self) -> &HashMap<u16, Bullet> {
        &self.bullets
    }

    /// obtains the position of the row as f32.
    fn get_block_row_position(row: usize) -> f32 {
        VERTICAL_PADDING + VERTICAL_BLOCK_SPACING * row as f32
    }

    /// obtains the position of the col as f32.
    fn get_block_col_position(col: usize) -> f32 {
        HORIZONTAL_PADDING + BLOCK_WIDTH * col as f32
    }

    /// returns an iterable over the valid blocks.
    pub fn get_blocks_iter(&self) -> impl Iterator<Item=BlockRect> + '_ {
        let mut index = 0;

        self.blocks.iter()
            .filter_map(move |blocktypeoption: &Option<BlockType>| {
                let (r, c) = (index % VERTICAL_BLOCKS, index / VERTICAL_BLOCKS);
                index += 1;

                let x: f32 = HORIZONTAL_PADDING + BLOCK_WIDTH * c as f32;
                let y: f32 = VERTICAL_PADDING + r as f32 * VERTICAL_BLOCK_SPACING;
                let w: f32 = BLOCK_WIDTH;
                let h: f32 = BLOCK_HEIGHT;

                (*blocktypeoption).map(|blocktype| BlockRect {x, y, w, h, blocktype})
            })
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

    /// Simulates the arena when delta time `dt` has passed and all the player's inputs are
    /// accounted for.
    ///
    /// the `inputs` variable represents the vector of inputs that the arena has received from the
    /// players (probably through network). It MUST have the same size as the players array and
    /// also have the indices of the input match the indices of the players that inputted them.
    pub fn update(&mut self,
                  dt: f32,
                  inputs: &HashMap<u8, InputMask>) {

        for (id, player) in self.players.iter_mut() {
            let default_input = InputMask::new();
            let input = inputs.get(id).unwrap_or(&default_input);
            Arena::update_player(player, *input, dt, &mut self.bulletcount, &mut self.bullets, &self.map, &self.blocks);
        }

        self.update_bullets(dt);
    }

    /// handles bullets flying off the map or colliding with players.
    fn update_bullets(&mut self, dt: f32) {
        // Updates all of the bullets' positions. If bullets fly off the map, ends its lifetime,
        // or hits the player, then remove it from the collection. Reports it over the network.
        let mut to_remove: Vec<u16> = Vec::with_capacity(self.players.len());
        for (id, bullet) in self.bullets_iterator_mut() {
            bullet.update(dt);

            let position_x = bullet.get_position().x;
            // removes bullet when flies off the arena.
            if !(0.0..=ARENA_WIDTH).contains(&position_x) {
                to_remove.push(*id as u16);
            }
        }

        for id in to_remove {
            self.bullets.remove(&id);
        }
    }

    /// updates the players in the arena based on their respective inputs.
    fn update_player(player: &mut Player,
                     input: InputMask,
                     dt: f32,
                     next_bullet_id: &mut u16,
                     bullets: &mut HashMap<u16, Bullet>,
                     map: &Map,
                     map_blocks: &[Option<BlockType>; VERTICAL_BLOCKS * HORIZONTAL_BLOCKS]) {

        // inputs
        let left_input = input.has_mask(Input::Left) as u8 as f32 * -1.0;
        let right_input = input.has_mask(Input::Right) as u8 as f32;
        let direction = left_input + right_input;
        let jump_input = input.has_mask(Input::Up);
        let shoot_input = input.has_mask(Input::Shoot);

        // important positions
        let player_bottom = player.position + Vec2::new(0.0, player.height);
        let left_grid_position = Arena::to_row_col(player_bottom);
        let right_grid_position = Arena::to_row_col(player_bottom + Vec2::new(player.width, 0.0));

        // TODO: calculates the acceleration experienced by the player, with all variables and
        // inputs accounted for.
        let mut lowest_block_y: f32 = ARENA_HEIGHT + player.height;
        let mut block_friction = Vec2::ZERO;
        let mut run_friction = direction * Vec2::new(AIR_FRICTION, 0.0);
        let mut bullet_hit = Vec2::ZERO;
        let mut run: Vec2;
        let mut drop_input: bool = false;
        let mut standing_on_block = false;
        let mut standing_on_blocktype: Option<BlockType> = None;


        let first_rowcol_below_opt = Arena::find_first_rowcol_below(map, &left_grid_position, &right_grid_position);

        if let Some((row, col)) = first_rowcol_below_opt {
            // we are standing on block if y position is lowest and player is falling.
            lowest_block_y = Arena::get_block_row_position(row);
            standing_on_block = player_bottom.y == lowest_block_y && player.velocity.y <= 0.0;

            if standing_on_block {
                // sets player's velocity y component to zero.
                player.velocity.y = 0.0;

                // we are already on a block, so the blocktype should not be None
                standing_on_blocktype = map_blocks[col * VERTICAL_BLOCKS + row];

                // can only drop down if we are standing on block, and not on the lowest platform.
                drop_input = input.has_mask(Input::Down) && row != VERTICAL_BLOCKS - 1;
            }
        }

        // Manages the normal force
        let normal_force = player.get_normal(standing_on_block);
        let fric_direction: f32 = -normalize_float(player.velocity.x);

        // if player's velocity is normalized to be 0, then we can directly set it to
        // prevent floating point rounding errors.
        player.velocity.x *= fric_direction.abs();

        if let Some(blocktype) = standing_on_blocktype {
            let coeff_friction = block::get_block_friction(blocktype);
            let normal_magnitude = normal_force.y.abs();

            // compares frictional force to force required to set player's velocity to 0,
            // then choose the smaller of the two magnitudes.
            let mut fric_magnitude = player.velocity.x.abs() * player.get_total_mass() / dt;
            fric_magnitude = f32::min(coeff_friction * normal_magnitude, fric_magnitude);
            block_friction = fric_direction * fric_magnitude * Vec2::X;

            // run_friction: the force to get the player moving (static friction).
            run_friction = direction * coeff_friction * normal_magnitude * Vec2::X;
        }


        // Disallows any acceleration input that is in the same direction as the player's
        // velocity if the player's velocity is already above its speed_cap.
        //
        // README: A better solution might employ correcting the run force by calculating its difference
        // against the maximum allowed acceleration to reach the speed cap rather than just zeroing
        // out the run input. This would probably result in a more "consistent" usage of the
        // speed_cap.
        let multiplier = 2.0;
        run = multiplier * run_friction;
        if (run.x * player.velocity.x > 0.0) && (player.velocity.x.abs() >= player.speed_cap) {
            run = Vec2::ZERO;
        }

        // updates the player after calculating all the applied forces above.
        player
            .add_weight_force()
            .add_normal_force(standing_on_block)
            .add_jump_force(standing_on_block, jump_input)
            .add_recoil_force(shoot_input, dt, next_bullet_id, bullets)
            .add_force(block_friction)
            .add_force(bullet_hit)
            .add_force(run);

        player.update(dt, lowest_block_y, drop_input, direction);
    }


    /// mutable iterator through all the bullets on the map
    fn bullets_iterator_mut(&mut self) -> impl Iterator<Item = (&u16, &mut Bullet)> + '_ {
        self.bullets.iter_mut()
    }

    /// iterator through all the bullets on the map
    pub fn bullets_iterator(&self) -> impl Iterator<Item = (&u16, &Bullet)> + '_ {
        self.bullets.iter()
    }

    /// returns the first (row, col) that has a block below the current player. If no such block exists,
    /// then returns None for the first argument. Bool represents whether the left or right edge
    /// of the player found the first block.
    ///
    /// FIXME: This only works if the player's width is less than that of the block's width.
    fn find_first_rowcol_below(map: &Map,
                            left_grid_position: &Option<(usize, usize)>,
                            right_grid_position: &Option<(usize, usize)>) -> Option<(usize, usize)> {

        let l_rowbelow_option: Option<usize>;
        let r_rowbelow_option: Option<usize>;
        let mut leftcol: usize = 0;
        let mut rightcol: usize = 0;

        if let Some((l_row, l_col)) = left_grid_position {
            l_rowbelow_option = map.first_row_below(*l_row, *l_col);
            leftcol = *l_col;
        } else {
            l_rowbelow_option = None;
        }

        if let Some((r_row, r_col)) = right_grid_position {
            r_rowbelow_option = map.first_row_below(*r_row, *r_col);
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

    /// obtains the approximate, compressed position.
    ///
    /// The approximation is done with the first i8 determining the location in the grid
    /// where each unit distance of the grid represents an arena block's space and is
    /// represented as the first two `i8`. The last two `u8` represents the subgrid location.
    pub fn get_approximate_position(position: Vec2) -> (i8, i8, u8, u8) {
        let grid_unit_x = BLOCK_WIDTH;
        let grid_unit_y = VERTICAL_BLOCK_SPACING;

        let [x, y] = position.to_array();
        let grid_x_f = (x / grid_unit_x).floor();
        let grid_y_f = (y / grid_unit_y).floor();
        let grid_x: i8 = grid_x_f as i8;
        let grid_y: i8 = grid_y_f as i8;

        let x_remain = x - grid_x_f * grid_unit_x;
        let y_remain = y - grid_y_f * grid_unit_y;

        let sub_unit_x = grid_unit_x / u8::MAX as f32;
        let sub_unit_y = grid_unit_y / u8::MAX as f32;

        let sub_x = (x_remain / sub_unit_x).floor() as u8;
        let sub_y = (y_remain / sub_unit_y).floor() as u8;

        (grid_x, grid_y, sub_x, sub_y)
    }

    /// converts the approximation to an actual Vec2 position.
    pub fn approx_to_position(grid_x: i8, grid_y: i8, sub_x: u8, sub_y: u8) -> Vec2 {
        let grid_unit_x = BLOCK_WIDTH;
        let grid_unit_y = VERTICAL_BLOCK_SPACING;

        let mut x = grid_unit_x * grid_x as f32;
        let mut y = grid_unit_y * grid_y as f32;

        x += sub_x as f32 * grid_unit_x / u8::MAX as f32;
        y += sub_y as f32 * grid_unit_y / u8::MAX as f32;

        Vec2::new(x, y)
    }
}

/// returns 1.0 if num is greater than the threshold and positive, -1.0 if num absolute value is
/// greater than threshold and negative, and 0.0 if its magnitude is less than the threshold.
fn normalize_float(num: f32) -> f32 {
    if f32::abs(num) <= ERROR_THRESHOLD {
        0.0
    } else if num > 0.0 {
        1.0
    } else {
        -1.0
    }
}
