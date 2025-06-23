//! Tests for the mainnet interfaces
//!
//! Run with: cargo test -p torus-client --features mainnet test_mainnet -- --nocapture

use std::str::FromStr;

use crate::{utils::hex_to_bytes, Network, TorusClient};
use subxt::{config::Header, utils::H256};

#[tokio::test]
async fn test_mainnet_connection() -> Result<(), Box<dyn std::error::Error>> {
    // Only test if the mainnet feature is enabled
    {
        println!("Connecting to Torus Mainnet...");
        let client = TorusClient::for_network(Network::Mainnet).await?;

        // Get block information
        let api = client.client();

        // Get latest block
        let hash = H256::from_slice(
            &hex_to_bytes("0x672214ecbe3d987850d54fc31e771c11e110577dbf59683e95942805ea23aea9")
                .unwrap(),
        );
        let block = api.blocks().at(hash).await?;
        let header = block.header();
        let block_hash = header.parent_hash;
        let block_number = header.number - 1;

        println!("Block: #{} (hash: {:?})", block_number, block_hash);

        // Get runtime version directly from the client
        let runtime_version = api.runtime_version();
        println!("\nRuntime Information:");
        println!("  Spec Version: {}", runtime_version.spec_version);
        println!(
            "  Transaction Version: {}",
            runtime_version.transaction_version
        );

        let account =
            subxt::utils::AccountId32::from_str("5GvBntw5j45K7kMwj9XahfwEW7ByJHRNPrSFmBzUyHcnaYNT")
                .unwrap();

        let call = crate::interfaces::mainnet::api::storage()
            .torus0()
            .staked_by_iter1(account);

        let mut res = client
            .client()
            .storage()
            .at(block_hash)
            .iter(call)
            .await
            .unwrap();

        while let Some(val) = res.next().await {
            let val = val.unwrap();

            let staker = val.keys.decoded().unwrap();
            eprintln!("{staker}: {}", val.value);
        }

        println!("Successfully accessed mainnet runtime");
    }

    #[cfg(not(feature = "mainnet"))]
    {
        println!("Skipping mainnet test as the 'mainnet' feature is not enabled");
    }

    Ok(())
}
