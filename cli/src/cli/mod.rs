use clap::Parser;

use balance::BalanceCliCommand;
use inquire::Password;
use key::KeyCliCommand;

use crate::{
    keypair::Keypair,
    store::{decrypt_key, Key},
};

mod balance;
mod key;

pub(super) async fn execute() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let (ctx, command) = cli.extract_context();

    match command {
        CliSubcommand::Balance { command } => match command {
            BalanceCliCommand::Free { key } => {
                balance::free_balance(&ctx, key).await?;
            }
            BalanceCliCommand::Staking { key } => balance::staking_balance(&ctx, key).await?,
            BalanceCliCommand::Staked { key } => balance::staked_balance(&ctx, key).await?,
            BalanceCliCommand::Show { key } => balance::show_balance(&ctx, key).await?,
            BalanceCliCommand::Stake {
                key,
                target,
                amount,
            } => balance::stake_balance(&ctx, key, target, amount).await?,
            BalanceCliCommand::Unstake {
                key,
                target,
                amount,
            } => balance::unstake_balance(&ctx, key, target, amount).await?,
            BalanceCliCommand::Transfer {
                key,
                target,
                amount,
            } => balance::transfer_balance(&ctx, key, target, amount).await?,
            BalanceCliCommand::TransferStake {
                key,
                source,
                target,
                amount,
            } => balance::transfer_staked_balance(&ctx, key, source, target, amount).await?,
        },
        CliSubcommand::Key { command } => match command {
            KeyCliCommand::List => key::list(&ctx)?,
            KeyCliCommand::Create { name, password } => key::create(&ctx, name, password)?,
            KeyCliCommand::Delete { name } => key::delete(&ctx, name)?,
            KeyCliCommand::Info { name } => key::info(&ctx, name)?,
        },
    }

    Ok(())
}

#[derive(clap::Parser)]
pub struct Cli {
    #[arg(short, long, default_value_t = 0)]
    pub debug: u8,

    #[arg(short, long, default_value_t = false)]
    pub testnet: bool,

    #[command(subcommand)]
    pub command: CliSubcommand,
}

impl Cli {
    pub fn extract_context(self) -> (CliCtx, CliSubcommand) {
        let is_testnet = self.testnet;

        (CliCtx::new(is_testnet), self.command)
    }
}

#[derive(clap::Subcommand)]
pub enum CliSubcommand {
    Balance {
        #[command(subcommand)]
        command: BalanceCliCommand,
    },
    Key {
        #[command(subcommand)]
        command: KeyCliCommand,
    },
}

pub struct CliCtx {
    testnet: bool,
}

impl CliCtx {
    pub fn new(is_testnet: bool) -> Self {
        Self {
            testnet: is_testnet,
        }
    }

    pub fn is_testnet(&self) -> bool {
        self.testnet
    }

    pub fn decrypt(&self, key: &Key) -> anyhow::Result<(Key, Keypair)> {
        if !key.encrypted {
            return Ok((key.clone(), Keypair::from_key(key.clone(), None)?));
        }

        let mut key = key.clone();

        let password = Password::new("Password: ")
            .without_confirmation()
            .prompt()?;

        decrypt_key(&mut key, &password)?;

        Ok((key.clone(), Keypair::from_key(key, Some(&password))?))
    }
}
