use std::str::FromStr;

use tabled::{
    settings::{disable::Remove, object::FirstRow},
    Table,
};
use torus_client::{
    client::TorusClient,
    subxt::utils::{AccountId32, MultiAddress},
};

use crate::{
    cli::CliCtx,
    store::{get_account, get_key},
};

#[derive(clap::Args)]
#[command(arg_required_else_help = true)]
pub struct BalanceCliCommand {
    pub key: Option<String>,

    #[command(subcommand)]
    pub sub_command: Option<BalanceCliSubCommand>,
}

#[derive(clap::Subcommand, Clone)]
pub enum BalanceCliSubCommand {
    /// Displays the key balance (free and reserved).
    Check {
        /// The saved key name or valid ss58 address.
        key: String,
    },

    /// Transfers balance to an account.
    Transfer {
        /// The saved key name or a valid ss58 address that will fund the transfer.
        key: String,
        /// The saved key name or a valid ss58 address that will receive the amount transfered.
        target: String,
        /// The amount to be transfered.
        amount: u128,
    },
}

pub async fn check(ctx: &CliCtx, key: String) -> anyhow::Result<()> {
    let account = get_account(&key)?;

    let data = if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .system()
            .storage()
            .account_get(&account)
            .await?
            .map(|info| (info.data.free, info.data.reserved, info.data.frozen))
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .system()
            .storage()
            .account_get(&account)
            .await?
            .map(|info| (info.data.free, info.data.reserved, info.data.frozen))
    }
    .unwrap_or((0, 0, 0));

    let mut table = Table::new(vec![
        (("free".to_string()), data.0.to_string()),
        (
            ("reserved (stake + others)".to_string()),
            (data.1 + data.2).to_string(),
        ),
    ]);
    table.with(Remove::row(FirstRow));

    println!("{table}");

    Ok(())
}

pub async fn transfer(
    ctx: &CliCtx,
    key: String,
    target: String,
    amount: u128,
) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let target = AccountId32::from_str(&target)?;

    ctx.confirm(&format!("transfer {amount} to {target}"))?;

    println!("Transfering...");

    if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .balances()
            .calls()
            .transfer_keep_alive_wait(MultiAddress::Id(target), amount, keypair)
            .await?
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .balances()
            .calls()
            .transfer_keep_alive_wait(MultiAddress::Id(target), amount, keypair)
            .await?
    };

    println!("Transfered successfully!");

    Ok(())
}
