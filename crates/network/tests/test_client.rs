use network::{client::Client, DEFAULT_PORT, PROTOCOL_ID};
use std::io::Result;

#[test]
fn client_init() {
    let _c1 = Client::new(DEFAULT_PORT - 1, PROTOCOL_ID).unwrap();
    let _c2 = Client::new(DEFAULT_PORT + 1, PROTOCOL_ID).unwrap();
}

#[test]
fn client_init_fail() {
    let _c1 = Client::new(DEFAULT_PORT, PROTOCOL_ID).unwrap();
    let c2 = Client::new(DEFAULT_PORT, PROTOCOL_ID);

    assert!(c2.is_err());
}

#[test]
fn client_connect() -> Result<()> {
    let mut c1 = Client::new(DEFAULT_PORT + 2, PROTOCOL_ID).unwrap();
    let c2 = Client::new(DEFAULT_PORT - 2, PROTOCOL_ID).unwrap();

    let success1 = c1.connect(format!("0.0.0.0:{}", DEFAULT_PORT - 2).as_str());
    let success2 = c2.connect(format!("0.0.0.0:{}", DEFAULT_PORT + 2).as_str());

    assert!(success1.is_ok());
    assert!(success2.is_ok());

    c1.send_data(&32_i32.to_be_bytes().to_vec())?;

    Ok(())
}
