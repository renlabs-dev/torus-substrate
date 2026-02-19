//! Weight control delegation tools for the Torus MCP server.
//!
//! "Weights" are how validators rate miners (0–65535 per miner).
//! These ratings directly control how emission rewards get distributed.
//!
//! Weight control delegation lets a validator say "hey, I trust you to
//! set weights on my behalf." The delegator pays a fee (weight_control_fee)
//! to the delegatee for this service.
//!
//! This is useful when a validator wants to participate in staking rewards
//! but doesn't want to actively assess miners themselves.

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};

use crate::{
    Client,
    utils::{account_id_from_name_or_ss58, keypair_from_name, name_or_key},
};

/// Params for delegating weight control to another validator.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct DelegateWeightControlRequest {
    /// The validator delegating their weight control (must be a dev account — signs the tx)
    account_name: String,
    /// The validator receiving the delegation — dev account name or SS58 address
    target_name: String,
}

/// Params for taking back weight control from a delegation.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct RegainWeightControlRequest {
    account_name: String,
}

/// Params for checking who an account has delegated weight control to.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct GetWeightControlDelegationRequest {
    /// Dev account name (e.g. "alice") or SS58 address (e.g. "5DoVVg...")
    account_name: String,
}

/// Delegates weight control to another validator.
/// After this, the target validator's weights will be used for this account
/// during emission distribution, and this account pays them a fee.
pub async fn delegate_weight_control(
    torus_client: &Client,
    request: DelegateWeightControlRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;
    let target_id = account_id_from_name_or_ss58(&request.target_name)?;

    match torus_client
        .emission0() // Weight control lives in the emission pallet
        .calls()
        .delegate_weight_control_wait(target_id, keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "Weight control delegated to {}",
            request.target_name
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Takes back weight control after a previous delegation.
/// After this, the account must set their own weights again.
pub async fn regain_weight_control(
    torus_client: &Client,
    request: RegainWeightControlRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;

    match torus_client
        .emission0()
        .calls()
        .regain_weight_control_wait(keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "Weight control regained",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Checks if an account has delegated weight control and to whom.
/// Returns the delegatee's name/address, or "Not delegating" if none.
pub async fn get_weight_control_delegation(
    torus_client: &Client,
    request: GetWeightControlDelegationRequest,
) -> Result<CallToolResult, ErrorData> {
    let account_id = account_id_from_name_or_ss58(&request.account_name)?;

    match torus_client
        .emission0()
        .storage()
        .weight_control_delegation_get(&account_id)
        .await
    {
        Ok(Some(delegatee)) => Ok(CallToolResult::success(vec![Content::text(format!(
            "Delegated to: {}",
            name_or_key(&delegatee)
        ))])),
        Ok(None) => Ok(CallToolResult::success(vec![Content::text(
            "Not delegating weight control",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}
