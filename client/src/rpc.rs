use std::marker::PhantomData;

use subxt::backend::rpc::RpcClient;
use subxt::ext::subxt_rpcs::client::RpcParams;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

use crate::client::TorusClient;

impl<C> TorusClient<C> {
    pub async fn rpc(&self) -> crate::Result<Rpc<C>> {
        Ok(Rpc {
            client: self.client.clone(),
            rpc_client: self.rpc_client.clone(),
            _pd: PhantomData,
        })
    }
}

pub struct Rpc<C> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) rpc_client: RpcClient,
    pub(crate) _pd: PhantomData<C>,
}

impl<C> Rpc<C> {
    pub async fn namespace_path_creation_cost(
        &self,
        account_id: AccountId32,
        path: impl AsRef<str>,
    ) -> crate::Result<(u128, u128)> {
        let mut rpc_params = RpcParams::new();
        rpc_params.push(account_id)?;

        let path = path.as_ref().as_bytes().to_vec();
        rpc_params.push(path)?;

        let res = self
            .rpc_client
            .request("torus0_namespacePathCreationCost", rpc_params)
            .await?;

        Ok(res)
    }
}
