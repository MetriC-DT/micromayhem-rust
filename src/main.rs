mod gamestate;
mod configuration;
mod utils;

use std::{path, env};

use ggez::{GameResult, ContextBuilder, event};
use gamestate::GameState;
use utils::Atlas;


pub const ICON_PATH: &str = "";
pub const GAME_TITLE: &str = "Micro Mayhem";
pub const AUTHOR: &str = "Derick Tseng";
pub const RESOURCES: &str = "resources";

pub const ASPECT_RATIO_X: f32 = 16.0;
pub const ASPECT_RATIO_Y: f32 = 9.0;

pub const ATLAS: &str = "sprites/platforms.png";
pub const MAPS_DIR: &str = "maps";
pub const FONTS_DIR: &str = "fonts";
pub const BACKGROUND_COLOR: u32 = 0xfae7c5;


fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push(RESOURCES);
        path
    } else {
        path::PathBuf::from(RESOURCES)
    };


    let mut cb = ContextBuilder::new(GAME_TITLE, AUTHOR);
    cb = configuration::load_configuration(cb);
    cb = cb.add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;
    let mut g = GameState::new(Arena::default());

    event::run(ctx, event_loop, g)
}
