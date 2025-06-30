use std::marker::PhantomData;

use subxt::{config::Header, OnlineClient, PolkadotConfig};

use crate::{chain::Chain, pallets::torus0::Torus0};

pub struct TorusClient<C: Chain> {
    client: OnlineClient<PolkadotConfig>,
    _pd: PhantomData<C>,
}

impl<C: Chain> TorusClient<C> {
    const MAINNET_URL: &'static str = "wss://api.torus.client";
    const TESTNET_URL: &'static str = "wss://api.testnet.torus.client";

    #[cfg(feature = "mainnet")]
    pub async fn for_mainnet() -> anyhow::Result<TorusClient<crate::chain::MainNet>> {
        Ok(TorusClient {
            client: OnlineClient::from_url(Self::MAINNET_URL).await?,
            _pd: PhantomData,
        })
    }

    #[cfg(feature = "testnet")]
    pub async fn for_testnet() -> anyhow::Result<TorusClient<crate::chain::TestNet>> {
        Ok(TorusClient {
            client: OnlineClient::from_url(Self::TESTNET_URL).await?,
            _pd: PhantomData,
        })
    }

    pub fn torus0(&self) -> Torus0<C> {
        Torus0 {
            client: self.client.clone(),
            _pd: PhantomData,
        }
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
