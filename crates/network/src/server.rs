use std::{net::SocketAddr, io};

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

    pub fn connect_addr(&mut self, addr: &SocketAddr) -> Result<(), io::Error> {
        self.serverclient.connect_addr(addr)?;
        Ok(())
    }

    pub fn send_data(&mut self, data: &[u8]) -> Result<(), TrySendError<Packet>> {
        self.serverclient.send_data(data)?;
        Ok(())
    }

    pub fn receive(&mut self) -> Result<Option<Vec<u8>>, io::Error> {
        self.serverclient.receive()
    }
}

impl Default for Server {
    fn default() -> Self {
        Server::new(DEFAULT_PORT, 4).unwrap()
    }
}
