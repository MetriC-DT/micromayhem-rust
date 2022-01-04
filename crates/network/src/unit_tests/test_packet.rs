use crate::packet::{Packet, ProtocolId, DATA_BYTES};

#[test]
fn test_init() {
    let protocol_id: ProtocolId = 14;
    let data: [u8; DATA_BYTES] = [8; DATA_BYTES];
    let sequence: u16 = 0;
    let ack: u16 = 0;
    let ackbitfield: u32 = 0;

    let packet = Packet::new(protocol_id, sequence, ack, ackbitfield, data);

    assert_eq!(packet.get_sequence(), sequence);
    assert_eq!(*packet.get_data(protocol_id).unwrap(), [8; DATA_BYTES]);
    assert_eq!(ack, packet.get_ack());
    assert!(packet.verify(protocol_id));
}

#[test]
fn test_from_into() {
    let protocol_id: ProtocolId = 14;
    let data: [u8; DATA_BYTES] = [u8::MAX; DATA_BYTES];
    let sequence: u16 = 20;
    let ack: u16 = 3;
    let ackbitfield: u32 = 0;

    let packet = Packet::new(protocol_id, sequence, ack, ackbitfield, data);
    let bytes: Vec<u8> = packet.into();

    let obtained_packet = Packet::from(bytes);

    assert_eq!(sequence, obtained_packet.get_sequence());
    assert_eq!(ack, obtained_packet.get_ack());
    assert!(obtained_packet.verify(protocol_id));
    assert_eq!(*obtained_packet.get_data(protocol_id).unwrap(), data);
}

#[test]
fn test_id_no_match() {
    let protocol_id = 14;
    let data: [u8; DATA_BYTES] = [u8::MAX; DATA_BYTES];
    let sequence: u16 = 20;
    let ack: u16 = 0;
    let ackbitfield: u32 = 0;

    let packet = Packet::new(protocol_id, sequence, ack, ackbitfield, data);

    assert_eq!(packet.get_data(protocol_id + 1), None);
}

#[test]
fn test_recency() {
    let s1 = 3;
    let s2 = 15;
    let protocol_id = 14;
    let data: [u8; DATA_BYTES] = [u8::MAX; DATA_BYTES];
    let ack: u16 = 0;
    let ackbitfield: u32 = 0;

    let packet1 = Packet::new(protocol_id, s1, ack, ackbitfield, data);
    let packet2 = Packet::new(protocol_id, s2, ack, ackbitfield, data);

    assert!(packet2.is_more_recent_than(&packet1));
    assert!(!packet1.is_more_recent_than(&packet2));
}
