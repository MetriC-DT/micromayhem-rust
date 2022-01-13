use std::io;

use game::{player::Player, input::InputMask, map::{Map, MapBlocksList}, block::BlockType};
use strum::{IntoEnumIterator, EnumCount};
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

    pub(crate) fn read_connect(self) -> Player {
        let namebytes: Vec<u8> = self.data.into_iter().collect();
        let name = String::from_utf8_lossy(&namebytes);
        Player::new(&name)
    }

    pub(crate) fn read_verify(self) -> (u8, Map) {
        let mut bytes = self.data;
        let id: u8 = *bytes.get(0).expect("Unable to get id");
        let mut starter_bit = 1;
        let mut mapblockslist: MapBlocksList = [0; BlockType::COUNT];

        for blocktype in BlockType::iter() {
            let bits: [u8; 16] = bytes[starter_bit..starter_bit+16]
                .try_into()
                .expect("Unable to obtain map bits.");

            let block_bits = i128::from_le_bytes(bits);
            mapblockslist[blocktype as usize] = block_bits;
            starter_bit += 16;
        }

        (id, Map::new(mapblockslist.into()).unwrap())
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

    pub(crate) fn write_verify(id: u8, map: &Map) -> Message {
        let mapdata = map.get_mapblocks_list();
        let mut data_vec = Vec::with_capacity(mapdata.len() + 1);
        data_vec.push(id);
        for blocktypelist in mapdata {
            data_vec.extend(blocktypelist.to_le_bytes());
        }

        Message {
            header: HeaderByte::Verify,
            data: data_vec,
        }
    }

    pub(crate) fn write_request() -> Message {
        Message {
            header: HeaderByte::Request,
            data: Vec::new()
        }
    }

    pub(crate) fn write_input(input: InputMask) -> Message {
        Message {
            header: HeaderByte::Input,
            data: vec![input.into()]
        }
    }
}
