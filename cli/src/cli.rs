use clap::{ArgGroup, Parser};
use inquire::{
    ui::{RenderConfig, Styled},
    Password, PasswordDisplayMode,
};

use crate::{
    action::{
        agent::{AgentInfoAction, RegisterAgentAction, UnregisterAgentAction, UpdateAgentAction},
        application::{ApplicationCreateAction, ApplicationListAction},
        balance::*,
        key::{CreateKeyAction, DeleteKeyAction, KeyInfoAction, ListKeysAction},
        namespace::{NamespaceInfoAction, RegisterNamespaceAction, UnregisterNamespaceAction},
        network::{PrintNetworkInfoAction, PrintNetworkSupplyAction, PrintTreasuryAddressAction},
        permission::{
            ExecutePermissionAction, RevokePermissionAction, SetPermissionAccumulationAction,
            SetPermissionEnforcementAuthorityAction,
        },
        proposal::{AddVoteAction, CreateProposalAction, Proposal, RemoveVoteAction},
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
    let mut ctx = CliActionContext::new(&cli);

    match cli.command {
        CliSubCommand::Agent(command) => match command.sub_command {
            Some(AgentCliSubCommand::Info { account }) => {
                execute::<AgentInfoAction>(&mut ctx, account).await?
            }
            Some(AgentCliSubCommand::Register {
                key,
                name,
                metadata,
                url,
            }) => execute::<RegisterAgentAction>(&mut ctx, (key, name, metadata, url)).await?,
            Some(AgentCliSubCommand::Unregister { key }) => {
                execute::<UnregisterAgentAction>(&mut ctx, key).await?
            }
            Some(AgentCliSubCommand::Update {
                key,
                url,
                metadata,
                staking_fee,
                weight_control_fee,
            }) => {
                execute::<UpdateAgentAction>(
                    &mut ctx,
                    (key, url, metadata, staking_fee, weight_control_fee),
                )
                .await?
            }

            None if command.account.is_some() => {
                execute::<AgentInfoAction>(&mut ctx, command.account.unwrap()).await?
            }
            _ => unreachable!(),
        },
        CliSubCommand::Balance(command) => match command.sub_command {
            Some(BalanceCliSubCommand::Check { key }) => {
                execute::<CheckBalanceAction>(&mut ctx, key).await?
            }
            Some(BalanceCliSubCommand::Transfer {
                key,
                target,
                amount,
            }) => execute::<TransferBalanceAction>(&mut ctx, (key, target, amount)).await?,
            None if command.key.is_some() => {
                execute::<CheckBalanceAction>(&mut ctx, command.key.unwrap()).await?
            }
            _ => unreachable!(),
        },
        CliSubCommand::Key(command) => match command.sub_command {
            Some(KeyCliSubCommand::List) => execute::<ListKeysAction>(&mut ctx, ()).await?,
            Some(KeyCliSubCommand::Create {
                name,
                no_password,
                mnemonic,
            }) => execute::<CreateKeyAction>(&mut ctx, (name, no_password, mnemonic)).await?,
            Some(KeyCliSubCommand::Delete { name }) => {
                execute::<DeleteKeyAction>(&mut ctx, name).await?
            }
            Some(KeyCliSubCommand::Info { name }) => {
                execute::<KeyInfoAction>(&mut ctx, name).await?
            }
            None if command.key.is_some() => {
                execute::<KeyInfoAction>(&mut ctx, command.key.unwrap()).await?
            }
            _ => unreachable!(),
        },
        CliSubCommand::Namespace(command) => match command.sub_command {
            Some(NamespaceCliSubCommand::Info { account }) => {
                execute::<NamespaceInfoAction>(&mut ctx, account).await?;
            }
            Some(NamespaceCliSubCommand::Register { key, path }) => {
                execute::<RegisterNamespaceAction>(&mut ctx, (key, path)).await?
            }
            Some(NamespaceCliSubCommand::Unregister { key, path }) => {
                execute::<UnregisterNamespaceAction>(&mut ctx, (key, path)).await?
            }
            None if command.account.is_some() => {
                execute::<NamespaceInfoAction>(&mut ctx, command.account.unwrap()).await?
            }
            _ => unreachable!(),
        },
        CliSubCommand::Stake(command) => match command.sub_command {
            Some(StakeCliSubCommand::Given { key }) => {
                execute::<GivenStakeAction>(&mut ctx, key).await?
            }
            Some(StakeCliSubCommand::Received { key }) => {
                execute::<ReceivedStakeAction>(&mut ctx, key).await?
            }
            Some(StakeCliSubCommand::Add {
                key,
                target,
                amount,
            }) => execute::<AddStakeAction>(&mut ctx, (key, target, amount)).await?,
            Some(StakeCliSubCommand::Remove {
                key,
                target,
                amount,
            }) => execute::<RemoveStakeAction>(&mut ctx, (key, target, amount)).await?,
            Some(StakeCliSubCommand::Transfer {
                key,
                source,
                target,
                amount,
            }) => execute::<TransferStakeAction>(&mut ctx, (key, source, target, amount)).await?,
            None if command.key.is_some() => {
                execute::<GivenStakeAction>(&mut ctx, command.key.unwrap()).await?
            }
            _ => unreachable!(),
        },
        CliSubCommand::Application(application_cli_command) => {
            match application_cli_command.sub_command {
                ApplicationCliSubCommand::List { page, elements } => {
                    execute::<ApplicationListAction>(
                        &mut ctx,
                        (page.unwrap_or(0), elements.unwrap_or(10)),
                    )
                    .await?
                }
                ApplicationCliSubCommand::Create {
                    key,
                    metadata,
                    remove,
                } => execute::<ApplicationCreateAction>(&mut ctx, (key, metadata, remove)).await?,
            }
        }
        CliSubCommand::Network(network_cli_command) => match network_cli_command.sub_command {
            NetworkCliSubCommand::Info => execute::<PrintNetworkInfoAction>(&mut ctx, ()).await?,
            NetworkCliSubCommand::Supply => {
                execute::<PrintNetworkSupplyAction>(&mut ctx, ()).await?
            }
            NetworkCliSubCommand::Treasury => {
                execute::<PrintTreasuryAddressAction>(&mut ctx, ()).await?
            }
        },
        CliSubCommand::Permission(permission_cli_command) => {
            match permission_cli_command.sub_command {
                PermissionCliSubCommand::Revoke { key, permission_id } => {
                    execute::<RevokePermissionAction>(&mut ctx, (key, permission_id)).await?
                }
                PermissionCliSubCommand::Execute {
                    key,
                    permission_id,
                    enforce,
                } => {
                    execute::<ExecutePermissionAction>(&mut ctx, (key, permission_id, enforce))
                        .await?
                }
                PermissionCliSubCommand::Accumulation {
                    key,
                    permission_id,
                    accumulating,
                } => {
                    execute::<SetPermissionAccumulationAction>(
                        &mut ctx,
                        (key, permission_id, accumulating),
                    )
                    .await?
                }
                PermissionCliSubCommand::EnforcementAuthority { sub_command } => {
                    match sub_command {
                        PermissionEnforcementAuthorityCliSubCommand::Remove {
                            key,
                            permission_id,
                        } => {
                            execute::<SetPermissionEnforcementAuthorityAction>(
                                &mut ctx,
                                (key, permission_id, None),
                            )
                            .await?
                        }
                        PermissionEnforcementAuthorityCliSubCommand::Add {
                            key,
                            permission_id,
                            votes,
                            members,
                        } => {
                            execute::<SetPermissionEnforcementAuthorityAction>(
                                &mut ctx,
                                (key, permission_id, Some((members, votes))),
                            )
                            .await?
                        }
                    }
                }
            }
        }
        CliSubCommand::Proposal(proposal_cli_command) => match proposal_cli_command.sub_command {
            ProposalCliSubCommand::Add { sub_command } => match sub_command {
                ProposalAddCliSubCommand::Emission {
                    key,
                    data,
                    params:
                        EmissionParams {
                            recycling_percentage,
                            treasury_percentage,
                            incentives_ratio,
                        },
                } => {
                    execute::<CreateProposalAction>(
                        &mut ctx,
                        (
                            key,
                            Proposal::Emission {
                                data,
                                recycling_percentage,
                                treasury_percentage,
                                incentives_ratio,
                            },
                        ),
                    )
                    .await?
                }
                ProposalAddCliSubCommand::GlobalParams {
                    key,
                    data,
                    params:
                        GlobalParamsParams {
                            min_name_length,
                            max_name_length,
                            min_weight_control_fee,
                            min_staking_fee,
                            dividends_participation_weight,
                            deposit_per_byte,
                            base_fee,
                            count_midpoint,
                            fee_steepness,
                            max_fee_multiplier,
                            proposal_cost,
                        },
                } => {
                    execute::<CreateProposalAction>(
                        &mut ctx,
                        (
                            key,
                            Proposal::GlobalParams {
                                data,
                                min_name_length,
                                max_name_length,
                                min_weight_control_fee,
                                min_staking_fee,
                                dividends_participation_weight,
                                deposit_per_byte,
                                base_fee,
                                count_midpoint,
                                fee_steepness,
                                max_fee_multiplier,
                                proposal_cost,
                            },
                        ),
                    )
                    .await?
                }
                ProposalAddCliSubCommand::GlobalCustom { key, data } => {
                    execute::<CreateProposalAction>(
                        &mut ctx,
                        (key, Proposal::GlobalCustom { data }),
                    )
                    .await?
                }
                ProposalAddCliSubCommand::TreasuryTransfer {
                    key,
                    data,
                    destination,
                    value,
                } => {
                    execute::<CreateProposalAction>(
                        &mut ctx,
                        (
                            key,
                            Proposal::TreasuryTransfer {
                                value,
                                destination,
                                data,
                            },
                        ),
                    )
                    .await?
                }
            },
            ProposalCliSubCommand::Vote { sub_command } => match sub_command {
                ProposalVoteCliSubCommand::Add {
                    key,
                    proposal_id,
                    agree,
                } => execute::<AddVoteAction>(&mut ctx, (key, proposal_id, agree)).await?,
                ProposalVoteCliSubCommand::Remove { key, proposal_id } => {
                    execute::<RemoveVoteAction>(&mut ctx, (key, proposal_id)).await?
                }
            },
        },
    };

    Ok(())
}

async fn execute<A: Action>(
    mut ctx: &mut CliActionContext,
    params: A::Params,
) -> anyhow::Result<()> {
    let result: anyhow::Result<()> = async {
        let action = A::create(&mut ctx, params).await?;

        if ctx.is_dry_run() {
            let fee = action.estimate_fee(&mut ctx).await?;

            if ctx.is_json() {
                let serialized = serde_json::to_string(&DryRunResponse { fee })?;
                println!("{serialized}");
            } else {
                println!("{}", format_torus(fee));
            }

            return Ok(());
        }

        if !ctx.is_json() {
            if let Some(changes) = action.get_changes(&mut ctx).await? {
                let desc = changes
                    .changes
                    .iter()
                    .map(|desc| format!(" - {desc}"))
                    .collect::<Vec<_>>()
                    .join("\n");
                let fee = if let Some(fee) = changes.fee {
                    format!(" (pays {} fee)\n", format_torus(fee))
                } else {
                    "".to_string()
                };
                let str =
                    format!("This command will: \n{desc}\n{fee} Do you want to continue? [y/N]");

                ctx.confirm(&str)?;
            }
        }

        let res = action.execute(&mut ctx).await?;

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

impl ActionContext for &mut CliActionContext {
    fn is_json(&self) -> bool {
        self.is_json
    }

    fn is_testnet(&self) -> bool {
        self.is_testnet
    }

    fn is_dry_run(&self) -> bool {
        self.is_dry_run
    }

    fn confirm(&mut self, desc: &str) -> anyhow::Result<()> {
        if self.skip_confirmation {
            return Ok(());
        }

        let res = Password::new(desc)
            .without_confirmation()
            .with_display_mode(PasswordDisplayMode::Full)
            .with_render_config(
                RenderConfig::empty()
                    .with_scroll_up_prefix(Styled::default())
                    .with_scroll_down_prefix(Styled::default())
                    .with_answered_prompt_prefix(Styled::default())
                    .with_highlighted_option_prefix(Styled::default())
                    .with_prompt_prefix(Styled::default()),
            )
            .prompt()?;

        if !res.to_lowercase().starts_with("y") {
            return Err(anyhow::anyhow!("Operation cancelled."));
        }

        Ok(())
    }

    fn decrypt(
        &mut self,
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

    fn info(&mut self, message: impl AsRef<str>) {
        if self.is_json {
            return;
        }

        println!("{}", message.as_ref());
    }

    fn is_mainnet(&self) -> bool {
        !self.is_testnet
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
    /// Commands related to agent applications.
    Application(ApplicationCliCommand),
    /// Commands related to agents.
    Agent(AgentCliCommand),
    /// Commands related to balance.
    Balance(BalanceCliCommand),
    /// Commands related to saved keys.
    Key(KeyCliCommand),
    /// Commands related to namespaces.
    Namespace(NamespaceCliCommand),
    /// Commands related to the network.
    Network(NetworkCliCommand),
    /// Commands Related to permissions.
    Permission(PermissionCliCommand),
    /// Commands Related to proposals.
    Proposal(ProposalCliCommand),
    /// Commands related to stake.
    Stake(StakeCliCommand),
}

#[derive(clap::Args)]
#[command(arg_required_else_help = true)]
pub struct ApplicationCliCommand {
    #[command(subcommand)]
    pub sub_command: ApplicationCliSubCommand,
}

#[derive(clap::Subcommand, Clone)]
pub enum ApplicationCliSubCommand {
    /// Prints the list of applications.
    List {
        #[arg(short, long)]
        page: Option<u32>,

        #[arg(short, long)]
        elements: Option<u32>,
    },
    /// Creates an agent application.
    Create {
        /// The saved key name that will become an agent.
        key: String,
        /// The metadata of the application.
        metadata: String,
        /// If it is an application to remove an agent.
        #[arg(short, long)]
        remove: bool,
    },
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
    /// Updates agent information.
    Update {
        /// The saved key name that will be updated.
        key: String,
        /// The new url.
        url: String,
        #[arg(long)]
        /// The new metadata.
        metadata: Option<String>,
        #[arg(long)]
        /// The new staking fee.
        staking_fee: Option<u32>,
        #[arg(long)]
        /// The new weight control fee.
        weight_control_fee: Option<u32>,
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

#[derive(clap::Args)]
#[command(arg_required_else_help = true)]
pub struct NetworkCliCommand {
    #[command(subcommand)]
    pub sub_command: NetworkCliSubCommand,
}

#[derive(clap::Subcommand, Clone)]
pub enum NetworkCliSubCommand {
    /// Prints information about the network.
    Info,
    /// Prints the current supply of the network.
    Supply,
    /// Prints the treasury address of the network.
    Treasury,
}

#[derive(clap::Args)]
pub struct PermissionCliCommand {
    #[command(subcommand)]
    pub sub_command: PermissionCliSubCommand,
}

#[derive(clap::Subcommand, Clone)]
pub enum PermissionCliSubCommand {
    /// Revokes a permission.
    Revoke { key: String, permission_id: String },
    /// Executes a permission.
    Execute {
        key: String,
        permission_id: String,
        #[arg(short, long)]
        enforce: bool,
    },
    /// Enables or disables the accumulation for the given permission.
    Accumulation {
        key: String,
        permission_id: String,
        accumulating: bool,
    },
    /// Modifies the enforcement authority of the given permission.
    EnforcementAuthority {
        #[command(subcommand)]
        sub_command: PermissionEnforcementAuthorityCliSubCommand,
    },
}

#[derive(clap::Subcommand, Clone)]
pub enum PermissionEnforcementAuthorityCliSubCommand {
    /// Removes the current enforcement authority from the given permission.
    Remove { key: String, permission_id: String },
    /// Adds an enforcement authority to the given permission.
    Add {
        key: String,
        permission_id: String,
        #[arg(value_delimiter = ',')]
        /// The controllers of the enforcement authority.
        /// (Comma-separated addresses or key names).
        members: Vec<String>,
        /// The votes needed.
        votes: u32,
    },
}

#[derive(clap::Args)]
pub struct ProposalCliCommand {
    #[command(subcommand)]
    pub sub_command: ProposalCliSubCommand,
}

#[derive(clap::Subcommand, Clone)]
pub enum ProposalCliSubCommand {
    /// Adds a proposal.
    Add {
        #[command(subcommand)]
        sub_command: ProposalAddCliSubCommand,
    },
    /// Votes on a proposal.
    Vote {
        #[command(subcommand)]
        sub_command: ProposalVoteCliSubCommand,
    },
}

#[derive(clap::Args, Clone)]
#[command(group = ArgGroup::default().id("emission-params").required(true).multiple(true).args(&["recycling_percentage", "treasury_percentage", "incentives_ratio"]))]
pub struct EmissionParams {
    #[arg(long)]
    recycling_percentage: Option<u8>,
    #[arg(long)]
    treasury_percentage: Option<u8>,
    #[arg(long)]
    incentives_ratio: Option<u8>,
}

#[derive(clap::Args, Clone)]
#[command(group = ArgGroup::default().id("global-params").required(true).multiple(true)
.args(&["min_name_length", "max_name_length", "min_weight_control_fee", "min_staking_fee", "dividends_participation_weight", "deposit_per_byte", "base_fee", "count_midpoint", "fee_steepness", "max_fee_multiplier", "proposal_cost"]))]
pub struct GlobalParamsParams {
    #[arg(long)]
    min_name_length: Option<u16>,
    #[arg(long)]
    max_name_length: Option<u16>,
    #[arg(long)]
    min_weight_control_fee: Option<u8>,
    #[arg(long)]
    min_staking_fee: Option<u8>,
    #[arg(long)]
    dividends_participation_weight: Option<u8>,
    #[arg(long)]
    deposit_per_byte: Option<u128>,
    #[arg(long)]
    base_fee: Option<u128>,
    #[arg(long)]
    count_midpoint: Option<u32>,
    #[arg(long)]
    fee_steepness: Option<u8>,
    #[arg(long)]
    max_fee_multiplier: Option<u32>,
    #[arg(long)]
    proposal_cost: Option<u128>,
}

#[derive(clap::Subcommand, Clone)]
pub enum ProposalAddCliSubCommand {
    /// Creates a new emission proposal.
    Emission {
        key: String,
        data: String,
        #[command(flatten)]
        params: EmissionParams,
    },
    /// Creates a new global params proposal.
    GlobalParams {
        key: String,
        data: String,
        #[command(flatten)]
        params: GlobalParamsParams,
    },
    /// Creates a new global custom proposal.
    GlobalCustom { key: String, data: String },
    /// Creates a new treasury transfer proposal.
    TreasuryTransfer {
        key: String,
        data: String,
        destination: String,
        value: u128,
    },
}

#[derive(clap::Subcommand, Clone)]
pub enum ProposalVoteCliSubCommand {
    /// Votes on a proposal.
    Add {
        key: String,
        proposal_id: u64,
        agree: bool,
    },
    /// Removes a vote from a proposal.
    Remove { key: String, proposal_id: u64 },
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
