use std::{net::SocketAddr, thread::sleep, time::Duration};

use crate::{server::Server, DEFAULT_PORT, client::Client};

#[test]
fn client_connect() {
    let mut c1 = Client::new(DEFAULT_PORT + 2).unwrap();
    let mut s1 = Server::new(DEFAULT_PORT - 2, 4).unwrap();

    c1.connect(&SocketAddr::from(([0,0,0,0], DEFAULT_PORT - 2)), "test").unwrap();

    sleep(Duration::from_millis(50));
    s1.receive();
    sleep(Duration::from_millis(50));
    c1.receive();
    sleep(Duration::from_millis(50));
    s1.receive();
    sleep(Duration::from_millis(50));
    c1.receive();

    assert_eq!(c1.get_remotes().len(), 1);
    assert_eq!(s1.get_remotes().len(), 1);
}
