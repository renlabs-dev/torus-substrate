use std::sync::Arc;

use jsonrpsee::{core::RpcResult, types::ErrorObject};
use pallet_torus0_api::{api::Torus0RuntimeApi, NamespacePathInner};
use polkadot_sdk::{
    sp_api::ProvideRuntimeApi,
    sp_blockchain::HeaderBackend,
    sp_runtime::{
        traits::{IdentifyAccount, Verify},
        MultiSignature,
    },
};
use torus_runtime::opaque::Block;

type Signature = MultiSignature;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
type Balance = u128;

#[jsonrpsee::proc_macros::rpc(client, server)]
pub trait Torus0Api {
    #[method(name = "torus0_namespacePathCreationCost")]
    async fn namespace_path_creation_cost(
        &self,
        account_id: AccountId,
        path: NamespacePathInner,
    ) -> RpcResult<(Balance, Balance)>;
}

pub struct Torus0Rpc<Client> {
    client: Arc<Client>,
}

impl<Client> Torus0Rpc<Client> {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl<Client> Torus0ApiServer for Torus0Rpc<Client>
where
    Client: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    Client::Api: Torus0RuntimeApi<Block, AccountId, Balance>,
{
    async fn namespace_path_creation_cost(
        &self,
        account_id: AccountId,
        path: NamespacePathInner,
    ) -> RpcResult<(Balance, Balance)> {
        let runtime = &*self.client.runtime_api();
        let at = self.client.info().best_hash;

        runtime
            .namespace_path_creation_cost(at, account_id, path)
            .map(|res| {
                res.map_err(|err| {
                    ErrorObject::owned(
                        1,
                        "namespace cost calculation failed",
                        Some(format!("{err:?}")),
                    )
                })
            })
            .map_err(|err| {
                ErrorObject::owned(1, "Runtime execution failed", Some(err.to_string()))
            })?
    }
}
