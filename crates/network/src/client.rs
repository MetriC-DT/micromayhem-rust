use std::net::{UdpSocket, SocketAddr};
use std::io::Result;

use crate::packet::{Packet, PACKET_BYTES};

/// Wrapper for the UDP client socket. Implementation of orderliness
/// and "reliability" based on the 
/// [Gaffer on Games articles](https://gafferongames.com/post/reliability_ordering_and_congestion_avoidance_over_udp/)
pub struct Client {
    socket: UdpSocket,
}

impl Client {
    pub fn new(port: u16) -> Result<Self> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(true)?;
        Ok(Self {socket})
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
    pub fn send_data(&self, data: Packet) -> Result<()> {
        let bytes: Vec<u8> = data.into();
        self.socket.send(&bytes)?;
        Ok(())
    }

    /// TODO - receives and parses data properly.
    pub fn receive(&self) -> Result<Packet> {
        let mut recv_buffer: [u8; PACKET_BYTES] = [0; PACKET_BYTES];
        self.socket.recv(&mut recv_buffer)?;

        let bytes: Vec<u8> = recv_buffer.into_iter().collect();
        let packet = Packet::from(bytes);

        Ok(packet)
    }
}
