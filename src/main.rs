use micromayhem::clientstate::ClientState;
use micromayhem::serverstate::ServerState;
use network::DEFAULT_PORT;
use network::server::Server;
use std::net::ToSocketAddrs;
use std::{env, io};
use std::path::{Path, self, PathBuf};
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
    let server = Server::new(DEFAULT_PORT, 4).expect("Cannot create server");
    let mut serverstate = ServerState::new(server);

    println!("Starting server on port {}", DEFAULT_PORT);
    serverstate.run();
}

/// runs the client side of the game (which handles drawing graphics on user's screen).
fn run_client() -> GameResult {
    // stopgap measure to allow user to connect to an online server.
    println!("Enter a server to connect to below:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let address = buffer.trim().to_socket_addrs()
        .expect("Unable to convert to a valid address")
        .next()
        .expect("No valid address found") ;

    println!("Enter your character name below:");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    name = name.trim().to_string();

    // loads atlas
    let resource_dir = load_resources();
    let atlaspath = resource_dir.join(Path::new(SPRITE_JSON));
    let atlas = Atlas::new(&atlaspath);

    let mut cb = ContextBuilder::new(GAME_TITLE, AUTHOR);
    cb = configuration::load_configuration(cb);
    cb = cb.add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;
    let g = ClientState::new(&mut ctx, &atlas, &address, &name)
        .expect("Unable to create new client state");

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
