use glam::Vec2;
use std::io::{self, Error};

use game::{player::Player, input::InputMask, map::{Map, MapBlocksList}, block::BlockType, arena::Arena, weaponscatalog::BulletType};
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
        let header_byte = self.header as u8;
        v.extend(header_byte.to_le_bytes());
        v.extend(&self.data);
        v
    }

    /// A `connect` is formatted as just the connect header.
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
    /// 2 - [(bullet_id_0 - u16, bullet_type - u8, approximation of bullet position), ... ]
    pub fn write_state(arena: &Arena) -> Message {
        let mut state_bytes = Vec::new();
        let num_players: u8 = arena.get_players().len().try_into().unwrap();
        state_bytes.extend(num_players.to_le_bytes());

        for (id, player) in arena.get_players() {
            state_bytes.extend(id.to_le_bytes());

            let (x, y, x_s, y_s) = Arena::get_approximate_position(player.position);
            state_bytes.extend(x.to_le_bytes());
            state_bytes.extend(y.to_le_bytes());
            state_bytes.extend(x_s.to_le_bytes());
            state_bytes.extend(y_s.to_le_bytes());
        }

        for (id, bullet) in arena.get_bullets() {
            // id sent
            state_bytes.extend(id.to_le_bytes());

            // bullettype
            let bullettype: u8 = bullet.get_bullet_type() as u8;
            state_bytes.extend(bullettype.to_le_bytes());

            // locations
            let (x, y, x_s, y_s) = Arena::get_approximate_position(bullet.get_position());
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
    ///
    /// returned data:
    /// (player_ids, player_positions, bullet_ids, bullet_types, bullet_positions)
    pub fn read_state(&self) -> Result<(Vec<u8>, Vec<Vec2>, Vec<u16>, Vec<BulletType>, Vec<Vec2>)> {
        let mut data_iter = self.data.iter();
        let mut player_count = *data_iter.next()
            .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unable to read player count"))?;

        player_count = u8::from_le(player_count);

        let mut player_ids = Vec::with_capacity(player_count.into());
        let mut player_positions = Vec::with_capacity(player_count.into());
        let mut bullet_ids = Vec::with_capacity(player_count as usize * 3);
        let mut bullet_types = Vec::with_capacity(player_count as usize * 3);
        let mut bullet_positions = Vec::with_capacity(player_count as usize * 3);

        for _ in 0..player_count {
            let id: u8 = u8::from_le(
                *data_iter.next()
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unable to read ID"))?
                );

            player_ids.push(id);
            player_positions.push(Message::read_next_position(&mut data_iter)?);
        }

        // total remaining bytes divided by bytes per bullet
        // 2 for id, 1 for type, 4 for position
        let num_bullets = data_iter.len() / (2 + 1 + 4);
        for _ in 0..num_bullets {
            let byte_1: u8 = *data_iter.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "Invalid data"))?;
            let byte_2: u8 = *data_iter.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "Invalid data"))?;
            let id: u16 = u16::from_le_bytes([byte_1, byte_2]);

            let type_byte: u8 = u8::from_le(
                *data_iter.next()
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Cannot determine type"))?);
            let bullettype: BulletType = BulletType::from_repr(type_byte as usize)
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Cannot determine type"))?;

            let position = Message::read_next_position(&mut data_iter)?;

            bullet_ids.push(id);
            bullet_types.push(bullettype);
            bullet_positions.push(position);
        }

        Ok((player_ids, player_positions, bullet_ids, bullet_types, bullet_positions))
    }

    /// obtains the position decoded from the bytes of the iterator.
    fn read_next_position<'a>(data_bytes: &mut impl Iterator<Item = &'a u8>) -> Result<Vec2> {
        // grabs the next 4 u8 data and gather it as point.
        let [x, y]: [i8; 2]  = {
            let mut pts: [i8; 2] = [0; 2];
            for pt in pts.iter_mut() {
                *pt = i8::from_le_bytes([
                    *data_bytes.next()
                    .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unable to read location"))?
                    ]);
            }
            pts
        };

        let [xs, ys]: [u8; 2] = {
            let mut pts: [u8; 2] = [0; 2];
            for pt in pts.iter_mut() {
                *pt = u8::from_le_bytes([
                    *data_bytes.next()
                    .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unable to read location"))?
                    ]);
            }
            pts
        };

        Ok(Arena::approx_to_position(x, y, xs, ys))
    }

    /// Reads the packet as a request packet.
    pub fn read_request(&self) -> Result<(u8, Player)> {
        let mut data_iter = self.data.iter();
        let id: u8 = u8::from_le(
            *data_iter.next()
            .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unable to read ID"))?
            );

        let namebytes: Vec<u8> = data_iter.cloned().collect();
        let name = String::from_utf8_lossy(&namebytes);
        Ok((id, Player::new(&name)))
    }

    /// Reads the packet as a verify packet.
    pub fn read_verify(&self) -> Result<(u8, Map)> {
        let bytes = &self.data;
        let id: u8 = u8::from_le(
            *bytes.get(0)
            .ok_or_else(|| Error::new(io::ErrorKind::InvalidData, "Unable to read ID"))?
            );

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
