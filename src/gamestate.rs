use game::arena::Arena;
use ggez::{event::EventHandler, GameResult, timer, graphics::{self, Color}};
use crate::{ASPECT_RATIO_X, ASPECT_RATIO_Y};
use crate::utils;

#[derive(Debug)]
pub struct GameState {
    arena: Arena,
}

impl GameState {
    pub fn new(arena: Arena) -> GameState {
        GameState { arena }
    }
}


const DESIRED_FPS: u32 = 60;

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Rely on ggez's built-in timer for deciding when to update the game, and how many times.
        // If the update is early, there will be no cycles, otherwises, the logic will run once for each
        // frame fitting in the time since the last update.
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // black bars
        graphics::clear(ctx, Color::BLACK);

        // used to draw aspect ratio.
        let (width, height) = graphics::size(ctx);

        let heightratio = height / ASPECT_RATIO_Y;
        let widthratio = width / ASPECT_RATIO_X;

        // multiply this with aspect ratio to get correct dimensions
        let multiplier = utils::min_float(heightratio, widthratio);

        graphics::present(ctx)
    }
}
