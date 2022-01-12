use crossbeam_channel::{Sender, Receiver, TrySendError};
use game::arena::Arena;
use laminar::{Socket, ErrorKind, Packet, SocketEvent};
use std::{net::SocketAddr, thread::{self, JoinHandle}, collections::HashMap, time::Duration};

use crate::message::{Message, HeaderByte};


/// Wrapper for the client socket. Implementation of orderliness
/// and "reliability" given by the `laminar` package.
///
/// TODO: enable some form of packet verification and other configurations
/// with the config options for laminar.
pub struct Server {
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    remotes: HashMap<SocketAddr, u8>,
    max_remotes: u8,
    arena: Option<Arena>,
    _poll_thread: JoinHandle<()>
}

impl Server {
    pub fn new(port: u16, max_remotes: u8) -> Result<Self, ErrorKind> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let mut socket = Socket::bind(addr)?;
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
        let remotes = HashMap::new();
        let _poll_thread = thread::spawn(move || socket.start_polling());
        let arena = Some(Arena::default());

        Ok(Self {sender, receiver, max_remotes, remotes, arena, _poll_thread})
    }

    /// connects to a valid address.
    fn add_remote(remotes: &mut HashMap<SocketAddr, u8>, addr: &SocketAddr, max_remotes: u8, id: u8) {
        if remotes.len() < max_remotes.into() {
            remotes.insert(*addr, id);
        }
    }

    /// removes the given socket from the remotes list and the arena.
    fn remove_remote(remotes: &mut HashMap<SocketAddr, u8>,
                     remote: &SocketAddr,
                     arena_opt: &mut Option<Arena>) {

        let player_id = remotes.remove(remote);
        if let (Some(arena), Some(id)) = (arena_opt, player_id) {
            arena.remove_player(id);
        }
    }

    /// sends the data contained in a packet to all connected clients.
    pub fn send_message(&self, message: &Message) -> Result<(), TrySendError<Packet>> {
        // sends the data to every one of the remotes.
        for (remote, _) in &self.remotes {
            Server::send_to(&self.sender, remote, &message)?;
        }
        Ok(())
    }

    pub(crate) fn get_remotes(&self) -> &HashMap<SocketAddr, u8> {
        &self.remotes
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
                        Server::send_to(sender, &addr, &Message::write_verify(id)).unwrap();
                        Server::add_remote(remotes, &addr, max_remotes, id);
                    }
                },

                HeaderByte::Request => {
                    // sends the compressed arena state.
                },

                HeaderByte::Input => {
                    // uses the received input to update the arena.
                    let inputmask = message.read_input();
                },

                HeaderByte::Disconnect => {
                    // removes the remote from the connected remotes.
                    // also, removes player from arena if possible.
                    Server::remove_remote(remotes, &addr, arena_opt);
                },

                _ => {unimplemented!()},
            }
        }
    }

    /// Receives a `Packet` and then only returns the data of the packet if it is
    /// more recent than the previous one.
    pub(crate) fn receive(&mut self) {
        for event in self.receiver.try_iter() {
            match event {
                SocketEvent::Packet(packet) => {
                    Server::on_packet_recv(&self.sender,
                                           &mut self.arena,
                                           &mut self.remotes,
                                           self.max_remotes,
                                           packet);
                },

                SocketEvent::Timeout(addr) => {
                    Server::remove_remote(&mut self.remotes, &addr, &mut self.arena);
                },

                SocketEvent::Disconnect(addr) => {
                    Server::remove_remote(&mut self.remotes, &addr, &mut self.arena);
                },

                _ => {},
            }
        }
    }
}
