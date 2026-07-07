#![allow(dead_code)]
//! Torus MCP Server — Entry point and tool registration.
//!
//! This is an MCP (Model Context Protocol) server that lets AI assistants
//! interact with the Torus blockchain. It exposes ~46 tools that can:
//! - Query chain state (agents, balances, proposals, etc.)
//! - Submit transactions (stake, transfer, vote, register, etc.)
//!
//! ## How it works
//! 1. The server connects to a Torus node via WebSocket
//! 2. It registers all tools with the MCP framework (rmcp)
//! 3. MCP clients (like Claude) can call these tools via JSON-RPC over stdio
//!
//! ## Account system
//! This MCP only supports hardcoded dev accounts (alice, bob, charlie, etc.)
//! Each tool that writes to the chain takes an `account_name` parameter
//! that maps to one of these dev keypairs. This is fine for testing but
//! would need real key management for production.

// Compile-time check: you can only target one network at a time
#[cfg(all(feature = "testnet", feature = "devnet"))]
compile_error!("only one of the following features can be enabled at a time: testnet, devnet.");

// --- Framework imports ---
use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{
    CallToolResult, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
};
use rmcp::transport::stdio; // Communicate with MCP client over stdin/stdout
use rmcp::{ErrorData, ServerHandler, ServiceExt, tool, tool_handler, tool_router};
use std::collections::HashMap;
use std::sync::Arc; // Arc = thread-safe reference counting pointer
use torus_client::client::TorusClient; // The blockchain client library
use torus_client::subxt_signer::sr25519::Keypair;
use torus_client::subxt_signer::sr25519::dev::{alice, bob, charlie, dave, eve, ferdie, one, two};
use tracing_subscriber::EnvFilter;

// --- Import request types from each module ---
// Each module defines its own request/response structs.
// We import them here so the #[tool] handlers can reference them.
use crate::agent::{
    AgentDeregisterRequest, AgentInfoRequest, AgentRegisterRequest, AgentWhitelistAddRequest,
    DelegateCuratorPermisionRequest, UpdateAgentRequest,
};
use crate::balance::BalanceCheckRequest;
use crate::emission::DelegateEmissionRequest;
use crate::governance::{
    AddCustomProposalRequest, AddTreasuryTransferProposalRequest, GetApplicationRequest,
    GetProposalRequest, RemoveVoteRequest, SubmitApplicationRequest, VoteDelegationRequest,
    VoteProposalRequest,
};
use crate::namespace::{
    NamespaceCreationRequest, NamespaceDelegationRequest, NamespaceDeletionRequest,
    PermissionSummaryRequest, RevokePermissionRequest, TogglePermissionAccumulationRequest,
};
use crate::queries::WhitelistCheckRequest;
use crate::staking::{
    AddStakeRequest, GetStakeRequest, GetStakersForAgentRequest, GetStakesForAccountRequest,
    RemoveStakeRequest, TransferStakeRequest,
};
use crate::transfer::TransferBalanceRequest;
use crate::weight_control::{
    DelegateWeightControlRequest, GetWeightControlDelegationRequest, RegainWeightControlRequest,
};
use crate::weights::SetWeightsRequest;

/// Re-export the auto-generated chain interfaces.
/// These are created at build time from the chain's metadata and contain
/// all the type definitions for extrinsics, storage, events, etc.
/// The feature flag determines which network's metadata to use.
pub mod interfaces {
    #[cfg(feature = "testnet")]
    pub use torus_client::interfaces::testnet::api::*;

    #[cfg(feature = "devnet")]
    pub use torus_client::interfaces::devnet::api::*;
}

// --- Module declarations ---
// Each module contains the handler functions and request/response types
// for a group of related tools. Rust's `mod` keyword is like an import
// that makes the module available as `modulename::function_name`.
mod agent; // Agent registration, info, whitelist, curator
mod balance; // Balance checking
mod consensus; // List consensus members
mod emission; // Emission delegation
mod governance; // Applications, proposals, voting
mod namespace; // Namespace CRUD, permissions
mod queries; // Read-only chain queries (agents list, burn, emission params)
mod staking; // Stake/unstake/transfer-stake, stake queries
mod transfer; // Token transfers
mod utils; // Helper functions (keypair_from_name, name_or_key)
mod weight_control; // Weight control delegation
mod weights; // Set validator weights

// Hardcoded dev accounts available for testing.
// lazy_static! creates a global variable that's initialized once on first access.
// These are well-known Substrate dev accounts with deterministic private keys.
lazy_static::lazy_static! {
    static ref ACCOUNTS: HashMap<String, Keypair> = HashMap::from([
        ("alice".to_string(), alice()),
        ("bob".to_string(), bob()),
        ("charlie".to_string(), charlie()),
        ("dave".to_string(), dave()),
        ("eve".to_string(), eve()),
        ("ferdie".to_string(), ferdie()),
        ("one".to_string(), one()),
        ("two".to_string(), two()),
    ]);
}

/// Type alias for the blockchain client.
/// TorusClient is generic over the network type (TestNet vs DevNet).
/// This alias lets the rest of the code just say `Client` without caring
/// which network we're targeting.
#[cfg(feature = "testnet")]
pub type Client = TorusClient<torus_client::chain::TestNet>;

#[cfg(feature = "devnet")]
pub type Client = TorusClient<torus_client::chain::DevNet>;

/// The main MCP server struct.
/// Holds a shared reference to the blockchain client (Arc = thread-safe)
/// and the tool router that maps tool names to handler functions.
#[derive(Clone)]
pub struct TorusMcp {
    torus_client: Arc<Client>,
    tool_router: ToolRouter<TorusMcp>,
}

/// Tool registration — this is where ALL tools are wired up.
///
/// The #[tool_router] macro generates the routing code that maps
/// incoming MCP tool calls to the correct handler function.
///
/// Each #[tool(description = "...")] method:
/// 1. Receives JSON params as a `Parameters<RequestType>` wrapper
/// 2. Delegates to the actual handler function in the corresponding module
/// 3. Returns a CallToolResult (success with content, or error)
///
/// The `description` is what MCP clients see when they list available tools.
#[tool_router]
impl TorusMcp {
    pub async fn new(torus_client: Client) -> Self {
        Self {
            torus_client: Arc::new(torus_client),
            tool_router: Self::tool_router(),
        }
    }

    // ================================================================
    // Agent tools — register/deregister/update agents on the network
    // ================================================================

    #[tool(
        description = "Registers an account as an agent on the chain via it's name on the preconfigured accounts dictionary."
    )]
    async fn register_agent(
        &self,
        Parameters(request): Parameters<AgentRegisterRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        agent::register_agent(&self.torus_client, request).await
    }

    #[tool(
        description = "Removes an account from the agent status on the chain via it's name on the preconfigured accounts dictionary."
    )]
    async fn deregister_agent(
        &self,
        Parameters(request): Parameters<AgentDeregisterRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        agent::deregister_agent(&self.torus_client, request).await
    }

    #[tool(
        description = "Gets information about an agent on the chain by the account name on the accounts dictionary."
    )]
    async fn get_agent_info(
        &self,
        Parameters(request): Parameters<AgentInfoRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        agent::get_agent_info(&self.torus_client, request).await
    }

    #[tool(description = "Updates an agent's URL, metadata, staking fee, or weight control fee.")]
    async fn update_agent(
        &self,
        Parameters(request): Parameters<UpdateAgentRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        agent::update_agent(&self.torus_client, request).await
    }

    #[tool(
        description = "Adds an agent to the whitelist. Signer must be a curator or admin dev account."
    )]
    async fn whitelist_agent(
        &self,
        Parameters(request): Parameters<AgentWhitelistAddRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        agent::whitelist_agent(&self.torus_client, request).await
    }

    #[tool(
        description = "Removes an agent from the whitelist. Signer must be a curator or admin dev account."
    )]
    async fn dewhitelist_agent(
        &self,
        Parameters(request): Parameters<AgentWhitelistAddRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        agent::dewhitelist_agent(&self.torus_client, request).await
    }

    #[tool(description = "Makes the given agent account a curator.")]
    async fn delegate_curator_permission(
        &self,
        Parameters(request): Parameters<DelegateCuratorPermisionRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        agent::delegate_curator_permission(&self.torus_client, request).await
    }

    // ================================================================
    // Balance tools — check account balances
    // ================================================================

    #[tool(description = "Checks the balance for the supplied account name.")]
    async fn check_account_balance(
        &self,
        Parameters(request): Parameters<BalanceCheckRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        balance::check_account_balance(&self.torus_client, request).await
    }

    // ================================================================
    // Transfer tools — send tokens between accounts
    // ================================================================

    #[tool(description = "Transfers tokens from one account to another.")]
    async fn transfer_balance(
        &self,
        Parameters(request): Parameters<TransferBalanceRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        transfer::transfer_balance(&self.torus_client, request).await
    }

    // ================================================================
    // Staking tools — stake/unstake tokens on agents
    // ================================================================

    #[tool(description = "Stakes tokens from a staker account to an agent.")]
    async fn add_stake(
        &self,
        Parameters(request): Parameters<AddStakeRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        staking::add_stake(&self.torus_client, request).await
    }

    #[tool(description = "Removes stake from a staker account on an agent.")]
    async fn remove_stake(
        &self,
        Parameters(request): Parameters<RemoveStakeRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        staking::remove_stake(&self.torus_client, request).await
    }

    #[tool(description = "Transfers stake from one agent to another.")]
    async fn transfer_stake(
        &self,
        Parameters(request): Parameters<TransferStakeRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        staking::transfer_stake(&self.torus_client, request).await
    }

    #[tool(description = "Gets the stake amount between a staker and an agent.")]
    async fn get_stake(
        &self,
        Parameters(request): Parameters<GetStakeRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        staking::get_stake(&self.torus_client, request).await
    }

    #[tool(description = "Lists all staking positions for an account.")]
    async fn get_stakes_for_account(
        &self,
        Parameters(request): Parameters<GetStakesForAccountRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        staking::get_stakes_for_account(&self.torus_client, request).await
    }

    #[tool(description = "Gets the total stake across the entire Torus network.")]
    async fn get_total_stake(&self) -> Result<CallToolResult, ErrorData> {
        staking::get_total_stake(&self.torus_client).await
    }

    #[tool(
        description = "Lists all stakers on a given agent and the total stake they hold. Accepts dev account names or mainnet SS58 addresses."
    )]
    async fn get_stakers_for_agent(
        &self,
        Parameters(request): Parameters<GetStakersForAgentRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        staking::get_stakers_for_agent(&self.torus_client, request).await
    }

    // ================================================================
    // Query tools — read-only chain state queries
    // ================================================================

    #[tool(description = "Lists all registered agents on the chain.")]
    async fn list_agents(&self) -> Result<CallToolResult, ErrorData> {
        queries::list_agents(&self.torus_client).await
    }

    #[tool(description = "Gets the current dynamic burn amount required for agent registration.")]
    async fn get_burn_amount(&self) -> Result<CallToolResult, ErrorData> {
        queries::get_burn_amount(&self.torus_client).await
    }

    #[tool(description = "Gets the burn configuration parameters.")]
    async fn get_burn_config(&self) -> Result<CallToolResult, ErrorData> {
        queries::get_burn_config(&self.torus_client).await
    }

    #[tool(description = "Gets the pending emission amount to be distributed next epoch.")]
    async fn get_pending_emission(&self) -> Result<CallToolResult, ErrorData> {
        queries::get_pending_emission(&self.torus_client).await
    }

    #[tool(
        description = "Gets the current incentives ratio (balance between miner incentives and validator dividends)."
    )]
    async fn get_incentives_ratio(&self) -> Result<CallToolResult, ErrorData> {
        queries::get_incentives_ratio(&self.torus_client).await
    }

    #[tool(description = "Gets the current emission recycling percentage.")]
    async fn get_emission_recycling_percentage(&self) -> Result<CallToolResult, ErrorData> {
        queries::get_emission_recycling_percentage(&self.torus_client).await
    }

    #[tool(description = "Checks whether an account is whitelisted.")]
    async fn check_whitelist_status(
        &self,
        Parameters(request): Parameters<WhitelistCheckRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        queries::check_whitelist_status(&self.torus_client, request).await
    }

    #[tool(description = "Gets the global governance configuration.")]
    async fn get_global_governance_config(&self) -> Result<CallToolResult, ErrorData> {
        queries::get_global_governance_config(&self.torus_client).await
    }

    // ================================================================
    // Weights and emission tools — validator weight setting
    // ================================================================

    #[tool(description = "Sets the weights of an agent account.")]
    async fn set_weights(
        &self,
        Parameters(request): Parameters<SetWeightsRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        weights::set_weights(&self.torus_client, request).await
    }

    #[tool(description = "List all consensus members.")]
    async fn list_consensus_members(&self) -> Result<CallToolResult, ErrorData> {
        consensus::list_consensus_members(&self.torus_client).await
    }

    #[tool(
        description = "Delegates or re-delegates an emission stream to the named agent. Delegating does not require the stream to be supplied, redelegating does."
    )]
    async fn delegate_emission(
        &self,
        Parameters(request): Parameters<DelegateEmissionRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        emission::delegate_emission(&self.torus_client, request).await
    }

    // ================================================================
    // Weight control tools — delegate weight-setting responsibility
    // ================================================================

    #[tool(description = "Delegates weight control to another validator (must be an allocator).")]
    async fn delegate_weight_control(
        &self,
        Parameters(request): Parameters<DelegateWeightControlRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        weight_control::delegate_weight_control(&self.torus_client, request).await
    }

    #[tool(description = "Regains weight control from a previous delegation.")]
    async fn regain_weight_control(
        &self,
        Parameters(request): Parameters<RegainWeightControlRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        weight_control::regain_weight_control(&self.torus_client, request).await
    }

    #[tool(description = "Checks if an account has delegated weight control and to whom.")]
    async fn get_weight_control_delegation(
        &self,
        Parameters(request): Parameters<GetWeightControlDelegationRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        weight_control::get_weight_control_delegation(&self.torus_client, request).await
    }

    // ================================================================
    // Namespace and permission tools — manage namespaces and access control
    // ================================================================

    #[tool(description = "Creates a namespace on the designated preconfigured account agent.")]
    async fn create_namespace_for_agent(
        &self,
        Parameters(request): Parameters<NamespaceCreationRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        namespace::create_namespace_for_agent(&self.torus_client, request).await
    }

    #[tool(description = "Deletes a namespace on the designated preconfigured account agent.")]
    async fn delete_namespace_for_agent(
        &self,
        Parameters(request): Parameters<NamespaceDeletionRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        namespace::delete_namespace_for_agent(&self.torus_client, request).await
    }

    #[tool(description = "Delegate a namespace permission for the supplied agent account name.")]
    async fn delegate_namespace_permission_for_agent(
        &self,
        Parameters(request): Parameters<NamespaceDelegationRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        namespace::delegate_namespace_permission_for_agent(&self.torus_client, request).await
    }

    #[tool(
        description = "Gets the summary of all permissions that affect the supplied account name. (emission, namespace and curator permissions)"
    )]
    async fn get_permission_summary_for_agent(
        &self,
        Parameters(request): Parameters<PermissionSummaryRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        namespace::get_permission_summary_for_agent(&self.torus_client, request).await
    }

    #[tool(description = "Revokes a permission by its ID (hex hash).")]
    async fn revoke_permission(
        &self,
        Parameters(request): Parameters<RevokePermissionRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        namespace::revoke_permission(&self.torus_client, request).await
    }

    #[tool(description = "Toggles whether a permission accumulates emissions.")]
    async fn toggle_permission_accumulation(
        &self,
        Parameters(request): Parameters<TogglePermissionAccumulationRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        namespace::toggle_permission_accumulation(&self.torus_client, request).await
    }

    // ================================================================
    // Governance tools — applications, proposals, voting
    // ================================================================

    #[tool(
        description = "Submits an application to add or remove an agent from the whitelist. Costs 100 TORUS (refunded if accepted)."
    )]
    async fn submit_application(
        &self,
        Parameters(request): Parameters<SubmitApplicationRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        governance::submit_application(&self.torus_client, request).await
    }

    #[tool(description = "Lists all agent applications.")]
    async fn list_applications(&self) -> Result<CallToolResult, ErrorData> {
        governance::list_applications(&self.torus_client).await
    }

    #[tool(description = "Gets details of a specific application by ID.")]
    async fn get_application(
        &self,
        Parameters(request): Parameters<GetApplicationRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        governance::get_application(&self.torus_client, request).await
    }

    #[tool(description = "Lists all governance proposals.")]
    async fn list_proposals(&self) -> Result<CallToolResult, ErrorData> {
        governance::list_proposals(&self.torus_client).await
    }

    #[tool(description = "Gets details of a specific proposal by ID.")]
    async fn get_proposal(
        &self,
        Parameters(request): Parameters<GetProposalRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        governance::get_proposal(&self.torus_client, request).await
    }

    #[tool(description = "Creates a custom global proposal with metadata.")]
    async fn add_custom_proposal(
        &self,
        Parameters(request): Parameters<AddCustomProposalRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        governance::add_custom_proposal(&self.torus_client, request).await
    }

    #[tool(description = "Creates a proposal to transfer funds from the DAO treasury.")]
    async fn add_treasury_transfer_proposal(
        &self,
        Parameters(request): Parameters<AddTreasuryTransferProposalRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        governance::add_treasury_transfer_proposal(&self.torus_client, request).await
    }

    #[tool(description = "Votes for or against a proposal.")]
    async fn vote_proposal(
        &self,
        Parameters(request): Parameters<VoteProposalRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        governance::vote_proposal(&self.torus_client, request).await
    }

    #[tool(description = "Removes a vote from a proposal.")]
    async fn remove_vote(
        &self,
        Parameters(request): Parameters<RemoveVoteRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        governance::remove_vote(&self.torus_client, request).await
    }

    #[tool(
        description = "Enables vote power delegation for an account (allows others to vote with your stake)."
    )]
    async fn enable_vote_delegation(
        &self,
        Parameters(request): Parameters<VoteDelegationRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        governance::enable_vote_delegation(&self.torus_client, request).await
    }

    #[tool(description = "Disables vote power delegation for an account.")]
    async fn disable_vote_delegation(
        &self,
        Parameters(request): Parameters<VoteDelegationRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        governance::disable_vote_delegation(&self.torus_client, request).await
    }
}

/// MCP server metadata — tells MCP clients what this server is and what it can do.
/// The #[tool_handler] macro wires up the tool router to actually handle incoming calls.
#[tool_handler]
impl ServerHandler for TorusMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server provides a comprehensive interface with the Torus blockchain. \
                 Use 'list_agents' to see all registered agents, 'check_account_balance' \
                 for balances, and various staking/governance/permission tools to interact \
                 with the chain."
                    .to_string(),
            ),
        }
    }
}

/// Entry point — starts the MCP server.
///
/// 1. Sets up logging (to stderr, so it doesn't interfere with MCP's stdout protocol)
/// 2. Connects to the Torus blockchain node
/// 3. Creates the MCP server and starts listening on stdin/stdout
///
/// #[tokio::main] turns this into an async runtime — needed because all
/// blockchain calls are async (they go over the network).
#[tokio::main]
async fn main() {
    // Set up logging — output goes to stderr (not stdout, which is used for MCP protocol)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    // Connect to the blockchain node
    // for_testnet() connects to wss://api.torus.network (mainnet/testnet)
    // for_devnet() connects to a local dev node
    #[cfg(feature = "testnet")]
    let torus_client = TorusClient::for_testnet()
        .await
        .expect("Failed to connect to testnet node");

    #[cfg(feature = "devnet")]
    let torus_client = TorusClient::for_devnet()
        .await
        .expect("Failed to connect to devnet node");

    tracing::info!("Connected to torus client");

    // Create the MCP server and start serving over stdio
    // stdio() = communicate via stdin (receives tool calls) and stdout (sends results)
    let service = TorusMcp::new(torus_client)
        .await
        .serve(stdio())
        .await
        .inspect_err(|e| {
            eprintln!("serving error: {e:?}");
        })
        .expect("Failed to start MCP server");

    // Block until the MCP client disconnects
    service
        .waiting()
        .await
        .expect("MCP server terminated unexpectedly");
}
