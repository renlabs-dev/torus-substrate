use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt_signer::sr25519::{
    Keypair,
    dev::{self, alice},
};

use crate::{
    Client,
    interfaces::{
        permission0::calls::types::delegate_curator_permission::{Duration, Revocation},
        runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap,
    },
    utils::keypair_from_name,
};

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentInfoRequest {
    account_name: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentInfoResponse {
    name: String,
    metadata: String,
    url: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentRegisterRequest {
    account_name: String,
    metadata: Option<String>,
    url: Option<String>,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentDeregisterRequest {
    account_name: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentWhitelistAddRequest {
    account_name: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentWhitelistRemoveRequest {
    account_name: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct DelegateCuratorPermisionRequest {
    account_name: String,
}

pub async fn register_agent(
    torus_client: &Client,
    request: AgentRegisterRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;

    if is_agent(torus_client, &keypair).await? {
        return Ok(CallToolResult::error(vec![Content::text(
            "The account is already registered as an agent.",
        )]));
    }

    match torus_client
        .torus0()
        .calls()
        .register_agent_wait(
            request.account_name.into_bytes(),
            request
                .url
                .unwrap_or_else(|| "test_agent_url".to_string())
                .into_bytes(),
            request
                .metadata
                .unwrap_or_else(|| "test_agent_metadata".to_string())
                .into_bytes(),
            keypair,
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "agent registered",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

pub async fn deregister_agent(
    torus_client: &Client,
    request: AgentDeregisterRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;

    if !is_agent(torus_client, &keypair).await? {
        return Ok(CallToolResult::error(vec![Content::text(
            "The account is not registered as an agent.",
        )]));
    }

    match torus_client
        .torus0()
        .calls()
        .deregister_agent_wait(keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "agent deregistered",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

pub async fn whitelist_agent(
    torus_client: &Client,
    request: AgentWhitelistAddRequest,
) -> Result<CallToolResult, ErrorData> {
    let key = keypair_from_name(request.account_name)?;
    match torus_client
        .governance()
        .calls()
        .add_to_whitelist_wait(key.public_key().to_account_id(), dev::alice())
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "successfully added to whitelist",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

pub async fn dewhitelist_agent(
    torus_client: &Client,
    request: AgentWhitelistAddRequest,
) -> Result<CallToolResult, ErrorData> {
    let key = keypair_from_name(request.account_name)?;
    match torus_client
        .governance()
        .calls()
        .remove_from_whitelist_wait(key.public_key().to_account_id(), dev::alice())
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "successfully removed from whitelist",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

pub async fn get_agent_info(
    torus_client: &Client,
    request: AgentInfoRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(request.account_name)?;

    let agent = match torus_client
        .torus0()
        .storage()
        .agents_get(&keypair.public_key().to_account_id())
        .await
    {
        Ok(Some(info)) => info,
        Ok(None) => {
            return Err(ErrorData::invalid_request(
                "The provided address is not an agent.",
                None,
            ));
        }
        Err(err) => {
            dbg!(&err);
            return Err(ErrorData::internal_error(err.to_string(), None));
        }
    };

    Ok(CallToolResult::success(vec![Content::json(
        AgentInfoResponse {
            name: String::from_utf8_lossy(&agent.name.0).to_string(),
            metadata: String::from_utf8_lossy(&agent.metadata.0).to_string(),
            url: String::from_utf8_lossy(&agent.url.0).to_string(),
        },
    )?]))
}

pub async fn delegate_curator_permission(
    torus_client: &Client,
    request: DelegateCuratorPermisionRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;

    match torus_client
        .permission0()
        .calls()
        .delegate_curator_permission_wait(
            keypair.public_key().to_account_id(),
            BoundedBTreeMap(vec![(None, u32::MAX)]),
            None,
            Duration::Indefinite,
            Revocation::RevocableByDelegator,
            u32::MAX,
            alice(),
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "agent {} is now a curator",
            &request.account_name
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

async fn is_agent(torus_client: &Client, keypair: &Keypair) -> Result<bool, ErrorData> {
    match torus_client
        .torus0()
        .storage()
        .agents_get(&keypair.public_key().to_account_id())
        .await
    {
        Ok(agent) => Ok(agent.is_some()),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}
