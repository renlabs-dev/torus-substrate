use std::fmt::Display;

use anyhow::anyhow;
use tabled::Table;
use torus_client::{client::TorusClient, subxt::utils::AccountId32};

use crate::{
    action::{Action, ActionContext},
    keypair::Keypair,
    store::{get_account, get_key},
    util::format_torus,
};

pub struct AgentInfoAction {
    account: AccountId32,
}

impl Action for AgentInfoAction {
    type Params = String;
    type ResponseData = AgentInfoResponse;

    async fn create(_ctx: &impl ActionContext, key: Self::Params) -> anyhow::Result<Self> {
        let account = get_account(&key)?;
        Ok(Self { account })
    }

    async fn execute(&self, ctx: &impl ActionContext) -> anyhow::Result<Self::ResponseData> {
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
        ctx: &impl ActionContext,
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

    async fn estimate_fee(&self, ctx: &impl ActionContext) -> anyhow::Result<u128> {
        let fee = if !ctx.is_testnet() {
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

    async fn confirmation_phrase(
        &self,
        ctx: &impl ActionContext,
    ) -> anyhow::Result<Option<String>> {
        let fee = self.estimate_fee(ctx).await?;
        Ok(Some(format!(
            "Are you sure you want to register {} as an agent? {}\n[y/N]",
            self.keypair.ss58_address(),
            if fee != 0 {
                format!("(there will be a {} torus fee)", format_torus(fee))
            } else {
                "".to_string()
            }
        )))
    }

    async fn execute(&self, ctx: &impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if !ctx.is_testnet() {
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

    async fn create(ctx: &impl ActionContext, key: Self::Params) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        Ok(Self { keypair })
    }

    async fn estimate_fee(&self, ctx: &impl ActionContext) -> anyhow::Result<u128> {
        let fee = if !ctx.is_testnet() {
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

    async fn confirmation_phrase(
        &self,
        ctx: &impl ActionContext,
    ) -> anyhow::Result<Option<String>> {
        let fee = self.estimate_fee(ctx).await?;
        Ok(Some(format!(
            "Are you sure you want to unregister {} from agent? {}\n[y/N]",
            self.keypair.ss58_address(),
            if fee != 0 {
                format!("(there will be a {} torus fee)", format_torus(fee))
            } else {
                "".to_string()
            }
        )))
    }

    async fn execute(&self, ctx: &impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if !ctx.is_testnet() {
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

async fn get_agent_info(
    ctx: &impl ActionContext,
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
