//! Emission delegation tool for the Torus MCP server.
//!
//! "Emission delegation" lets an agent share a portion of their emission stream
//! with another agent. This is done via the permission0 pallet's stream permission
//! system — it creates an on-chain permission contract that allocates a percentage
//! of the delegator's emission stream to the recipient.
//!
//! This is one of the more complex tools because it involves:
//! - Looking up the agent's root namespace (via an RPC call)
//! - Constructing a stream permission with allocation, distribution, and duration params
//! - Submitting the permission delegation extrinsic

use std::str::FromStr;

use crate::interfaces::runtime_types::{
    bounded_collections::bounded_btree_map::BoundedBTreeMap,
    pallet_permission0::permission::{
        EnforcementAuthority, PermissionDuration, RevocationTerms,
        stream::{DistributionControl, StreamAllocation},
    },
    sp_arithmetic::per_things::Percent,
};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt::utils::H256; // H256 = 256-bit hash, used for permission/stream IDs

use crate::{
    Client,
    utils::{account_id_from_name_or_ss58, keypair_from_name},
};

/// Params for delegating (or re-delegating) an emission stream.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct DelegateEmissionRequest {
    /// Optional hex hash of the stream to delegate FROM.
    /// If None, uses the agent's root namespace (default stream).
    /// Required when re-delegating (changing an existing delegation).
    stream_hex: Option<String>,
    /// The agent delegating their emission — must be a dev account to sign
    agent_name: String,
    /// The agent receiving the delegated emission — dev account name or SS58 address
    target_name: String,
    /// Percentage of the stream to delegate (0–100)
    amount: u8,
    /// How/when the emission gets distributed
    distribution: Distribution,
    /// How long the delegation lasts
    duration: Duration,
}

/// Controls when/how the delegated emission is distributed.
/// This is a simplified MCP-side enum that mirrors the on-chain DistributionControl type.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Distribution {
    /// Delegatee must manually trigger distribution
    Manual,
    /// Automatically distribute when accumulated amount reaches this threshold
    Automatic(u128),
    /// Distribute at a specific block number
    AtBlock(u64),
    /// Distribute every N blocks
    Interval(u64),
}

impl Distribution {
    /// Converts our MCP-side enum into the auto-generated on-chain type.
    /// We need this because the on-chain types are generated from chain metadata
    /// and aren't directly serializable by schemars (MCP's JSON schema library).
    pub fn as_generated_type(&self) -> DistributionControl {
        match self {
            Distribution::Manual => DistributionControl::Manual,
            Distribution::Automatic(v) => DistributionControl::Automatic(*v),
            Distribution::AtBlock(v) => DistributionControl::AtBlock(*v),
            Distribution::Interval(v) => DistributionControl::Interval(*v),
        }
    }
}

/// How long the emission delegation lasts.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Duration {
    /// Expires at a specific block number
    UntilBlock(u64),
    /// Lasts forever (until explicitly revoked)
    Indefinite,
}

impl Duration {
    /// Converts to the auto-generated on-chain type.
    pub fn as_generated_type(&self) -> PermissionDuration {
        match self {
            Duration::UntilBlock(v) => PermissionDuration::UntilBlock(*v),
            Duration::Indefinite => PermissionDuration::Indefinite,
        }
    }
}

/// Scaffolded for future use — revocation term options for emission delegation.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Revocation {}

/// Delegates (or re-delegates) an emission stream to another agent.
///
/// If `stream_hex` is not provided, it automatically looks up the agent's
/// root namespace hash via an RPC call. This is the default stream that
/// every agent gets when they register.
pub async fn delegate_emission(
    torus_client: &Client,
    request: DelegateEmissionRequest,
) -> Result<CallToolResult, ErrorData> {
    let source_keypair = keypair_from_name(&request.agent_name)?;
    let target_id = account_id_from_name_or_ss58(&request.target_name)?;

    // Either use the provided stream hash, or look up the agent's root namespace
    let stream = if let Some(stream_hex) = request.stream_hex {
        H256::from_str(&stream_hex)
            .map_err(|e| ErrorData::invalid_request(format!("Invalid stream hex: {e}"), None))?
    } else {
        // RPC call to get the root namespace hash for this account
        torus_client
            .rpc()
            .root_namespace_for_account(source_keypair.public_key().to_account_id())
            .await
            .map_err(|e| {
                ErrorData::internal_error(format!("Failed to get root namespace: {e}"), None)
            })?
    };

    // Submit the stream permission delegation extrinsic
    match torus_client
        .permission0()
        .calls()
        .delegate_stream_permission_wait(
            // Recipients map: target gets max share (u16::MAX = 100%)
            BoundedBTreeMap(vec![(target_id, u16::MAX)]),
            // Allocate a percentage of the specified stream
            StreamAllocation::Streams(BoundedBTreeMap(vec![(stream, Percent(request.amount))])),
            request.distribution.as_generated_type(),
            request.duration.as_generated_type(),
            RevocationTerms::RevocableByDelegator, // Delegator can revoke anytime
            EnforcementAuthority::None,            // No third-party enforcer
            None,                                  // No accumulation threshold
            None,                                  // No additional metadata
            source_keypair,                        // Signer
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "Successfully delegated",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}
