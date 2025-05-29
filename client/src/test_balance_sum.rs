//! Test to sum all Torus account balances from mainnet
//!
//! Run with: cargo test -p torus-client --features mainnet test_balance_sum -- --nocapture

use crate::{Network, TorusClient};

#[tokio::test]
async fn test_sum_all_balances() -> Result<(), Box<dyn std::error::Error>> {
    // Only test if the mainnet feature is enabled
    #[cfg(feature = "mainnet")]
    {
        println!("Connecting to Torus Mainnet...");
        let client = TorusClient::for_network(Network::Mainnet).await?;

        // Use the generated interfaces to query the balances storage
        #[cfg(feature = "mainnet")]
        {
            use crate::interfaces::mainnet::api;

            let api_client = client.client();

            // Create a storage query to fetch all accounts
            let accounts_query = api::storage().system().account_iter();

            // Get the latest block for querying
            let block = api_client.blocks().at_latest().await?;

            // Fetch all account balances
            let mut accounts_stream = block.storage().iter(accounts_query).await?;

            let mut total_balance = 0u128;
            let mut account_count = 0;

            // Iterate through all accounts and sum their balances
            println!("Fetching all account balances...");

            let mut accounts_vec = Vec::new();

            // Use StreamExt::next() to manually iterate through the stream
            while let Some(result) = accounts_stream.next().await {
                if let Ok(key_value_pair) = result {
                    let account_info = key_value_pair.value;
                    let free = account_info.data.free;
                    let reserved = account_info.data.reserved;
                    let total = free + reserved;

                    accounts_vec.push((account_info.nonce, free, reserved, total));
                    total_balance += total;
                    account_count += 1;
                }
            }

            // Sort accounts by total balance (descending)
            accounts_vec.sort_by(|a, b| b.3.cmp(&a.3));

            // Display summary
            println!("\nTotal Accounts: {}", account_count);
            println!("Total Balance: {} (raw units)", total_balance);

            // Format for human-readable display (assuming 12 decimals for Torus token)
            let total_balance_human = total_balance as f64 / 1_000_000_000_000f64;
            println!("Total Balance: {:.2} TORUS", total_balance_human);

            // Display top 10 accounts by balance
            println!("\nTop 10 accounts by balance:");
            println!("Nonce | Free Balance | Reserved Balance | Total Balance");
            println!("-----|-------------|-----------------|-------------");

            for (nonce, free, reserved, total) in accounts_vec.iter().take(10) {
                let free_human = *free as f64 / 1_000_000_000_000f64;
                let reserved_human = *reserved as f64 / 1_000_000_000_000f64;
                let total_human = *total as f64 / 1_000_000_000_000f64;

                println!(
                    "{:4} | {:12.4} | {:15.4} | {:12.4}",
                    nonce, free_human, reserved_human, total_human
                );
            }
        }
    }

    #[cfg(not(feature = "mainnet"))]
    {
        println!("Skipping balance sum test as the 'mainnet' feature is not enabled");
    }

    Ok(())
}
