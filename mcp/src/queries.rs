//! Read-only query tools for the Torus MCP server.
//!
//! These tools fetch on-chain state without submitting any transactions.
//! They're safe to call anytime — they don't cost gas or modify anything.
//! Think of them as "SELECT" queries against the blockchain's database.

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt::ext::futures::StreamExt;

use crate::{
    Client,
    utils::{account_id_from_name_or_ss58, name_or_key},
};

// --- Response types ---

/// One entry in the agents list.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentListEntry {
    /// Human-readable name the agent registered with
    name: String,
    /// Account address (dev name or SS58 address)
    account: String,
    /// API URL the agent exposes
    url: String,
    /// Free-form metadata string
    metadata: String,
}

/// Wrapper for the list_agents response.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AgentListResponse {
    agents: Vec<AgentListEntry>,
}

// --- Request types (only needed for tools that take parameters) ---

/// Params for checking if an account is whitelisted.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct WhitelistCheckRequest {
    /// Dev account name (e.g. "alice") or SS58 address (e.g. "5DoVVg...")
    account_name: String,
}

// --- Handler functions ---

/// Lists all registered agents on the chain.
/// Iterates the Agents storage map and collects them into a JSON array.
pub async fn list_agents(torus_client: &Client) -> Result<CallToolResult, ErrorData> {
    // agents_iter() returns an async stream of (AccountId, AgentData) pairs
    let mut stream = match torus_client.torus0().storage().agents_iter().await {
        Ok(stream) => stream,
        Err(err) => {
            dbg!(&err);
            return Err(ErrorData::internal_error(err.to_string(), None));
        }
    };

    let mut agents = Vec::new();
    while let Some(item) = stream.next().await {
        let (id, agent) = match item {
            Ok(kv) => kv,
            Err(err) => {
                dbg!(&err);
                return Err(ErrorData::internal_error(err.to_string(), None));
            }
        };
        agents.push(AgentListEntry {
            // Agent data is stored as raw bytes on-chain (BoundedVec<u8>)
            // .0 accesses the inner Vec<u8>, then we convert to a String
            name: String::from_utf8_lossy(&agent.name.0).to_string(),
            account: name_or_key(&id),
            url: String::from_utf8_lossy(&agent.url.0).to_string(),
            metadata: String::from_utf8_lossy(&agent.metadata.0).to_string(),
        });
    }

    Ok(CallToolResult::success(vec![Content::json(
        AgentListResponse { agents },
    )?]))
}

/// Gets the current burn amount — the cost (in planck) to register a new agent.
/// This value adjusts dynamically based on registration activity.
pub async fn get_burn_amount(torus_client: &Client) -> Result<CallToolResult, ErrorData> {
    match torus_client.torus0().storage().burn().await {
        // Returns Option<u128> — None means no burn is set (shouldn't happen normally)
        Ok(amount) => Ok(CallToolResult::success(vec![Content::text(format!(
            "{}",
            amount.unwrap_or(0)
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}

/// Gets the full burn configuration (target registrations, adjust factor, etc.)
/// Returns a debug-formatted string of the config struct.
pub async fn get_burn_config(torus_client: &Client) -> Result<CallToolResult, ErrorData> {
    match torus_client.torus0().storage().burn_config().await {
        Ok(Some(config)) => Ok(CallToolResult::success(vec![Content::text(format!(
            // {:?} is Rust's Debug format — prints the struct with field names
            "{config:?}"
        ))])),
        Ok(None) => Ok(CallToolResult::success(vec![Content::text(
            "No burn config found",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}

/// Gets the amount of tokens waiting to be distributed in the next emission epoch.
/// Emissions accumulate every block and get distributed at regular intervals.
pub async fn get_pending_emission(torus_client: &Client) -> Result<CallToolResult, ErrorData> {
    match torus_client.emission0().storage().pending_emission().await {
        Ok(amount) => Ok(CallToolResult::success(vec![Content::text(format!(
            "{}",
            amount.unwrap_or(0)
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}

/// Gets the incentives ratio — how emissions are split between miners and validators.
/// Returns a percentage (e.g. "50%" means 50% to miners, 50% to validators).
pub async fn get_incentives_ratio(torus_client: &Client) -> Result<CallToolResult, ErrorData> {
    match torus_client.emission0().storage().incentives_ratio().await {
        Ok(Some(ratio)) => Ok(CallToolResult::success(vec![Content::text(format!(
            // ratio is a Percent struct, .0 gets the inner u8 value
            "{}%",
            ratio.0
        ))])),
        Ok(None) => Ok(CallToolResult::success(vec![Content::text(
            "No incentives ratio set",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}

/// Gets what percentage of emissions get recycled back into the system.
pub async fn get_emission_recycling_percentage(
    torus_client: &Client,
) -> Result<CallToolResult, ErrorData> {
    match torus_client
        .emission0()
        .storage()
        .emission_recycling_percentage()
        .await
    {
        Ok(Some(pct)) => Ok(CallToolResult::success(vec![Content::text(format!(
            "{}%",
            pct.0
        ))])),
        Ok(None) => Ok(CallToolResult::success(vec![Content::text(
            "No emission recycling percentage set",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}

/// Checks if a specific account is on the governance whitelist.
/// Whitelisted accounts have special privileges (e.g. can skip application process).
pub async fn check_whitelist_status(
    torus_client: &Client,
    request: WhitelistCheckRequest,
) -> Result<CallToolResult, ErrorData> {
    let account_id = account_id_from_name_or_ss58(&request.account_name)?;

    match torus_client
        .governance()
        .storage()
        .whitelist_get(&account_id)
        .await
    {
        Ok(result) => {
            // whitelist_get returns Option — Some means whitelisted, None means not
            let is_whitelisted = result.is_some();
            Ok(CallToolResult::success(vec![Content::text(format!(
                "{is_whitelisted}"
            ))]))
        }
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}

/// Gets the global governance configuration (vote thresholds, proposal costs, etc.)
/// Returns a debug-formatted string of the config struct.
pub async fn get_global_governance_config(
    torus_client: &Client,
) -> Result<CallToolResult, ErrorData> {
    match torus_client
        .governance()
        .storage()
        .global_governance_config()
        .await
    {
        Ok(Some(config)) => Ok(CallToolResult::success(vec![Content::text(format!(
            "{config:?}"
        ))])),
        Ok(None) => Ok(CallToolResult::success(vec![Content::text(
            "No global governance config found",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}
