use std::str::FromStr;

use tabled::Table;
use torus_client::{
    client::TorusClient,
    subxt::{ext::futures::TryStreamExt, utils::AccountId32},
};

use crate::{
    cli::CliCtx,
    store::{get_account, get_key},
};

#[derive(clap::Parser)]
#[command(arg_required_else_help = true)]
pub struct StakeCliCommand {
    pub key: Option<String>,
    #[command(subcommand)]
    pub sub_command: Option<StakeCliSubCommand>,
}

#[derive(clap::Subcommand, Clone)]
pub enum StakeCliSubCommand {
    /// Lists all stake the provided key has given to other accounts.
    Given {
        /// The saved key name or ss58 valid address.
        key: String,
    },
    /// Lists all stake received by the provided key.
    Received {
        /// The saved key name or ss58 valid address.
        key: String,
    },
    /// Stakes the target key by amount.
    Add {
        /// The saved key name or ss58 valid address.
        key: String,
        /// The saved key name or ss58 valid address that will receive the stake.
        target: String,
        /// The amount to be staked.
        amount: u128,
    },
    /// Removes stake from the given target.
    Remove {
        /// The saved key name or ss58 valid address that is staking the target.
        key: String,
        /// The saved key name or ss58 valid address that will lose the stake.
        target: String,
        /// The amount to be removed.
        amount: u128,
    },
    /// Transfers stake from the source key to the target key.
    Transfer {
        /// The saved key name or ss58 valid address that is staking the source.
        key: String,
        /// The saved key name or ss58 valid address that will lose stake.
        source: String,
        /// The saved key name or ss58 valid address that will receive stake.
        target: String,
        /// The amount to be transfered.
        amount: u128,
    },
}

#[derive(tabled::Tabled)]
struct StakingEntry {
    target: String,
    amount: u128,
}

pub async fn given(ctx: &CliCtx, key: String) -> anyhow::Result<()> {
    let account = get_account(&key)?;

    let staking = if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .storage()
            .staking_to_iter1(&account)
            .await?
            .try_collect::<Vec<_>>()
            .await?
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .storage()
            .staking_to_iter1(&account)
            .await?
            .try_collect::<Vec<_>>()
            .await?
    };

    let staking = staking
        .into_iter()
        .map(|((_, target), amount)| StakingEntry {
            target: target.to_string(),
            amount,
        });

    let table = Table::new(staking);

    println!("{table}");

    Ok(())
}

#[derive(tabled::Tabled)]
struct StakedEntry {
    source: String,
    amount: u128,
}

pub async fn received(ctx: &CliCtx, key: String) -> anyhow::Result<()> {
    let account = get_account(&key)?;

    let staked = if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .storage()
            .staked_by_iter1(&account)
            .await?
            .try_collect::<Vec<_>>()
            .await?
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .storage()
            .staked_by_iter1(&account)
            .await?
            .try_collect::<Vec<_>>()
            .await?
    };

    let staked = staked.into_iter().map(|((_, source), amount)| StakedEntry {
        source: source.to_string(),
        amount,
    });

    let table = Table::new(staked);

    println!("{table}");

    Ok(())
}

pub async fn add(ctx: &CliCtx, key: String, target: String, amount: u128) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let target = AccountId32::from_str(&target)?;

    ctx.confirm(&format!("add {amount} stake to {target}"))?;

    println!("Staking...");

    if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .calls()
            .add_stake_wait(target, amount, keypair)
            .await?
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .calls()
            .add_stake_wait(target, amount, keypair)
            .await?
    };

    println!("Staked successfully!");

    Ok(())
}

pub async fn remove(ctx: &CliCtx, key: String, target: String, amount: u128) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let target = AccountId32::from_str(&target)?;

    ctx.confirm(&format!("remove {amount} stake from {target}"))?;

    println!("Unstaking...");

    if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .calls()
            .remove_stake_wait(target, amount, keypair)
            .await?
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .calls()
            .remove_stake_wait(target, amount, keypair)
            .await?
    };

    println!("Unstaked successfully!");

    Ok(())
}

pub async fn transfer(
    ctx: &CliCtx,
    key: String,
    source: String,
    target: String,
    amount: u128,
) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let source = AccountId32::from_str(&source)?;
    let target = AccountId32::from_str(&target)?;

    ctx.confirm(&format!(
        "transfer {amount} stake from {source} to {target}"
    ))?;

    println!("Transfering stake...");

    if ctx.is_testnet() {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .calls()
            .transfer_stake_wait(source, target, amount, keypair)
            .await?
    } else {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .calls()
            .transfer_stake_wait(source, target, amount, keypair)
            .await?
    };

    println!("Stake transfered successfully!");

    Ok(())
}
