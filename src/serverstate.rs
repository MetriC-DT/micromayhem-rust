use game::{arena::Arena, input::InputMask};
use network::server::Server;

pub struct ServerState {
    server: Server,
    arena: Arena,
}

impl ServerState {
    pub fn new() -> Self {
        let arena = Arena::default();
        let server = Server::default();
        Self { arena, server }
    }

    /// obtains all of the player's inputs (and other messages)
    /// then updates the state of the game.
    pub fn update(&mut self, dt: f32) {
        // obtain player inputs from network and feed it to the arena update function.
        let messages = self.server.receive();

        // FIXME: for now, the only types a server can receive are `Input` types.
        for message in messages {
            let sender = message.addr();
            let data = message.payload().get(0).unwrap();
            let input = InputMask::from(*data);
        }
    }
}
