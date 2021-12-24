mod gamestate;
mod configuration;

use std::{path, env};

use ggez::{GameResult, ContextBuilder, event};
use gamestate::GameState;
use game::arena::Arena;
use micromayhem::*;

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let mut g = GameState::new(Arena::default().unwrap());

    let mut cb = ContextBuilder::new(GAME_TITLE, AUTHOR);
    cb = configuration::load_configuration(cb);
    cb = cb.add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;

    event::run(ctx, event_loop, g);
}
