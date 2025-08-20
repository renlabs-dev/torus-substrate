use std::collections::HashMap;

use crate::{
    emission::Distribution,
    interfaces::runtime_types::{
        bounded_collections::{
            bounded_btree_map::BoundedBTreeMap, bounded_btree_set::BoundedBTreeSet,
            bounded_vec::BoundedVec,
        },
        pallet_permission0::permission::{
            PermissionDuration, PermissionScope, RevocationTerms,
            emission::{DistributionControl, EmissionAllocation},
        },
    },
};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt::ext::futures::StreamExt;

use crate::{
    Client,
    utils::{keypair_from_name, name_or_key},
};

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct NamespaceCreationRequest {
    agent_name: String,
    namespace_path: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct NamespaceDeletionRequest {
    agent_name: String,
    namespace_path: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct NamespaceDelegationRequest {
    from_agent: String,
    to_agent: String,
    namespace_path: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct PermissionSummaryRequest {
    account_name: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct PermissionSummaryResponse {
    permissions: Vec<Permission>,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Permission {
    Namespace((NamespacePermission, Direction)),
    Curator((CuratorPermission, Direction)),
    Emission((EmissionPermission, Direction)),
}

#[derive(Clone, schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Direction {
    DelegatingTo(Vec<String>),
    DelegatedFrom(String),
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct NamespacePermission {
    path: String,
    parent: Option<String>,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct CuratorPermission {}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct EmissionPermission {
    allocation: Allocation,
    distribution: Distribution,
    targets: HashMap<String, u16>,
    accumulating: bool,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Allocation {
    Streams(HashMap<String, u8>),
    FixedAmount(u128),
}

pub async fn create_namespace_for_agent(
    torus_client: &Client,
    request: NamespaceCreationRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.agent_name)?;

    match torus_client
        .torus0()
        .calls()
        .create_namespace_wait(
            BoundedVec(request.namespace_path.as_bytes().to_vec()),
            keypair,
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "namespace created",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

pub async fn delete_namespace_for_agent(
    torus_client: &Client,
    request: NamespaceDeletionRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.agent_name)?;

    match torus_client
        .torus0()
        .calls()
        .delete_namespace_wait(
            BoundedVec(request.namespace_path.as_bytes().to_vec()),
            keypair,
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "namespace deleted",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

pub async fn delegate_namespace_permission_for_agent(
    torus_client: &Client,
    request: NamespaceDelegationRequest,
) -> Result<CallToolResult, ErrorData> {
    let from_keypair = keypair_from_name(&request.from_agent)?;
    let to_account_id = keypair_from_name(&request.to_agent)?
        .public_key()
        .to_account_id();

    match torus_client
        .permission0()
        .calls()
        .delegate_namespace_permission_wait(
            to_account_id,
            BoundedBTreeMap(vec![(
                None,
                BoundedBTreeSet(vec![BoundedVec(request.namespace_path.as_bytes().to_vec())]),
            )]),
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            1,
            from_keypair,
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "namespace deleted",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

pub async fn get_permission_summary_for_agent(
    torus_client: &Client,
    request: PermissionSummaryRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(request.account_name)?;
    let account_id = keypair.public_key().to_account_id();

    let mut iter = match torus_client
        .permission0()
        .storage()
        .permissions_iter()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            dbg!(&err);
            return Err(ErrorData::invalid_request(err.to_string(), None));
        }
    };

    let mut permissions = vec![];

    while let Some(ele) = iter.next().await {
        let (_hash, contract) = match ele {
            Ok(res) => res,
            Err(err) => {
                dbg!(&err);
                return Err(ErrorData::invalid_request(err.to_string(), None));
            }
        };

        match contract.scope {
            PermissionScope::Emission(emission) => {
                let direction = if contract.delegator == account_id {
                    let mut recipients = vec![name_or_key(&contract.recipient)];
                    recipients.append(
                        &mut emission
                            .targets
                            .0
                            .iter()
                            .map(|(target, _)| name_or_key(target))
                            .collect::<Vec<String>>(),
                    );
                    Direction::DelegatingTo(recipients)
                } else {
                    Direction::DelegatedFrom(name_or_key(&contract.delegator))
                };

                let allocation = match emission.allocation {
                    EmissionAllocation::Streams(bounded_btree_map) => Allocation::Streams(
                        bounded_btree_map
                            .0
                            .iter()
                            .map(|(stream, percent)| (stream.to_string(), percent.0))
                            .collect::<HashMap<_, _>>(),
                    ),
                    EmissionAllocation::FixedAmount(amount) => Allocation::FixedAmount(amount),
                };

                let distribution = match emission.distribution {
                    DistributionControl::Manual => Distribution::Manual,
                    DistributionControl::Automatic(value) => Distribution::Automatic(value),
                    DistributionControl::AtBlock(value) => Distribution::AtBlock(value),
                    DistributionControl::Interval(value) => Distribution::Interval(value),
                };

                let permission = EmissionPermission {
                    allocation,
                    distribution,
                    targets: emission
                        .targets
                        .0
                        .iter()
                        .map(|(account, amount)| (name_or_key(account), *amount))
                        .collect(),
                    accumulating: emission.accumulating,
                };

                permissions.push(Permission::Emission((permission, direction)));
            }
            PermissionScope::Curator(_) => {
                let direction = if contract.delegator == account_id {
                    Direction::DelegatingTo(vec![name_or_key(&contract.recipient)])
                } else {
                    Direction::DelegatedFrom(name_or_key(&contract.delegator))
                };

                permissions.push(Permission::Curator((CuratorPermission {}, direction)));
            }
            PermissionScope::Namespace(namespace) => {
                let direction = if contract.delegator == account_id {
                    Direction::DelegatingTo(vec![name_or_key(&contract.recipient)])
                } else {
                    Direction::DelegatedFrom(name_or_key(&contract.delegator))
                };

                for (parent, path) in namespace.paths.0 {
                    for path in path.0 {
                        let permission = NamespacePermission {
                            path: String::from_utf8_lossy(&path.0.0[..]).to_string(),
                            parent: parent.map(|hash| hash.to_string()),
                        };

                        permissions.push(Permission::Namespace((permission, direction.clone())));
                    }
                }
            }
        }
    }

    Ok(CallToolResult::success(vec![
        Content::json(PermissionSummaryResponse { permissions }).unwrap(),
    ]))
}
