use torus_client::client::TorusClient;

#[tokio::main]
pub async fn main() {
    let client = TorusClient::for_mainnet().await.unwrap();

    let pending_emission = client
        .emission0()
        .storage()
        .pending_emission()
        .await
        .unwrap()
        .unwrap_or(0);

    println!("pending emission: {pending_emission}");
}
