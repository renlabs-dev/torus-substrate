use subxt_signer::ecdsa::dev::alice;
use torus_client::client::TorusClient;

#[tokio::main]
pub async fn main() {
    let signer = alice(); // change it to your signer
    let name = "alice agent".as_bytes().to_vec();
    let url = "url".as_bytes().to_vec();
    let metadata = "metadata".as_bytes().to_vec();

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
