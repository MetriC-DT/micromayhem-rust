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

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let dt: f32 = timer::delta(ctx).as_secs_f32();
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
