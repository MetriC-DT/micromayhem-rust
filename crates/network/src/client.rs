use crossbeam::channel::{Sender, Receiver};

use game::{arena::Arena, player::Player};
use laminar::{Socket, Packet, SocketEvent};
use std::{net::SocketAddr, thread::{self, JoinHandle}, io::{self, Result, ErrorKind}, time::Duration};

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

    /// Connects the client to the given `remote` server.
    pub fn connect(&mut self, remote: &SocketAddr, name: &str) -> Result<()> {
        let message = Message::write_connect(name);
        Client::send_to(&self.sender, remote, &message)?;
        Ok(())
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

        let packet = Packet::reliable_unordered(*remote, message.to_vec());
        match sender.send_timeout(packet, Duration::from_millis(50)) {
            Ok(_) => {
                println!("{:?}", message.data);
                Ok(())
            },

            Err(e) => Err(io::Error::new(ErrorKind::Other, e))
        }
    }

    /// function to call when the client receives a packet.
    fn on_packet_recv(arena_opt: &mut Option<Arena>,
                      id_opt: &mut Option<u8>,
                      client_remote: &mut Option<SocketAddr>,
                      packet: Packet) {

        let payload = packet.payload();
        let remote = packet.addr();
        let m = Message::try_from(payload.to_vec());

        if let Ok(message) = m {
            match message.header {
                HeaderByte::State => {
                    // updates this client's arena.
                }

                HeaderByte::Verify => {
                    // updates player ID and arena.
                    Client::set_remote(client_remote, &remote);
                    let (id, map) = message.read_verify();
                    let mut new_arena = Arena::new(map);

                    // TODO: should not need this because server will
                    // send its own updated arena with all players.
                    new_arena.add_player(Player::new("TODO-CHANGED"), id);
                    *id_opt = Some(id);
                    *arena_opt = Some(new_arena);
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
                    if self.remote == None || packet.addr() == self.remote.unwrap() {
                        Client::on_packet_recv(&mut self.arena, &mut self.id, &mut self.remote, packet);
                    }
                },

                SocketEvent::Timeout(_addr) => {
                    // Client::remove_remote(&mut self.remote);
                },

                SocketEvent::Disconnect(_addr) => {
                    // Client::remove_remote(&mut self.remote);
                },

                SocketEvent::Connect(_addr) => {
                    // Client::add_remote(&mut self.remotes, &addr, self.max_remotes);
                }
            }
        }
    }
}
