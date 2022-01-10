use crossbeam_channel::{Sender, Receiver, TrySendError};
use laminar::{Socket, ErrorKind, Packet, SocketEvent};
use std::{net::{SocketAddr, ToSocketAddrs}, io, thread::{self, JoinHandle}};

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

    /// Verifies that the string can be transformed to a valid address,
    /// then appends a remote server for this client to communicate with.
    pub fn connect(&mut self, addr: &str) -> Result<(), io::Error> {
        let sock_addrs = addr.to_socket_addrs()?;

        for sock_addr in sock_addrs {
            self.connect_addr(&sock_addr)?;
        }
        Ok(())
    }

    /// connects to a valid address.
    pub fn connect_addr(&mut self, addr: &SocketAddr) -> Result<(), io::Error> {
        if self.remotes.len() < self.max_remotes.into() {
            self.remotes.push(*addr);
            Ok(())
        } else {
            Err(io::ErrorKind::AddrInUse.into())
        }
    }

    /// sets the maximum number of remote clients that this client can talk to.
    /// Mainly used for server configuration.
    pub(crate) fn set_max_remotes(&mut self, n: u8) {
        self.max_remotes = n;
    }

    fn remove_remote(&mut self, remote: &SocketAddr) -> Result<(), io::Error> {
        let index = self.remotes.iter().position(|x| x == remote);
        if let Some(i) = index {
            self.remotes.swap_remove(i);
            Ok(())
        } else {
            Err(io::ErrorKind::NotFound.into())
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
    pub fn receive(&mut self) -> Result<Option<Vec<u8>>, io::Error> {
        let resultdata = self.receiver.try_recv();

        if let Ok(event) = resultdata {
            match event {
                SocketEvent::Packet(packet) => Ok(Some(packet.payload().to_vec())),
                SocketEvent::Connect(addr) => {self.connect_addr(&addr)?; Ok(None)},
                SocketEvent::Timeout(addr) => {self.remove_remote(&addr)?; Ok(None)},
                SocketEvent::Disconnect(addr) => {self.remove_remote(&addr)?; Ok(None)},
            }
        }
        else {
            Err(io::ErrorKind::InvalidData.into())
        }
    }
}
