use network::{client::Client, DEFAULT_PORT, server::Server};

use std::{io::Result, thread::sleep, time::Duration, net::SocketAddr};

#[test]
fn client_init() {
    let _c1 = Client::new(DEFAULT_PORT - 1).unwrap();
    let _s1 = Client::new(DEFAULT_PORT + 1).unwrap();
}

#[test]
fn client_init_fail() {
    let _c1 = Client::new(DEFAULT_PORT).unwrap();
    let s1 = Server::new(DEFAULT_PORT, 4);

    assert!(s1.is_err());
}

#[test]
fn client_recv() -> Result<()> {
    let mut c1 = Client::new(DEFAULT_PORT + 3).unwrap();
    let mut s1 = Server::new(DEFAULT_PORT - 3, 4).unwrap();
    c1.connect(&SocketAddr::from(([0,0,0,0], DEFAULT_PORT - 3))).unwrap();

    let payload: i32 = 32;
    c1.send_data(&payload.to_be_bytes().to_vec()).unwrap();

    sleep(Duration::from_millis(50));

    let data = &s1.receive();

    assert_eq!(data.len(), 1);

    // checks first 4 bytes and converts it into an i32 to check if it matches
    let data = &data[0][0..4].try_into().unwrap();
    let result = i32::from_be_bytes(*data);

    assert_eq!(result, payload);

    Ok(())
}

#[test]
fn client_recv_2() -> Result<()> {
    let mut c1 = Client::new(DEFAULT_PORT + 4).unwrap();
    let mut s1 = Server::new(DEFAULT_PORT - 4, 4).unwrap();
    c1.connect(&SocketAddr::from(([0,0,0,0], DEFAULT_PORT - 4))).unwrap();

    sleep(Duration::from_millis(50));

    let payload_1: i32 = 32;
    let payload_2: i32 = 33;
    c1.send_data(&payload_1.to_le_bytes().to_vec()).unwrap();

    // required in order to actually receive the data from the sender
    sleep(Duration::from_millis(50));

    let data = &s1.receive();
    assert_eq!(data.len(), 1);

    // 2nd data received.
    c1.send_data(&payload_2.to_le_bytes().to_vec()).unwrap();

    // required in order to actually receive the data from the sender
    sleep(Duration::from_millis(50));

    let data = &s1.receive();

    assert_eq!(data.len(), 1);

    // checks first 4 bytes and converts it into an i32 to check if it matches
    let bytes = &data[0][0..4];
    let result = i32::from_le_bytes(bytes.try_into().unwrap());

    assert_eq!(result, payload_2);

    Ok(())
}

#[test]
fn client_recv_consecutive() -> Result<()> {
    let mut c1 = Client::new(DEFAULT_PORT + 5).unwrap();
    let mut s1 = Server::new(DEFAULT_PORT - 5, 4).unwrap();
    c1.connect(&SocketAddr::from(([0,0,0,0], DEFAULT_PORT - 5))).unwrap();

    let payload_1: i32 = 32;
    let payload_2: i32 = 33;

    c1.send_data(&payload_1.to_le_bytes().to_vec()).unwrap();
    c1.send_data(&payload_2.to_le_bytes().to_vec()).unwrap();

    sleep(Duration::from_millis(50));

    let data = s1.receive();
    assert_eq!(data.len(), 2);
    Ok(())
}
