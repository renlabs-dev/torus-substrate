//! Governance tools for the Torus MCP server.
//!
//! The governance pallet is Torus's democracy system. It handles:
//! - **Applications**: Agents apply to join/leave the network (costs 100 TORUS, refunded if accepted)
//! - **Proposals**: Anyone can propose changes to network parameters, treasury transfers, etc.
//! - **Voting**: Stakers vote on proposals (weighted by stake)
//! - **Vote delegation**: You can let someone else vote on your behalf
//!
//! Most write operations require the signer to be a registered agent with sufficient stake.

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use torus_client::subxt::ext::futures::StreamExt;

use crate::{
    Client,
    utils::{account_id_from_name_or_ss58, keypair_from_name, name_or_key},
};

// =====================================================================
// Request types — define what JSON params each tool accepts
// =====================================================================

/// Params for submitting an agent application.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct SubmitApplicationRequest {
    /// Dev account paying the application fee (100 TORUS)
    account_name: String,
    /// Agent to apply for — dev account name (e.g. "bob") or SS58 address (e.g. "5DoVVg...")
    agent_name: String,
    /// Free-form text describing why this agent should be added/removed
    metadata: String,
    /// false = applying to ADD agent, true = applying to REMOVE agent
    removing: bool,
}

/// Params for looking up a specific application by its numeric ID.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct GetApplicationRequest {
    application_id: u32,
}

/// Params for looking up a specific proposal by its numeric ID.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct GetProposalRequest {
    proposal_id: u64,
}

/// Params for creating a custom (free-form) governance proposal.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AddCustomProposalRequest {
    account_name: String,
    /// Description of what you're proposing
    metadata: String,
}

/// Params for proposing a treasury transfer (send DAO funds to someone).
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct AddTreasuryTransferProposalRequest {
    account_name: String,
    /// Amount to transfer from treasury (in planck)
    value: u128,
    /// Who receives the funds — dev account name or SS58 address
    destination_name: String,
    /// Why this transfer should happen
    metadata: String,
}

/// Params for voting on a proposal.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct VoteProposalRequest {
    account_name: String,
    proposal_id: u64,
    /// true = vote FOR, false = vote AGAINST
    agree: bool,
}

/// Params for removing a previously cast vote.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct RemoveVoteRequest {
    account_name: String,
    proposal_id: u64,
}

/// Params for enabling/disabling vote delegation.
/// When enabled, others can vote using your stake weight.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct VoteDelegationRequest {
    account_name: String,
}

// =====================================================================
// Response types — simplified versions of on-chain data for MCP output
// =====================================================================

/// Simplified view of an agent application.
/// Uses format!("{:?}") for enum fields (status, action) because the on-chain
/// types are complex generated enums — Debug formatting gives readable output
/// like "Open", "Accepted", "Removing" without us needing to mirror them.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct ApplicationResponse {
    id: u32,
    payer: String,
    agent: String,
    metadata: String,
    /// Debug-formatted enum: "Open", "Accepted", "Rejected", etc.
    status: String,
    /// Debug-formatted enum: "Add" or "Remove"
    action: String,
}

/// Wrapper for list_applications response.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct ApplicationListResponse {
    /// Total number of applications on-chain
    total: usize,
    /// Applications returned (capped at 50)
    applications: Vec<ApplicationResponse>,
}

/// Simplified view of a governance proposal.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct ProposalResponse {
    id: u64,
    proposer: String,
    /// Debug-formatted: "Open", "Accepted", "Rejected", "Expired"
    status: String,
    /// Debug-formatted: "Custom", "TreasuryTransfer", "GlobalParams", etc.
    data_type: String,
    metadata: String,
    /// Block number when this proposal expires
    expiration_block: u64,
}

/// Wrapper for list_proposals response.
#[derive(schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct ProposalListResponse {
    proposals: Vec<ProposalResponse>,
}

// =====================================================================
// Handler functions
// =====================================================================

/// Submits an application to add or remove an agent.
/// Costs 100 TORUS (refunded if the application is accepted by governance).
pub async fn submit_application(
    torus_client: &Client,
    request: SubmitApplicationRequest,
) -> Result<CallToolResult, ErrorData> {
    let payer_keypair = keypair_from_name(&request.account_name)?;
    let agent_id = account_id_from_name_or_ss58(&request.agent_name)?;

    match torus_client
        .governance()
        .calls()
        .submit_application_wait(
            agent_id,
            request.metadata.into_bytes(),
            request.removing,
            payer_keypair,
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "Application submitted for agent {}",
            request.agent_name
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Lists ALL applications on-chain (open, accepted, rejected — everything).
/// Iterates the AgentApplications storage map.
pub async fn list_applications(torus_client: &Client) -> Result<CallToolResult, ErrorData> {
    let mut stream = match torus_client
        .governance()
        .storage()
        .agent_applications_iter()
        .await
    {
        Ok(stream) => stream,
        Err(err) => {
            dbg!(&err);
            return Err(ErrorData::internal_error(err.to_string(), None));
        }
    };

    let mut all = Vec::new();
    while let Some(item) = stream.next().await {
        let (id, app) = match item {
            Ok(kv) => kv,
            Err(err) => {
                dbg!(&err);
                return Err(ErrorData::internal_error(err.to_string(), None));
            }
        };
        all.push(ApplicationResponse {
            id,
            payer: name_or_key(&app.payer_key),
            agent: name_or_key(&app.agent_key),
            metadata: String::from_utf8_lossy(&app.data.0).to_string(),
            status: format!("{:?}", app.status),
            action: format!("{:?}", app.action),
        });
    }

    let total = all.len();
    // Sort descending by id so the most recent appear first, then cap at 50
    all.sort_by(|a, b| b.id.cmp(&a.id));
    all.truncate(50);

    Ok(CallToolResult::success(vec![Content::json(
        ApplicationListResponse {
            total,
            applications: all,
        },
    )?]))
}

/// Gets a single application by its numeric ID.
pub async fn get_application(
    torus_client: &Client,
    request: GetApplicationRequest,
) -> Result<CallToolResult, ErrorData> {
    match torus_client
        .governance()
        .storage()
        .agent_applications_get(&request.application_id)
        .await
    {
        Ok(Some(app)) => Ok(CallToolResult::success(vec![Content::json(
            ApplicationResponse {
                id: request.application_id,
                payer: name_or_key(&app.payer_key),
                agent: name_or_key(&app.agent_key),
                metadata: String::from_utf8_lossy(&app.data.0).to_string(),
                status: format!("{:?}", app.status),
                action: format!("{:?}", app.action),
            },
        )?])),
        Ok(None) => Err(ErrorData::invalid_request("Application not found", None)),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}

/// Lists ALL governance proposals on-chain.
pub async fn list_proposals(torus_client: &Client) -> Result<CallToolResult, ErrorData> {
    let mut stream = match torus_client.governance().storage().proposals_iter().await {
        Ok(stream) => stream,
        Err(err) => {
            dbg!(&err);
            return Err(ErrorData::internal_error(err.to_string(), None));
        }
    };

    let mut proposals = Vec::new();
    while let Some(item) = stream.next().await {
        let (id, proposal) = match item {
            Ok(kv) => kv,
            Err(err) => {
                dbg!(&err);
                return Err(ErrorData::internal_error(err.to_string(), None));
            }
        };
        proposals.push(ProposalResponse {
            id,
            proposer: name_or_key(&proposal.proposer),
            status: format!("{:?}", proposal.status),
            data_type: format!("{:?}", proposal.data),
            metadata: String::from_utf8_lossy(&proposal.metadata.0).to_string(),
            expiration_block: proposal.expiration_block,
        });
    }

    Ok(CallToolResult::success(vec![Content::json(
        ProposalListResponse { proposals },
    )?]))
}

/// Gets a single proposal by its numeric ID.
pub async fn get_proposal(
    torus_client: &Client,
    request: GetProposalRequest,
) -> Result<CallToolResult, ErrorData> {
    match torus_client
        .governance()
        .storage()
        .proposals_get(&request.proposal_id)
        .await
    {
        Ok(Some(proposal)) => Ok(CallToolResult::success(vec![Content::json(
            ProposalResponse {
                id: request.proposal_id,
                proposer: name_or_key(&proposal.proposer),
                status: format!("{:?}", proposal.status),
                data_type: format!("{:?}", proposal.data),
                metadata: String::from_utf8_lossy(&proposal.metadata.0).to_string(),
                expiration_block: proposal.expiration_block,
            },
        )?])),
        Ok(None) => Err(ErrorData::invalid_request("Proposal not found", None)),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::internal_error(err.to_string(), None))
        }
    }
}

/// Creates a custom governance proposal (free-form text, no automatic execution).
/// Requires the signer to have enough stake to propose.
pub async fn add_custom_proposal(
    torus_client: &Client,
    request: AddCustomProposalRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;

    match torus_client
        .governance()
        .calls()
        .add_global_custom_proposal_wait(request.metadata.into_bytes(), keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "Custom proposal submitted",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Creates a proposal to transfer funds from the DAO treasury to a destination account.
/// If the proposal passes voting, the transfer happens automatically.
pub async fn add_treasury_transfer_proposal(
    torus_client: &Client,
    request: AddTreasuryTransferProposalRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;
    let dest_id = account_id_from_name_or_ss58(&request.destination_name)?;

    match torus_client
        .governance()
        .calls()
        .add_dao_treasury_transfer_proposal_wait(
            request.value,
            dest_id,
            request.metadata.into_bytes(),
            keypair,
        )
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "Treasury transfer proposal submitted",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Casts a vote on an existing proposal. Vote weight is proportional to stake.
pub async fn vote_proposal(
    torus_client: &Client,
    request: VoteProposalRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;

    match torus_client
        .governance()
        .calls()
        .vote_proposal_wait(request.proposal_id, request.agree, keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "Vote {} on proposal {}",
            if request.agree { "for" } else { "against" },
            request.proposal_id
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Removes a previously cast vote from a proposal (lets you change your mind).
pub async fn remove_vote(
    torus_client: &Client,
    request: RemoveVoteRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;

    match torus_client
        .governance()
        .calls()
        .remove_vote_proposal_wait(request.proposal_id, keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "Vote removed from proposal {}",
            request.proposal_id
        ))])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Enables vote delegation — allows other accounts to vote using your stake weight.
/// This is opt-in: you must explicitly enable it before others can delegate to you.
pub async fn enable_vote_delegation(
    torus_client: &Client,
    request: VoteDelegationRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;

    match torus_client
        .governance()
        .calls()
        .enable_vote_delegation_wait(keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "Vote delegation enabled",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}

/// Disables vote delegation — stops others from voting with your stake.
pub async fn disable_vote_delegation(
    torus_client: &Client,
    request: VoteDelegationRequest,
) -> Result<CallToolResult, ErrorData> {
    let keypair = keypair_from_name(&request.account_name)?;

    match torus_client
        .governance()
        .calls()
        .disable_vote_delegation_wait(keypair)
        .await
    {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(
            "Vote delegation disabled",
        )])),
        Err(err) => {
            dbg!(&err);
            Err(ErrorData::invalid_request(format!("{err:?}"), None))
        }
    }
}
