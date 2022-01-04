use std::net::{UdpSocket, SocketAddr};
use std::io::Result;

use game::arena::Arena;

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

    pub fn get_socket(&self) -> &UdpSocket {
        &self.socket
    }

    /// sends the data stripped from the arena to the server.
    pub fn send_data(&self, arena: &Arena) -> Result<()> {
        let pos = arena.get_player().position;
        let [x, y] = pos.to_array();
        let mut data = x.to_be_bytes().to_vec();
        let mut ybytes = y.to_be_bytes().to_vec();
        data.append(&mut ybytes);

        self.socket.send(&data)?;
        Ok(())
    }

    pub fn receive(&self) -> Result<[f32; 2]> {
        // TODO: run a sizeof over the data sent.
        const NUMBYTES: usize = 8;
        let mut data: [u8; NUMBYTES] = [0; NUMBYTES];
        let bytesread = self.socket.recv(&mut data)?;

        // TODO: write a from data type.
        let mut xbytes: [u8; 4] = [0; 4];
        let mut ybytes: [u8; 4] = [0; 4];
        xbytes.copy_from_slice(&data[0..4]);
        ybytes.copy_from_slice(&data[4..8]);
        let x = f32::from_be_bytes(xbytes);
        let y = f32::from_be_bytes(ybytes);
        Ok([x, y])
    }
}
