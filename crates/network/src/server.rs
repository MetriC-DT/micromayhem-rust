use std::io;

use crate::{client::Client, packet::ProtocolId};

/// Represents the server that collects information sent to it from the various connected clients.
struct Server {
    serverclient: Client,
}

impl Server {
    pub fn new(port: u16,
               protocol: ProtocolId,
               max_remotes: u8) -> Result<Server, io::Error> {

        let mut serverclient = Client::new(port, protocol)?;
        serverclient.set_max_remotes(max_remotes);
        Ok(Self { serverclient })
    }
}
