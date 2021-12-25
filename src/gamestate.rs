use game::{block::{BLOCK_HEIGHT, BLOCK_WIDTH}, map::{MAP_WIDTH, MAP_HEIGHT, PADDING_HEIGHT, PADDING_WIDTH, Map}, player::Player};
use glam::Vec2;
use ggez::{event::EventHandler, GameResult, timer, graphics::{self, Color, Canvas}, conf::NumSamples};
use crate::{ASPECT_RATIO_X, ASPECT_RATIO_Y, BACKGROUND_COLOR};
use crate::utils;


// total width is (number of blocks horizontally + padding on both sides)
const ARENA_WIDTH: f32 = BLOCK_WIDTH * ((MAP_WIDTH as f32) + 2.0 * (PADDING_WIDTH as f32));

// total height is (number of blocks vertically + padding on both sides)
const ARENA_HEIGHT: f32 = BLOCK_HEIGHT * ((MAP_HEIGHT as f32) + 2.0 * (PADDING_HEIGHT as f32));

// the ticks per second for the physics simulation.
const DESIRED_FPS: u32 = 60;

#[derive(Debug)]
pub struct GameState {
    mapcanvas: graphics::Canvas,
    map: Map,
    player: Player,
}

impl GameState {
    pub fn new(ctx: &mut ggez::Context, map: Map, player: Player) -> GameState {
        let color_format = ggez::graphics::get_window_color_format(ctx);
        let width: u16 = ARENA_WIDTH.ceil() as u16;
        let height: u16 = ARENA_WIDTH.ceil() as u16;
        let numsamples = NumSamples::One;

        let mapcanvas: Canvas = Canvas::new(ctx, width, height, numsamples, color_format)
            .expect("Unable to create new canvas");

        // TODO: draw map onto the alternate canvas (done only once, because map stays static)

        GameState { mapcanvas, map, player }
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
