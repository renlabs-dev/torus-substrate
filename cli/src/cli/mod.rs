use clap::Parser;

use balance::BalanceCliCommand;
use inquire::Password;
use key::KeyCliCommand;

use crate::{
    cli::{
        key::KeyCliSubCommand,
        stake::{StakeCliCommand, StakeCliSubCommand},
    },
    keypair::Keypair,
    store::{decrypt_key, Key},
};

mod balance;
mod key;
mod stake;

pub(super) async fn execute() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let (ctx, command) = cli.extract_context();

    match command {
        CliSubCommand::Balance(command) => match command.sub_command {
            Some(balance::BalanceCliSubCommand::Check { key }) => balance::check(&ctx, key).await?,
            Some(balance::BalanceCliSubCommand::Transfer {
                key,
                target,
                amount,
            }) => balance::transfer(&ctx, key, target, amount).await?,
            None if command.key.is_some() => balance::check(&ctx, command.key.unwrap()).await?,
            _ => {}
        },
        CliSubCommand::Key(command) => match command.sub_command {
            KeyCliSubCommand::List => key::list(&ctx)?,
            KeyCliSubCommand::Create { name, password } => key::create(&ctx, name, password)?,
            KeyCliSubCommand::Delete { name } => key::delete(&ctx, name)?,
            KeyCliSubCommand::Info { name } => key::info(&ctx, name)?,
        },
        CliSubCommand::Stake(command) => match command.sub_command {
            Some(StakeCliSubCommand::Given { key }) => stake::given(&ctx, key).await?,
            Some(StakeCliSubCommand::Received { key }) => stake::received(&ctx, key).await?,
            Some(StakeCliSubCommand::Add {
                key,
                target,
                amount,
            }) => stake::add(&ctx, key, target, amount).await?,
            Some(StakeCliSubCommand::Remove {
                key,
                target,
                amount,
            }) => stake::remove(&ctx, key, target, amount).await?,
            Some(StakeCliSubCommand::Transfer {
                key,
                source,
                target,
                amount,
            }) => stake::transfer(&ctx, key, source, target, amount).await?,
            None if command.key.is_some() => stake::given(&ctx, command.key.unwrap()).await?,
            _ => {}
        },
    }

    Ok(())
}

#[derive(clap::Parser)]
#[command(name = "torurs", version, about)]
pub struct Cli {
    #[arg(short, long, default_value_t = false)]
    pub testnet: bool,

    #[arg(short, long, default_value_t = false)]
    pub yes: bool,

    #[command(subcommand)]
    pub command: CliSubCommand,
}

impl Cli {
    pub fn extract_context(self) -> (CliCtx, CliSubCommand) {
        let is_testnet = self.testnet;

        (CliCtx::new(is_testnet), self.command)
    }
}

#[derive(clap::Subcommand)]
pub enum CliSubCommand {
    Balance(BalanceCliCommand),
    Key(KeyCliCommand),
    Stake(StakeCliCommand),
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
