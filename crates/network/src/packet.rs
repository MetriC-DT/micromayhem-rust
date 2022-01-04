use std::mem::size_of;

/// Number of bytes for the protocol id.
pub const PROTOCOL_ID_BYTES: usize = size_of::<ProtocolId>();

/// Number of bytes for the data buffer.
pub const DATA_BYTES: usize = 256;

/// Number of bytes for the sequence number and ack.
pub const SEQUENCE_BYTES: usize = size_of::<u16>();

/// number of bytes for the acknowledgement bitfield
pub const BITFIELD_BYTES: usize = size_of::<u32>();

/// Number of bytes per packet.
pub const PACKET_BYTES: usize = DATA_BYTES + PROTOCOL_ID_BYTES;

/// Type alias for protocol id, a 32 bit header tag on every packet.
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
/// https://gafferongames.com/post/reliability_ordering_and_congestion_avoidance_over_udp/
pub struct Packet {
    protocol_id: ProtocolId,
    sequence: u16,
    ack: u16,
    ackbitfield: u32,
    data: [u8; DATA_BYTES],
}

impl Packet {
    pub fn new(protocol_id: ProtocolId,
               sequence: u16,
               ack: u16,
               ackbitfield: u32,
               data: [u8; DATA_BYTES]) -> Self {

        Self { protocol_id, sequence, ack, ackbitfield, data }
    }

    /// verifies, then gets the data, provided the given id matches with the packet's protocol id.
    /// data can only be extracted from the packet if the `protocol_id` given matches 
    /// this packet's protocol id.
    ///
    /// assumes id > 0. Otherwise, invalid packets may not be verified properly.
    pub fn get_data(&self, id: ProtocolId) -> Option<&[u8; DATA_BYTES]> {
        if id > 0 && id == self.protocol_id {
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
}

impl Into<Vec<u8>> for Packet {
    fn into(self) -> Vec<u8> {
        // protocol
        let mut packet_bytes = self.protocol_id.to_be_bytes().to_vec();

        // sequence
        packet_bytes.extend_from_slice(&self.sequence.to_be_bytes());

        // ack
        packet_bytes.extend_from_slice(&self.ack.to_be_bytes());

        // ack bitfield
        packet_bytes.extend_from_slice(&self.ackbitfield.to_be_bytes());

        // data
        packet_bytes.extend_from_slice(&self.data);

        packet_bytes
    }
}

impl From<Vec<u8>> for Packet {
    fn from(bytes: Vec<u8>) -> Self {
        let mut start = 0;

        // parse protocol id.
        let protocol_id_bytes = &bytes[start..start+PROTOCOL_ID_BYTES].try_into();
        start += PROTOCOL_ID_BYTES;

        // README: if unable to obtain protocol_id, thne fill it in with pure zeroes.
        // Therefore, when verifying the validity of the packet, we can just throw 
        // bad packets out because it will not match our protocol id
        // (which should not be all zeroes)
        let protocol_id: ProtocolId = ProtocolId::from_be_bytes(
            protocol_id_bytes.unwrap_or_else(|_| [0; PROTOCOL_ID_BYTES]));

        // parse sequence bytes.
        let sequence_bytes = &bytes[start..start+SEQUENCE_BYTES].try_into();
        start += SEQUENCE_BYTES;

        // README: 0 same with sequence bytes.
        let sequence = u16::from_be_bytes(
            sequence_bytes.unwrap_or_else(|_| [0; SEQUENCE_BYTES]));

        // parse ack bytes.
        let ack_bytes = &bytes[start..start+SEQUENCE_BYTES].try_into();
        start += SEQUENCE_BYTES;

        // README: 0 same with ack bytes.
        let ack = u16::from_be_bytes(
            ack_bytes.unwrap_or_else(|_| [0; SEQUENCE_BYTES]));

        // parse ack bitfield.
        let ackbits_bytes = &bytes[start..start+BITFIELD_BYTES].try_into();
        start += BITFIELD_BYTES;

        let ackbitfield = u32::from_be_bytes(
            ackbits_bytes.unwrap_or_else(|_| [0; BITFIELD_BYTES]));


        // README: if unable to generate data, then fill in data with pure zeroes.
        let data_bytes = &bytes[start..].try_into();
        let data: [u8; DATA_BYTES] = data_bytes.unwrap_or_else(|_| [0; DATA_BYTES]);


        Self {protocol_id, sequence, ack, ackbitfield, data}
    }
}