use futures::StreamExt;
use torus_client::{client::TorusClient, interfaces::mainnet::api::torus0::events::StakeAdded};

#[tokio::main]
pub async fn main() {
    let client = TorusClient::for_mainnet().await.unwrap();

    let mut events = client.events().subscribe::<StakeAdded>().await.unwrap();

    while let Some(Ok(event)) = events.next().await {
        println!("{} added {} stake to {}", event.0, event.2, event.1);
    }
}
