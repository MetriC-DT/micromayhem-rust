use network::{server::Server, DEFAULT_PORT};

use crate::TICK_RATE;

struct ServerState {
    tickrate: u8,
    server: Server,
}

impl ServerState {
    pub fn new(tickrate: u8, server: Server) -> Self {
        Self { tickrate, server }
    }
}
