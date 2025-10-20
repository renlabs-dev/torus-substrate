use std::fmt::Display;

use anyhow::anyhow;
use tabled::Table;
use torus_client::{client::TorusClient, interfaces, subxt::utils::AccountId32};

use crate::{
    action::{Action, ActionContext, Changes},
    keypair::Keypair,
    store::{get_account, get_key},
    util::to_percent_u8,
};

pub struct AgentInfoAction {
    account: AccountId32,
}

impl Action for AgentInfoAction {
    type Params = String;
    type ResponseData = AgentInfoResponse;

    async fn create(_ctx: &mut impl ActionContext, key: Self::Params) -> anyhow::Result<Self> {
        let account = get_account(&key)?;
        Ok(Self { account })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        get_agent_info(ctx, &self.account)
            .await?
            .ok_or(anyhow::anyhow!("Not an agent."))
    }
}

#[derive(serde::Serialize, tabled::Tabled)]
pub struct AgentInfoResponse {
    address: String,
    name: String,
    metadata: String,
    url: String,
    last_update_block: u64,
    registration_block: u64,
}

impl AgentInfoResponse {
    fn new(
        address: String,
        name: Vec<u8>,
        metadata: Vec<u8>,
        url: Vec<u8>,
        last_update_block: u64,
        registration_block: u64,
    ) -> Self {
        Self {
            address,
            name: String::from_utf8_lossy(&name[..]).to_string(),
            metadata: String::from_utf8_lossy(&metadata[..]).to_string(),
            url: String::from_utf8_lossy(&url[..]).to_string(),
            last_update_block,
            registration_block,
        }
    }
}

impl Display for AgentInfoResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = Table::kv(std::iter::once(self));
        write!(f, "{table}")
    }
}

pub struct RegisterAgentAction {
    keypair: Keypair,
    name: String,
    metadata: String,
    url: String,
}

impl Action for RegisterAgentAction {
    type Params = (String, String, String, String);
    type ResponseData = AgentInfoResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, name, metadata, url): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        Ok(Self {
            keypair,
            name,
            metadata,
            url,
        })
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .register_agent_fee(
                    self.name.clone().into(),
                    self.metadata.clone().into(),
                    self.url.clone().into(),
                    self.keypair.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .register_agent_fee(
                    self.name.clone().into(),
                    self.metadata.clone().into(),
                    self.url.clone().into(),
                    self.keypair.clone(),
                )
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;
        Ok(Some(Changes {
            changes: vec![format!(
                "register an agent with name {} which is not changeable later",
                self.name
            )],
            fee: Some(fee),
        }))
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .register_agent_wait(
                    self.name.clone().into(),
                    self.metadata.clone().into(),
                    self.url.clone().into(),
                    self.keypair.clone(),
                )
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .register_agent_wait(
                    self.name.clone().into(),
                    self.metadata.clone().into(),
                    self.url.clone().into(),
                    self.keypair.clone(),
                )
                .await?;
        }

        let account = self.keypair.account();
        get_agent_info(ctx, &account)
            .await?
            .ok_or(anyhow!("Something went wrong!"))
    }
}

pub struct UnregisterAgentAction {
    keypair: Keypair,
}

impl Action for UnregisterAgentAction {
    type Params = String;
    type ResponseData = UnregisterAgentActionResponse;

    async fn create(ctx: &mut impl ActionContext, key: Self::Params) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        Ok(Self { keypair })
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .deregister_agent_fee(self.keypair.clone())
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .deregister_agent_fee(self.keypair.clone())
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;

        Ok(Some(Changes {
            changes: vec![format!(
                "Unregister the agent {}",
                self.keypair.ss58_address()
            )],
            fee: Some(fee),
        }))
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .deregister_agent_wait(self.keypair.clone())
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .deregister_agent_wait(self.keypair.clone())
                .await?;
        }

        Ok(UnregisterAgentActionResponse)
    }
}

#[derive(serde::Serialize)]
pub struct UnregisterAgentActionResponse;

impl Display for UnregisterAgentActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unregistered successfully!")
    }
}

pub struct UpdateAgentAction {
    key: Keypair,
    url: String,
    metadata: Option<String>,
    staking_fee: Option<u32>,
    weight_control_fee: Option<u32>,
}

impl Action for UpdateAgentAction {
    type Params = (String, String, Option<String>, Option<u32>, Option<u32>);
    type ResponseData = UpdateAgentActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, url, metadata, staking_fee, weight_control_fee): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        Ok(Self {
            key: keypair,
            url,
            metadata,
            staking_fee,
            weight_control_fee,
        })
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let staking_fee = self.staking_fee.map(to_percent_u8).transpose()?;
        let weight_control_fee = self.weight_control_fee.map(to_percent_u8).transpose()?;

        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .update_agent_fee(
                    self.url.as_bytes().to_vec(),
                    self.metadata.clone().map(|str| str.as_bytes().to_vec()),
                    staking_fee.map(
                        interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent,
                    ),
                    weight_control_fee.map(
                        interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent,
                    ),
                    self.key.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .update_agent_fee(
                    self.url.as_bytes().to_vec(),
                    self.metadata.clone().map(|str| str.as_bytes().to_vec()),
                    staking_fee.map(
                        interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent,
                    ),
                    weight_control_fee.map(
                        interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent,
                    ),
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
                "Update agent `{}` information",
                self.key.ss58_address()
            )],
            fee: Some(fee),
        }))
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        let staking_fee = self.staking_fee.map(to_percent_u8).transpose()?;
        let weight_control_fee = self.weight_control_fee.map(to_percent_u8).transpose()?;

        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .update_agent_wait(
                    self.url.as_bytes().to_vec(),
                    self.metadata.clone().map(|str| str.as_bytes().to_vec()),
                    staking_fee.map(
                        interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent,
                    ),
                    weight_control_fee.map(
                        interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent,
                    ),
                    self.key.clone(),
                )
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .update_agent_wait(
                    self.url.as_bytes().to_vec(),
                    self.metadata.clone().map(|str| str.as_bytes().to_vec()),
                    staking_fee.map(
                        interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent,
                    ),
                    weight_control_fee.map(
                        interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent,
                    ),
                    self.key.clone(),
                )
                .await?;
        }

        Ok(UpdateAgentActionResponse)
    }
}

#[derive(serde::Serialize)]
pub struct UpdateAgentActionResponse;

impl Display for UpdateAgentActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Updated successfully")
    }
}

async fn get_agent_info(
    ctx: &mut impl ActionContext,
    account: &AccountId32,
) -> anyhow::Result<Option<AgentInfoResponse>> {
    let agent_info = if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .storage()
            .agents_get(account)
            .await?
            .map(|agent| {
                AgentInfoResponse::new(
                    agent.key.to_string(),
                    agent.name.0,
                    agent.metadata.0,
                    agent.url.0,
                    agent.last_update_block,
                    agent.registration_block,
                )
            })
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .storage()
            .agents_get(account)
            .await?
            .map(|agent| {
                AgentInfoResponse::new(
                    agent.key.to_string(),
                    agent.name.0,
                    agent.metadata.0,
                    agent.url.0,
                    agent.last_update_block,
                    agent.registration_block,
                )
            })
    };

    Ok(agent_info)
}
