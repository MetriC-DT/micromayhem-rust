use crossbeam_channel::{Sender, Receiver, TrySendError};
use laminar::{Socket, ErrorKind, Packet, SocketEvent};
use std::{net::SocketAddr, thread::{self, JoinHandle}};

/// Wrapper for the client socket. Implementation of orderliness
/// and "reliability" given by the `laminar` package.
///
/// TODO: enable some form of packet verification and other configurations
/// with the config options for laminar.
pub struct Client {
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    remotes: Vec<SocketAddr>,
    max_remotes: u8,
    _poll_thread: JoinHandle<()>
}

impl Client {
    pub fn new(port: u16) -> Result<Self, ErrorKind> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let mut socket = Socket::bind(addr)?;
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
        let max_remotes = 1;
        let remotes = Vec::new();
        let _poll_thread = thread::spawn(move || socket.start_polling());

        Ok(Self {sender, receiver, max_remotes, remotes, _poll_thread})
    }

    /// Appends the remote addr that the client can communicate with
    pub fn connect(&mut self, addr: &SocketAddr) {
        Client::add_remote(&mut self.remotes, addr, self.max_remotes);
    }

    /// connects to a valid address.
    fn add_remote(remotes: &mut Vec<SocketAddr>, addr: &SocketAddr, max_remotes: u8) {
        if remotes.len() < max_remotes.into() {
            remotes.push(*addr);
        }
    }

    /// sets the maximum number of remote clients that this client can talk to.
    /// Mainly used for server configuration.
    pub(crate) fn set_max_remotes(&mut self, n: u8) {
        self.max_remotes = n;
    }

    fn remove_remote(remotes: &mut Vec<SocketAddr>, remote: &SocketAddr) {
        let index = remotes.iter().position(|x| x == remote);
        if let Some(i) = index {
            remotes.swap_remove(i);
        }
    }

    /// sends the data contained in a packet to a server.
    /// Also increments the sequence counter for the client, which may result in overflowing.
    pub fn send_data(&mut self, data: &[u8]) -> Result<(), TrySendError<Packet>> {
        // sends the data to every one of the remotes.
        for remote in &self.remotes {
            let packet = Packet::reliable_sequenced(*remote, data.to_vec(), None);
            self.sender.try_send(packet)?;
        }
        Ok(())
    }

    /// Receives a `Packet` and then only returns the data of the packet if it is
    /// more recent than the previous one.
    pub fn receive(&mut self) -> Vec<Vec<u8>> {
        let mut returned_data = Vec::with_capacity(self.remotes.len());
        for event in self.receiver.try_iter() {
            match event {
                SocketEvent::Packet(packet) => {
                    returned_data.push(packet.payload().to_vec())
                },

                SocketEvent::Connect(addr) => {
                    Client::add_remote(&mut self.remotes, &addr, self.max_remotes);
                },

                SocketEvent::Timeout(addr) => {
                    Client::remove_remote(&mut self.remotes, &addr);
                },

                SocketEvent::Disconnect(addr) => {
                    Client::remove_remote(&mut self.remotes, &addr);
                },
            }
        }

        returned_data
    }
}
