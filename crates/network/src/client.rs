use std::{net::{UdpSocket, SocketAddr, ToSocketAddrs}, io::{self, ErrorKind}};

use crate::packet::{Packet, PACKET_BYTES, ProtocolId};

/// Wrapper for the UDP client socket. Implementation of orderliness
/// and "reliability" based on the 
/// [Gaffer on Games articles](https://gafferongames.com/post/reliability_ordering_and_congestion_avoidance_over_udp/)
pub struct Client {
    socket: UdpSocket,
    sequence: u16,
    protocol: ProtocolId,
    ack: u16,
    ack_bitfield: u32,
    recv_buffer: Vec<u8>,
    recent_packet: Option<Packet>,
    remotes: Vec<String>,
    max_remotes: u8
}

impl Client {
    pub fn new(port: u16, protocol: ProtocolId) -> Result<Self, io::Error> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let socket = UdpSocket::bind(addr)?;
        let sequence = 0;
        let ack = u16::MAX - 33; // HACK: lets the first recv ack bitfield be zero because 33 > 32 bits (u32)
        let ack_bitfield = 0;
        let recv_buffer = vec![0; PACKET_BYTES];
        let recent_packet = None;
        let max_remotes = 1;
        let remotes = Vec::new();
        socket.set_nonblocking(true)?;
        Ok(Self {socket, sequence, protocol, ack, ack_bitfield, recv_buffer, recent_packet, remotes, max_remotes})
    }

    /// Verifies that the string can be transformed to a valid address,
    /// then appends a remote server for this client to communicate with.
    pub fn connect(&mut self, addr: &str) -> Result<(), io::Error> {
        if self.remotes.len() < self.max_remotes.into() {
            addr.to_socket_addrs()?;
            self.remotes.push(addr.to_string());
            Ok(())
        } else {
            Err(ErrorKind::AddrInUse.into())
        }
    }

    /// sets the maximum number of remote clients that this client can talk to.
    /// Mainly used for server configuration.
    pub(crate) fn set_max_remotes(&mut self, n: u8) {
        self.max_remotes = n;
    }

    /// obtains the raw socket of this client.
    pub fn get_socket(&self) -> &UdpSocket {
        &self.socket
    }

    /// sends the data contained in a packet to a server.
    /// Also increments the sequence counter for the client, which may result in overflowing.
    pub fn send_data(&mut self, data: &[u8]) -> Result<(), io::Error> {
        let packet = Packet::new(self.protocol, self.sequence, self.ack, self.ack_bitfield, data)?;
        let bytes: Vec<u8> = packet.into();

        // sends the data to every one of the remotes.
        for remote in &self.remotes {
            self.socket.send_to(&bytes[..], remote)?;
        }

        self.sequence = self.sequence.wrapping_add(1);
        Ok(())
    }

    /// Receives a `Packet` and then only returns the data of the packet if it is
    /// more recent than the previous one. Otherwise, returns None.
    ///
    /// Returns an io::Error when socket fails to `recv()` or when the received 
    /// data cannot be constructed into a Packet struct.
    pub fn receive(&mut self) -> Result<Option<&Vec<u8>>, io::Error> {
        self.socket.recv(&mut self.recv_buffer)?;
        self.recent_packet = Packet::try_from(&self.recv_buffer[..]).ok();

        if let Some(packet) = &self.recent_packet {
            let new_ack = packet.get_sequence();

            // if the received packet is more recent compared to the previously received
            // packet, then set the newest ack to the received packet.
            if packet.is_more_recent_than(self.ack) {
                let seq_diff = Client::get_seq_diff(new_ack, self.ack);
                self.ack = new_ack;

                // subtract 1 because a shift of 0 means 1 ack before.
                let shift = seq_diff - 1;

                // returns 0 when shifted left by over 32 bits.
                let new_frame = u32::checked_shl(self.ack_bitfield, seq_diff.into()).unwrap_or(0);
                let prior_ack = u32::checked_shl(1, shift.into()).unwrap_or(0);
                self.ack_bitfield = new_frame | prior_ack;
                Ok(packet.get_data(self.protocol))
            }
            else {
                // we only need to set the ack_bitfield.
                // self.ack is more current than the new_ack
                let seq_diff = Client::get_seq_diff(self.ack, new_ack);

                // safe version of this code:
                // `self.ack_bitfield |= 1 << (seq_diff - 1)`
                self.ack_bitfield |= u32::checked_shl(1, (seq_diff - 1).into()).unwrap_or(0);
                Ok(None)
            }
        }
        else {
            Err(ErrorKind::InvalidData.into())
        }
    }

    /// obtains the bitfield ack for this client.
    pub fn get_bitfield(&self) -> u32 {
        self.ack_bitfield
    }

    /// obtains the sequence number for this client.
    pub fn get_sequence(&self) -> u16 {
        self.sequence
    }

    /// Obtains the difference between sequence numbers. For example, if the newer seq number
    /// is 34 and the older is 32, `get_seq_diff` returns 2. Supports wraparound, so if newer is
    /// `0` and older is `u16::MAX`, then it returns `1`.
    fn get_seq_diff(newer: u16, older: u16) -> u16 {
        if newer >= older {
            newer - older
        } else {
            u16::MAX - older + newer + 1
        }
    }
}

// seq_diff unit test
#[test]
fn seq_diff_test() {
    assert_eq!(Client::get_seq_diff(3, 0), 3);
    assert_eq!(Client::get_seq_diff(2, u16::MAX - 1), 4);
}
