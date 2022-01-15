use crossbeam::channel::{Sender, Receiver};
use game::{arena::Arena, input::InputMask};
use laminar::{Socket, Packet, SocketEvent};
use std::{net::SocketAddr, thread::{self, JoinHandle}, collections::HashMap, io::{self, ErrorKind}};
use crate::message::{Message, HeaderByte};
use std::io::Result;

/// Wrapper for the client socket. Implementation of orderliness
/// and "reliability" given by the `laminar` package.
///
/// TODO: enable some form of packet verification and other configurations
/// with the config options for laminar.
#[derive(Debug)]
pub struct Server {
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    remotes: HashMap<SocketAddr, u8>,
    inputs: HashMap<u8, InputMask>,
    max_remotes: u8,
    arena: Arena,
    next_id: u8,
    _poll_thread: JoinHandle<()>
}

impl Server {
    pub fn new(port: u16, max_remotes: u8) -> Result<Self> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let s = Socket::bind(addr);
        match s {
            Ok(mut socket) => {
                let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
                let remotes = HashMap::new();
                let inputs = HashMap::new();
                let _poll_thread = thread::spawn(move || socket.start_polling());
                let arena = Arena::default();
                let next_id = 0;

                Ok(Self {sender, receiver, max_remotes, remotes, inputs, arena, next_id, _poll_thread})
            },

            Err(e) => {Err(io::Error::new(ErrorKind::Other, e))}
        }
    }

    /// connects to a valid address.
    fn add_remote(remotes: &mut HashMap<SocketAddr, u8>,
                  addr: &SocketAddr,
                  max_remotes: u8,
                  id: u8) -> bool {

        if Server::can_add_new_client(remotes, max_remotes) {
            remotes.insert(*addr, id);
            true
        } else {
            false
        }
    }

    fn can_add_new_client(remotes: &mut HashMap<SocketAddr, u8>, max_remotes: u8) -> bool {
        remotes.len() < max_remotes.into()
    }

    /// removes the given socket from the remotes list and the arena.
    fn remove_remote(remotes: &mut HashMap<SocketAddr, u8>,
                     remote: &SocketAddr,
                     arena: &mut Arena) {

        let player_id = remotes.remove(remote);
        if let Some(id) = player_id {
            arena.remove_player(id);
            println!("Removed {}", id);
        }
    }

    /// sends the data contained in a packet to all connected clients.
    pub fn send_message(&self, message: &Message) -> Result<()> {
        // sends the data to every one of the remotes.
        for remote in self.remotes.keys() {
            Server::send_to(&self.sender, remote, message)?;
        }
        Ok(())
    }

    pub(crate) fn get_remotes(&self) -> &HashMap<SocketAddr, u8> {
        &self.remotes
    }

    /// sends the data to a remote socket.
    fn send_to(sender: &Sender<Packet>, remote: &SocketAddr, message: &Message) -> Result<()> {
        let packet = Packet::reliable_unordered(*remote, message.to_vec());
        match sender.try_send(packet) {
            Ok(_) => Ok(()),
            Err(e) => Err(io::Error::new(ErrorKind::Other, e)),
        }
    }

    /// function to call when the client receives a packet.
    fn on_packet_recv(sender: &Sender<Packet>,
                      arena: &mut Arena,
                      remotes: &mut HashMap<SocketAddr, u8>,
                      inputs: &mut HashMap<u8, InputMask>,
                      max_remotes: u8,
                      packet: Packet,
                      next_id: &mut u8) {

        let payload = packet.payload();
        let addr = packet.addr();
        let m = Message::try_from(payload.to_vec());

        if let Ok(message) = m {
            match message.header {
                HeaderByte::Connect => {
                    // acknowledges the player, sends them verification, containing map and ID.

                    // finds the next available free number
                    let mut id = *next_id;
                    while arena.get_players().contains_key(&id) {
                        id = id.wrapping_add(1);
                    }

                    let verification = Message::write_verify(id, arena.get_map());
                    Server::send_to(sender, &addr, &verification).unwrap();

                    // the next id gets updated.
                    *next_id = next_id.wrapping_add(1);
                },

                HeaderByte::Request => {
                    // sends the compressed arena state.
                    let request_data  = message.read_request();
                    if let Ok((id, player)) = request_data {
                        let successful = Server::add_remote(remotes, &addr, max_remotes, id);
                        if successful {
                            arena.add_player(player, id);
                        }
                    }
                },

                HeaderByte::Input => {
                    // uses the received input to update the arena.
                    let inputmask = message.read_input();
                    let id_option = remotes.get(&addr);

                    if let Some(id) = id_option {
                        inputs.insert(*id, inputmask);
                        println!("Received {} from {}", inputmask, *id);
                    }
                },

                HeaderByte::Disconnect => {
                    // removes the remote from the connected remotes.
                    // also, removes player from arena if possible.
                    Server::remove_remote(remotes, &addr, arena);
                },

                _ => println!("Unknown packet received")
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
                                           &mut self.inputs,
                                           self.max_remotes,
                                           packet,
                                           &mut self.next_id);
                },

                SocketEvent::Timeout(addr) => {
                    Server::remove_remote(&mut self.remotes, &addr, &mut self.arena);
                },

                _ => {},
            }
        }
    }

    /// updates the server arena.
    pub fn tick(&mut self, dt: f32) {
        self.inputs.clear();
        self.receive();

        self.arena.update(dt, &self.inputs);
    }
}
