use crate::message::HeaderByte;

#[test]
fn headerbyte_from_test() {
    let a: u8 = 1;
    assert_eq!(HeaderByte::from_repr(a), Some(HeaderByte::Disconnect));
}
