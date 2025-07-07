use std::sync::Arc;

use jsonrpsee::{core::RpcResult, types::ErrorObject};
use pallet_permission0_api::{Permission0RuntimeApi, StreamId};
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

#[jsonrpsee::proc_macros::rpc(client, server)]
pub trait Permission0StreamApi {
    #[method(name = "permission0_rootStreamIdForAccount")]
    async fn root_stream_id_for_account(&self, account: AccountId) -> RpcResult<StreamId>;
}

pub struct Permission0Rpc<Client> {
    client: Arc<Client>,
}

impl<Client> Permission0Rpc<Client> {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl<Client> Permission0StreamApiServer for Permission0Rpc<Client>
where
    Client: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    Client::Api: Permission0RuntimeApi<Block, AccountId>,
{
    async fn root_stream_id_for_account(&self, account: AccountId) -> RpcResult<StreamId> {
        let runtime = &*self.client.runtime_api();
        let at = self.client.info().best_hash;

        runtime
            .root_stream_id_for_account(at, account)
            .map_err(|err| ErrorObject::owned(1, "Runtime execution failed", Some(err.to_string())))
    }
}
