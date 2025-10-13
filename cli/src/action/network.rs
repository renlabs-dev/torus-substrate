use std::fmt::Display;

use tabled::Table;
use torus_client::{client::TorusClient, subxt::utils::AccountId32};

use crate::action::{Action, ActionContext};

pub struct PrintNetworkInfoAction;

impl Action for PrintNetworkInfoAction {
    type Params = ();
    type ResponseData = PrintNetworkInfoActionResponse;

    async fn create(_ctx: &mut impl ActionContext, _params: Self::Params) -> anyhow::Result<Self> {
        Ok(Self)
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        let global_params = fetch_global_params(ctx).await?;

        Ok(PrintNetworkInfoActionResponse { global_params })
    }
}

#[derive(serde::Serialize, tabled::Tabled)]
pub struct GlobalParams {
    pub last_block: u32,
    pub min_name_length: u16,
    pub max_name_length: u16,
    pub min_weight_control_fee: u8,
    pub min_staking_fee: u8,
    pub dividends_paticipation_weight: u8,
    pub deposit_per_byte: u128,
    pub base_fee: u128,
    pub count_midpoint: u32,
    pub fee_steepness: u8,
    pub max_fee_multiplier: u32,
    pub proposal_cost: u128,
    pub proposal_expiration: u64,
    pub agent_application_cost: u128,
    pub agent_application_expiration: u64,
    pub proposal_reward_treasury_allocation: u8,
    pub max_proposal_reward_treasury_allocation: u128,
    pub proposal_reward_interval: u64,
    pub recycling_percentage: u8,
}

#[derive(serde::Serialize)]
pub struct PrintNetworkInfoActionResponse {
    #[serde(flatten)]
    pub global_params: GlobalParams,
}

impl Display for PrintNetworkInfoActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = Table::kv(std::iter::once(&self.global_params));
        write!(f, "{table}")
    }
}

pub struct PrintNetworkSupplyAction;

impl Action for PrintNetworkSupplyAction {
    type Params = ();
    type ResponseData = PrintNetworkSupplyActionResponse;

    async fn create(_ctx: &mut impl ActionContext, _params: Self::Params) -> anyhow::Result<Self> {
        Ok(Self)
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        let (total_issuance, total_stake) = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            (
                client
                    .balances()
                    .storage()
                    .total_issuance()
                    .await?
                    .unwrap_or(0),
                client.torus0().storage().total_stake().await?.unwrap_or(0),
            )
        } else {
            let client = TorusClient::for_testnet().await?;
            (
                client
                    .balances()
                    .storage()
                    .total_issuance()
                    .await?
                    .unwrap_or(0),
                client.torus0().storage().total_stake().await?.unwrap_or(0),
            )
        };

        Ok(PrintNetworkSupplyActionResponse {
            total_issuance,
            total_stake,
        })
    }
}

#[derive(serde::Serialize, tabled::Tabled)]
pub struct PrintNetworkSupplyActionResponse {
    total_issuance: u128,
    total_stake: u128,
}

impl Display for PrintNetworkSupplyActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = Table::kv(std::iter::once(&self));
        write!(f, "{table}")
    }
}

pub struct PrintTreasuryAddressAction;

impl Action for PrintTreasuryAddressAction {
    type Params = ();
    type ResponseData = PrintTreasuryAddressActionResponse;

    async fn create(_ctx: &mut impl ActionContext, _params: Self::Params) -> anyhow::Result<Self> {
        Ok(Self)
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        let address = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client.governance().storage().dao_treasury_address().await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client.governance().storage().dao_treasury_address().await?
        }
        .ok_or(anyhow::anyhow!("No treasury address found!"))?;

        Ok(PrintTreasuryAddressActionResponse { address })
    }
}

#[derive(serde::Serialize)]
pub struct PrintTreasuryAddressActionResponse {
    address: AccountId32,
}

impl Display for PrintTreasuryAddressActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The treasury address is `{}`", self.address)
    }
}

pub async fn fetch_global_params(ctx: &mut impl ActionContext) -> anyhow::Result<GlobalParams> {
    let (
        last_block,
        min_name_length,
        max_name_length,
        dividends_paticipation_weight,
        min_weight_control_fee,
        min_staking_fee,
        deposit_per_byte,
        base_fee,
        count_midpoint,
        fee_steepness,
        max_fee_multiplier,
        proposal_cost,
        proposal_expiration,
        agent_application_cost,
        agent_application_expiration,
        proposal_reward_treasury_allocation,
        max_proposal_reward_treasury_allocation,
        proposal_reward_interval,
        recycling_percentage,
    ) = if ctx.is_mainnet() {
        let client = TorusClient::for_mainnet().await?;
        let torus0_storage = client.torus0().storage();
        let fees = torus0_storage.fee_constraints().await?;
        let namespace_pricing_config = torus0_storage.namespace_pricing_config().await?;
        let governance_storage = client.governance().storage();
        let governance_config = governance_storage.global_governance_config().await?;
        (
            client.latest_block().await?.number(),
            torus0_storage.min_name_length().await?.unwrap_or_default(),
            torus0_storage.max_name_length().await?.unwrap_or_default(),
            torus0_storage
                .dividends_participation_weight()
                .await?
                .map(|percent| percent.0)
                .unwrap_or_default(),
            fees.as_ref()
                .map(|fees| fees.min_weight_control_fee.0)
                .unwrap_or_default(),
            fees.map(|fees| fees.min_staking_fee.0).unwrap_or_default(),
            namespace_pricing_config
                .as_ref()
                .map(|cfg| cfg.deposit_per_byte)
                .unwrap_or_default(),
            namespace_pricing_config
                .as_ref()
                .map(|cfg| cfg.base_fee)
                .unwrap_or_default(),
            namespace_pricing_config
                .as_ref()
                .map(|cfg| cfg.count_midpoint)
                .unwrap_or_default(),
            namespace_pricing_config
                .as_ref()
                .map(|cfg| cfg.fee_steepness.0)
                .unwrap_or_default(),
            namespace_pricing_config
                .as_ref()
                .map(|cfg| cfg.max_fee_multiplier)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.proposal_cost)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.proposal_expiration)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.agent_application_cost)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.agent_application_expiration)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.proposal_reward_treasury_allocation.0)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.max_proposal_reward_treasury_allocation)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.proposal_reward_interval)
                .unwrap_or_default(),
            client
                .emission0()
                .storage()
                .emission_recycling_percentage()
                .await?
                .map(|perc| perc.0)
                .unwrap_or_default(),
        )
    } else {
        let client = TorusClient::for_testnet().await?;
        let torus0_storage = client.torus0().storage();
        let fees = torus0_storage.fee_constraints().await?;
        let namespace_pricing_config = torus0_storage.namespace_pricing_config().await?;
        let governance_storage = client.governance().storage();
        let governance_config = governance_storage.global_governance_config().await?;
        (
            client.latest_block().await?.number(),
            torus0_storage.min_name_length().await?.unwrap_or_default(),
            torus0_storage.max_name_length().await?.unwrap_or_default(),
            torus0_storage
                .dividends_participation_weight()
                .await?
                .map(|percent| percent.0)
                .unwrap_or_default(),
            fees.as_ref()
                .map(|fees| fees.min_weight_control_fee.0)
                .unwrap_or_default(),
            fees.map(|fees| fees.min_staking_fee.0).unwrap_or_default(),
            namespace_pricing_config
                .as_ref()
                .map(|cfg| cfg.deposit_per_byte)
                .unwrap_or_default(),
            namespace_pricing_config
                .as_ref()
                .map(|cfg| cfg.base_fee)
                .unwrap_or_default(),
            namespace_pricing_config
                .as_ref()
                .map(|cfg| cfg.count_midpoint)
                .unwrap_or_default(),
            namespace_pricing_config
                .as_ref()
                .map(|cfg| cfg.fee_steepness.0)
                .unwrap_or_default(),
            namespace_pricing_config
                .as_ref()
                .map(|cfg| cfg.max_fee_multiplier)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.proposal_cost)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.proposal_expiration)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.agent_application_cost)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.agent_application_expiration)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.proposal_reward_treasury_allocation.0)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.max_proposal_reward_treasury_allocation)
                .unwrap_or_default(),
            governance_config
                .as_ref()
                .map(|governace_config| governace_config.proposal_reward_interval)
                .unwrap_or_default(),
            client
                .emission0()
                .storage()
                .emission_recycling_percentage()
                .await?
                .map(|perc| perc.0)
                .unwrap_or_default(),
        )
    };

    Ok(GlobalParams {
        last_block,
        min_name_length,
        max_name_length,
        dividends_paticipation_weight,
        min_weight_control_fee,
        min_staking_fee,
        deposit_per_byte,
        base_fee,
        count_midpoint,
        fee_steepness,
        max_fee_multiplier,
        proposal_cost,
        proposal_expiration,
        agent_application_cost,
        agent_application_expiration,
        proposal_reward_treasury_allocation,
        max_proposal_reward_treasury_allocation,
        proposal_reward_interval,
        recycling_percentage,
    })
}
