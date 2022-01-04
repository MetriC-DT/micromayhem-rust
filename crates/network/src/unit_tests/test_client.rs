use crate::client::Client;

#[test]
fn client_init() {
    let c: Client = Client::new(30000).unwrap();
}
