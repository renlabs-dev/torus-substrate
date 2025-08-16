use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};

use crate::{Client, utils::keypair_from_name};

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct BalanceCheckRequest {
    account_name: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct BalanceCheckResponse {
    free: u128,
    reserved: u128,
    frozen: u128,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct FaucetRequest {
    account_name: String,
}

pub async fn check_account_balance(
    torus_client: &Client,
    request: BalanceCheckRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(request.account_name)?;

    match torus_client
        .system()
        .storage()
        .account_get(&keypair.public_key().to_account_id())
        .await
    {
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
