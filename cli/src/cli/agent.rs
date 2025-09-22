use tabled::Table;
use torus_client::{client::TorusClient, subxt::utils::AccountId32};

use crate::{
    cli::CliCtx,
    store::{get_account, get_key},
};

#[derive(clap::Args)]
#[command(arg_required_else_help = true)]
pub struct AgentCliCommand {
    pub account: Option<String>,

    #[command(subcommand)]
    pub sub_command: Option<AgentCliSubCommand>,
}

#[derive(clap::Subcommand, Clone)]
pub enum AgentCliSubCommand {
    /// Prints information about the given agent key.
    Info {
        /// The saved key name or ss58 valid address.
        account: String,
    },
    /// Registers an agent.
    Register {
        /// The saved key name that will become an agent.
        key: String,
        /// The name of the agent.
        name: String,
        /// The metadata of the agent.
        metadata: String,
        /// The url of the agent.
        url: String,
    },
}

#[derive(tabled::Tabled)]
struct AgentInfo {
    address: String,
    name: String,
    metadata: String,
    url: String,
    last_update_block: u64,
    registration_block: u64,
}

impl AgentInfo {
    fn new(
        address: String,
        name: Vec<u8>,
        metadata: Vec<u8>,
        url: Vec<u8>,
        last_update_block: u64,
        registration_block: u64,
    ) -> Self {
        Self {
            address,
            name: String::from_utf8_lossy(&name[..]).to_string(),
            metadata: String::from_utf8_lossy(&metadata[..]).to_string(),
            url: String::from_utf8_lossy(&url[..]).to_string(),
            last_update_block,
            registration_block,
        }
    }
}

pub async fn info(ctx: &CliCtx, account: String) -> anyhow::Result<()> {
    let account = get_account(&account)?;

    println!("Fetching agent data...");

    let agent_info = get_agent_info(ctx, &account).await?;

    if let Some(agent_info) = agent_info {
        let table = Table::kv(std::iter::once(agent_info));
        println!("{table}");
    } else {
        println!("{account} is not an agent.");
    }

    Ok(())
}

pub async fn register(
    ctx: &CliCtx,
    key: String,
    name: String,
    metadata: String,
    url: String,
) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    ctx.confirm(&format!(
        "register an agent with the IMMUTABLE name of {name}"
    ))?;

    println!("Registering agent...");

    if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .calls()
            .register_agent_wait(
                name.as_bytes().to_vec(),
                url.as_bytes().to_vec(),
                metadata.as_bytes().to_vec(),
                keypair.clone(),
            )
            .await?;
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .calls()
            .register_agent_wait(
                name.as_bytes().to_vec(),
                url.as_bytes().to_vec(),
                metadata.as_bytes().to_vec(),
                keypair.clone(),
            )
            .await?;
    };

    let agent_info = get_agent_info(ctx, &keypair.account()).await?;
    let Some(agent_info) = agent_info else {
        println!("Something went wrong...");
        return Ok(());
    };

    println!("Agent registered successfully!");

    let table = Table::kv(std::iter::once(agent_info));
    println!("{table}");

    Ok(())
}

async fn get_agent_info(ctx: &CliCtx, account: &AccountId32) -> anyhow::Result<Option<AgentInfo>> {
    let agent_info = if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .storage()
            .agents_get(account)
            .await?
            .map(|agent| {
                AgentInfo::new(
                    agent.key.to_string(),
                    agent.name.0,
                    agent.metadata.0,
                    agent.url.0,
                    agent.last_update_block,
                    agent.registration_block,
                )
            })
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .storage()
            .agents_get(account)
            .await?
            .map(|agent| {
                AgentInfo::new(
                    agent.key.to_string(),
                    agent.name.0,
                    agent.metadata.0,
                    agent.url.0,
                    agent.last_update_block,
                    agent.registration_block,
                )
            })
    };

    Ok(agent_info)
}
