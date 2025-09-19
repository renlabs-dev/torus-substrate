use clap::Parser;

use balance::BalanceCliCommand;
use inquire::{Password, PasswordDisplayMode};
use key::KeyCliCommand;

use crate::{
    cli::{
        agent::{AgentCliCommand, AgentCliSubCommand},
        key::KeyCliSubCommand,
        namespace::{NamespaceCliCommand, NamespaceCliSubCommand},
        stake::{StakeCliCommand, StakeCliSubCommand},
    },
    keypair::Keypair,
    store::{decrypt_key, Key},
};

mod agent;
mod balance;
mod key;
mod namespace;
mod stake;

pub(super) async fn execute() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let (ctx, command) = cli.extract_context();

    match command {
        CliSubCommand::Agent(command) => match command.sub_command {
            Some(AgentCliSubCommand::Info { account }) => agent::info(&ctx, account).await?,
            Some(AgentCliSubCommand::Register {
                key,
                name,
                metadata,
                url,
            }) => agent::register(&ctx, key, name, metadata, url).await?,
            None if command.account.is_some() => {
                agent::info(&ctx, command.account.unwrap()).await?
            }
            _ => {}
        },
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
            KeyCliSubCommand::Create {
                name,
                no_password,
                mnemonic,
            } => key::create(&ctx, name, no_password, mnemonic)?,
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
        CliSubCommand::Namespace(command) => match command.sub_command {
            Some(NamespaceCliSubCommand::Info { account }) => {
                namespace::info(&ctx, account).await?
            }
            Some(NamespaceCliSubCommand::Register { key, path }) => {
                namespace::register(&ctx, key, path).await?
            }
            None if command.account.is_some() => {
                namespace::info(&ctx, command.account.unwrap()).await?
            }
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
        let yes = self.yes;

        (CliCtx::new(is_testnet, yes), self.command)
    }
}

#[derive(clap::Subcommand)]
pub enum CliSubCommand {
    Agent(AgentCliCommand),
    Balance(BalanceCliCommand),
    Key(KeyCliCommand),
    Namespace(NamespaceCliCommand),
    Stake(StakeCliCommand),
}

pub struct CliCtx {
    testnet: bool,
    skip_confirmation: bool,
}

impl CliCtx {
    pub fn new(is_testnet: bool, skip_confirmation: bool) -> Self {
        Self {
            testnet: is_testnet,
            skip_confirmation,
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

    pub fn confirm(&self, action: &str) -> anyhow::Result<()> {
        if self.skip_confirmation {
            return Ok(());
        }

        let res = Password::new(&format!("Are you sure you want to {action}? [y/N] "))
            .without_confirmation()
            .with_display_mode(PasswordDisplayMode::Full)
            .prompt()?;

        if !res.to_lowercase().starts_with("y") {
            return Err(anyhow::anyhow!("not confirmed"));
        }

        Ok(())
    }
}
