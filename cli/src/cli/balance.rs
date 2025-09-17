use std::{path::Path, str::FromStr};

use torus_client::{
    client::TorusClient,
    subxt::{
        ext::futures::TryStreamExt,
        utils::{AccountId32, MultiAddress},
    },
};

use crate::{cli::CliCtx, store::get_key};

#[derive(clap::Subcommand)]
pub enum BalanceCliCommand {
    Free {
        key: String,
    },
    Staking {
        key: String,
    },
    Staked {
        key: String,
    },
    Show {
        key: String,
    },
    Stake {
        key: String,
        target: String,
        amount: u128,
    },
    Unstake {
        key: String,
        target: String,
        amount: u128,
    },
    Transfer {
        key: String,
        target: String,
        amount: u128,
    },
    TransferStake {
        key: String,
        source: String,
        target: String,
        amount: u128,
    },
}

pub async fn free_balance(ctx: &CliCtx, key: String) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let account = keypair.account();

    let free_balance = if ctx.is_testnet() {
        let client = TorusClient::for_mainnet().await?;
        client
            .balances()
            .storage()
            .account_get(&account)
            .await?
            .map(|data| data.free)
    } else {
        let client = TorusClient::for_testnet().await?;
        client
            .balances()
            .storage()
            .account_get(&account)
            .await?
            .map(|data| data.free)
    };

    println!(
        "\"{}\"'s free balance: {}",
        Path::new(&key.path)
            .file_name()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or(key.path),
        free_balance.unwrap_or(0)
    );

    Ok(())
}

pub async fn staking_balance(ctx: &CliCtx, key: String) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let account = keypair.account();

    let staked_balance = if ctx.is_testnet() {
        let client = TorusClient::for_mainnet().await?;
        client
            .balances()
            .storage()
            .account_get(&account)
            .await?
            .map(|data| data.reserved)
    } else {
        let client = TorusClient::for_testnet().await?;
        client
            .balances()
            .storage()
            .account_get(&account)
            .await?
            .map(|data| data.reserved)
    };

    println!(
        "\"{}\"'s staking balance: {}",
        Path::new(&key.path)
            .file_name()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or(key.path),
        staked_balance.unwrap_or(0)
    );

    Ok(())
}

pub async fn staked_balance(ctx: &CliCtx, key: String) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let account = keypair.account();

    let staked_balance = if ctx.is_testnet() {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .storage()
            .staked_by_iter1(&account)
            .await?
            .try_fold(0u128, async move |a, (_, amount)| Ok(a + amount))
            .await?
    } else {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .storage()
            .staked_by_iter1(&account)
            .await?
            .try_fold(0u128, async move |a, (_, amount)| Ok(a + amount))
            .await?
    };

    println!(
        "\"{}\"'s staked balance: {}",
        Path::new(&key.path)
            .file_name()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or(key.path),
        staked_balance
    );

    Ok(())
}

pub async fn show_balance(ctx: &CliCtx, key: String) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let account = keypair.account();

    let full_balance = if ctx.is_testnet() {
        let client = TorusClient::for_mainnet().await?;
        client
            .balances()
            .storage()
            .account_get(&account)
            .await?
            .map(|data| data.reserved + data.free + data.frozen)
    } else {
        let client = TorusClient::for_testnet().await?;
        client
            .balances()
            .storage()
            .account_get(&account)
            .await?
            .map(|data| data.reserved + data.free + data.frozen)
    };

    println!(
        "\"{}\"'s full balance: {}",
        Path::new(&key.path)
            .file_name()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or(key.path),
        full_balance.unwrap_or(0)
    );

    Ok(())
}

pub async fn transfer_balance(
    ctx: &CliCtx,
    key: String,
    target: String,
    amount: u128,
) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let target = AccountId32::from_str(&target)?;

    println!("Transfering...");

    if ctx.is_testnet() {
        let client = TorusClient::for_mainnet().await?;
        client
            .balances()
            .calls()
            .transfer_keep_alive_wait(MultiAddress::Id(target), amount, keypair)
            .await?
    } else {
        let client = TorusClient::for_testnet().await?;
        client
            .balances()
            .calls()
            .transfer_keep_alive_wait(MultiAddress::Id(target), amount, keypair)
            .await?
    };

    println!("Transfered successfully!");

    Ok(())
}

pub async fn stake_balance(
    ctx: &CliCtx,
    key: String,
    target: String,
    amount: u128,
) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let target = AccountId32::from_str(&target)?;

    println!("Staking...");

    if ctx.is_testnet() {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .calls()
            .add_stake_wait(target, amount, keypair)
            .await?
    } else {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .calls()
            .add_stake_wait(target, amount, keypair)
            .await?
    };

    println!("Staked successfully!");

    Ok(())
}

pub async fn transfer_staked_balance(
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

    println!("Transfering stake...");

    if ctx.is_testnet() {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .calls()
            .transfer_stake_wait(source, target, amount, keypair)
            .await?
    } else {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .calls()
            .add_stake_wait(target, amount, keypair)
            .await?
    };

    println!("Stake transfered successfully!");

    Ok(())
}

pub async fn unstake_balance(
    ctx: &CliCtx,
    key: String,
    target: String,
    amount: u128,
) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    let (_key, keypair) = ctx.decrypt(&key)?;

    let target = AccountId32::from_str(&target)?;

    println!("Unstaking...");

    if ctx.is_testnet() {
        let client = TorusClient::for_mainnet().await?;
        client
            .torus0()
            .calls()
            .remove_stake_wait(target, amount, keypair)
            .await?
    } else {
        let client = TorusClient::for_testnet().await?;
        client
            .torus0()
            .calls()
            .remove_stake_wait(target, amount, keypair)
            .await?
    };

    println!("Unstaked successfully!");

    Ok(())
}
