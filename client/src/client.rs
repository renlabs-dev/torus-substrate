use std::marker::PhantomData;

use subxt::{config::Header, OnlineClient, PolkadotConfig};

use crate::chain::Chain;

pub struct TorusClient<C> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) url: String,
    _pd: PhantomData<C>,
}

impl TorusClient<()> {
    const MAINNET_URL: &'static str = "wss://api.torus.network";
    const TESTNET_URL: &'static str = "wss://api.testnet.torus.network";

    #[cfg(feature = "mainnet")]
    pub async fn for_mainnet() -> crate::Result<TorusClient<crate::chain::MainNet>> {
        Ok(TorusClient {
            client: OnlineClient::from_url(Self::MAINNET_URL).await?,
            url: Self::MAINNET_URL.to_string(),
            _pd: PhantomData,
        })
    }

    #[cfg(feature = "testnet")]
    pub async fn for_testnet() -> crate::Result<TorusClient<crate::chain::TestNet>> {
        Ok(TorusClient {
            client: OnlineClient::from_url(Self::TESTNET_URL).await?,
            url: Self::TESTNET_URL.to_string(),
            _pd: PhantomData,
        })
    }

    pub async fn for_url<C: Chain>(url: impl AsRef<str>) -> crate::Result<TorusClient<C>> {
        Ok(TorusClient {
            client: OnlineClient::from_url(url.as_ref().to_string()).await?,
            url: url.as_ref().to_string(),
            _pd: PhantomData,
        })
    }
}

impl<C: Chain> TorusClient<C> {
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
