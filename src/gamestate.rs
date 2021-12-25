use game::arena::Arena;
use ggez::{event::EventHandler, GameResult, timer, graphics::{self, Color, Canvas}, conf::NumSamples};
use crate::{ASPECT_RATIO_X, ASPECT_RATIO_Y, BACKGROUND_COLOR};
use crate::utils;

#[derive(Debug)]
pub struct GameState {
    arena: Arena,
    mapcanvas: graphics::Canvas,
}

impl GameState {
    pub fn new(arena: Arena, ctx: &mut ggez::Context) -> GameState {
        let color_format = ggez::graphics::get_window_color_format(ctx);
        let width: u16 = (arena.width).ceil() as u16;
        let height: u16 = (arena.height).ceil() as u16;
        let numsamples = NumSamples::One;

        let mapcanvas: Canvas = Canvas::new(ctx, width, height, numsamples, color_format)
            .expect("Unable to create new canvas");

        // draw map onto the alternate canvas (done only once, because map stays static)


        GameState {
            arena,
            mapcanvas
        }
    }
}

const DESIRED_FPS: u32 = 60;

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Rely on ggez's built-in timer for deciding when to update the game, and how many times.
        // If the update is early, there will be no cycles, otherwises, the logic will run once for each
        // frame fitting in the time since the last update.
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);
            self.arena.update(dt);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // color background
        graphics::clear(ctx, Color::from_rgb_u32(BACKGROUND_COLOR));

        // centers player on the screen.


        // used to draw aspect ratio.
        let (width, height) = graphics::size(ctx);

        let heightratio = height / ASPECT_RATIO_Y;
        let widthratio = width / ASPECT_RATIO_X;

        // multiply this with aspect ratio to get correct dimensions
        let multiplier = utils::min_float(heightratio, widthratio);

        graphics::present(ctx)
    }
}
