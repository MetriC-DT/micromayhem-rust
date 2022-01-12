use std::io;

use game::{player::Player, input::InputMask};
use strum_macros::FromRepr;

#[derive(Debug, PartialEq, FromRepr, Clone, Copy)]
#[repr(u8)]
pub(crate) enum HeaderByte {
    Connect,
    Disconnect,
    Verify,
    Request,
    State,
    Input
}

#[derive(Debug, Clone)]
pub struct Message {
    pub(crate) header: HeaderByte,
    pub(crate) data: Vec<u8>
}

impl TryFrom<Vec<u8>> for Message {
    type Error = io::Error;

    fn try_from(packetdata: Vec<u8>) -> Result<Message, io::Error> {
        let mut data_iter = packetdata.into_iter();
        let headerbyte = data_iter.next();
        if let Some(i) = headerbyte {
            let header = HeaderByte::from_repr(i).unwrap();
            let data = data_iter.collect();
            Ok(Message { header, data })
        } else {
            Err(io::ErrorKind::InvalidData.into())
        }
    }
}

impl Message {
    pub(crate) fn to_vec(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(1 + self.data.len());
        v.push(self.header as u8);
        v.extend(self.data.clone());
        v
    }

    pub(crate) fn read_connect(&self) -> Player {
        let namebytes: Vec<u8> = self.data.clone().into_iter().collect();
        let name = String::from_utf8_lossy(&namebytes);
        Player::new(&name)
    }

    pub(crate) fn read_input(&self) -> InputMask {
        let mask = self.data.iter().next();
        if let Some(mask) = mask {
            InputMask::from(*mask)
        }
        else {
            InputMask::new()
        }
    }

    pub(crate) fn write_connect(name: &str) -> Message {
        Message {
            header: HeaderByte::Connect,
            data: name.as_bytes().into_iter().cloned().collect()
        }
    }

    pub(crate) fn write_verify() -> Message {
        Message {
            header: HeaderByte::Verify,
            data: Vec::new()
        }
    }

    pub(crate) fn write_request() -> Message {
        Message {
            header: HeaderByte::Request,
            data: Vec::new()
        }
    }

    pub(crate) fn write_input(&self, input: InputMask) -> Message {
        Message {
            header: HeaderByte::Input,
            data: vec![input.into()]
        }
    }
}
