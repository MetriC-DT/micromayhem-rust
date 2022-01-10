use crate::{client::Client, DEFAULT_PORT, PROTOCOL_ID};
use std::{io::Result, thread::sleep, time::Duration};

#[test]
fn client_recv_2() -> Result<()> {
    let mut c1 = Client::new(DEFAULT_PORT + 4).unwrap();
    let mut c2 = Client::new(DEFAULT_PORT - 4).unwrap();
    c1.connect(format!("0.0.0.0:{}", DEFAULT_PORT - 4).as_str())?;
    c2.connect(format!("0.0.0.0:{}", DEFAULT_PORT + 4).as_str())?;

    let payload_1: i32 = 32;
    let payload_2: i32 = 33;
    c1.send_data(&payload_1.to_le_bytes().to_vec()).unwrap();

    sleep(Duration::from_millis(50));

    let data = c2.receive()?;
    assert!(data.is_some());

    // 2nd data received.
    c1.send_data(&payload_2.to_le_bytes().to_vec()).unwrap();

    sleep(Duration::from_millis(50));

    let data = c2.receive()?;

    assert!(data.is_some());

    // checks first 4 bytes and converts it into an i32 to check if it matches
    let data = &data.unwrap()[0..4].try_into().unwrap();
    let result = i32::from_le_bytes(*data);

    assert_eq!(result, payload_2);

    Ok(())
}
