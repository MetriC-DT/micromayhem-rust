use crate::packet::{Packet, ProtocolId, DATA_BYTES, PACKET_BYTES};
use std::io::Result;

#[test]
fn test_init() -> Result<()> {
    let protocol_id: ProtocolId = 14;
    let data = vec![8; DATA_BYTES];
    let sequence: u16 = 0;
    let ack: u16 = 0;
    let ackbitfield: u32 = 0;

    let packet = Packet::new(protocol_id, sequence, ack, ackbitfield, &data)?;

    assert_eq!(packet.get_sequence(), sequence);
    assert_eq!(ack, packet.get_ack());
    assert!(packet.verify(protocol_id));
    assert_eq!(*packet.get_data(protocol_id).unwrap(), [8; DATA_BYTES]);

    Ok(())
}

#[test]
fn test_from_into() -> Result<()> {
    let protocol_id: ProtocolId = 14;
    let data = vec![u8::MAX; DATA_BYTES];
    let sequence: u16 = 20;
    let ack: u16 = 3;
    let ackbitfield: u32 = 0;

    let packet = Packet::new(protocol_id, sequence, ack, ackbitfield, &data)?;
    let bytes: Vec<u8> = packet.into();

    assert_eq!(bytes.len(), PACKET_BYTES);

    let obtained_packet = Packet::try_from(&bytes).unwrap();

    assert_eq!(sequence, obtained_packet.get_sequence());
    assert_eq!(ack, obtained_packet.get_ack());
    assert!(obtained_packet.verify(protocol_id));
    assert_eq!(*obtained_packet.get_data(protocol_id).unwrap(), data);

    Ok(())
}

#[test]
fn test_id_no_match() -> Result<()> {
    let protocol_id = 14;
    let data = vec![u8::MAX; DATA_BYTES];
    let sequence: u16 = 20;
    let ack: u16 = 0;
    let ackbitfield: u32 = 0;

    let packet = Packet::new(protocol_id, sequence, ack, ackbitfield, &data)?;

    assert_eq!(packet.get_data(protocol_id + 1), None);

    Ok(())
}

#[test]
fn test_recency() -> Result<()> {
    let s1 = 3;
    let s2 = 15;
    let protocol_id = 14;
    let data = vec![u8::MAX; DATA_BYTES];
    let ack: u16 = 0;
    let ackbitfield: u32 = 0;

    let packet1 = Packet::new(protocol_id, s1, ack, ackbitfield, &data)?;
    let packet2 = Packet::new(protocol_id, s2, ack, ackbitfield, &data)?;

    assert!(packet2.is_more_recent_than(packet1.get_sequence()));
    assert!(!packet1.is_more_recent_than(packet2.get_sequence()));

    Ok(())
}
