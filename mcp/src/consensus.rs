//! Consensus member listing tool for the Torus MCP server.
//!
//! Consensus members are the agents participating in the current emission epoch.
//! Each member has:
//! - **weights**: ratings they've assigned to other agents (0–65535)
//! - **last_incentives**: their incentive score from the previous epoch
//! - **last_dividends**: their dividend score from the previous epoch
//!
//! This data drives the emission distribution algorithm (see docs/linear-emission.md).

use std::collections::HashMap;

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt::ext::futures::StreamExt;

use crate::{Client, utils::name_or_key};

/// Wrapper for the list response.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct ConsensusMembersResponse {
    consensus_members: Vec<ConsensusMember>,
}

/// A single consensus member's data.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct ConsensusMember {
    /// Account name or SS58 address
    name: String,
    /// Map of agent_name → weight (0–65535). Higher weight = more reward for that agent.
    weights: HashMap<String, u16>,
    /// Incentive score from last emission round (u16, used for pruning decisions)
    last_incentives: u16,
    /// Dividend score from last emission round (u16, used for pruning decisions)
    last_dividends: u16,
}

/// Lists all current consensus members from the emission0 pallet.
/// This iterates the ConsensusMembers storage map in the emission pallet.
pub async fn list_consensus_members(torus_client: &Client) -> Result<CallToolResult, ErrorData> {
    let mut stream = match torus_client
        .emission0()
        .storage()
        .consensus_members_iter()
        .await
    {
        Ok(stream) => stream,
        Err(err) => {
            dbg!(&err);
            return Err(ErrorData::internal_error(err.to_string(), None));
        }
    };

    let mut members = Vec::new();

    while let Some(item) = stream.next().await {
        let (id, member) = match item {
            Ok(kv) => kv,
            Err(err) => {
                dbg!(&err);
                return Err(ErrorData::internal_error(err.to_string(), None));
            }
        };
        members.push(ConsensusMember {
            name: name_or_key(&id),
            // Convert the on-chain BoundedBTreeMap of (AccountId → u16)
            // into a HashMap of (name_or_address → u16) for readability
            weights: member
                .weights
                .0 // .0 unwraps BoundedBTreeMap to get inner Vec
                .into_iter()
                .map(|(id, weight)| (name_or_key(&id), weight))
                .collect(),
            last_incentives: member.last_incentives,
            last_dividends: member.last_dividends,
        });
    }

    Ok(CallToolResult::success(vec![Content::json(
        ConsensusMembersResponse {
            consensus_members: members,
        },
    )?]))
}
