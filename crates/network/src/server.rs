use laminar::ErrorKind;

use crate::{client::Client, DEFAULT_PORT};

/// Represents the server that collects information sent to it from the various connected clients.
struct Server {
    serverclient: Client,
}

impl Server {
    pub fn new(port: u16, max_remotes: u8) -> Result<Self, ErrorKind> {
        let mut serverclient = Client::new(port)?;
        serverclient.set_max_remotes(max_remotes);
        Ok(Self { serverclient })
    }
}

impl Default for Server {
    fn default() -> Self {
        Server::new(DEFAULT_PORT, 4).unwrap()
    }
}
