//! Namespace and permission tools for the Torus MCP server.
//!
//! Namespaces are hierarchical paths (dot-separated) that agents own on-chain.
//! They're used for organizing emission streams and permissions.
//!
//! **Path format**: All agent-owned paths have the form `agent.{agent_name}.{suffix}`.
//! The tools here accept just the suffix (e.g. `"memory"` or `"tools.search"`) and
//! automatically prepend `agent.{agent_name}.` — so you never need to write the prefix.
//!
//! Valid characters per segment: lowercase ASCII letters, digits, `-`, `_`.
//! Max segment length: 63 characters. Max total path length: 256 characters.
//!
//! The permission system is the most complex part of Torus — it supports:
//! - **Namespace permissions**: delegate control over namespace paths
//! - **Curator permissions**: delegate whitelist management
//! - **Stream permissions**: delegate emission stream allocation (see emission.rs)
//! - **Wallet permissions**: delegate staking control
//!
//! Each permission is a "contract" with a delegator, recipient, scope, duration,
//! and revocation terms. Permissions are identified by H256 hashes.

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
            stream::{DistributionControl, StreamAllocation},
            wallet::WalletScopeType,
        },
    },
};
use std::str::FromStr;

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt::ext::futures::StreamExt;
use torus_client::subxt::utils::H256; // 256-bit hash used as permission IDs

use crate::{
    Client,
    utils::{account_id_from_name_or_ss58, keypair_from_name, name_or_key},
};

// =====================================================================
// Request types
// =====================================================================

/// Params for creating a new namespace.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct NamespaceCreationRequest {
    /// The agent that will own this namespace
    agent_name: String,
    /// The namespace path suffix (e.g. "memory" or "tools.search").
    /// The full on-chain path will be "agent.{agent_name}.{namespace_path}".
    /// Valid characters: lowercase letters, digits, hyphens, underscores. No uppercase or slashes.
    namespace_path: String,
}

/// Params for deleting a namespace.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct NamespaceDeletionRequest {
    agent_name: String,
    /// The namespace path suffix to delete (e.g. "memory").
    /// The full on-chain path will be "agent.{agent_name}.{namespace_path}".
    namespace_path: String,
}

/// Params for delegating namespace access to another agent.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct NamespaceDelegationRequest {
    /// Agent delegating the permission (must own the namespace)
    from_agent: String,
    /// Agent receiving the permission
    to_agent: String,
    /// The namespace path suffix to delegate (e.g. "memory").
    /// The full on-chain path will be "agent.{from_agent}.{namespace_path}".
    namespace_path: String,
}

/// Params for getting all permissions affecting an account.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct PermissionSummaryRequest {
    /// Dev account name (e.g. "alice") or SS58 address (e.g. "5DoVVg...")
    pub account_name: String,
}

// =====================================================================
// Response types — simplified MCP-friendly versions of on-chain data
// =====================================================================

/// A single permission entry with its on-chain ID.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct PermissionEntry {
    /// Hex-encoded H256 permission ID — pass this to revoke_permission or toggle_permission_accumulation
    pub id: String,
    /// The permission details
    pub detail: Permission,
}

/// Summary of all permissions related to an account.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct PermissionSummaryResponse {
    /// Total number of permissions found for this account
    total: usize,
    /// Permissions returned (capped at 50)
    permissions: Vec<PermissionEntry>,
}

/// A single permission — can be one of 4 types, each with a direction.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Permission {
    Namespace((NamespacePermission, Direction)),
    Curator((CuratorPermission, Direction)),
    Stream((StreamPermission, Direction)),
    Wallet((WalletPermission, Direction)),
}

/// Whether this permission is one we gave out or one we received.
#[derive(Clone, schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Direction {
    /// We delegated this permission TO these accounts
    DelegatingTo(Vec<String>),
    /// We received this permission FROM this account
    DelegatedFrom(String),
}

/// Namespace permission details.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct NamespacePermission {
    /// The full on-chain namespace path (e.g. "agent.alice.memory")
    path: String,
    /// Parent namespace hash, if this is a sub-namespace
    parent: Option<String>,
}

/// Curator permission (no extra fields — either you have it or you don't).
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct CuratorPermission {}

/// Stream (emission) permission details.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct StreamPermission {
    /// How much of the stream is allocated
    allocation: Allocation,
    /// When/how it gets distributed
    distribution: Distribution,
    /// Map of recipient_name → share weight
    recipients: HashMap<String, u16>,
    /// Whether this permission accumulates emissions over time
    accumulating: bool,
}

/// How the stream allocation is defined.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum Allocation {
    /// Percentage-based allocation from named streams
    Streams(HashMap<String, u8>),
    /// Fixed token amount
    FixedAmount(u128),
}

/// Wallet permission details.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct WalletPermission {
    /// What kind of wallet access is granted
    r#type: WalletPermissionType,
}

/// Types of wallet permissions.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum WalletPermissionType {
    Stake {
        /// Whether the recipient can transfer stake to other agents
        can_transfer_stake: bool,
        /// Whether only the recipient can stake (exclusive access)
        exclusive_stake_access: bool,
    },
}

// =====================================================================
// Handler functions
// =====================================================================

/// Creates a new namespace owned by the specified agent.
/// Constructs the full on-chain path as "agent.{agent_name}.{namespace_path}".
pub async fn create_namespace_for_agent(
    torus_client: &Client,
    request: NamespaceCreationRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.agent_name)?;
    let full_path = format!("agent.{}.{}", request.agent_name, request.namespace_path);

    match torus_client
        .torus0()
        .calls()
        .create_namespace_wait(BoundedVec(full_path.as_bytes().to_vec()), keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "namespace created: {full_path}"
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

/// Deletes a namespace owned by the specified agent.
/// Constructs the full on-chain path as "agent.{agent_name}.{namespace_path}".
pub async fn delete_namespace_for_agent(
    torus_client: &Client,
    request: NamespaceDeletionRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.agent_name)?;
    let full_path = format!("agent.{}.{}", request.agent_name, request.namespace_path);

    match torus_client
        .torus0()
        .calls()
        .delete_namespace_wait(BoundedVec(full_path.as_bytes().to_vec()), keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "namespace deleted: {full_path}"
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

/// Delegates namespace permission from one agent to another.
/// This creates an on-chain permission contract that lets the recipient
/// operate within the specified namespace path.
/// Constructs the full on-chain path as "agent.{from_agent}.{namespace_path}".
pub async fn delegate_namespace_permission_for_agent(
    torus_client: &Client,
    request: NamespaceDelegationRequest,
) -> Result<CallToolResult, ErrorData> {
    let from_keypair = keypair_from_name(&request.from_agent)?;
    let to_account_id = account_id_from_name_or_ss58(&request.to_agent)?;
    let full_path = format!("agent.{}.{}", request.from_agent, request.namespace_path);

    match torus_client
        .permission0()
        .calls()
        .delegate_namespace_permission_wait(
            to_account_id,
            // Nested bounded collections: Map<Option<parent_hash>, Set<paths>>
            BoundedBTreeMap(vec![(
                None, // No parent namespace (top-level)
                BoundedBTreeSet(vec![BoundedVec(full_path.as_bytes().to_vec())]),
            )]),
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            1, // Max usage count
            from_keypair,
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "namespace permission delegated: {full_path}"
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(err.to_string(), None))
        }
    }
}

/// Gets a summary of ALL permissions affecting an account.
/// Iterates every permission on-chain and filters for ones where
/// this account is either the delegator or a recipient.
///
/// This is the most complex read tool — it matches on all 4 permission
/// scope types (Stream, Curator, Namespace, Wallet) and converts each
/// into a simplified MCP-friendly representation.
pub async fn get_permission_summary_for_agent(
    torus_client: &Client,
    request: PermissionSummaryRequest,
) -> Result<CallToolResult, ErrorData> {
    let account_id = account_id_from_name_or_ss58(request.account_name)?;

    // Iterate ALL permissions on-chain (could be slow on a busy chain)
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
        let (hash, contract) = match ele {
            Ok(res) => res,
            Err(err) => {
                dbg!(&err);
                return Err(ErrorData::invalid_request(err.to_string(), None));
            }
        };

        let permission_id = hash.to_string();

        // Match on the permission scope type and build the appropriate response.
        // Each arm first checks whether this account is actually involved (delegator OR recipient).
        // If not, we skip the permission entirely — otherwise we'd return every permission on-chain.
        match contract.scope {
            // --- Stream (emission) permissions ---
            PermissionScope::Stream(stream) => {
                let is_delegator = contract.delegator == account_id;
                let is_recipient = stream.recipients.0.iter().any(|(k, _)| k == &account_id);
                if !is_delegator && !is_recipient {
                    continue;
                }

                let direction = if is_delegator {
                    let recipients: Vec<_> = stream
                        .recipients
                        .0
                        .iter()
                        .map(|(target, _)| name_or_key(target))
                        .collect();
                    Direction::DelegatingTo(recipients)
                } else {
                    Direction::DelegatedFrom(name_or_key(&contract.delegator))
                };

                let allocation = match stream.allocation {
                    StreamAllocation::Streams(bounded_btree_map) => Allocation::Streams(
                        bounded_btree_map
                            .0
                            .iter()
                            .map(|(stream, percent)| (stream.to_string(), percent.0))
                            .collect::<HashMap<_, _>>(),
                    ),
                    StreamAllocation::FixedAmount(amount) => Allocation::FixedAmount(amount),
                };

                let distribution = match stream.distribution {
                    DistributionControl::Manual => Distribution::Manual,
                    DistributionControl::Automatic(value) => Distribution::Automatic(value),
                    DistributionControl::AtBlock(value) => Distribution::AtBlock(value),
                    DistributionControl::Interval(value) => Distribution::Interval(value),
                };

                let detail = Permission::Stream((
                    StreamPermission {
                        allocation,
                        distribution,
                        recipients: stream
                            .recipients
                            .0
                            .iter()
                            .map(|(account, amount)| (name_or_key(account), *amount))
                            .collect(),
                        accumulating: stream.accumulating,
                    },
                    direction,
                ));

                permissions.push(PermissionEntry {
                    id: permission_id,
                    detail,
                });
            }
            // --- Curator permissions ---
            PermissionScope::Curator(curator) => {
                let is_delegator = contract.delegator == account_id;
                let is_recipient = curator.recipient == account_id;
                if !is_delegator && !is_recipient {
                    continue;
                }

                let direction = if is_delegator {
                    Direction::DelegatingTo(vec![name_or_key(&curator.recipient)])
                } else {
                    Direction::DelegatedFrom(name_or_key(&contract.delegator))
                };

                permissions.push(PermissionEntry {
                    id: permission_id,
                    detail: Permission::Curator((CuratorPermission {}, direction)),
                });
            }
            // --- Namespace permissions ---
            PermissionScope::Namespace(namespace) => {
                let is_delegator = contract.delegator == account_id;
                let is_recipient = namespace.recipient == account_id;
                if !is_delegator && !is_recipient {
                    continue;
                }

                let direction = if is_delegator {
                    Direction::DelegatingTo(vec![name_or_key(&namespace.recipient)])
                } else {
                    Direction::DelegatedFrom(name_or_key(&contract.delegator))
                };

                // A single permission can cover multiple paths under multiple parents
                for (parent, path) in namespace.paths.0 {
                    for path in path.0 {
                        let detail = Permission::Namespace((
                            NamespacePermission {
                                path: String::from_utf8_lossy(&path.0.0[..]).to_string(),
                                parent: parent.map(|hash| hash.to_string()),
                            },
                            direction.clone(),
                        ));
                        permissions.push(PermissionEntry {
                            id: permission_id.clone(),
                            detail,
                        });
                    }
                }
            }
            // --- Wallet permissions ---
            PermissionScope::Wallet(wallet) => {
                let is_delegator = contract.delegator == account_id;
                let is_recipient = wallet.recipient == account_id;
                if !is_delegator && !is_recipient {
                    continue;
                }

                let direction = if is_delegator {
                    Direction::DelegatingTo(vec![name_or_key(&wallet.recipient)])
                } else {
                    Direction::DelegatedFrom(name_or_key(&contract.delegator))
                };

                let detail = Permission::Wallet((
                    WalletPermission {
                        r#type: match wallet.r#type {
                            WalletScopeType::Stake(stake) => WalletPermissionType::Stake {
                                can_transfer_stake: stake.can_transfer_stake,
                                exclusive_stake_access: stake.exclusive_stake_access,
                            },
                        },
                    },
                    direction,
                ));

                permissions.push(PermissionEntry {
                    id: permission_id,
                    detail,
                });
            }
        }
    }

    let total = permissions.len();
    permissions.truncate(50);

    Ok(CallToolResult::success(vec![Content::json(
        PermissionSummaryResponse { total, permissions },
    )?]))
}

// =====================================================================
// Permission management tools (added in MCP expansion)
// =====================================================================

/// Params for revoking a permission by its hash ID.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct RevokePermissionRequest {
    /// Account that originally delegated the permission
    account_name: String,
    /// Hex-encoded H256 hash identifying the permission (e.g. "0xabcd...")
    permission_id: String,
}

/// Params for toggling emission accumulation on a permission.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct TogglePermissionAccumulationRequest {
    account_name: String,
    /// Hex-encoded H256 hash identifying the permission
    permission_id: String,
    /// true = accumulate emissions, false = distribute immediately
    accumulating: bool,
}

/// Revokes a permission contract by its ID.
/// Only the delegator (the one who created the permission) can revoke it,
/// and only if the revocation terms allow it.
pub async fn revoke_permission(
    torus_client: &Client,
    request: RevokePermissionRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;
    // Parse the hex string into an H256 hash
    let permission_id = H256::from_str(&request.permission_id)
        .map_err(|e| ErrorData::invalid_request(format!("Invalid permission ID: {e}"), None))?;

    match torus_client
        .permission0()
        .calls()
        .revoke_permission_wait(permission_id, keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "Permission revoked",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Toggles whether a stream permission accumulates emissions.
/// When accumulating=true, emissions build up until manually distributed.
/// When accumulating=false, emissions are distributed each epoch.
pub async fn toggle_permission_accumulation(
    torus_client: &Client,
    request: TogglePermissionAccumulationRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;
    let permission_id = H256::from_str(&request.permission_id)
        .map_err(|e| ErrorData::invalid_request(format!("Invalid permission ID: {e}"), None))?;

    match torus_client
        .permission0()
        .calls()
        .toggle_permission_accumulation_wait(permission_id, request.accumulating, keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "Permission accumulation set to {}",
            request.accumulating
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}
