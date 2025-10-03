// mod agent;
// mod balance;
// mod key;
// mod namespace;
// mod stake;

use clap::Parser;
use inquire::{Password, PasswordDisplayMode};

use crate::{
    action::{
        agent::{AgentInfoAction, RegisterAgentAction, UnregisterAgentAction},
        balance::*,
        key::{CreateKeyAction, DeleteKeyAction, KeyInfoAction, ListKeysAction},
        namespace::{NamespaceInfoAction, RegisterNamespaceAction, UnregisterNamespaceAction},
        stake::{
            AddStakeAction, GivenStakeAction, ReceivedStakeAction, RemoveStakeAction,
            TransferStakeAction,
        },
        Action, ActionContext,
    },
    keypair::Keypair,
    store::decrypt_key,
    util::format_torus,
};

pub(super) async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let ctx = CliActionContext::new(&cli);

    match cli.command {
        CliSubCommand::Agent(command) => match command.sub_command {
            Some(AgentCliSubCommand::Info { account }) => {
                execute::<AgentInfoAction>(&ctx, account).await?
            }
            Some(AgentCliSubCommand::Register {
                key,
                name,
                metadata,
                url,
            }) => execute::<RegisterAgentAction>(&ctx, (key, name, metadata, url)).await?,
            Some(AgentCliSubCommand::Unregister { key }) => {
                execute::<UnregisterAgentAction>(&ctx, key).await?
            }
            None if command.account.is_some() => {
                execute::<AgentInfoAction>(&ctx, command.account.unwrap()).await?
            }
            _ => unreachable!(),
        },
        CliSubCommand::Balance(command) => match command.sub_command {
            Some(BalanceCliSubCommand::Check { key }) => {
                execute::<CheckBalanceAction>(&ctx, key).await?
            }
            Some(BalanceCliSubCommand::Transfer {
                key,
                target,
                amount,
            }) => execute::<TransferBalanceAction>(&ctx, (key, target, amount)).await?,
            None if command.key.is_some() => {
                execute::<CheckBalanceAction>(&ctx, command.key.unwrap()).await?
            }
            _ => unreachable!(),
        },
        CliSubCommand::Key(command) => match command.sub_command {
            Some(KeyCliSubCommand::List) => execute::<ListKeysAction>(&ctx, ()).await?,
            Some(KeyCliSubCommand::Create {
                name,
                no_password,
                mnemonic,
            }) => execute::<CreateKeyAction>(&ctx, (name, no_password, mnemonic)).await?,
            Some(KeyCliSubCommand::Delete { name }) => {
                execute::<DeleteKeyAction>(&ctx, name).await?
            }
            Some(KeyCliSubCommand::Info { name }) => execute::<KeyInfoAction>(&ctx, name).await?,
            None if command.key.is_some() => {
                execute::<KeyInfoAction>(&ctx, command.key.unwrap()).await?
            }
            _ => unreachable!(),
        },
        CliSubCommand::Namespace(command) => match command.sub_command {
            Some(NamespaceCliSubCommand::Info { account }) => {
                execute::<NamespaceInfoAction>(&ctx, account).await?;
            }
            Some(NamespaceCliSubCommand::Register { key, path }) => {
                execute::<RegisterNamespaceAction>(&ctx, (key, path)).await?
            }
            Some(NamespaceCliSubCommand::Unregister { key, path }) => {
                execute::<UnregisterNamespaceAction>(&ctx, (key, path)).await?
            }
            None if command.account.is_some() => {
                execute::<NamespaceInfoAction>(&ctx, command.account.unwrap()).await?
            }
            _ => unreachable!(),
        },
        CliSubCommand::Stake(command) => match command.sub_command {
            Some(StakeCliSubCommand::Given { key }) => {
                execute::<GivenStakeAction>(&ctx, key).await?
            }
            Some(StakeCliSubCommand::Received { key }) => {
                execute::<ReceivedStakeAction>(&ctx, key).await?
            }
            Some(StakeCliSubCommand::Add {
                key,
                target,
                amount,
            }) => execute::<AddStakeAction>(&ctx, (key, target, amount)).await?,
            Some(StakeCliSubCommand::Remove {
                key,
                target,
                amount,
            }) => execute::<RemoveStakeAction>(&ctx, (key, target, amount)).await?,
            Some(StakeCliSubCommand::Transfer {
                key,
                source,
                target,
                amount,
            }) => execute::<TransferStakeAction>(&ctx, (key, source, target, amount)).await?,
            None => todo!(),
        },
    };

    Ok(())
}

async fn execute<A: Action>(ctx: &CliActionContext, params: A::Params) -> anyhow::Result<()> {
    let result: anyhow::Result<()> = async {
        let action = A::create(&ctx, params).await?;

        if ctx.is_dry_run() {
            let fee = action.estimate_fee(&ctx).await?;

            if ctx.is_json() {
                let serialized = serde_json::to_string(&DryRunResponse { fee })?;
                println!("{serialized}");
            } else {
                println!("{}", format_torus(fee));
            }

            return Ok(());
        }

        if !ctx.is_json() {
            if let Some(desc) = action.confirmation_phrase(&ctx).await? {
                ctx.confirm(&desc)?;
            }
        }

        let res = action.execute(&ctx).await?;

        if ctx.is_json() {
            let serialized = serde_json::to_string(&JsonResponse::Success { data: res })?;
            println!("{serialized}");
        } else {
            let string = res.to_string();
            println!("{string}");
        }

        Ok(())
    }
    .await;

    if let Err(err) = result {
        if ctx.is_json {
            let serialized = serde_json::to_string(&JsonResponse::<()>::Failure {
                message: err.to_string(),
            })?;
            println!("{serialized}");
        } else {
            println!("{err}");
        }

        return Ok(());
    }

    Ok(())
}
pub struct CliActionContext {
    is_dry_run: bool,
    is_json: bool,
    is_testnet: bool,
    skip_confirmation: bool,
}

impl CliActionContext {
    pub fn new(cli: &Cli) -> Self {
        let is_dry_run = cli.dry_run || std::env::var("TORURS_DRY_RUN").is_ok();
        let is_json = cli.json || std::env::var("TORURS_JSON").is_ok();
        let is_testnet = cli.testnet || std::env::var("TORURS_TESTNET").is_ok();
        let skip_confirmation = cli.yes || std::env::var("TORURS_SKIP_CONFIRMATION").is_ok();

        Self {
            is_dry_run,
            is_json,
            is_testnet,
            skip_confirmation,
        }
    }
}

impl ActionContext for &CliActionContext {
    fn is_json(&self) -> bool {
        self.is_json
    }

    fn is_testnet(&self) -> bool {
        self.is_testnet
    }

    fn is_dry_run(&self) -> bool {
        self.is_dry_run
    }

    fn confirm(&self, desc: &str) -> anyhow::Result<()> {
        if self.skip_confirmation {
            return Ok(());
        }

        let res = Password::new(desc)
            .without_confirmation()
            .with_display_mode(PasswordDisplayMode::Full)
            .prompt()?;

        if !res.to_lowercase().starts_with("y") {
            return Err(anyhow::anyhow!("Operation cancelled."));
        }

        Ok(())
    }

    fn decrypt(
        &self,
        key: &crate::store::Key,
    ) -> anyhow::Result<(crate::store::Key, crate::keypair::Keypair)> {
        if !key.encrypted {
            return Ok((key.clone(), Keypair::from_key(key.clone())?));
        }

        let mut key = key.clone();

        let password = Password::new("Password: ")
            .without_confirmation()
            .prompt()?;

        decrypt_key(&mut key, &password)?;

        Ok((key.clone(), Keypair::from_key(key)?))
    }

    fn info(&self, message: impl AsRef<str>) {
        if self.is_json {
            return;
        }

        println!("{}", message.as_ref());
    }
}

#[derive(clap::Parser)]
#[command(name = "torurs", version, about)]
pub struct Cli {
    #[arg(short, long, default_value_t = false)]
    pub testnet: bool,

    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    #[arg(long, default_value_t = false)]
    pub json: bool,

    #[arg(short, long, default_value_t = false)]
    pub yes: bool,

    #[command(subcommand)]
    pub command: CliSubCommand,
}

#[derive(clap::Subcommand)]
pub enum CliSubCommand {
    /// Commands related to agents.
    Agent(AgentCliCommand),
    /// Commands related to balance.
    Balance(BalanceCliCommand),
    /// Commands related to saved keys.
    Key(KeyCliCommand),
    /// Commands related to namespaces.
    Namespace(NamespaceCliCommand),
    /// Commands related to stake.
    Stake(StakeCliCommand),
}

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
    /// Unregisters an agent.
    Unregister {
        /// The saved key name that will be unregistered as an agent.
        key: String,
    },
}

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

#[derive(clap::Parser)]
#[command(arg_required_else_help = true)]
pub struct KeyCliCommand {
    pub key: Option<String>,

    #[command(subcommand)]
    pub sub_command: Option<KeyCliSubCommand>,
}

#[derive(clap::Subcommand)]
pub enum KeyCliSubCommand {
    /// Lists all saved keys.
    List,
    /// Creates a new random key and saves it encrypted with the given password.
    /// Tip: Use --mnemonic to create the key from a mnemonic instead of a random seed.
    Create {
        /// The name the key will be save on the disk.
        name: String,

        /// Disables the need of a password.
        /// This also means that the key will not be encrypted.
        #[arg(short, long)]
        no_password: bool,

        /// Allows for regenerating a key from a mnemonic.
        #[arg(short, long)]
        mnemonic: bool,
    },
    /// Deletes a saved key.
    Delete { name: String },
    /// Prints all information about a saved key.
    Info { name: String },
}

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
    /// Registers a new namespace.
    Unregister {
        /// The saved key name of the agent that will lose the namespace.
        key: String,
        /// The namespace path to be unregistered.
        path: String,
    },
}

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

#[derive(serde::Serialize)]
#[serde(tag = "status")]
pub enum JsonResponse<T> {
    Success { data: T },
    Failure { message: String },
}

#[derive(serde::Serialize)]
pub struct DryRunResponse {
    fee: u128,
}
