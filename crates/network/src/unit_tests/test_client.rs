use std::{thread::{self, sleep}, net::SocketAddr, time::Duration};

use crate::{server::Server, DEFAULT_PORT, client::Client};

#[test]
fn client_connect() {
    let mut s1 = Server::new(DEFAULT_PORT - 2, 4).unwrap();

    let t = thread::spawn(|| {
        let mut c1 = Client::new(DEFAULT_PORT + 2).unwrap();
        // sends connect request
        c1.connect(&SocketAddr::from(([0,0,0,0], DEFAULT_PORT-2)), "test").unwrap();
        sleep(Duration::from_millis(50));
        c1.receive();
        sleep(Duration::from_millis(50));
        c1.receive();
        assert!(c1.try_get_remote().is_some());
    });

    sleep(Duration::from_millis(50));

    // receives connect request and sends back verification.
    s1.receive();
    sleep(Duration::from_millis(50));

    assert_eq!(s1.get_remotes().len(), 1);

    t.join().unwrap();
}

#[test]
fn client_verification() {
    let server_port = DEFAULT_PORT - 1;
    let client_port = DEFAULT_PORT + 1;
    let mut s1 = Server::new(server_port, 2).unwrap();

    let t1 = thread::spawn(move || {
        let mut c1 = Client::new(client_port).unwrap();
        // sends connect request
        c1.connect(&SocketAddr::from(([0,0,0,0], server_port)), "test").unwrap();
        sleep(Duration::from_millis(50));
        c1.receive();
        sleep(Duration::from_millis(50));
        c1.receive();
        assert!(c1.try_get_id().is_some());
        assert_eq!(c1.try_get_id().unwrap(), 0);

        assert!(c1.try_get_arena().is_some());
    });

    sleep(Duration::from_millis(50));

    // receives connect request and sends back verification.
    s1.receive();
    sleep(Duration::from_millis(50));

    assert_eq!(s1.get_remotes().len(), 1);

    let client_addr = SocketAddr::from(([127,0,0,1], client_port));
    let connected_client = s1.get_remotes().get(&client_addr);
    assert!(connected_client.is_some());

    let connected_client_id = connected_client.unwrap();
    assert_eq!(*connected_client_id, 0);
    t1.join().unwrap();
}
