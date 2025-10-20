use std::fmt::Display;

use torus_client::client::TorusClient;

use crate::{
    action::{network::fetch_global_params, Action, ActionContext},
    keypair::Keypair,
    store::{get_account, get_key},
    util::to_percent_u8,
};

pub enum Proposal {
    Emission {
        data: String,
        recycling_percentage: Option<u8>,
        treasury_percentage: Option<u8>,
        incentives_ratio: Option<u8>,
    },
    GlobalParams {
        data: String,
        min_name_length: Option<u16>,
        max_name_length: Option<u16>,
        min_weight_control_fee: Option<u8>,
        min_staking_fee: Option<u8>,
        dividends_participation_weight: Option<u8>,
        deposit_per_byte: Option<u128>,
        base_fee: Option<u128>,
        count_midpoint: Option<u32>,
        fee_steepness: Option<u8>,
        max_fee_multiplier: Option<u32>,
        proposal_cost: Option<u128>,
    },
    GlobalCustom {
        data: String,
    },
    TreasuryTransfer {
        value: u128,
        destination: String,
        data: String,
    },
}

pub struct CreateProposalAction {
    key: Keypair,
    proposal: Proposal,
}

impl Action for CreateProposalAction {
    type Params = (String, Proposal);
    type ResponseData = CreateProposalActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, proposal): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        Ok(Self {
            key: keypair,
            proposal,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            match &self.proposal {
                Proposal::Emission {
                    recycling_percentage,
                    treasury_percentage,
                    incentives_ratio,
                    data,
                } => {
                    let current_recycling_percentage = client
                        .emission0()
                        .storage()
                        .emission_recycling_percentage()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();
                    let current_treasury_percentage = client
                        .governance()
                        .storage()
                        .treasury_emission_fee()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();
                    let current_incentives_ratio = client
                        .emission0()
                        .storage()
                        .incentives_ratio()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();

                    client
                        .governance()
                        .calls()
                        .add_emission_proposal_wait(
                            torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(recycling_percentage.unwrap_or(current_recycling_percentage) as u32)?),
                            torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(treasury_percentage.unwrap_or(current_treasury_percentage) as u32)?),
                            torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(incentives_ratio.unwrap_or(current_incentives_ratio) as u32)?),
                            data.clone().as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?;
                }
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
                } => {
                    let params = fetch_global_params(ctx).await?;
                    client
                        .governance()
                        .calls()
                        .add_global_params_proposal_wait(
                            torus_client::interfaces::mainnet::api::runtime_types::pallet_governance::proposal::GlobalParamsData {
                                min_name_length: min_name_length.unwrap_or(params.min_name_length),
                                max_name_length: max_name_length.unwrap_or(params.max_name_length),
                                min_weight_control_fee: min_weight_control_fee.unwrap_or(params.min_weight_control_fee),
                                min_staking_fee: min_staking_fee.unwrap_or(params.min_staking_fee),
                                dividends_participation_weight: torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(dividends_participation_weight.unwrap_or(params.dividends_paticipation_weight) as u32)?),
                                namespace_pricing_config: torus_client::interfaces::mainnet::api::runtime_types::pallet_torus0::namespace::NamespacePricingConfig {
                                    deposit_per_byte: deposit_per_byte.unwrap_or(params.deposit_per_byte),
                                    base_fee: base_fee.unwrap_or(params.base_fee),
                                    count_midpoint: count_midpoint.unwrap_or(params.count_midpoint),
                                    fee_steepness: torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(fee_steepness.unwrap_or(params.fee_steepness) as u32)?),
                                    max_fee_multiplier: max_fee_multiplier.unwrap_or(params.max_fee_multiplier),
                                },
                                proposal_cost: proposal_cost.unwrap_or(params.proposal_cost)
                            },
                            data.as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?;
                }
                Proposal::GlobalCustom { data } => {
                    client
                        .governance()
                        .calls()
                        .add_global_custom_proposal_wait(data.as_bytes().to_vec(), self.key.clone())
                        .await?
                }
                Proposal::TreasuryTransfer {
                    value,
                    destination,
                    data,
                } => {
                    let destination_key = get_account(destination)?;

                    client
                        .governance()
                        .calls()
                        .add_dao_treasury_transfer_proposal_wait(
                            *value,
                            destination_key,
                            data.as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?;
                }
            }
        } else {
            let client = TorusClient::for_testnet().await?;
            match &self.proposal {
                Proposal::Emission {
                    recycling_percentage,
                    treasury_percentage,
                    incentives_ratio,
                    data,
                } => {
                    let current_recycling_percentage = client
                        .emission0()
                        .storage()
                        .emission_recycling_percentage()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();
                    let current_treasury_percentage = client
                        .governance()
                        .storage()
                        .treasury_emission_fee()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();
                    let current_incentives_ratio = client
                        .emission0()
                        .storage()
                        .incentives_ratio()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();

                    client
                        .governance()
                        .calls()
                        .add_emission_proposal_wait(
                            torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(recycling_percentage.unwrap_or(current_recycling_percentage) as u32)?),
                            torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(treasury_percentage.unwrap_or(current_treasury_percentage) as u32)?),
                            torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(incentives_ratio.unwrap_or(current_incentives_ratio) as u32)?),
                            data.clone().as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?;
                }
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
                } => {
                    let params = fetch_global_params(ctx).await?;
                    client
                        .governance()
                        .calls()
                        .add_global_params_proposal_wait(
                            torus_client::interfaces::testnet::api::runtime_types::pallet_governance::proposal::GlobalParamsData {
                                min_name_length: min_name_length.unwrap_or(params.min_name_length),
                                max_name_length: max_name_length.unwrap_or(params.max_name_length),
                                min_weight_control_fee: min_weight_control_fee.unwrap_or(params.min_weight_control_fee),
                                min_staking_fee: min_staking_fee.unwrap_or(params.min_staking_fee),
                                dividends_participation_weight: torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(dividends_participation_weight.unwrap_or(params.dividends_paticipation_weight) as u32)?),
                                namespace_pricing_config: torus_client::interfaces::testnet::api::runtime_types::pallet_torus0::namespace::NamespacePricingConfig {
                                    deposit_per_byte: deposit_per_byte.unwrap_or(params.deposit_per_byte),
                                    base_fee: base_fee.unwrap_or(params.base_fee),
                                    count_midpoint: count_midpoint.unwrap_or(params.count_midpoint),
                                    fee_steepness: torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(fee_steepness.unwrap_or(params.fee_steepness) as u32)?),
                                    max_fee_multiplier: max_fee_multiplier.unwrap_or(params.max_fee_multiplier),
                                },
                                proposal_cost: proposal_cost.unwrap_or(params.proposal_cost)
                            },
                            data.as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?
                }
                Proposal::GlobalCustom { data } => {
                    client
                        .governance()
                        .calls()
                        .add_global_custom_proposal_wait(data.as_bytes().to_vec(), self.key.clone())
                        .await?;
                }
                Proposal::TreasuryTransfer {
                    value,
                    destination,
                    data,
                } => {
                    let destination_key = get_account(destination)?;

                    client
                        .governance()
                        .calls()
                        .add_dao_treasury_transfer_proposal_wait(
                            *value,
                            destination_key,
                            data.as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?;
                }
            }
        }

        Ok(CreateProposalActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            match &self.proposal {
                Proposal::Emission {
                    recycling_percentage,
                    treasury_percentage,
                    incentives_ratio,
                    data,
                } => {
                    let current_recycling_percentage = client
                        .emission0()
                        .storage()
                        .emission_recycling_percentage()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();
                    let current_treasury_percentage = client
                        .governance()
                        .storage()
                        .treasury_emission_fee()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();
                    let current_incentives_ratio = client
                        .emission0()
                        .storage()
                        .incentives_ratio()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();

                    client
                        .governance()
                        .calls()
                        .add_emission_proposal_fee(
                            torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(recycling_percentage.unwrap_or(current_recycling_percentage) as u32)?),
                            torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(treasury_percentage.unwrap_or(current_treasury_percentage) as u32)?),
                            torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(incentives_ratio.unwrap_or(current_incentives_ratio) as u32)?),
                            data.clone().as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?
                }
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
                } => {
                    let params = fetch_global_params(ctx).await?;
                    client
                        .governance()
                        .calls()
                        .add_global_params_proposal_fee(
                            torus_client::interfaces::mainnet::api::runtime_types::pallet_governance::proposal::GlobalParamsData {
                                min_name_length: min_name_length.unwrap_or(params.min_name_length),
                                max_name_length: max_name_length.unwrap_or(params.max_name_length),
                                min_weight_control_fee: min_weight_control_fee.unwrap_or(params.min_weight_control_fee),
                                min_staking_fee: min_staking_fee.unwrap_or(params.min_staking_fee),
                                dividends_participation_weight: torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(dividends_participation_weight.unwrap_or(params.dividends_paticipation_weight) as u32)?),
                                namespace_pricing_config: torus_client::interfaces::mainnet::api::runtime_types::pallet_torus0::namespace::NamespacePricingConfig {
                                    deposit_per_byte: deposit_per_byte.unwrap_or(params.deposit_per_byte),
                                    base_fee: base_fee.unwrap_or(params.base_fee),
                                    count_midpoint: count_midpoint.unwrap_or(params.count_midpoint),
                                    fee_steepness: torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(fee_steepness.unwrap_or(params.fee_steepness) as u32)?),
                                    max_fee_multiplier: max_fee_multiplier.unwrap_or(params.max_fee_multiplier),
                                },
                                proposal_cost: proposal_cost.unwrap_or(params.proposal_cost)
                            },
                            data.as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?
                }
                Proposal::GlobalCustom { data } => {
                    client
                        .governance()
                        .calls()
                        .add_global_custom_proposal_fee(data.as_bytes().to_vec(), self.key.clone())
                        .await?
                }
                Proposal::TreasuryTransfer {
                    value,
                    destination,
                    data,
                } => {
                    let destination_key = get_account(destination)?;

                    client
                        .governance()
                        .calls()
                        .add_dao_treasury_transfer_proposal_fee(
                            *value,
                            destination_key,
                            data.as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?
                }
            }
        } else {
            let client = TorusClient::for_testnet().await?;
            match &self.proposal {
                Proposal::Emission {
                    recycling_percentage,
                    treasury_percentage,
                    incentives_ratio,
                    data,
                } => {
                    let current_recycling_percentage = client
                        .emission0()
                        .storage()
                        .emission_recycling_percentage()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();
                    let current_treasury_percentage = client
                        .governance()
                        .storage()
                        .treasury_emission_fee()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();
                    let current_incentives_ratio = client
                        .emission0()
                        .storage()
                        .incentives_ratio()
                        .await?
                        .map(|perc| perc.0)
                        .unwrap_or_default();

                    client
                        .governance()
                        .calls()
                        .add_emission_proposal_fee(
                            torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(recycling_percentage.unwrap_or(current_recycling_percentage) as u32)?),
                            torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(treasury_percentage.unwrap_or(current_treasury_percentage) as u32)?),
                            torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(incentives_ratio.unwrap_or(current_incentives_ratio) as u32)?),
                            data.clone().as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?
                }
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
                } => {
                    let params = fetch_global_params(ctx).await?;
                    client
                        .governance()
                        .calls()
                        .add_global_params_proposal_fee(
                            torus_client::interfaces::testnet::api::runtime_types::pallet_governance::proposal::GlobalParamsData {
                                min_name_length: min_name_length.unwrap_or(params.min_name_length),
                                max_name_length: max_name_length.unwrap_or(params.max_name_length),
                                min_weight_control_fee: min_weight_control_fee.unwrap_or(params.min_weight_control_fee),
                                min_staking_fee: min_staking_fee.unwrap_or(params.min_staking_fee),
                                dividends_participation_weight: torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(dividends_participation_weight.unwrap_or(params.dividends_paticipation_weight) as u32)?),
                                namespace_pricing_config: torus_client::interfaces::testnet::api::runtime_types::pallet_torus0::namespace::NamespacePricingConfig {
                                    deposit_per_byte: deposit_per_byte.unwrap_or(params.deposit_per_byte),
                                    base_fee: base_fee.unwrap_or(params.base_fee),
                                    count_midpoint: count_midpoint.unwrap_or(params.count_midpoint),
                                    fee_steepness: torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(to_percent_u8(fee_steepness.unwrap_or(params.fee_steepness) as u32)?),
                                    max_fee_multiplier: max_fee_multiplier.unwrap_or(params.max_fee_multiplier),
                                },
                                proposal_cost: proposal_cost.unwrap_or(params.proposal_cost)
                            },
                            data.as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?
                }
                Proposal::GlobalCustom { data } => {
                    client
                        .governance()
                        .calls()
                        .add_global_custom_proposal_fee(data.as_bytes().to_vec(), self.key.clone())
                        .await?
                }
                Proposal::TreasuryTransfer {
                    value,
                    destination,
                    data,
                } => {
                    let destination_key = get_account(destination)?;

                    client
                        .governance()
                        .calls()
                        .add_dao_treasury_transfer_proposal_fee(
                            *value,
                            destination_key,
                            data.as_bytes().to_vec(),
                            self.key.clone(),
                        )
                        .await?
                }
            }
        };

        Ok(fee)
    }

    async fn get_changes(
        &self,
        ctx: &mut impl ActionContext,
    ) -> anyhow::Result<Option<super::Changes>> {
        let fee = self.estimate_fee(ctx).await?;

        let proposal_str = match self.proposal {
            Proposal::Emission { .. } => "Emission",
            Proposal::GlobalParams { .. } => "Global Parameters",
            Proposal::GlobalCustom { .. } => "Global Custom",
            Proposal::TreasuryTransfer { .. } => "Treasury Transfer",
        };
        Ok(Some(super::Changes {
            changes: vec![format!("Create a {proposal_str} proposal")],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct CreateProposalActionResponse;

impl Display for CreateProposalActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Proposal created successfully!")
    }
}

pub struct AddVoteAction {
    key: Keypair,
    proposal_id: u64,
    agree: bool,
}

impl Action for AddVoteAction {
    type Params = (String, u64, bool);
    type ResponseData = AddVoteActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, proposal_id, agree): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;
        Ok(Self {
            key: keypair,
            proposal_id,
            agree,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .governance()
                .calls()
                .vote_proposal_wait(self.proposal_id, self.agree, self.key.clone())
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .governance()
                .calls()
                .vote_proposal_wait(self.proposal_id, self.agree, self.key.clone())
                .await?;
        }

        Ok(AddVoteActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .governance()
                .calls()
                .vote_proposal_fee(self.proposal_id, self.agree, self.key.clone())
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .governance()
                .calls()
                .vote_proposal_fee(self.proposal_id, self.agree, self.key.clone())
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(
        &self,
        ctx: &mut impl ActionContext,
    ) -> anyhow::Result<Option<super::Changes>> {
        let fee = self.estimate_fee(ctx).await?;
        Ok(Some(super::Changes {
            changes: vec![format!(
                "Vote {} proposal {}",
                if self.agree { "for" } else { "against" },
                self.proposal_id
            )],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct AddVoteActionResponse;

impl Display for AddVoteActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Voted successfully")
    }
}

pub struct RemoveVoteAction {
    key: Keypair,
    proposal_id: u64,
}

impl Action for RemoveVoteAction {
    type Params = (String, u64);
    type ResponseData = RemoveVoteActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, proposal_id): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        Ok(Self {
            key: keypair,
            proposal_id,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .governance()
                .calls()
                .remove_vote_proposal_wait(self.proposal_id, self.key.clone())
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .governance()
                .calls()
                .remove_vote_proposal_wait(self.proposal_id, self.key.clone())
                .await?;
        }

        Ok(RemoveVoteActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .governance()
                .calls()
                .remove_vote_proposal_fee(self.proposal_id, self.key.clone())
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .governance()
                .calls()
                .remove_vote_proposal_fee(self.proposal_id, self.key.clone())
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(
        &self,
        ctx: &mut impl ActionContext,
    ) -> anyhow::Result<Option<super::Changes>> {
        let fee = self.estimate_fee(ctx).await?;
        Ok(Some(super::Changes {
            changes: vec![format!("Remove vote on proposal {}", self.proposal_id)],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct RemoveVoteActionResponse;

impl Display for RemoveVoteActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Removed vote successfully")
    }
}
