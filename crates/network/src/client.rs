use std::net::{UdpSocket, SocketAddr};
use std::io::{Result, ErrorKind};

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
    recent_packet: Option<Packet>
}

impl Client {
    pub fn new(port: u16, protocol: ProtocolId) -> Result<Self> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let socket = UdpSocket::bind(addr)?;
        let sequence = 0;
        let ack = u16::MAX;
        let ack_bitfield = 0;
        let recv_buffer = Vec::with_capacity(PACKET_BYTES);
        let recent_packet = None;
        socket.set_nonblocking(true)?;
        Ok(Self {socket, sequence, protocol, ack, ack_bitfield, recv_buffer, recent_packet})
    }

    pub fn connect(&self, addr: &str) -> Result<()> {
        self.socket.connect(addr)?;
        Ok(())
    }

    /// obtains the raw socket of this client.
    pub fn get_socket(&self) -> &UdpSocket {
        &self.socket
    }

    /// sends the data contained in a packet to a server.
    /// Also increments the sequence counter for the client, which may result in overflowing.
    #[allow(arithmetic_overflow)]
    pub fn send_data(&mut self, data: &Vec<u8>) -> Result<()> {
        let packet = Packet::new(self.protocol, self.sequence, self.ack, self.ack_bitfield, data)?;
        let bytes: Vec<u8> = packet.into();
        self.socket.send(&bytes)?;
        self.sequence += 1;
        Ok(())
    }

    /// Receives a `Packet` and then only returns the data of the packet if it is
    /// more recent than the previous one. Otherwise, returns None.
    ///
    /// Returns an io::Error when socket fails to `recv()` or when the received 
    /// data cannot be constructed into a Packet struct.
    #[allow(arithmetic_overflow)]
    pub fn receive(&mut self) -> Result<Option<&Vec<u8>>> {
        self.socket.recv(&mut self.recv_buffer)?;
        self.recent_packet = Packet::try_from(&self.recv_buffer).ok();

        if let Some(packet) = &self.recent_packet {
            let new_ack = packet.get_sequence();

            // if we are more recent, then we set the acks
            if packet.is_more_recent_than(self.ack) {
                let seq_diff = Client::get_seq_diff(new_ack, self.ack);

                // subtract 1 because a shift of 0 means 1 ack before.
                let shift = seq_diff - 1;

                self.ack = new_ack;
                self.ack_bitfield = (self.ack_bitfield << seq_diff) | (1 << shift);
                Ok(packet.get_data(self.protocol))
            }
            else {
                // we only need to set the ack_bitfield.
                // self.ack is more current than the new_ack
                let seq_diff = Client::get_seq_diff(self.ack, new_ack);
                self.ack_bitfield |= 1 << (seq_diff - 1);
                Ok(None)
            }
        }
        else {
            Err(ErrorKind::InvalidData.into())
        }
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
