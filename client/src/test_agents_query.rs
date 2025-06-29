//! Quick test for querying Torus0 agents

use crate::generated_wrappers::{get_torus0_agents_by_account_id32, query_map_torus0_staked_by};
use crate::{Network, TorusClient};

#[tokio::test]
async fn test_query_staked_by_map() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "mainnet")]
    {
        println!("Connecting to mainnet...");
        let client = TorusClient::for_network(Network::Mainnet).await?;
        let api = client.client();

        println!("Querying staked by map...");
        let staked_by = query_map_torus0_staked_by(api).await?;

        println!("Found {} agents with stakers", staked_by.len());

        for (agent, stakers_map) in staked_by.iter().take(3) {
            println!("Agent: {}", agent);
            for (staker, stake) in stakers_map.iter().take(2) {
                println!("  Staker: {} | Stake: {}", staker, stake);
            }
        }
    }

    #[cfg(not(feature = "mainnet"))]
    {
        println!("Mainnet feature not enabled");
    }

    Ok(())
}

#[tokio::test]
async fn test_specific_agent() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "mainnet")]
    {
        use std::str::FromStr;
        use subxt::ext::subxt_core::utils::AccountId32;

        println!("Testing specific agent: 5GbgDadME6WijYSiB6cNPicpR83WogQwx5Jfs5yXAbnM2v5y");
        let client = TorusClient::for_network(Network::Mainnet).await?;
        let api = client.client();

        let account_id = AccountId32::from_str("5GbgDadME6WijYSiB6cNPicpR83WogQwx5Jfs5yXAbnM2v5y")?;

        match get_torus0_agents_by_account_id32(api, account_id).await? {
            Some(agent) => {
                println!("Agent found: {:?}", agent);
            }
            None => {
                println!("Agent not found");
            }
        }
    }

    #[cfg(not(feature = "mainnet"))]
    {
        println!("Mainnet feature not enabled");
    }

    Ok(())
}
