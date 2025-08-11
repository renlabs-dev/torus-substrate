use subxt_signer::ecdsa::dev::{alice, bob};
use torus_client::client::TorusClient;

#[tokio::main]
pub async fn main() {
    let agent_key = alice().public_key().to_account_id(); //change it to your agent's account id
    let name = "alice agent".as_bytes().to_vec();
    let url = "url".as_bytes().to_vec();
    let metadata = "metadata".as_bytes().to_vec();
    let signer = bob(); // change it to your signer

    let client = TorusClient::for_mainnet().await.unwrap();

    if let Err(err) = client
        .torus0()
        .calls()
        .register_agent_wait(name, url, metadata, signer)
        .await
    {
        print!("could not register agent: {err}");
    }
}
