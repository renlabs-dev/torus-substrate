#[cfg(all(feature = "testnet", feature = "devnet"))]
compile_error!("only one of the following features can be enabled at a time: testnet, devnet.");

use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{
    CallToolResult, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
};
use rmcp::transport::stdio;
use rmcp::{ErrorData, ServerHandler, ServiceExt, tool, tool_handler, tool_router};
use std::collections::HashMap;
use std::sync::Arc;
use torus_client::chain::TestNet;
use torus_client::client::TorusClient;
use torus_client::subxt_signer::sr25519::Keypair;
use torus_client::subxt_signer::sr25519::dev::{alice, bob, charlie, dave, ferdie, one, two};
use tracing_subscriber::EnvFilter;

use crate::agent::{
    AgentDeregisterRequest, AgentInfoRequest, AgentRegisterRequest, AgentWhitelistAddRequest,
};
use crate::balance::BalanceCheckRequest;
use crate::namespace::{
    NamespaceCreationRequest, NamespaceDelegationRequest, NamespaceDeletionRequest,
    NamespaceSummaryRequest,
};
use crate::weights::SetWeightsRequest;

mod agent;
mod balance;
mod consensus;
mod namespace;
mod utils;
mod weights;

lazy_static::lazy_static! {
    static ref ACCOUNTS: HashMap<String, Keypair> = HashMap::from([
        ("alice".to_string(), alice()),
        ("bob".to_string(), bob()),
        ("charlie".to_string(), charlie()),
        ("dave".to_string(), dave()),
        ("ferdie".to_string(), ferdie()),
        ("one".to_string(), one()),
        ("two".to_string(), two()),
    ]);
}

#[cfg(feature = "testnet")]
pub type Client = TorusClient<TestNet>;

#[cfg(feature = "devnet")]
pub type Client = TorusClient<DevNet>;

#[derive(Clone)]
pub struct TorusMcp {
    torus_client: Arc<Client>,
    tool_router: ToolRouter<TorusMcp>,
}

#[tool_router]
impl TorusMcp {
    pub async fn new(torus_client: Client) -> Self {
        Self {
            torus_client: Arc::new(torus_client),
            tool_router: Self::tool_router(),
        }
    }

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

    #[tool(description = "Adds an agent to the whitelist (uses alice as the signer).")]
    async fn whitelist_agent(
        &self,
        Parameters(request): Parameters<AgentWhitelistAddRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        agent::whitelist_agent(&self.torus_client, request).await
    }

    #[tool(description = "Removes an agent from the whitelist (uses alice as the signer).")]
    async fn dewhitelist_agent(
        &self,
        Parameters(request): Parameters<AgentWhitelistAddRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        agent::dewhitelist_agent(&self.torus_client, request).await
    }

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
        description = "Gets the summary of all namespaces delegated by or delegated to the supplied account name."
    )]
    async fn get_namespace_summary_for_agent(
        &self,
        Parameters(request): Parameters<NamespaceSummaryRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        namespace::get_namespace_summary_for_agent(&self.torus_client, request).await
    }

    #[tool(description = "Checks the balance for the supplied account name.")]
    async fn check_account_balance(
        &self,
        Parameters(request): Parameters<BalanceCheckRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        balance::check_account_balance(&self.torus_client, request).await
    }

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
}

#[tool_handler]
impl ServerHandler for TorusMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides an interface with the torus blockchain. Agents can be inspected with the 'get_agent_info' tool.".to_string()),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    #[cfg(feature = "testnet")]
    let torus_client = TorusClient::for_testnet().await.unwrap();

    #[cfg(feature = "devnet")]
    let torus_client = TorusClient::for_devnet().await.unwrap();

    tracing::info!("Connected to torus client");

    let service = TorusMcp::new(torus_client)
        .await
        .serve(stdio())
        .await
        .inspect_err(|e| {
            eprintln!("serving error: {e:?}");
        })
        .unwrap();

    service.waiting().await.unwrap();
}
