//! Tests for the mainnet interfaces
//!
//! Run with: cargo test -p torus-client --features mainnet test_mainnet -- --nocapture

use crate::{Network, TorusClient};
use subxt::config::Header;

#[tokio::test]
async fn test_mainnet_connection() -> Result<(), Box<dyn std::error::Error>> {
    // Only test if the mainnet feature is enabled
    #[cfg(feature = "mainnet")]
    {
        println!("Connecting to Torus Mainnet...");
        let client = TorusClient::for_network(Network::Mainnet).await?;

        // Get block information
        let api = client.client();

        // Get latest block
        let block = api.blocks().at_latest().await?;
        let header = block.header();
        let block_hash = header.hash();
        let block_number = header.number;

        println!("Latest block: #{} (hash: {:?})", block_number, block_hash);

        // Get runtime version directly from the client
        let runtime_version = api.runtime_version();
        println!("\nRuntime Information:");
        println!("  Spec Version: {}", runtime_version.spec_version);
        println!(
            "  Transaction Version: {}",
            runtime_version.transaction_version
        );

        // Use metadata to check available APIs
        #[cfg(feature = "mainnet")]
        {
            use crate::interfaces::mainnet::api;
            println!("\nAvailable pallets:");
            for pallet in api::PALLETS {
                println!("  - {}", pallet);
            }
        }

        println!("Successfully accessed mainnet runtime");
    }

    #[cfg(not(feature = "mainnet"))]
    {
        println!("Skipping mainnet test as the 'mainnet' feature is not enabled");
    }

    Ok(())
}
