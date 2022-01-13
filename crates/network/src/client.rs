use crossbeam_channel::{Sender, Receiver};
use game::arena::Arena;
use laminar::{Socket, Packet, SocketEvent};
use std::{net::SocketAddr, thread::{self, JoinHandle}, time::Duration, io::{self, Result, ErrorKind}};

use crate::message::{Message, HeaderByte};


/// Wrapper for the client socket. Implementation of orderliness
/// and "reliability" given by the `laminar` package.
///
/// TODO: enable some form of packet verification and other configurations
/// with the config options for laminar.
#[derive(Debug)]
pub struct Client {
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    remote: Option<SocketAddr>,
    arena: Option<Arena>,
    id: Option<u8>,
    _poll_thread: JoinHandle<()>,
}

impl Client {
    pub fn new(port: u16) -> Result<Self> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        match Socket::bind(addr) {
            Ok(mut socket) => {
                let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
                let remote = None;
                let _poll_thread = thread::spawn(move || socket.start_polling());
                let arena = None;
                let id = None;

                Ok(Self {sender, receiver, remote, arena, id, _poll_thread})
            },

            Err(e) => Err(io::Error::new(ErrorKind::Other, e)),
        }
    }

    /// Client-only call.
    ///
    /// returns io::Error::TimedOut if the server does not respond,
    /// returns io::Error::InvalidData if the server sends corrupted data.
    /// returns io::Error::BrokenPipe if the client cannot send info to server.
    pub fn connect(&mut self, remote: &SocketAddr, name: &str) -> Result<()> {
        let message = Message::write_connect(name);
        Client::send_to(&self.sender, remote, &message)?;

        // timeout for server to send the validation packet.
        let validation = self.receiver.recv_timeout(Duration::from_secs(5));

        match validation {
            Ok(SocketEvent::Packet(packet)) => {
                Client::set_remote(&mut self.remote, remote);
                let m = Message::try_from(packet.payload().to_vec())?;

                let (id, map) = m.read_verify();
                self.id = Some(id);
                self.arena = Some(Arena::new(map));
                Ok(())
            },
            Err(e) => Err(io::Error::new(ErrorKind::TimedOut, e)),
            Ok(_) => unimplemented!(),
        }
    }

    /// connects to a valid address, if we are allowed to.
    fn set_remote(remote: &mut Option<SocketAddr>, addr: &SocketAddr) -> bool {
        if let None = remote {
            *remote = Some(*addr);
            true
        } else {
            false
        }
    }

    /// removes the given socket from the remotes list.
    fn remove_remote(remote: &mut Option<SocketAddr>) {
        *remote = None;
    }

    pub(crate) fn try_get_remote(&self) -> &Option<SocketAddr> {
        &self.remote
    }

    /// sends the data contained in a packet to a server.
    /// Also increments the sequence counter for the client, which may result in overflowing.
    pub fn send_message(&self, message: &Message) -> Result<()> {
        if let Some(remote) = self.remote {
            Client::send_to(&self.sender, &remote, &message)?;
            Ok(())
        } else {
            Err(io::Error::new(ErrorKind::NotFound, "Remote not initialized"))
        }
    }

    pub fn try_get_arena(&self) -> Option<&Arena> {
        self.arena.as_ref()
    }

    pub fn try_get_id(&self) -> &Option<u8> {
        &self.id
    }

    /// sends the data to a remote socket.
    fn send_to(sender: &Sender<Packet>,
               remote: &SocketAddr,
               message: &Message) -> Result<()> {

        let packet = Packet::reliable_sequenced(*remote, message.to_vec(), None);
        match sender.try_send(packet) {
            Ok(_) => Ok(()),
            Err(e) => Err(io::Error::new(ErrorKind::Other, e))
        }
    }

    /// function to call when the client receives a packet.
    fn on_packet_recv(arena_opt: &mut Option<Arena>, packet: Packet) {
        let payload = packet.payload();
        let m = Message::try_from(payload.to_vec());

        if let Ok(message) = m {
            match message.header {
                HeaderByte::State => {
                    // updates this client's arena.
                }

                _ => { unimplemented!() },
            }
        }
    }

    /// Receives a `Packet` and then only returns the data of the packet if it is
    /// more recent than the previous one.
    pub fn receive(&mut self) {
        for event in self.receiver.try_iter() {
            match event {
                SocketEvent::Packet(packet) => {
                    if packet.addr() == self.remote.expect("No Remotes") {
                        Client::on_packet_recv(&mut self.arena, packet);
                    }
                },

                SocketEvent::Timeout(_addr) => {
                    Client::remove_remote(&mut self.remote);
                },

                SocketEvent::Disconnect(_addr) => {
                    Client::remove_remote(&mut self.remote);
                },

                SocketEvent::Connect(_addr) => {
                    // Client::add_remote(&mut self.remotes, &addr, self.max_remotes);
                }
            }
        }
    }
}
