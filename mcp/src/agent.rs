//! Agent management tools for the Torus MCP server.
//!
//! Agents are the core participants in the Torus network. They can be:
//! - **Miners**: produce off-chain utility via their API (exposed at their URL)
//! - **Validators**: rate miners by setting weights, earning dividends in return
//!
//! This module handles agent lifecycle:
//! - Register/deregister agents
//! - Get agent info
//! - Update agent settings (URL, metadata, fees)
//! - Whitelist/dewhitelist agents (requires curator/admin signer)
//! - Delegate curator permissions (requires admin signer)

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt_signer::sr25519::Keypair;

use crate::{
    Client,
    interfaces::{
        // These types come from the auto-generated chain interfaces.
        // They're created at build time from the chain's metadata.
        permission0::calls::types::delegate_curator_permission::{Duration, Revocation},
        runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap,
        runtime_types::sp_arithmetic::per_things::Percent,
    },
    utils::{account_id_from_name_or_ss58, keypair_from_name},
};

// =====================================================================
// Request/Response types
// =====================================================================

/// Params for getting agent info by name.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentInfoRequest {
    /// Dev account name (e.g. "alice") or SS58 address (e.g. "5DoVVgN7R6vH...")
    account_name: String,
}

/// Agent info returned to MCP client.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentInfoResponse {
    /// The name the agent registered with
    name: String,
    /// Free-form metadata string
    metadata: String,
    /// The URL where the agent's API is accessible
    url: String,
}

/// Params for registering a new agent.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentRegisterRequest {
    /// Dev account to register as an agent
    account_name: String,
    /// Optional metadata (defaults to "test_agent_metadata")
    metadata: Option<String>,
    /// Optional URL (defaults to "test_agent_url")
    url: Option<String>,
}

/// Params for deregistering an agent.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentDeregisterRequest {
    account_name: String,
}

/// Params for adding an agent to the whitelist.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentWhitelistAddRequest {
    /// Dev account name of the agent to whitelist (target)
    account_name: String,
    /// Dev account name of the curator/admin signing this tx (e.g. "alice")
    signer_name: String,
}

/// Params for removing an agent from the whitelist (shares struct with add).
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentWhitelistRemoveRequest {
    account_name: String,
}

/// Params for making an agent a curator (governance role).
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct DelegateCuratorPermisionRequest {
    /// Dev account name of the agent receiving curator permissions (target)
    account_name: String,
    /// Dev account name of the admin signing this tx (e.g. "alice")
    signer_name: String,
}

// =====================================================================
// Handler functions
// =====================================================================

/// Registers a dev account as an agent on the chain.
/// Checks if already registered first to give a better error message.
/// The agent gets a name, URL, and metadata stored on-chain.
pub async fn register_agent(
    torus_client: &Client,
    request: AgentRegisterRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;

    // Pre-check: don't waste gas if already registered
    if is_agent(torus_client, &keypair).await? {
        return Ok(CallToolResult::error(vec![Content::text(
            "The account is already registered as an agent.",
        )]));
    }

    match torus_client
        .torus0()
        .calls()
        .register_agent_wait(
            request.account_name.into_bytes(), // name (stored as bytes on-chain)
            request
                .url
                .unwrap_or_else(|| "test_agent_url".to_string())
                .into_bytes(),
            request
                .metadata
                .unwrap_or_else(|| "test_agent_metadata".to_string())
                .into_bytes(),
            keypair, // signer
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "agent registered (tx submitted)",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Removes an agent from the chain.
/// Checks if the account is actually an agent first.
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
            "agent deregistered (tx submitted)",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

/// Adds an agent to the governance whitelist.
/// The signer must be a curator or admin (e.g. "alice" on devnet).
pub async fn whitelist_agent(
    torus_client: &Client,
    request: AgentWhitelistAddRequest,
) -> Result<CallToolResult, ErrorData> {
    let signer = keypair_from_name(&request.signer_name)?;
    let account_id = account_id_from_name_or_ss58(&request.account_name)?;
    match torus_client
        .governance()
        .calls()
        .add_to_whitelist_wait(account_id, signer)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "successfully added to whitelist (tx submitted)",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

/// Removes an agent from the governance whitelist.
/// The signer must be a curator or admin (e.g. "alice" on devnet).
pub async fn dewhitelist_agent(
    torus_client: &Client,
    request: AgentWhitelistAddRequest,
) -> Result<CallToolResult, ErrorData> {
    let signer = keypair_from_name(&request.signer_name)?;
    let account_id = account_id_from_name_or_ss58(&request.account_name)?;
    match torus_client
        .governance()
        .calls()
        .remove_from_whitelist_wait(account_id, signer)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "successfully removed from whitelist (tx submitted)",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

/// Gets on-chain info for an agent: name, URL, and metadata.
/// Accepts either a dev account name ("alice") or an SS58 address ("5DoVVg...").
/// Returns an error if the account is not registered as an agent.
pub async fn get_agent_info(
    torus_client: &Client,
    request: AgentInfoRequest,
) -> Result<CallToolResult, ErrorData> {
    let account_id = account_id_from_name_or_ss58(request.account_name)?;

    let agent = match torus_client
        .torus0()
        .storage()
        .agents_get(&account_id)
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

    // Agent fields are BoundedVec<u8> on-chain, so we convert to String
    Ok(CallToolResult::success(vec![Content::json(
        AgentInfoResponse {
            name: String::from_utf8_lossy(&agent.name.0).to_string(),
            metadata: String::from_utf8_lossy(&agent.metadata.0).to_string(),
            url: String::from_utf8_lossy(&agent.url.0).to_string(),
        },
    )?]))
}

/// Grants curator permissions to an agent.
/// Curators can manage the whitelist of agents.
/// The signer must be an existing curator or admin (e.g. "alice" on devnet).
pub async fn delegate_curator_permission(
    torus_client: &Client,
    request: DelegateCuratorPermisionRequest,
) -> Result<CallToolResult, ErrorData> {
    let signer = keypair_from_name(&request.signer_name)?;
    let account_id = account_id_from_name_or_ss58(&request.account_name)?;

    match torus_client
        .permission0()
        .calls()
        .delegate_curator_permission_wait(
            account_id,
            BoundedBTreeMap(vec![(None, u32::MAX)]),
            None,
            Duration::Indefinite,
            Revocation::RevocableByDelegator,
            u32::MAX,
            signer,
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "agent {} is now a curator (tx submitted)",
            &request.account_name
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

/// Params for updating an agent's settings.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct UpdateAgentRequest {
    /// The agent account to update
    account_name: String,
    /// New URL for the agent's API endpoint
    url: String,
    /// Optional new metadata (None = keep existing)
    metadata: Option<String>,
    /// Optional new staking fee percentage (0-100, None = keep existing)
    staking_fee: Option<u8>,
    /// Optional new weight control fee percentage (0-100, None = keep existing)
    weight_control_fee: Option<u8>,
}

/// Updates an existing agent's on-chain settings.
/// You can change URL (required), and optionally metadata and fee percentages.
/// The Percent type wraps a u8 (0-100) for the fee fields.
pub async fn update_agent(
    torus_client: &Client,
    request: UpdateAgentRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;

    match torus_client
        .torus0()
        .calls()
        .update_agent_wait(
            request.url.into_bytes(),
            request.metadata.map(|m| m.into_bytes()),
            request.staking_fee.map(Percent), // Wrap u8 in Percent newtype
            request.weight_control_fee.map(Percent),
            keypair,
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "Agent updated successfully (tx submitted)",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Helper: checks if an account is registered as an agent.
/// Used by register/deregister to give better error messages.
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
