use std::{path::{self, Path}, env};
use game::arena::Arena;
use ggez::{GameResult, ContextBuilder, event};

use micromayhem::gamestate::GameState;
use micromayhem::utils::Atlas;
use micromayhem::configuration;
use micromayhem::{RESOURCES, AUTHOR, GAME_TITLE, SPRITE_JSON};

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push(RESOURCES);

        path
    } else {
        path::PathBuf::from(RESOURCES)
    };

    // loads atlas
    let atlaspath = Path::new(SPRITE_JSON);
    let atlas = Atlas::parse_atlas_json(&resource_dir.join(&atlaspath));

    let mut cb = ContextBuilder::new(GAME_TITLE, AUTHOR);
    cb = configuration::load_configuration(cb);
    cb = cb.add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;
    let mut g = GameState::new(Arena::default(), &mut ctx, atlas);

    event::run(ctx, event_loop, g)
}
