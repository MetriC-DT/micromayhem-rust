use std::{mem::size_of, io::{ErrorKind, self}, array::TryFromSliceError};

/// Number of bytes for the protocol id.
pub const PROTOCOL_ID_BYTES: usize = size_of::<ProtocolId>();

/// Number of bytes for the data buffer.
pub const DATA_BYTES: usize = 256;

/// Number of bytes for the sequence number and ack.
pub const SEQUENCE_BYTES: usize = size_of::<u16>();

/// number of bytes for the acknowledgement bitfield
pub const BITFIELD_BYTES: usize = size_of::<u32>();

/// Number of bytes maximum per packet.
pub const PACKET_BYTES: usize = PROTOCOL_ID_BYTES + SEQUENCE_BYTES * 2 + BITFIELD_BYTES + DATA_BYTES;

/// Type alias for protocol id, a 16 bit header tag on every packet.
pub type ProtocolId = u16;


/// Represents the UDP packet to be sent over the network.
///
/// `data` can only be extracted from the packet if the `protocol_id` given matches 
/// this packet's protocol id.
///
/// `ack` is the acknowledgement that the remote has received packet with `sequence`
///
/// Because we might receive more packets than we can ack at once, the ackbitfield is used
/// to acknowledge prior sequence numbers that have not been acked yet. if the `nth` bit is set,
/// then this means we acknowledge sequence number `ack - n`.
///
/// `sequence` represents the sequence number of the packet to determine ordering sent.
/// Therefore, `sequence` should be incremented by 1 on every packet send.
///
/// More information on the algorithm and implementation can be found at:
/// [Gaffer on Games](https://gafferongames.com/post/reliability_ordering_and_congestion_avoidance_over_udp/)
///
/// TODO: implement some form of redundancy check (e.g. CRC32) to prevent data corruption.
/// [Explanation](https://gafferongames.com/post/serialization_strategies/)
#[derive(Debug)]
pub struct Packet {
    protocol_id: ProtocolId,
    sequence: u16,
    ack: u16,
    ackbitfield: u32,
    data: Vec<u8>,
}

impl Packet {
    pub fn new(protocol_id: ProtocolId,
               sequence: u16,
               ack: u16,
               ackbitfield: u32,
               dataref: &[u8]) -> Result<Self, io::Error> {

        if dataref.len() <= DATA_BYTES {
            let data = dataref.to_vec();
            Ok(Self { protocol_id, sequence, ack, ackbitfield, data})
        } else {
            Err(ErrorKind::InvalidData.into())
        }
    }

    /// verifies, then gets the data, provided the given id matches with the packet's protocol id.
    /// data can only be extracted from the packet if the `protocol_id` given matches 
    /// this packet's protocol id.
    ///
    /// assumes id > 0. Otherwise, invalid packets may not be verified properly.
    pub fn get_data(&self, id: ProtocolId) -> Option<&Vec<u8>> {
        if self.protocol_id != 0 && self.protocol_id == id {
            Some(&self.data)
        } else {
            None
        }
    }

    /// Obtains the sequence number of the packet.
    pub fn get_sequence(&self) -> u16 {
        self.sequence
    }

    /// Obtains the acknowledgement that the remote has received packet 
    /// with the sequence number that is returned.
    pub fn get_ack(&self) -> u16 {
        self.ack
    }

    /// verifies if the packet's protocol ID is equal to the given id.
    pub fn verify(&self, id: ProtocolId) -> bool {
        self.protocol_id == id
    }

    /// returns true if this packet is more recent than another packet (comparison of 
    /// sequence numbers). The sequence number should allow for overflow wrap around.
    pub fn is_more_recent_than(&self, otherseq: u16) -> bool {
        let tolerance = u16::MAX / 2 + 1;
        let (s1, s2) = (self.sequence, otherseq);

        ( (s1 > s2) && (s1 - s2 <= tolerance) )
            || ( (s1 < s2) && (s2 - s1 > tolerance ) )
    }
}

impl Into<Vec<u8>> for Packet {
    fn into(self) -> Vec<u8> {
        // protocol
        let mut packet_bytes: Vec<u8> = self.protocol_id
            .to_le_bytes()
            .into_iter()
            .collect();

        // sequence
        packet_bytes.extend_from_slice(&self.sequence.to_le_bytes());

        // ack
        packet_bytes.extend_from_slice(&self.ack.to_le_bytes());

        // ack bitfield
        packet_bytes.extend_from_slice(&self.ackbitfield.to_le_bytes());

        // data
        packet_bytes.extend_from_slice(&self.data);

        packet_bytes
    }
}


impl TryFrom<&[u8]> for Packet {
    type Error = TryFromSliceError;

    fn try_from(bytes: &[u8]) -> Result<Packet, Self::Error> {
        let mut start = 0;

        // parse protocol id.
        let protocol_id_bytes = &bytes[start..start+PROTOCOL_ID_BYTES].try_into()?;
        let protocol_id: ProtocolId = ProtocolId::from_le_bytes(*protocol_id_bytes);
        start += PROTOCOL_ID_BYTES;

        // parse sequence bytes.
        let sequence_bytes = &bytes[start..start+SEQUENCE_BYTES].try_into()?;
        let sequence = u16::from_le_bytes(*sequence_bytes);
        start += SEQUENCE_BYTES;

        // parse ack bytes.
        let ack_bytes = &bytes[start..start+SEQUENCE_BYTES].try_into()?;
        let ack = u16::from_le_bytes(*ack_bytes);
        start += SEQUENCE_BYTES;

        // parse ack bitfield.
        let ackbits_bytes = &bytes[start..start+BITFIELD_BYTES].try_into()?;
        let ackbitfield = u32::from_le_bytes(*ackbits_bytes);
        start += BITFIELD_BYTES;

        // data becomes the remaining bits.
        // TODO: we might want to check the length of data in case of 
        // adversarial data packets that get inputted.
        let data: Vec<u8> = bytes[start..].to_vec();

        Ok(Self {protocol_id, sequence, ack, ackbitfield, data})
    }
}
