use std::collections::HashMap;

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt::utils::AccountId32;

use crate::{Client, utils::keypair_from_name};

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct SetWeightsRequest {
    account_name: String,
    weights_by_account_name: HashMap<String, u16>,
}

pub async fn set_weights(
    torus_client: &Client,
    request: SetWeightsRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(request.account_name)?;
    let weights = request
        .weights_by_account_name
        .into_iter()
        .map(|(name, weight)| match keypair_from_name(name) {
            Ok(keypair) => Ok((keypair.public_key().to_account_id(), weight)),
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<(AccountId32, u16)>, _>>()?;

    match torus_client
        .emission0()
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
