use crossbeam::channel::{Sender, Receiver};

use game::{arena::Arena, player::Player, map::Map, weapon::Bullet};
use glam::Vec2;
use laminar::{Socket, Packet, SocketEvent};
use std::{net::SocketAddr, thread::{self, JoinHandle}, io::{self, Result, ErrorKind}};

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
    name: String,
    _poll_thread: JoinHandle<()>,
}

impl Client {
    pub fn new(port: u16, name: &str) -> Result<Self> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        match Socket::bind(addr) {
            Ok(mut socket) => {
                let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
                let remote = None;
                let _poll_thread = thread::spawn(move || socket.start_polling());
                let arena = None;
                let id = None;
                let name = name.to_string();

                Ok(Self {sender, receiver, remote, arena, id, name, _poll_thread})
            },

            Err(e) => Err(io::Error::new(ErrorKind::Other, e)),
        }
    }

    /// Connects the client to the given `remote` server.
    pub fn connect(&mut self, remote: &SocketAddr) -> Result<()> {
        let message = Message::write_connect();
        Client::send_to(&self.sender, remote, &message)?;
        Ok(())
    }

    /// connects to a valid address, if we are allowed to.
    fn set_remote(remote: &mut Option<SocketAddr>, addr: &SocketAddr) -> bool {
        if remote.is_none() {
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
            Client::send_to(&self.sender, &remote, message)?;
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
        match sender.try_send(packet) {
            Ok(_) => { Ok(()) },

            Err(e) => Err(io::Error::new(ErrorKind::Other, e))
        }
    }

    /// function to call when the client receives a packet.
    fn on_packet_recv(arena_opt: &mut Option<Arena>,
                      id_opt: &mut Option<u8>,
                      client_remote: &mut Option<SocketAddr>,
                      name: &str,
                      sender: &Sender<Packet>,
                      packet: Packet) {

        let payload = packet.payload();
        let remote = packet.addr();
        let m = Message::try_from(payload.to_vec());

        if let Ok(message) = m {
            match message.header {
                HeaderByte::State => {
                    // updates this client's arena.
                    let state = message.read_state();
                    if let (Some(arena), Ok((p_ids, p_positions, b_ids, b_type, b_positions))) = (arena_opt, state) {
                        // updates players and positions.
                        // TODO - clear only disconnected players
                        for (id, pos) in p_ids.iter().zip(p_positions) {
                            let p = arena.get_mut_player(*id);
                            if let Some(player) = p {
                                // TODO - actually send the direction
                                player.update_position(pos, 1.0);
                            }
                            else {
                                let player = arena.add_player(Player::new(&id.to_string()), *id);
                                // TODO - actually send the direction
                                player.update_position(pos, 1.0);
                            }
                        }

                        // TODO - clear only dead bullets.
                        arena.clear_bullets();
                        for ((id, bullettype), position) in b_ids.iter().zip(b_type).zip(b_positions) {
                            let bullet = Bullet::new(position, Vec2::ZERO, bullettype, 0, *id);
                            arena.update_bullet(bullet);
                        }
                    } else {
                        println!("Received invalid state packet");
                    }
                },

                HeaderByte::Verify => {
                    // updates player ID and arena.
                    let batch: Result<(u8, Map)> = message.read_verify();
                    if let Ok((id, map)) = batch {
                        let mut new_arena = Arena::new(map);

                        // TODO - should remove.
                        new_arena.add_player(Player::new(name), id);

                        *id_opt = Some(id);
                        *arena_opt = Some(new_arena);

                        Client::set_remote(client_remote, &remote);

                        let request = Message::write_request(name, id);
                        Client::send_to(sender, &remote, &request).unwrap();
                    }
                    else {
                        println!("Invalid Verify request received");
                    }
                },

                _ => { println!("Invalid header byte received") },
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
                        Client::on_packet_recv(&mut self.arena, &mut self.id, &mut self.remote, self.name.as_str(), &self.sender, packet);
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
