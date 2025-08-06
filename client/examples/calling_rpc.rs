use subxt_signer::ecdsa::dev::alice;
use torus_client::{chain::MainNet, client::TorusClient};

#[tokio::main]
pub async fn main() {
    let client = TorusClient::for_url::<MainNet>("wss://api-30.nodes.torus.network")
        .await
        .unwrap();

    let account = alice().public_key().to_account_id();
    let path = "agent.test.namespace";
    let (fee, deposit) = client
        .rpc()
        .namespace_path_creation_cost(account, path)
        .await
        .unwrap();

    println!(
        "namespace path `agent.test.namespace` creation cost: {}",
        fee + deposit
    );
}
