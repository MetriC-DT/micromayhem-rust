use crossbeam_channel::{Sender, Receiver, TrySendError};
use game::arena::Arena;
use laminar::{Socket, ErrorKind, Packet, SocketEvent};
use std::{net::SocketAddr, thread::{self, JoinHandle, sleep}, time::Duration};

use crate::message::{Message, HeaderByte};


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
    arena: Option<Arena>,
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
        let arena = None;

        Ok(Self {sender, receiver, max_remotes, remotes, arena, _poll_thread})
    }

    pub fn connect(&mut self, remote: &SocketAddr, name: &str) -> Result<(), TrySendError<Packet>> {
        let message = Message::write_connect(name);
        Client::send_to(&self.sender, remote, &message).unwrap();
        Ok(())
    }

    /// Appends the remote addr that the client can communicate with
    fn add_remote_addr(&mut self, addr: &SocketAddr) {
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

    /// removes the given socket from the remotes list.
    fn swap_remove_remote(remotes: &mut Vec<SocketAddr>, remote: &SocketAddr) {
        let index = remotes.iter().position(|x| x == remote);
        if let Some(i) = index {
            remotes.swap_remove(i);
        }
    }

    /// sends the data contained in a packet to a server.
    /// Also increments the sequence counter for the client, which may result in overflowing.
    pub fn send_message(&self, message: &Message) -> Result<(), TrySendError<Packet>> {
        // sends the data to every one of the remotes.
        for remote in &self.remotes {
            Client::send_to(&self.sender, remote, &message)?;
        }
        Ok(())
    }

    pub(crate) fn get_remotes(&self) -> &Vec<SocketAddr> {
        &self.remotes
    }

    /// sends the data to a remote socket.
    fn send_to(sender: &Sender<Packet>, remote: &SocketAddr, message: &Message) -> Result<(), TrySendError<Packet>> {
        let packet = Packet::reliable_sequenced(*remote, message.to_vec(), None);
        Ok(sender.try_send(packet)?)
    }

    /// function to call when the client receives a packet.
    fn on_packet_recv(sender: &Sender<Packet>, arena: &mut Option<Arena>, packet: Packet) {
        let payload = packet.payload();
        let addr = packet.addr();
        let m = Message::try_from(payload.into_iter().cloned().collect::<Vec<u8>>());

        if let Ok(message) = m {
            match message.header {
                HeaderByte::Connect => {
                    Client::send_to(sender, &addr, &Message::write_verify()).unwrap();
                },
                HeaderByte::Verify => {
                    Client::send_to(sender, &addr, &Message::write_request()).unwrap();
                },
                HeaderByte::Request => {
                    // sends the compressed arena state.
                },

                HeaderByte::Disconnect => todo!(),
                HeaderByte::State => todo!(),
                HeaderByte::Input => todo!(),
            }
        }
    }

    /// Receives a `Packet` and then only returns the data of the packet if it is
    /// more recent than the previous one.
    ///
    /// FIXME: Use the Connect event to monitor actual connections instead of the packet hack.
    pub fn receive(&mut self) {
        for event in self.receiver.try_iter() {
            match event {
                SocketEvent::Packet(packet) => {
                    Client::on_packet_recv(&self.sender, &mut self.arena, packet);
                },

                SocketEvent::Timeout(addr) => {
                    Client::swap_remove_remote(&mut self.remotes, &addr);
                },

                SocketEvent::Disconnect(addr) => {
                    Client::swap_remove_remote(&mut self.remotes, &addr);
                },

                SocketEvent::Connect(addr) => {
                    Client::add_remote(&mut self.remotes, &addr, self.max_remotes);
                }
            }
        }
    }
}
