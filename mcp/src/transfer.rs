//! Token transfer tool for the Torus MCP server.
//!
//! This sends TORUS tokens from one account to another.
//! Uses `transfer_keep_alive` which prevents the sender's account
//! from being "reaped" (deleted) if their balance drops below the
//! existential deposit (0.1 TORUS). The transaction will fail instead.

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};

// MultiAddress wraps an AccountId for extrinsic parameters.
// The `Id` variant means "use a raw AccountId32" (as opposed to Index, Address20, etc.)
use torus_client::subxt::utils::MultiAddress;

use crate::{
    Client,
    utils::{account_id_from_name_or_ss58, keypair_from_name},
};

/// Params for transferring tokens between two accounts.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct TransferBalanceRequest {
    /// Dev account sending the tokens (e.g. "alice") — must be a dev account to sign
    from_account_name: String,
    /// Dev account receiving the tokens (e.g. "bob") or SS58 address (e.g. "5DoVVg...")
    to_account_name: String,
    /// Amount in planck (1 TORUS = 10^18 planck)
    amount: u128,
}

/// Transfers tokens from one dev account to another account or SS58 address.
/// This submits an extrinsic to the chain and waits for block inclusion.
pub async fn transfer_balance(
    torus_client: &Client,
    request: TransferBalanceRequest,
) -> Result<CallToolResult, ErrorData> {
    let from_keypair = keypair_from_name(&request.from_account_name)?;
    let to_id = account_id_from_name_or_ss58(&request.to_account_name)?;

    // Wrap the destination AccountId in MultiAddress::Id — this is how
    // Substrate's balances pallet expects the destination parameter
    let dest = MultiAddress::Id(to_id);

    match torus_client
        .balances() // Access the balances pallet (standard Substrate pallet)
        .calls()
        .transfer_keep_alive_wait(dest, request.amount, from_keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully transferred {} from {} to {}",
            request.amount, request.from_account_name, request.to_account_name
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}
