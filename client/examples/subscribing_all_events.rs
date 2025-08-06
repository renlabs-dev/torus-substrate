use futures::StreamExt;
use torus_client::client::TorusClient;

#[tokio::main]
pub async fn main() {
    let client = TorusClient::for_mainnet().await.unwrap();

    let mut events = client.events().subscribe_unfiltered().await.unwrap();

    while let Some(Ok(event)) = events.next().await {
        println!(
            "event received {}::{}",
            event.pallet_name(),
            event.variant_name()
        );
    }
}
