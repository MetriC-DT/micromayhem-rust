use std::io::{self, Error};

use game::{player::Player, input::InputMask, map::{Map, MapBlocksList}, block::BlockType, arena::Arena};
use strum::{IntoEnumIterator, EnumCount};
use strum_macros::FromRepr;
use std::io::Result;

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

    fn try_from(packetdata: Vec<u8>) -> Result<Message> {
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
    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(1 + self.data.len());
        v.push(self.header as u8);
        v.extend(self.data.clone());
        v
    }

    pub fn write_connect() -> Message {
        Message {
            header: HeaderByte::Connect,
            data: Vec::new(),
        }
    }

    /// A `request` is formatted with its data being:
    /// first byte = id of player,
    /// rest = name
    pub fn write_request(name: &str, id: u8) -> Message {
        let mut data_vec = Vec::from(id.to_le_bytes());
        data_vec.extend(name.bytes());

        Message {
            header: HeaderByte::Request,
            data: data_vec
        }
    }

    /// A `verify` message is formatted with:
    /// first byte = id of player,
    /// rest = map
    pub fn write_verify(id: u8, map: &Map) -> Message {
        let mapdata = map.get_mapblocks_list();
        let mut data_vec = Vec::with_capacity(mapdata.len() + 1);
        data_vec.extend(id.to_le_bytes());
        for blocktypelist in mapdata {
            data_vec.extend(blocktypelist.to_le_bytes());
        }

        Message {
            header: HeaderByte::Verify,
            data: data_vec,
        }
    }

    /// An `input` message is formatted with just an InputMask converted to a u8 byte.
    pub fn write_input(input: InputMask) -> Message {
        let data: u8 = input.into();
        let data_vec = Vec::from(data.to_le_bytes());
        Message {
            header: HeaderByte::Input,
            data: data_vec
        }
    }

    /// an `arena` message is formatted with
    ///
    /// 0 - number of players
    /// 1 - [(player_id_0 - u8, approximation of player position), ... ]
    /// 2 - [(bullet_id_0 - u16, approximation of bullet position, bullet_type - u8), ... ]
    pub fn write_state(arena: &Arena) -> Message {
        let mut state_bytes = Vec::new();
        let num_players: u8 = arena.get_players().len().try_into().unwrap();
        state_bytes.push(num_players);

        for (id, player) in arena.get_players() {
            state_bytes.push(*id);
            let (x, y, x_s, y_s) = Arena::get_approximate_position(player.position);
            state_bytes.extend(x.to_le_bytes());
            state_bytes.extend(y.to_le_bytes());
            state_bytes.extend(x_s.to_le_bytes());
            state_bytes.extend(y_s.to_le_bytes());
        }

        Message {
            header: HeaderByte::State,
            data: state_bytes
        }
    }

    /// Reads the packet as a gamestate packet.
    pub fn read_state<E>(self) -> Result<Arena> {
        Ok(Arena::default())
    }

    /// Reads the packet as a request packet.
    pub fn read_request(&self) -> Result<(u8, Player)> {
        let mut data_iter = self.data.iter();
        let mut id = *data_iter.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Unable to read ID"))?;

        id = u8::from_le(id);
        let namebytes: Vec<u8> = data_iter.cloned().collect();
        let name = String::from_utf8_lossy(&namebytes);
        Ok((id, Player::new(&name)))
    }

    /// Reads the packet as a verify packet.
    pub fn read_verify(&self) -> Result<(u8, Map)> {
        let bytes = &self.data;
        let mut id: u8 = *bytes.get(0)
            .ok_or_else(|| Error::new(io::ErrorKind::InvalidData, "Unable to read ID"))?;

        id = u8::from_le(id);
        let mut starter_bit = 1;
        let mut mapblockslist: MapBlocksList = [0; BlockType::COUNT];

        for blocktype in BlockType::iter() {
            let bits: [u8; 16] = bytes[starter_bit..starter_bit+16]
                .try_into()
                .map_err(|_| Error::new(io::ErrorKind::InvalidData, "Unable to read map"))?;

            let block_bits = i128::from_le_bytes(bits);
            mapblockslist[blocktype as usize] = block_bits;
            starter_bit += 16;
        }

        let constructed_map = Map::new(mapblockslist.into())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        Ok((id, constructed_map))
    }

    /// Reads the packet as an input packet.
    ///
    /// TODO - returns an err result if unable to do so.
    pub fn read_input(&self) -> InputMask {
        let mask = self.data.get(0);
        if let Some(mask) = mask {
            let maskdata = u8::from_le(*mask);
            InputMask::from(maskdata)
        }
        else {
            InputMask::new()
        }
    }
}
