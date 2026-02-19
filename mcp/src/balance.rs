//! Balance checking tool for the Torus MCP server.
//!
//! Queries the Substrate System pallet's Account storage to get
//! an account's free, reserved, and frozen balances.
//! All amounts are in planck (1 TORUS = 10^18 planck).

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};

use crate::{Client, utils::account_id_from_name_or_ss58};

/// Params for checking an account's balance.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct BalanceCheckRequest {
    /// Dev account name (e.g. "alice") or SS58 address (e.g. "5DoVVg...")
    account_name: String,
}

/// Balance breakdown returned to the MCP client.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct BalanceCheckResponse {
    /// Tokens available for transfer/staking
    free: u128,
    /// Tokens locked by the system (e.g. for identity deposits)
    reserved: u128,
    /// Tokens frozen (e.g. staked tokens that can't be transferred)
    frozen: u128,
}

/// Unused for now — was intended for a faucet tool to mint test tokens.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct FaucetRequest {
    account_name: String,
}

/// Checks the balance of a dev account.
/// Queries the System pallet's Account storage (standard Substrate storage
/// that every chain has — tracks balances, nonces, etc.)
pub async fn check_account_balance(
    torus_client: &Client,
    request: BalanceCheckRequest,
) -> Result<CallToolResult, ErrorData> {
    let account_id = account_id_from_name_or_ss58(request.account_name)?;

    match torus_client
        .system() // System pallet — built into every Substrate chain
        .storage()
        .account_get(&account_id)
        .await
    {
        // Returns Option<AccountInfo> — None means the account doesn't exist on-chain
        Ok(data) => Ok(CallToolResult::success(vec![Content::json(
            data.map(|data| BalanceCheckResponse {
                free: data.data.free,
                reserved: data.data.reserved,
                frozen: data.data.frozen,
            })
            .unwrap_or(BalanceCheckResponse {
                free: 0,
                reserved: 0,
                frozen: 0,
            }),
        )?])),
        Err(err) => Err(ErrorData::internal_error(format!("{err:?}"), None)),
    }
}
