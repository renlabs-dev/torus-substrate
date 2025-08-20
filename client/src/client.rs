use std::marker::PhantomData;

use subxt::{backend::rpc::RpcClient, blocks::Block, utils::H256, OnlineClient, PolkadotConfig};

use crate::chain::Chain;

#[derive(Clone)]
pub struct TorusClient<C> {
    pub(crate) rpc_client: RpcClient,
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) url: String,
    _pd: PhantomData<C>,
}

impl TorusClient<()> {
    const MAINNET_URL: &'static str = "wss://api.torus.network";
    const TESTNET_URL: &'static str = "wss://api.testnet.torus.network";
    const DEVNET_URL: &'static str = "ws://127.0.0.1:9944";

    #[cfg(feature = "mainnet")]
    pub async fn for_mainnet() -> crate::Result<TorusClient<crate::chain::MainNet>> {
        let rpc_client = RpcClient::from_insecure_url(Self::MAINNET_URL).await?;
        let client = OnlineClient::from_rpc_client(rpc_client.clone()).await?;

        Ok(TorusClient {
            rpc_client,
            client,
            url: Self::MAINNET_URL.to_string(),
            _pd: PhantomData,
        })
    }

    #[cfg(feature = "testnet")]
    pub async fn for_testnet() -> crate::Result<TorusClient<crate::chain::TestNet>> {
        let rpc_client = RpcClient::from_insecure_url(Self::TESTNET_URL).await?;
        let client = OnlineClient::from_rpc_client(rpc_client.clone()).await?;

        Ok(TorusClient {
            rpc_client,
            client,
            url: Self::TESTNET_URL.to_string(),
            _pd: PhantomData,
        })
    }

    #[cfg(feature = "devnet")]
    pub async fn for_devnet() -> crate::Result<TorusClient<crate::chain::DevNet>> {
        let rpc_client = RpcClient::from_insecure_url(Self::DEVNET_URL).await?;
        let client = OnlineClient::from_rpc_client(rpc_client.clone()).await?;

        Ok(TorusClient {
            rpc_client,
            client,
            url: Self::DEVNET_URL.to_string(),
            _pd: PhantomData,
        })
    }

    pub async fn for_url<C: Chain>(url: impl AsRef<str>) -> crate::Result<TorusClient<C>> {
        let rpc_client = RpcClient::from_insecure_url(url.as_ref()).await?;
        let client = OnlineClient::from_rpc_client(rpc_client.clone()).await?;

        Ok(TorusClient {
            rpc_client,
            client,
            url: url.as_ref().to_string(),
            _pd: PhantomData,
        })
    }
}

impl<C: Chain> TorusClient<C> {
    /// Get the latest block
    pub async fn latest_block(
        &self,
    ) -> crate::Result<Block<PolkadotConfig, OnlineClient<PolkadotConfig>>> {
        Ok(self.client.blocks().at_latest().await?)
    }

    /// Get a block via its hash
    pub async fn block_at(
        &self,
        hash: H256,
    ) -> crate::Result<Block<PolkadotConfig, OnlineClient<PolkadotConfig>>> {
        Ok(self.client.blocks().at(hash).await?)
    }

    /// Access to the underlying subxt client
    pub fn inner_client(&self) -> &OnlineClient<PolkadotConfig> {
        &self.client
    }

    /// Access to the underlying subxt client
    pub fn inner_rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }
}
