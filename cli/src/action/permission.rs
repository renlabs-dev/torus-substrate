use std::{fmt::Display, str::FromStr};

use sp_core::H256;
use torus_client::{client::TorusClient, subxt::utils::AccountId32};

use crate::{
    action::{self, Action, ActionContext, Changes},
    keypair::Keypair,
    store::{get_account, get_key},
};
pub struct RevokePermissionAction {
    key: Keypair,
    permission_id: H256,
}

impl Action for RevokePermissionAction {
    type Params = (String, String);
    type ResponseData = RevokePermissionActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, permission_id): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let permission_id = H256::from_str(&permission_id)?;

        Ok(Self {
            key: keypair,
            permission_id,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .revoke_permission_wait(self.permission_id, self.key.clone())
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .revoke_permission_wait(self.permission_id, self.key.clone())
                .await?;
        }

        Ok(RevokePermissionActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .revoke_permission_fee(self.permission_id, self.key.clone())
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .revoke_permission_fee(self.permission_id, self.key.clone())
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(
        &self,
        ctx: &mut impl ActionContext,
    ) -> anyhow::Result<Option<super::Changes>> {
        let fee = self.estimate_fee(ctx).await?;

        Ok(Some(super::Changes {
            changes: vec![format!(
                "Revoke permission {}",
                self.permission_id.to_string()
            )],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct RevokePermissionActionResponse;

impl Display for RevokePermissionActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permission revoked successfully!")
    }
}

pub struct ExecutePermissionAction {
    pub key: Keypair,
    pub permission_id: H256,
    pub enforce: bool,
}

impl Action for ExecutePermissionAction {
    type Params = (String, String, bool);
    type ResponseData = ExecutePermissionActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, permission_id, enforce): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let permission_id = H256::from_str(&permission_id)?;

        Ok(Self {
            key: keypair,
            permission_id,
            enforce,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            if !self.enforce {
                client
                    .permission0()
                    .calls()
                    .execute_permission_wait(self.permission_id, self.key.clone())
                    .await?;
            } else {
                client
                    .permission0()
                    .calls()
                    .enforcement_execute_permission_wait(self.permission_id, self.key.clone())
                    .await?;
            }
        } else {
            let client = TorusClient::for_testnet().await?;
            if !self.enforce {
                client
                    .permission0()
                    .calls()
                    .execute_permission_wait(self.permission_id, self.key.clone())
                    .await?;
            } else {
                client
                    .permission0()
                    .calls()
                    .enforcement_execute_permission_wait(self.permission_id, self.key.clone())
                    .await?;
            }
        }

        Ok(ExecutePermissionActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            if !self.enforce {
                client
                    .permission0()
                    .calls()
                    .execute_permission_fee(self.permission_id, self.key.clone())
                    .await?
            } else {
                client
                    .permission0()
                    .calls()
                    .enforcement_execute_permission_fee(self.permission_id, self.key.clone())
                    .await?
            }
        } else {
            let client = TorusClient::for_testnet().await?;
            if !self.enforce {
                client
                    .permission0()
                    .calls()
                    .execute_permission_fee(self.permission_id, self.key.clone())
                    .await?
            } else {
                client
                    .permission0()
                    .calls()
                    .enforcement_execute_permission_fee(self.permission_id, self.key.clone())
                    .await?
            }
        };

        Ok(fee)
    }

    async fn get_changes(
        &self,
        ctx: &mut impl ActionContext,
    ) -> anyhow::Result<Option<action::Changes>> {
        let fee = self.estimate_fee(ctx).await?;

        Ok(Some(Changes {
            changes: vec![format!(
                "Execute the permission {}",
                self.permission_id.to_string()
            )],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct ExecutePermissionActionResponse;

impl Display for ExecutePermissionActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permission executed successfully!")
    }
}

pub struct SetPermissionAccumulationAction {
    pub key: Keypair,
    pub permission_id: H256,
    pub accumulating: bool,
}

impl Action for SetPermissionAccumulationAction {
    type Params = (String, String, bool);
    type ResponseData = SetPermissionAccumulationActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, permission_id, accumulating): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let permission_id = H256::from_str(&permission_id)?;

        Ok(Self {
            key: keypair,
            permission_id,
            accumulating,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .toggle_permission_accumulation_wait(
                    self.permission_id,
                    self.accumulating,
                    self.key.clone(),
                )
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .toggle_permission_accumulation_wait(
                    self.permission_id,
                    self.accumulating,
                    self.key.clone(),
                )
                .await?;
        }

        Ok(SetPermissionAccumulationActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .toggle_permission_accumulation_fee(
                    self.permission_id,
                    self.accumulating,
                    self.key.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .toggle_permission_accumulation_fee(
                    self.permission_id,
                    self.accumulating,
                    self.key.clone(),
                )
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;

        Ok(Some(Changes {
            changes: vec![format!(
                "{} accumulating on the permission {}",
                if self.accumulating { "Start" } else { "Stop" },
                self.permission_id.to_string()
            )],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct SetPermissionAccumulationActionResponse;

impl Display for SetPermissionAccumulationActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permission accumulation set successfully!")
    }
}

pub struct SetPermissionEnforcementAuthorityAction {
    pub key: Keypair,
    pub permission_id: H256,
    pub enforcement_authority: Option<(Vec<AccountId32>, u32)>,
}

impl Action for SetPermissionEnforcementAuthorityAction {
    type Params = (String, String, Option<(Vec<String>, u32)>);
    type ResponseData = SetPermissionEnforcementAuthorityActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, permission_id, enforcement_authority): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let permission_id = H256::from_str(&permission_id)?;

        let enforcement_authority = match enforcement_authority {
            Some((vec, votes)) => Some((
                vec.iter()
                    .map(|acc| get_account(acc))
                    .collect::<anyhow::Result<Vec<_>>>()?,
                votes,
            )),
            None => None,
        };

        Ok(Self {
            key: keypair,
            permission_id,
            enforcement_authority,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .set_enforcement_authority_wait(
                    self.permission_id,
                    match &self.enforcement_authority {
                        Some((accounts, votes)) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy { controllers: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()), required_votes: *votes },
                        None => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                    },
                    self.key.clone(),
                )
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .set_enforcement_authority_wait(
                    self.permission_id,
                    match &self.enforcement_authority {
                        Some((accounts, votes)) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy { controllers: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()), required_votes: *votes },
                        None => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                    },
                    self.key.clone(),
                )
                .await?;
        }

        Ok(SetPermissionEnforcementAuthorityActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .set_enforcement_authority_fee(
                    self.permission_id,
                    match &self.enforcement_authority {
                        Some((accounts, votes)) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy { controllers: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()), required_votes: *votes },
                        None => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                    },
                    self.key.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .set_enforcement_authority_fee(
                    self.permission_id,
                    match &self.enforcement_authority {
                        Some((accounts, votes)) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy { controllers: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()), required_votes: *votes },
                        None => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                    },
                    self.key.clone(),
                )
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;

        let changes = match &self.enforcement_authority {
            Some((accounts, votes)) => vec![
                format!(
                    "Add {} accounts as controllers on permission {}",
                    accounts.len(),
                    self.permission_id.to_string()
                ),
                format!("Set the required votes to {}", votes),
            ],
            None => vec![format!(
                "Remove enforcement authority from permission {}",
                self.permission_id.to_string()
            )],
        };

        Ok(Some(Changes {
            changes,
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct SetPermissionEnforcementAuthorityActionResponse;

impl Display for SetPermissionEnforcementAuthorityActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enforce authority set successfully!")
    }
}
