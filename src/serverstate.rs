use std::{time::{Instant, Duration}, thread};

use network::{server::Server, message::Message};

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
            self.server.tick(DELTA_T);
            let time_elapsed = start_time.elapsed();
            let diff_from_target = Duration::from_secs_f32(DELTA_T) - time_elapsed;

            let game_state_message = Message::write_state(self.server.get_arena());
            self.server.send_message(&game_state_message).expect("Message should send");

            thread::sleep(diff_from_target);
        }
    }
}
