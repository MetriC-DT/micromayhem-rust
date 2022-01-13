use std::{time::{Instant, Duration}, thread};

use network::server::Server;

use crate::DELTA_T;

pub struct ServerState {
    server: Server,
}

impl ServerState {
    pub fn new(server: Server) -> Self {
        Self { server }
    }

    pub fn run(&mut self) {
        loop {
            let start_time = Instant::now();
            self.server.update(DELTA_T);
            let time_elapsed = start_time.elapsed();
            let diff_from_target = Duration::from_secs_f32(DELTA_T) - time_elapsed;

            thread::sleep(diff_from_target);
        }
    }
}
