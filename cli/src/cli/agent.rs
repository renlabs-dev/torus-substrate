use tabled::Table;
use torus_client::client::TorusClient;

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
    Info {
        account: String,
    },
    Register {
        key: String,
        name: String,
        metadata: String,
        url: String,
    },
}

#[derive(tabled::Tabled)]
struct AgentInfo {
    name: String,
    metadata: String,
    url: String,
    last_update_block: u64,
    registration_block: u64,
}

impl AgentInfo {
    fn new(
        name: Vec<u8>,
        metadata: Vec<u8>,
        url: Vec<u8>,
        last_update_block: u64,
        registration_block: u64,
    ) -> Self {
        Self {
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

    let agent_info = if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .storage()
            .agents_get(&account)
            .await?
            .map(|agent| {
                AgentInfo::new(
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
            .agents_get(&account)
            .await?
            .map(|agent| {
                AgentInfo::new(
                    agent.name.0,
                    agent.metadata.0,
                    agent.url.0,
                    agent.last_update_block,
                    agent.registration_block,
                )
            })
    };

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
                keypair,
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
                keypair,
            )
            .await?;
    };

    println!("Agent registered successfully!");

    Ok(())
}
