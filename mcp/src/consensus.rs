use std::collections::HashMap;

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt::ext::futures::StreamExt;

use crate::{Client, utils::name_or_key};

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct ConsensusMembersResponse {
    consensus_members: Vec<ConsensusMember>,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct ConsensusMember {
    name: String,
    weights: HashMap<String, u16>,
    last_incentives: u16,
    last_dividends: u16,
}

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

    while let Some(Ok((id, member))) = stream.next().await {
        members.push(ConsensusMember {
            name: name_or_key(&id),
            weights: member
                .weights
                .0
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
