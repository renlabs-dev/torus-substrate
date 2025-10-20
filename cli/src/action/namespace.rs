use std::fmt::Display;

use tabled::Table;
use torus_client::{
    client::TorusClient,
    subxt::{
        ext::futures::{StreamExt, TryStreamExt},
        utils::AccountId32,
    },
};

use crate::{
    action::{Action, ActionContext, Changes},
    keypair::Keypair,
    store::{get_account, get_key},
};

pub struct NamespaceInfoAction {
    account: AccountId32,
}

impl Action for NamespaceInfoAction {
    type Params = String;
    type ResponseData = NamespaceInfoActionResponse;

    async fn create(_ctx: &mut impl ActionContext, account: Self::Params) -> anyhow::Result<Self> {
        let account = get_account(&account)?;
        Ok(Self { account })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        ctx.info("Fetching namespace data...");

        let entries = if ctx.is_testnet() {
            let client = TorusClient::for_testnet().await?;
            client
            .torus0()
            .storage()
            .namespaces_iter1(&torus_client::interfaces::testnet::api::runtime_types::pallet_torus0::namespace::NamespaceOwnership::Account(self.account.clone()))
            .await?
            .map(|res| match res {
                Ok(((_, path), _)) => Ok(NamespaceEntry { path: String::from_utf8_lossy(&path.0.0).to_string() }),
                Err(err) => Err(err),
            }).try_collect::<Vec<_>>().await?
        } else {
            let client = TorusClient::for_mainnet().await?;
            client
            .torus0()
            .storage()
            .namespaces_iter1(&torus_client::interfaces::mainnet::api::runtime_types::pallet_torus0::namespace::NamespaceOwnership::Account(self.account.clone()))
            .await?
            .map(|res| match res {
                Ok(((_, path), _)) => Ok(NamespaceEntry { path: String::from_utf8_lossy(&path.0.0).to_string() }),
                Err(err) => Err(err),
            }).try_collect::<Vec<_>>().await?
        };

        Ok(NamespaceInfoActionResponse { entries })
    }
}

#[derive(serde::Serialize, tabled::Tabled)]
struct NamespaceEntry {
    path: String,
}

#[derive(serde::Serialize)]
pub struct NamespaceInfoActionResponse {
    entries: Vec<NamespaceEntry>,
}

impl Display for NamespaceInfoActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = Table::new(&self.entries);
        write!(f, "{table}")
    }
}

pub struct RegisterNamespaceAction {
    keypair: Keypair,
    path: String,
}

impl Action for RegisterNamespaceAction {
    type Params = (String, String);
    type ResponseData = RegisterNamespaceActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, path): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;
        Ok(Self { keypair, path })
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_testnet() {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .create_namespace_fee(torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(self.path.as_bytes().to_vec()), self.keypair.clone())
                .await?
        } else {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .create_namespace_fee(torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(self.path.as_bytes().to_vec()), self.keypair.clone())
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let (fee, deposit) = if ctx.is_testnet() {
            let client = TorusClient::for_testnet().await?;
            client
                .rpc()
                .namespace_path_creation_cost(self.keypair.account(), self.path.clone())
                .await?
        } else {
            let client = TorusClient::for_mainnet().await?;
            client
                .rpc()
                .namespace_path_creation_cost(self.keypair.account(), self.path.clone())
                .await?
        };

        Ok(Some(Changes {
            changes: vec![
                format!("Register namespace {}", self.path,),
                format!("Charge {} torus for it", deposit + fee),
            ],
            fee: Some(fee),
        }))
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        ctx.info("Registering namespace...");

        if ctx.is_testnet() {
            let client = TorusClient::for_testnet().await?;
            client
            .torus0()
            .calls()
            .create_namespace_wait(torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(self.path.as_bytes().to_vec()), self.keypair.clone())
            .await?;
        } else {
            let client = TorusClient::for_mainnet().await?;
            client
            .torus0()
            .calls()
            .create_namespace_wait(torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(self.path.as_bytes().to_vec()), self.keypair.clone())
            .await?;
        };

        Ok(RegisterNamespaceActionResponse)
    }
}

#[derive(serde::Serialize)]
pub struct RegisterNamespaceActionResponse;

impl Display for RegisterNamespaceActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Namespace registered successfully!")
    }
}

pub struct UnregisterNamespaceAction {
    keypair: Keypair,
    path: String,
}

impl Action for UnregisterNamespaceAction {
    type Params = (String, String);
    type ResponseData = UnregisterNamespaceActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, path): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;
        Ok(Self { keypair, path })
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_testnet() {
            let client = TorusClient::for_testnet().await?;
            client
            .torus0()
            .calls()
            .delete_namespace_fee(torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(self.path.as_bytes().to_vec()), self.keypair.clone())
            .await?
        } else {
            let client = TorusClient::for_mainnet().await?;
            client
            .torus0()
            .calls()
            .delete_namespace_fee(torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(self.path.as_bytes().to_vec()), self.keypair.clone())
            .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;

        Ok(Some(Changes {
            changes: vec![format!("Unregister namespace {}", self.path,)],
            fee: Some(fee),
        }))
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        ctx.info("Unregistering namespace...");

        if ctx.is_testnet() {
            let client = TorusClient::for_testnet().await?;
            client
            .torus0()
            .calls()
            .delete_namespace_wait(torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(self.path.as_bytes().to_vec()), self.keypair.clone())
            .await?;
        } else {
            let client = TorusClient::for_mainnet().await?;
            client
            .torus0()
            .calls()
            .delete_namespace_wait(torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(self.path.as_bytes().to_vec()), self.keypair.clone())
            .await?;
        };

        Ok(UnregisterNamespaceActionResponse)
    }
}

#[derive(serde::Serialize)]
pub struct UnregisterNamespaceActionResponse;

impl Display for UnregisterNamespaceActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Namespace unregistered successfully!")
    }
}
