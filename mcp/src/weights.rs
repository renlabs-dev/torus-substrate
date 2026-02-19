//! Validator weight-setting tool for the Torus MCP server.
//!
//! Validators rate miners by assigning "weights" — numbers from 0 to 65535.
//! These weights directly control how emission rewards are distributed:
//! - Higher weight on a miner = that miner gets more incentive tokens
//! - The validator who rated productive miners also gets more dividends
//!
//! Weights are set as a batch — you provide a map of {agent_name: weight}
//! and all weights get updated at once in a single extrinsic.

use std::collections::HashMap;

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt::utils::AccountId32;

use crate::{
    Client,
    utils::{account_id_from_name_or_ss58, keypair_from_name},
};

/// Params for setting weights.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct SetWeightsRequest {
    /// The validator setting weights (must be an allocator with enough stake)
    account_name: String,
    /// Map of agent_name → weight (0–65535).
    /// Example: {"bob": 10000, "charlie": 5000}
    /// Keys can be dev account names or SS58 addresses.
    /// Higher value = more reward allocated to that agent.
    weights_by_account_name: HashMap<String, u16>,
}

/// Sets weights for a validator.
/// Converts the name→weight map into AccountId→weight pairs, then submits.
pub async fn set_weights(
    torus_client: &Client,
    request: SetWeightsRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(request.account_name)?;

    // Convert each agent name to its AccountId32
    // The collect::<Result<Vec<_>, _>>()? pattern collects results and
    // short-circuits on the first error (if any name is invalid)
    let weights = request
        .weights_by_account_name
        .into_iter()
        .map(|(name, weight)| account_id_from_name_or_ss58(&name).map(|id| (id, weight)))
        .collect::<Result<Vec<(AccountId32, u16)>, _>>()?;

    match torus_client
        .emission0() // Weights live in the emission pallet
        .calls()
        .set_weights_wait(weights, keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "set weights successfully",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}
