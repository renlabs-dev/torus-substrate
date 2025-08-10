use futures::StreamExt;
use torus_client::client::TorusClient;

#[tokio::main]
pub async fn main() {
    let client = TorusClient::for_mainnet().await.unwrap();

    let total_stake: u128 = client
        .torus0()
        .storage()
        .staking_to_iter()
        .await
        .unwrap()
        .fold(0u128, |value, entry| async move {
            let ((_, _), stake) = entry.unwrap();
            value + stake
        })
        .await;

    println!("torus total stake: {total_stake}");
}
