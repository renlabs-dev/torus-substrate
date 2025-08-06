use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::{
    interfaces::testnet::api::runtime_types::{
        bounded_collections::{
            bounded_btree_map::BoundedBTreeMap, bounded_btree_set::BoundedBTreeSet,
            bounded_vec::BoundedVec,
        },
        pallet_permission0::permission::{PermissionDuration, PermissionScope, RevocationTerms},
    },
    subxt::{ext::futures::StreamExt, utils::to_hex},
};

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
pub struct NamespaceSummaryRequest {
    account_name: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct NamespaceSummaryResponse {
    namespaces: Vec<Namespace>,
}

#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Namespace {
    Delegating {
        to: String,
        path: String,
        parent: Option<String>,
    },
    Delegated {
        by: String,
        path: String,
        parent: Option<String>,
    },
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
        Ok(res) => Ok(CallToolResult::success(vec![Content::text(format!(
            "namespace created in block {}",
            to_hex(res)
        ))])),
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
        Ok(res) => Ok(CallToolResult::success(vec![Content::text(format!(
            "namespace deleted in block {}",
            to_hex(res)
        ))])),
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
        Ok(res) => Ok(CallToolResult::success(vec![Content::text(format!(
            "namespace deleted in block {}",
            to_hex(res)
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

pub async fn get_namespace_summary_for_agent(
    torus_client: &Client,
    request: NamespaceSummaryRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(request.account_name)?;

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

    let mut namespaces = vec![];

    while let Some(ele) = iter.next().await {
        let (_hash, contract) = match ele {
            Ok(res) => res,
            Err(err) => {
                dbg!(&err);
                return Err(ErrorData::invalid_request(err.to_string(), None));
            }
        };

        let PermissionScope::Namespace(namespace) = contract.scope else {
            continue;
        };

        if contract.delegator == keypair.public_key().to_account_id() {
            for (parent, path) in namespace.paths.0 {
                for path in path.0 {
                    namespaces.push(Namespace::Delegating {
                        to: name_or_key(&contract.recipient),
                        path: String::from_utf8_lossy(&path.0.0[..]).to_string(),
                        parent: parent.map(|hash| hash.to_string()),
                    });
                }
            }
        } else {
            for (parent, path) in namespace.paths.0 {
                for path in path.0 {
                    namespaces.push(Namespace::Delegated {
                        by: name_or_key(&contract.delegator),
                        path: String::from_utf8_lossy(&path.0.0[..]).to_string(),
                        parent: parent.map(|hash| hash.to_string()),
                    });
                }
            }
        }
    }

    Ok(CallToolResult::success(vec![
        Content::json(NamespaceSummaryResponse { namespaces }).unwrap(),
    ]))
}
