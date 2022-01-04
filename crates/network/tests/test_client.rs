use network::client::Client;

#[test]
fn new_client() {
    let c: Client = Client::new(30000).unwrap();
}
