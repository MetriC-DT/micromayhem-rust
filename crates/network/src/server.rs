use std::net::SocketAddr;

use crossbeam_channel::TrySendError;
use laminar::{ErrorKind, Packet};

use crate::{client::Client, DEFAULT_PORT};

/// Represents the server that collects information sent to it from the various connected clients.
pub struct Server {
    serverclient: Client,
}

impl Server {
    pub fn new(port: u16, max_remotes: u8) -> Result<Self, ErrorKind> {
        let mut serverclient = Client::new(port)?;
        serverclient.set_max_remotes(max_remotes);
        Ok(Self { serverclient })
    }

    pub fn send_data(&mut self, data: &[u8]) -> Result<(), TrySendError<Packet>> {
        Ok(self.serverclient.send_data(data)?)
    }

    pub fn receive(&mut self) -> Vec<Vec<u8>> {
        self.serverclient.receive()
    }

    pub fn get_remotes(&self) -> &Vec<SocketAddr> {
        self.serverclient.get_remotes()
    }
}

impl Default for Server {
    fn default() -> Self {
        Server::new(DEFAULT_PORT, 4).expect("Cannot create default server")
    }
}
