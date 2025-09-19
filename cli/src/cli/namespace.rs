use tabled::Table;
use torus_client::{
    client::TorusClient,
    subxt::ext::futures::{StreamExt, TryStreamExt},
};

use crate::{
    cli::CliCtx,
    store::{get_account, get_key},
};

#[derive(clap::Args)]
#[command(arg_required_else_help = true)]
pub struct NamespaceCliCommand {
    pub account: Option<String>,

    #[command(subcommand)]
    pub sub_command: Option<NamespaceCliSubCommand>,
}

#[derive(clap::Subcommand, Clone)]
pub enum NamespaceCliSubCommand {
    /// Prints all namespaces belonging to the given account.
    Info {
        /// The saved key name or valid ss58 address to be searched.
        account: String,
    },
    /// Registers a new namespace.
    Register {
        /// The saved key name of the agent that will hold the namespace.
        key: String,
        /// The namespace path to be created.
        path: String,
    },
}

#[derive(tabled::Tabled)]
struct NamespaceEntry {
    path: String,
}

pub async fn info(ctx: &CliCtx, account: String) -> anyhow::Result<()> {
    let account = get_account(&account)?;

    println!("Fetching agent data...");

    let entries = if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .storage()
            .namespaces_iter1(&torus_client::interfaces::testnet::api::runtime_types::pallet_torus0::namespace::NamespaceOwnership::Account(account))
            .await?
            .map(|res| match res {
                Ok(((_, path), _)) => Ok(NamespaceEntry { path: String::from_utf8_lossy(&path.0.0).to_string() }),
                Err(err) => Err(err),
            }).try_collect::<Vec<_>>().await?
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .storage()
            .namespaces_iter1(&torus_client::interfaces::mainnet::api::runtime_types::pallet_torus0::namespace::NamespaceOwnership::Account(account))
            .await?
            .map(|res| match res {
                Ok(((_, path), _)) => Ok(NamespaceEntry { path:dbg!(String::from_utf8_lossy(&path.0.0).to_string()) }),
                Err(err) => Err(err),
            }).try_collect::<Vec<_>>().await?
    };

    let table = Table::new(entries);

    print!("{table}");

    Ok(())
}

pub async fn register(ctx: &CliCtx, key: String, path: String) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let (fee, deposit) = if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .rpc()
            .namespace_path_creation_cost(keypair.account(), path.clone())
            .await?
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .rpc()
            .namespace_path_creation_cost(keypair.account(), path.clone())
            .await?
    };

    ctx.confirm(&format!(
        "register namespace {path} for a fee of {fee} and deposit of {deposit}"
    ))?;

    println!("Registering namespace...");

    if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .calls()
            .create_namespace_wait(torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(path.as_bytes().to_vec()), keypair)
            .await?;
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .calls()
            .create_namespace_wait(torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(path.as_bytes().to_vec()), keypair)
            .await?;
    };

    println!("Namespace registered successfully!");

    Ok(())
}
