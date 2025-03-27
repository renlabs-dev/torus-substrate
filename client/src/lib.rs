//! # Torus Client
//!
//! A client library for interacting with the Torus blockchain network using subxt.

pub mod utils;

// Generated interfaces for different networks
pub mod interfaces {
    // These modules will be populated by the `just gen_interfaces` command
    #[cfg(feature = "mainnet")]
    pub mod mainnet;

    #[cfg(feature = "testnet")]
    pub mod testnet;

    // #[cfg(feature = "dev")]
    // pub mod dev;
}

// Tests
#[cfg(test)]
mod test_balance_sum;
#[cfg(test)]
mod test_mainnet;

use subxt::{config::Header, OnlineClient, PolkadotConfig};

/// Network type to connect to
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Mainnet,
    Testnet,
    Dev,
}

impl Network {
    /// Get the URL for this network
    pub fn url(&self) -> &'static str {
        match self {
            Network::Mainnet => "wss://api.torus.network",
            Network::Testnet => "wss://api.testnet.torus.network",
            Network::Dev => "ws://localhost:9944",
        }
    }

    /// Get a descriptive name for this network
    pub fn name(&self) -> &'static str {
        match self {
            Network::Mainnet => "Torus Mainnet",
            Network::Testnet => "Torus Testnet",
            Network::Dev => "Torus Development",
        }
    }
}

/// TorusClient provides functionality for interacting with a Torus node
pub struct TorusClient {
    client: OnlineClient<PolkadotConfig>,
    network: Network,
}

impl TorusClient {
    /// Create a new client by connecting to a Torus node at the given URL
    pub async fn new(url: &str) -> Result<Self, subxt::Error> {
        let client = OnlineClient::<PolkadotConfig>::from_url(url).await?;

        // Determine network based on URL (default to Dev if unclear)
        let network = match url {
            url if url.contains("api.torus.network") && !url.contains("testnet") => {
                Network::Mainnet
            }
            url if url.contains("api.testnet.torus.network") => Network::Testnet,
            _ => Network::Dev,
        };

        Ok(Self { client, network })
    }

    /// Create a new client for a specific network
    pub async fn for_network(network: Network) -> Result<Self, subxt::Error> {
        Self::new(network.url()).await
    }

    /// Get a reference to the inner subxt client
    pub fn client(&self) -> &OnlineClient<PolkadotConfig> {
        &self.client
    }

    /// Get the network this client is connected to
    pub fn network(&self) -> Network {
        self.network
    }

    /// Get the latest block hash
    pub async fn latest_block_hash(&self) -> Result<subxt::utils::H256, subxt::Error> {
        let block = self.client.blocks().at_latest().await?;
        let hash = block.header().hash();
        Ok(hash)
    }

    /// Get the latest block number
    pub async fn latest_block_number(&self) -> Result<u32, subxt::Error> {
        let block = self.client.blocks().at_latest().await?;
        Ok(block.header().number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_urls() {
        // This doesn't actually connect, just checks the URL parsing
        assert_eq!(Network::Mainnet.url(), "wss://api.torus.network");
        assert_eq!(Network::Testnet.url(), "wss://api.testnet.torus.network");
        assert_eq!(Network::Dev.url(), "ws://localhost:9944");
    }

    // Note: Add actual connection tests when local node is available
}
