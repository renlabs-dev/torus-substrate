//! Staking tools for the Torus MCP server.
//!
//! These tools let you stake/unstake tokens on agents and query staking positions.
//! Staking is how you "vote with your wallet" — you lock TORUS tokens on an agent
//! to show support. Higher stake = more influence in the network.
//!
//! Signer fields (account_name) must be dev accounts (alice, bob, etc.) — we need the private key.
//! Agent target fields (agent_name, from_agent_name, to_agent_name) accept dev names OR SS58 addresses.
//! The `amount` fields are in planck (1 TORUS = 10^18 planck).

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt::ext::futures::StreamExt;

use crate::{
    Client,
    utils::{account_id_from_name_or_ss58, keypair_from_name, name_or_key},
};

// --- Request structs ---
// Each struct defines the JSON parameters that the MCP tool accepts.
// The `derive` macros auto-generate:
//   - JsonSchema: so MCP clients know what params are expected
//   - Deserialize: to parse incoming JSON into this struct
//   - Serialize: to convert back to JSON if needed

/// Params for staking tokens on an agent.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AddStakeRequest {
    /// Dev account name doing the staking (e.g. "alice") — must be a dev account to sign
    account_name: String,
    /// Agent to stake on — dev account name (e.g. "bob") or SS58 address (e.g. "5DoVVg...")
    agent_name: String,
    /// Amount in planck (smallest unit). 1 TORUS = 10^18 planck.
    amount: u128,
}

/// Params for removing stake from an agent.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct RemoveStakeRequest {
    /// Dev account name — must be a dev account to sign
    account_name: String,
    /// Agent to unstake from — dev account name or SS58 address
    agent_name: String,
    amount: u128,
}

/// Params for moving stake from one agent to another (same staker).
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct TransferStakeRequest {
    /// The account that owns the stake — must be a dev account to sign
    account_name: String,
    /// Agent to take stake from — dev account name or SS58 address
    from_agent_name: String,
    /// Agent to give stake to — dev account name or SS58 address
    to_agent_name: String,
    amount: u128,
}

/// Params for querying a specific staker→agent stake amount.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct GetStakeRequest {
    /// Dev account name or SS58 address of the staker
    staker_name: String,
    /// Dev account name or SS58 address of the agent
    agent_name: String,
}

/// Params for listing all staking positions of one account.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct GetStakesForAccountRequest {
    /// Dev account name or SS58 address
    account_name: String,
}

/// Params for listing all stakers on a given agent (and their total staked amount).
/// Accepts dev account names (e.g. "alice") or SS58 addresses (mainnet agents).
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct GetStakersForAgentRequest {
    /// Dev account name (e.g. "bob") or SS58 address (e.g. "5DoVVg...")
    agent_name: String,
}

// --- Response structs ---
// These define what the tool returns as JSON to the MCP client.

/// A single staking position (who and how much).
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct StakeEntry {
    /// Agent name or SS58 address if not a dev account
    agent: String,
    /// Staked amount in planck
    amount: u128,
}

/// List of all staking positions for an account.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct StakesResponse {
    stakes: Vec<StakeEntry>,
}

/// One staker and their staked amount on a specific agent.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct StakerEntry {
    /// Staker account name or SS58 address
    staker: String,
    /// Staked amount in planck
    amount: u128,
}

/// All stakers on an agent, plus the total sum.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct StakersResponse {
    stakers: Vec<StakerEntry>,
    /// Sum of all individual stakes (in planck)
    total_stake: u128,
}

// --- Handler functions ---
// Each function follows the same pattern:
// 1. Convert account name strings → keypairs using keypair_from_name()
// 2. Call the chain via torus_client.pallet().calls().method_wait(params, signer)
//    The "_wait" suffix means it waits for the transaction to be included in a block.
// 3. Match on Ok/Err and return a CallToolResult

/// Stakes tokens from a staker account onto an agent.
/// This locks the tokens — they can't be transferred while staked.
pub async fn add_stake(
    torus_client: &Client,
    request: AddStakeRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;
    let agent_id = account_id_from_name_or_ss58(&request.agent_name)?;

    match torus_client
        .torus0()
        .calls()
        .add_stake_wait(
            agent_id,
            request.amount, // how much
            keypair,        // who is signing/paying
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully staked {} to agent {}",
            request.amount, request.agent_name
        ))])),
        Err(err) => {
            dbg!(&err); // Print to stderr for debugging
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Removes (unstakes) tokens from an agent back to the staker.
pub async fn remove_stake(
    torus_client: &Client,
    request: RemoveStakeRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;
    let agent_id = account_id_from_name_or_ss58(&request.agent_name)?;

    match torus_client
        .torus0()
        .calls()
        .remove_stake_wait(agent_id, request.amount, keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully removed {} stake from agent {}",
            request.amount, request.agent_name
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Moves stake from one agent to another without unstaking first.
/// The signer (account_name) must be the one who originally staked.
pub async fn transfer_stake(
    torus_client: &Client,
    request: TransferStakeRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;
    let from_id = account_id_from_name_or_ss58(&request.from_agent_name)?;
    let to_id = account_id_from_name_or_ss58(&request.to_agent_name)?;

    match torus_client
        .torus0()
        .calls()
        .transfer_stake_wait(from_id, to_id, request.amount, keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully transferred {} stake from agent {} to agent {}",
            request.amount, request.from_agent_name, request.to_agent_name
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Queries the exact stake amount between a specific staker and agent.
/// This is a read-only storage query — no transaction needed.
pub async fn get_stake(
    torus_client: &Client,
    request: GetStakeRequest,
) -> Result<CallToolResult, ErrorData> {
    let staker_id = account_id_from_name_or_ss58(&request.staker_name)?;
    let agent_id = account_id_from_name_or_ss58(&request.agent_name)?;

    // Storage queries use .storage() instead of .calls()
    // They read on-chain state without submitting a transaction
    match torus_client
        .torus0()
        .storage() // Read-only access to on-chain storage
        .staking_to_get(&staker_id, &agent_id)
        .await
    {
        // Returns Option<u128> — None means no stake exists
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

/// Lists all staking positions for one account across all agents.
/// Uses an iterator to scan the storage map.
pub async fn get_stakes_for_account(
    torus_client: &Client,
    request: GetStakesForAccountRequest,
) -> Result<CallToolResult, ErrorData> {
    let account_id = account_id_from_name_or_ss58(&request.account_name)?;

    // _iter1 means "iterate the second key of a double-map, with the first key fixed"
    // The StakingTo map is: (staker, agent) → amount
    // So iter1(staker) gives us all (staker, agent) pairs for that staker
    let mut stream = match torus_client
        .torus0()
        .storage()
        .staking_to_iter1(&account_id)
        .await
    {
        Ok(stream) => stream,
        Err(err) => {
            dbg!(&err);
            return Err(ErrorData::internal_error(err.to_string(), None));
        }
    };

    let mut stakes = Vec::new();
    while let Some(item) = stream.next().await {
        let ((_staker, agent), amount) = match item {
            Ok(kv) => kv,
            Err(err) => {
                dbg!(&err);
                return Err(ErrorData::internal_error(err.to_string(), None));
            }
        };
        stakes.push(StakeEntry {
            agent: name_or_key(&agent), // Shows "bob" instead of raw address for dev accounts
            amount,
        });
    }

    // Return as JSON using Content::json()
    Ok(CallToolResult::success(vec![Content::json(
        StakesResponse { stakes },
    )?]))
}

/// Lists all stakers on a specific agent and the total stake they hold.
///
/// Uses `StakedBy` storage (indexed by agent → staker → amount) to efficiently
/// fetch all staking positions for a given agent without scanning the full map.
/// Accepts both dev account names and mainnet SS58 addresses.
pub async fn get_stakers_for_agent(
    torus_client: &Client,
    request: GetStakersForAgentRequest,
) -> Result<CallToolResult, ErrorData> {
    let agent_id = account_id_from_name_or_ss58(&request.agent_name)?;

    // staked_by_iter1 iterates the StakedBy double-map with the agent key fixed,
    // yielding all (agent, staker) → amount entries for that agent.
    let mut stream = match torus_client
        .torus0()
        .storage()
        .staked_by_iter1(&agent_id)
        .await
    {
        Ok(stream) => stream,
        Err(err) => {
            dbg!(&err);
            return Err(ErrorData::internal_error(err.to_string(), None));
        }
    };

    let mut stakers = Vec::new();
    let mut total_stake = 0u128;
    while let Some(item) = stream.next().await {
        let ((_, staker), amount) = match item {
            Ok(kv) => kv,
            Err(err) => {
                dbg!(&err);
                return Err(ErrorData::internal_error(err.to_string(), None));
            }
        };
        total_stake = total_stake.saturating_add(amount);
        stakers.push(StakerEntry {
            staker: name_or_key(&staker),
            amount,
        });
    }

    Ok(CallToolResult::success(vec![Content::json(
        StakersResponse {
            stakers,
            total_stake,
        },
    )?]))
}

/// Gets the total stake across the entire network.
pub async fn get_total_stake(torus_client: &Client) -> Result<CallToolResult, ErrorData> {
    match torus_client.torus0().storage().total_stake().await {
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
