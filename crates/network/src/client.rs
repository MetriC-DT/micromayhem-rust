use crossbeam_channel::{Sender, Receiver, TrySendError};
use game::arena::Arena;
use game::input::InputMask;
use laminar::{Socket, Packet, SocketEvent};
use laminar::ErrorKind;
use std::{net::SocketAddr, thread::{self, JoinHandle}, collections::HashMap, time::Duration, io};

use crate::message::{Message, HeaderByte};


/// Wrapper for the client socket. Implementation of orderliness
/// and "reliability" given by the `laminar` package.
///
/// TODO: enable some form of packet verification and other configurations
/// with the config options for laminar.
pub struct Client {
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    remotes: HashMap<SocketAddr, u8>,
    max_remotes: u8,
    arena: Option<Arena>,
    id: Option<u8>,
    _poll_thread: JoinHandle<()>,
}

impl Client {
    pub fn new(port: u16) -> Result<Self, ErrorKind> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let mut socket = Socket::bind(addr)?;
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
        let max_remotes = 1;
        let remotes = HashMap::new();
        let _poll_thread = thread::spawn(move || socket.start_polling());
        let arena = None;
        let id = None;

        Ok(Self {sender, receiver, max_remotes, remotes, arena, id, _poll_thread})
    }

    /// Client-only call.
    ///
    /// returns io::Error::TimedOut if the server does not respond,
    /// returns io::Error::InvalidData if the server sends corrupted data.
    /// returns io::Error::BrokenPipe if the client cannot send info to server.
    pub fn connect(&mut self, remote: &SocketAddr, name: &str) -> Result<(), io::Error> {
        let message = Message::write_connect(name);
        if let Ok(()) = Client::send_to(&self.sender, remote, &message) {

            // timeout for server to send the validation packet.
            let validation = self.receiver.recv_timeout(Duration::from_secs(5));
            if let Ok(SocketEvent::Packet(packet)) = validation {
                Client::add_remote(&mut self.remotes, remote, self.max_remotes, 0);
                let m = Message::try_from(packet.payload().to_vec())?;
                self.id = Some(m.read_verify());
                Ok(())
            } else {
                Err(io::ErrorKind::TimedOut.into())
            }

        }
        else {
            Err(io::ErrorKind::BrokenPipe.into())
        }

    }

    /// connects to a valid address.
    fn add_remote(remotes: &mut HashMap<SocketAddr, u8>, addr: &SocketAddr, max_remotes: u8, id: u8) {
        if remotes.len() < max_remotes.into() {
            remotes.insert(*addr, id);
        }
    }

    /// removes the given socket from the remotes list.
    fn remove_remote(remotes: &mut HashMap<SocketAddr, u8>, remote: &SocketAddr) {
        remotes.remove(remote);
    }

    pub(crate) fn get_remotes(&self) -> &HashMap<SocketAddr, u8> {
        &self.remotes
    }

    /// sends the data contained in a packet to a server.
    /// Also increments the sequence counter for the client, which may result in overflowing.
    pub fn send_message(&self, message: &Message) -> Result<(), TrySendError<Packet>> {
        // sends the data to every one of the remotes.
        for (remote, _) in &self.remotes {
            Client::send_to(&self.sender, remote, &message)?;
        }
        Ok(())
    }

    /// sends the data to a remote socket.
    fn send_to(sender: &Sender<Packet>, remote: &SocketAddr, message: &Message) -> Result<(), TrySendError<Packet>> {
        let packet = Packet::reliable_sequenced(*remote, message.to_vec(), None);
        Ok(sender.try_send(packet)?)
    }

    /// function to call when the client receives a packet.
    fn on_packet_recv(sender: &Sender<Packet>,
                      arena_opt: &mut Option<Arena>,
                      remotes: &mut HashMap<SocketAddr, u8>,
                      max_remotes: u8,
                      packet: Packet) {

        let payload = packet.payload();
        let addr = packet.addr();
        let m = Message::try_from(payload.to_vec());

        if let Ok(message) = m {
            match message.header {
                HeaderByte::Connect => {
                    // adds player into arena, and adds player into connected remotes.
                    if let Some(arena) = arena_opt {
                        let id = arena.add_player(message.read_connect());
                        Client::send_to(sender, &addr, &Message::write_verify(id)).unwrap();
                        Client::add_remote(remotes, &addr, max_remotes, id);
                    }
                },

                HeaderByte::Verify => {
                    Client::send_to(sender, &addr, &Message::write_request()).unwrap();
                },

                HeaderByte::State => {
                    // updates this client's arena.
                }

                _ => { unimplemented!() },
            }
        }
    }

    /// Receives a `Packet` and then only returns the data of the packet if it is
    /// more recent than the previous one.
    fn receive(&mut self) {
        for event in self.receiver.try_iter() {
            match event {
                SocketEvent::Packet(packet) => {
                    Client::on_packet_recv(&self.sender,
                                           &mut self.arena,
                                           &mut self.remotes,
                                           self.max_remotes,
                                           packet);
                },

                SocketEvent::Timeout(addr) => {
                    Client::remove_remote(&mut self.remotes, &addr);
                },

                SocketEvent::Disconnect(addr) => {
                    Client::remove_remote(&mut self.remotes, &addr);
                },

                SocketEvent::Connect(_addr) => {
                    // Client::add_remote(&mut self.remotes, &addr, self.max_remotes);
                }
            }
        }
    }
}
