use game::arena::Arena;
use ggez::{event::EventHandler, GameResult, timer, graphics};
use ggez::graphics::Color;
use crate::BACKGROUND_COLOR;


// the ticks per second for the physics simulation.
const DESIRED_FPS: u32 = 60;

#[derive(Debug)]
pub struct GameState {
    arena: Arena,
}

impl GameState {
    pub fn new(arena: Arena) -> GameState {
        GameState {arena}
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Rely on ggez's built-in timer for deciding when to update the game, and how many times.
        // If the update is early, there will be no cycles, otherwises, the logic will run once for each
        // frame fitting in the time since the last update.
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // color background
        graphics::clear(ctx, Color::from_rgb_u32(BACKGROUND_COLOR));

        graphics::present(ctx)
    }
}
