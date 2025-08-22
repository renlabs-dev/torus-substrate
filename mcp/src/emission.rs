use std::str::FromStr;

use crate::interfaces::runtime_types::{
    bounded_collections::bounded_btree_map::BoundedBTreeMap,
    pallet_permission0::permission::{
        EnforcementAuthority, PermissionDuration, RevocationTerms,
        emission::{DistributionControl, EmissionAllocation},
    },
    sp_arithmetic::per_things::Percent,
};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt::utils::H256;

use crate::{Client, utils::keypair_from_name};

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct DelegateEmissionRequest {
    stream_hex: Option<String>,
    agent_name: String,
    target_name: String,
    amount: u8,
    distribution: Distribution,
    duration: Duration,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Distribution {
    Manual,
    Automatic(u128),
    AtBlock(u64),
    Interval(u64),
}

impl Distribution {
    pub fn as_generated_type(&self) -> DistributionControl {
        match self {
            Distribution::Manual => DistributionControl::Manual,
            Distribution::Automatic(v) => DistributionControl::Automatic(*v),
            Distribution::AtBlock(v) => DistributionControl::AtBlock(*v),
            Distribution::Interval(v) => DistributionControl::Interval(*v),
        }
    }
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Duration {
    UntilBlock(u64),
    Indefinite,
}

impl Duration {
    pub fn as_generated_type(&self) -> PermissionDuration {
        match self {
            Duration::UntilBlock(v) => PermissionDuration::UntilBlock(*v),
            Duration::Indefinite => PermissionDuration::Indefinite,
        }
    }
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Revocation {}

pub async fn delegate_emission(
    torus_client: &Client,
    request: DelegateEmissionRequest,
) -> Result<CallToolResult, ErrorData> {
    let source_keypair = keypair_from_name(&request.agent_name)?;
    let target_keypair = keypair_from_name(&request.target_name)?;

    let stream = if let Some(stream_hex) = request.stream_hex {
        H256::from_str(&stream_hex).unwrap()
    } else {
        torus_client
            .rpc()
            .root_namespace_for_account(source_keypair.public_key().to_account_id())
            .await
            .unwrap()
    };

    match torus_client
        .permission0()
        .calls()
        .delegate_emission_permission_wait(
            target_keypair.public_key().to_account_id(),
            EmissionAllocation::Streams(BoundedBTreeMap(vec![(stream, Percent(request.amount))])),
            BoundedBTreeMap(vec![(
                target_keypair.public_key().to_account_id(),
                u16::MAX,
            )]),
            request.distribution.as_generated_type(),
            request.duration.as_generated_type(),
            RevocationTerms::RevocableByDelegator,
            EnforcementAuthority::None,
            source_keypair,
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
