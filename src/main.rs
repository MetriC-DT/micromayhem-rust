use micromayhem::clientstate::ClientState;
use std::io::Write;
use std::{env, io};
use std::path::{Path, self, PathBuf};
use game::arena::Arena;
use ggez::{GameResult, ContextBuilder, event};

use gui::spriteloader::Atlas;
use micromayhem::configuration;
use micromayhem::{RESOURCES, AUTHOR, GAME_TITLE, SPRITE_JSON};

fn main() -> GameResult {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && &args[1] == "server" {
        run_server();
        return Ok(())
    } else {
        run_client()
    }
}

/// runs the server side of the game, which only handles physics and player interaction.
fn run_server() {
    let a = Arena::default();
}

/// runs the client side of the game (which handles drawing graphics on user's screen).
fn run_client() -> GameResult {
    // stopgap measure to allow user to connect to an online server.
    print!("Enter a server to connect to: ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let address = buffer.trim();

    // loads atlas
    let resource_dir = load_resources();
    let atlaspath = resource_dir.join(Path::new(SPRITE_JSON));
    let atlas = Atlas::new(&atlaspath);

    let mut cb = ContextBuilder::new(GAME_TITLE, AUTHOR);
    cb = configuration::load_configuration(cb);
    cb = cb.add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;
    let mut g = ClientState::new(Arena::default(), &mut ctx, &atlas);

    event::run(ctx, event_loop, g);
}

/// obtains the resources directory.
fn load_resources() -> PathBuf {
    // loads the resources directory so we can access the sprites and other resources.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push(RESOURCES);
        path
    } else {
        path::PathBuf::from(RESOURCES)
    }
}
