use crate::frame::traits::ExistenceRequirement;
use crate::BoundedBTreeSet;
use crate::BoundedVec;
use crate::DebugNoBound;
use crate::TypeInfo;
use crate::{
    AccountIdOf, BalanceOf, Block, DaoTreasuryAddress, Error, GlobalGovernanceConfig, Proposals,
    UnrewardedProposals,
};
use crate::{GovernanceConfiguration, NotDelegatingVotingPower};
use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::frame_election_provider_support::Get;
use polkadot_sdk::frame_support::traits::Currency;
use polkadot_sdk::polkadot_sdk_frame::traits::CheckedAdd;
use polkadot_sdk::sp_runtime::SaturatedConversion;
use polkadot_sdk::sp_std::{collections::btree_set::BTreeSet, vec::Vec};
use polkadot_sdk::{
    frame_support::{dispatch::DispatchResult, ensure, storage::with_storage_layer},
    sp_core::ConstU32,
    sp_runtime::{BoundedBTreeMap, DispatchError, Percent},
    sp_tracing::error,
};
use substrate_fixed::types::I92F36;

pub type ProposalId = u64;

#[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Proposal<T: crate::Config> {
    pub id: ProposalId,
    pub proposer: AccountIdOf<T>,
    pub expiration_block: Block,
    pub data: ProposalData<T>,
    pub status: ProposalStatus<T>,
    pub metadata: BoundedVec<u8, ConstU32<256>>,
    pub proposal_cost: BalanceOf<T>,
    pub creation_block: Block,
}

impl<T: crate::Config> Proposal<T> {
    /// Whether the proposal is still active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(self.status, ProposalStatus::Open { .. })
    }

    pub fn execution_block(&self) -> Block {
        match self.data {
            ProposalData::Emission { .. } => self.creation_block.saturating_add(21_600),
            _ => self.expiration_block,
        }
    }

    /// Marks a proposal as accepted and overrides the storage value.
    pub fn accept(
        mut self,
        block: Block,
        stake_for: BalanceOf<T>,
        stake_against: BalanceOf<T>,
    ) -> DispatchResult {
        ensure!(self.is_active(), crate::Error::<T>::ProposalIsFinished);

        self.status = ProposalStatus::Accepted {
            block,
            stake_for,
            stake_against,
        };

        Proposals::<T>::insert(self.id, &self);
        crate::Pallet::<T>::deposit_event(crate::Event::ProposalAccepted(self.id));

        self.execute_proposal()?;

        Ok(())
    }

    fn execute_proposal(self) -> DispatchResult {
        let _ = <T as crate::Config>::Currency::transfer(
            &crate::DaoTreasuryAddress::<T>::get(),
            &self.proposer,
            self.proposal_cost,
            ExistenceRequirement::AllowDeath,
        );

        match self.data {
            ProposalData::GlobalParams(data) => {
                let GlobalParamsData {
                    min_name_length,
                    max_name_length,
                    max_allowed_agents,
                    max_allowed_weights,
                    min_stake_per_weight,
                    min_weight_control_fee,
                    min_staking_fee,
                    dividends_participation_weight,
                    proposal_cost,
                } = data;

                pallet_torus0::MinNameLength::<T>::set(min_name_length);
                pallet_torus0::MaxNameLength::<T>::set(max_name_length);
                pallet_torus0::MaxAllowedAgents::<T>::set(max_allowed_agents);
                pallet_torus0::DividendsParticipationWeight::<T>::set(
                    dividends_participation_weight,
                );
                pallet_torus0::FeeConstraints::<T>::mutate(|constraints| {
                    constraints.min_weight_control_fee =
                        Percent::from_percent(min_weight_control_fee);
                    constraints.min_staking_fee = Percent::from_percent(min_staking_fee);
                });
                pallet_emission0::MaxAllowedWeights::<T>::set(max_allowed_weights);
                pallet_emission0::MinStakePerWeight::<T>::set(min_stake_per_weight);
                crate::GlobalGovernanceConfig::<T>::mutate(|config| {
                    config.proposal_cost = proposal_cost;
                });
            }

            ProposalData::TransferDaoTreasury { account, amount } => {
                <T as crate::Config>::Currency::transfer(
                    &DaoTreasuryAddress::<T>::get(),
                    &account,
                    amount,
                    ExistenceRequirement::AllowDeath,
                )
                .map_err(|_| crate::Error::<T>::InternalError)?;
            }

            ProposalData::Emission {
                recycling_percentage,
                treasury_percentage,
                incentives_ratio,
            } => {
                pallet_emission0::EmissionRecyclingPercentage::<T>::set(recycling_percentage);
                crate::TreasuryEmissionFee::<T>::set(treasury_percentage);
                pallet_emission0::IncentivesRatio::<T>::set(incentives_ratio);
            }

            ProposalData::GlobalCustom => {}
        }

        Ok(())
    }

    /// Marks a proposal as refused and overrides the storage value.
    pub fn refuse(
        mut self,
        block: Block,
        stake_for: BalanceOf<T>,
        stake_against: BalanceOf<T>,
    ) -> DispatchResult {
        ensure!(self.is_active(), crate::Error::<T>::ProposalIsFinished);

        self.status = ProposalStatus::Refused {
            block,
            stake_for,
            stake_against,
        };

        Proposals::<T>::insert(self.id, &self);
        crate::Pallet::<T>::deposit_event(crate::Event::ProposalRefused(self.id));

        Ok(())
    }

    /// Marks a proposal as expired and overrides the storage value.
    pub fn expire(mut self, block_number: u64) -> DispatchResult {
        ensure!(self.is_active(), crate::Error::<T>::ProposalIsFinished);
        ensure!(
            block_number >= self.expiration_block,
            crate::Error::<T>::InvalidProposalFinalizationParameters
        );

        self.status = ProposalStatus::Expired;

        Proposals::<T>::insert(self.id, &self);
        crate::Pallet::<T>::deposit_event(crate::Event::ProposalExpired(self.id));

        Ok(())
    }
}

#[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub enum ProposalStatus<T: crate::Config> {
    Open {
        votes_for: BoundedBTreeSet<AccountIdOf<T>, ConstU32<{ u32::MAX }>>,
        votes_against: BoundedBTreeSet<AccountIdOf<T>, ConstU32<{ u32::MAX }>>,
        stake_for: BalanceOf<T>,
        stake_against: BalanceOf<T>,
    },
    Accepted {
        block: Block,
        stake_for: BalanceOf<T>,
        stake_against: BalanceOf<T>,
    },
    Refused {
        block: Block,
        stake_for: BalanceOf<T>,
        stake_against: BalanceOf<T>,
    },
    Expired,
}

// TODO: add Agent URL max length
#[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub struct GlobalParamsData<T: crate::Config> {
    pub min_name_length: u16,
    pub max_name_length: u16,
    pub max_allowed_agents: u16,
    pub max_allowed_weights: u16,
    pub min_stake_per_weight: BalanceOf<T>,
    pub min_weight_control_fee: u8,
    pub min_staking_fee: u8,
    pub dividends_participation_weight: Percent,
    pub proposal_cost: BalanceOf<T>,
}

impl<T: crate::Config> Default for GlobalParamsData<T> {
    fn default() -> Self {
        Self {
            min_name_length: T::DefaultMinNameLength::get(),
            max_name_length: T::DefaultMaxNameLength::get(),
            max_allowed_agents: T::DefaultMaxAllowedAgents::get(),
            max_allowed_weights: T::DefaultMaxAllowedWeights::get(),
            min_stake_per_weight: 0,
            min_weight_control_fee: T::DefaultMinWeightControlFee::get(),
            min_staking_fee: T::DefaultMinStakingFee::get(),
            dividends_participation_weight:
                <T as pallet_torus0::Config>::DefaultDividendsParticipationWeight::get(),
            proposal_cost: T::DefaultProposalCost::get(),
        }
    }
}

impl<T: crate::Config> GlobalParamsData<T> {
    pub fn validate(&self) -> DispatchResult {
        ensure!(
            self.min_name_length > 1,
            crate::Error::<T>::InvalidMinNameLength
        );

        ensure!(
            (self.max_name_length as u32) < T::MaxAgentNameLengthConstraint::get(),
            crate::Error::<T>::InvalidMaxNameLength
        );

        ensure!(
            self.max_allowed_agents <= 50000,
            crate::Error::<T>::InvalidMaxAllowedAgents
        );

        ensure!(
            self.max_allowed_weights <= 2000,
            crate::Error::<T>::InvalidMaxAllowedWeights
        );

        ensure!(
            self.min_weight_control_fee <= 100,
            crate::Error::<T>::InvalidMinWeightControlFee
        );

        ensure!(
            self.min_staking_fee <= 100,
            crate::Error::<T>::InvalidMinStakingFee
        );

        ensure!(
            self.proposal_cost <= 50_000_000_000_000_000_000_000,
            crate::Error::<T>::InvalidProposalCost
        );

        Ok(())
    }
}

#[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub enum ProposalData<T: crate::Config> {
    GlobalParams(GlobalParamsData<T>),
    GlobalCustom,
    Emission {
        recycling_percentage: Percent,
        treasury_percentage: Percent,
        incentives_ratio: Percent,
    },
    TransferDaoTreasury {
        account: AccountIdOf<T>,
        amount: BalanceOf<T>,
    },
}

impl<T: crate::Config> ProposalData<T> {
    #[must_use]
    pub fn required_stake(&self) -> Percent {
        match self {
            Self::Emission { .. } => Percent::from_parts(10),
            Self::GlobalCustom | Self::TransferDaoTreasury { .. } => Percent::from_parts(50),
            Self::GlobalParams { .. } => Percent::from_parts(40),
        }
    }
}

#[derive(DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub struct UnrewardedProposal<T: crate::Config> {
    pub block: Block,
    pub votes_for: BoundedBTreeMap<AccountIdOf<T>, BalanceOf<T>, ConstU32<{ u32::MAX }>>,
    pub votes_against: BoundedBTreeMap<AccountIdOf<T>, BalanceOf<T>, ConstU32<{ u32::MAX }>>,
}

#[allow(clippy::too_many_arguments)]
pub fn add_global_params_proposal<T: crate::Config>(
    proposer: AccountIdOf<T>,
    data: GlobalParamsData<T>,
    metadata: Vec<u8>,
) -> DispatchResult {
    data.validate()?;
    let data = ProposalData::<T>::GlobalParams(data);

    add_proposal::<T>(proposer, data, metadata)
}

pub fn add_global_custom_proposal<T: crate::Config>(
    proposer: AccountIdOf<T>,
    metadata: Vec<u8>,
) -> DispatchResult {
    add_proposal(proposer, ProposalData::<T>::GlobalCustom, metadata)
}

pub fn add_dao_treasury_transfer_proposal<T: crate::Config>(
    proposer: AccountIdOf<T>,
    value: BalanceOf<T>,
    destination_key: AccountIdOf<T>,
    metadata: Vec<u8>,
) -> DispatchResult {
    let data = ProposalData::<T>::TransferDaoTreasury {
        account: destination_key,
        amount: value,
    };

    add_proposal::<T>(proposer, data, metadata)
}

pub fn add_emission_proposal<T: crate::Config>(
    proposer: AccountIdOf<T>,
    recycling_percentage: Percent,
    treasury_percentage: Percent,
    incentives_ratio: Percent,
    metadata: Vec<u8>,
) -> DispatchResult {
    ensure!(
        recycling_percentage
            .checked_add(&treasury_percentage)
            .is_some(),
        crate::Error::<T>::InvalidEmissionProposalData
    );

    let data = ProposalData::<T>::Emission {
        recycling_percentage,
        treasury_percentage,
        incentives_ratio,
    };

    add_proposal::<T>(proposer, data, metadata)
}

fn add_proposal<T: crate::Config>(
    proposer: AccountIdOf<T>,
    data: ProposalData<T>,
    metadata: Vec<u8>,
) -> DispatchResult {
    ensure!(
        !metadata.is_empty(),
        crate::Error::<T>::ProposalDataTooSmall
    );
    ensure!(
        metadata.len() <= 256,
        crate::Error::<T>::ProposalDataTooLarge
    );

    let config = GlobalGovernanceConfig::<T>::get();

    let cost = config.proposal_cost;
    <T as crate::Config>::Currency::transfer(
        &proposer,
        &crate::DaoTreasuryAddress::<T>::get(),
        cost,
        ExistenceRequirement::AllowDeath,
    )
    .map_err(|_| crate::Error::<T>::NotEnoughBalanceToApply)?;

    let proposal_id: u64 = crate::Proposals::<T>::iter()
        .count()
        .try_into()
        .map_err(|_| crate::Error::<T>::InternalError)?;

    let current_block: u64 =
        TryInto::try_into(<polkadot_sdk::frame_system::Pallet<T>>::block_number())
            .ok()
            .expect("blockchain will not exceed 2^64 blocks; QED.");

    let proposal = Proposal::<T> {
        id: proposal_id,
        proposer,
        expiration_block: current_block.saturating_add(config.proposal_expiration),
        data,
        status: ProposalStatus::Open {
            votes_for: BoundedBTreeSet::new(),
            votes_against: BoundedBTreeSet::new(),
            stake_for: 0,
            stake_against: 0,
        },
        metadata: BoundedVec::truncate_from(metadata),
        proposal_cost: cost,
        creation_block: current_block,
    };

    crate::Proposals::<T>::insert(proposal_id, proposal);

    Ok(())
}

pub fn tick_proposals<T: crate::Config>(block_number: Block) {
    let not_delegating = NotDelegatingVotingPower::<T>::get().into_inner();

    let proposals = Proposals::<T>::iter().filter(|(_, p)| p.is_active());

    if block_number % 100 != 0 {
        return;
    }

    for (id, proposal) in proposals {
        let res = with_storage_layer(|| tick_proposal(&not_delegating, block_number, proposal));
        if let Err(err) = res {
            error!("failed to tick proposal {id}: {err:?}, skipping...");
        }
    }
}

pub fn get_minimal_stake_to_execute_with_percentage<T: crate::Config>(
    threshold: Percent,
) -> BalanceOf<T> {
    let stake = pallet_torus0::TotalStake::<T>::get();

    stake
        .saturated_into::<BalanceOf<T>>()
        .checked_mul(threshold.deconstruct() as u128)
        .unwrap_or_default()
        .checked_div(100)
        .unwrap_or_default()
}

fn tick_proposal<T: crate::Config>(
    not_delegating: &BTreeSet<T::AccountId>,
    block_number: u64,
    mut proposal: Proposal<T>,
) -> DispatchResult {
    let ProposalStatus::Open {
        votes_for,
        votes_against,
        ..
    } = &proposal.status
    else {
        return Err(Error::<T>::ProposalIsFinished.into());
    };

    let votes_for: Vec<(AccountIdOf<T>, BalanceOf<T>)> = votes_for
        .iter()
        .cloned()
        .map(|id| {
            let stake = calc_stake::<T>(not_delegating, &id);
            (id, stake)
        })
        .collect();
    let votes_against: Vec<(AccountIdOf<T>, BalanceOf<T>)> = votes_against
        .iter()
        .cloned()
        .map(|id| {
            let stake = calc_stake::<T>(not_delegating, &id);
            (id, stake)
        })
        .collect();

    let stake_for_sum: BalanceOf<T> = votes_for.iter().map(|(_, stake)| stake).sum();
    let stake_against_sum: BalanceOf<T> = votes_against.iter().map(|(_, stake)| stake).sum();

    if block_number < proposal.expiration_block {
        if let ProposalStatus::Open {
            stake_for,
            stake_against,
            ..
        } = &mut proposal.status
        {
            *stake_for = stake_for_sum;
            *stake_against = stake_against_sum;
        }
        Proposals::<T>::set(proposal.id, Some(proposal.clone()));
    }

    if block_number < proposal.execution_block() {
        return Ok(());
    }

    let total_stake = stake_for_sum.saturating_add(stake_against_sum);
    let minimal_stake_to_execute =
        get_minimal_stake_to_execute_with_percentage::<T>(proposal.data.required_stake());

    if total_stake >= minimal_stake_to_execute {
        create_unrewarded_proposal::<T>(proposal.id, block_number, votes_for, votes_against);
        if stake_against_sum > stake_for_sum {
            proposal.refuse(block_number, stake_for_sum, stake_against_sum)
        } else {
            proposal.accept(block_number, stake_for_sum, stake_against_sum)
        }
    } else if block_number >= proposal.expiration_block {
        create_unrewarded_proposal::<T>(proposal.id, block_number, votes_for, votes_against);
        proposal.expire(block_number)
    } else {
        Ok(())
    }
}

fn create_unrewarded_proposal<T: crate::Config>(
    proposal_id: u64,
    block_number: Block,
    votes_for: Vec<(AccountIdOf<T>, BalanceOf<T>)>,
    votes_against: Vec<(AccountIdOf<T>, BalanceOf<T>)>,
) {
    let mut reward_votes_for = BoundedBTreeMap::new();
    for (key, value) in votes_for {
        reward_votes_for
            .try_insert(key, value)
            .expect("this wont exceed u32::MAX");
    }

    let mut reward_votes_against: BoundedBTreeMap<
        T::AccountId,
        BalanceOf<T>,
        ConstU32<{ u32::MAX }>,
    > = BoundedBTreeMap::new();
    for (key, value) in votes_against {
        reward_votes_against
            .try_insert(key, value)
            .expect("this probably wont exceed u32::MAX");
    }

    UnrewardedProposals::<T>::insert(
        proposal_id,
        UnrewardedProposal::<T> {
            block: block_number,
            votes_for: reward_votes_for,
            votes_against: reward_votes_against,
        },
    );
}

#[inline]
fn calc_stake<T: crate::Config>(
    not_delegating: &BTreeSet<T::AccountId>,
    voter: &T::AccountId,
) -> BalanceOf<T> {
    let own_stake: BalanceOf<T> = if !not_delegating.contains(voter) {
        0
    } else {
        pallet_torus0::stake::sum_staking_to::<T>(voter)
    };

    let delegated_stake = pallet_torus0::stake::get_staked_by_vector::<T>(voter)
        .into_iter()
        .filter(|(staker, _)| !not_delegating.contains(staker))
        .map(|(_, stake)| stake)
        .sum();

    own_stake.saturating_add(delegated_stake)
}

pub fn tick_proposal_rewards<T: crate::Config>(block_number: u64) {
    let governance_config = crate::GlobalGovernanceConfig::<T>::get();
    let reached_interval = block_number
        .checked_rem(governance_config.proposal_reward_interval)
        .is_some_and(|r| r == 0);
    if !reached_interval {
        return;
    }

    let mut n: u16 = 0;
    let mut account_stakes: BoundedBTreeMap<T::AccountId, BalanceOf<T>, ConstU32<{ u32::MAX }>> =
        BoundedBTreeMap::new();
    let mut total_allocation: I92F36 = I92F36::from_num(0);
    for (proposal_id, unrewarded_proposal) in UnrewardedProposals::<T>::iter() {
        if unrewarded_proposal.block
            < block_number.saturating_sub(governance_config.proposal_reward_interval)
        {
            continue;
        }

        for (acc_id, stake) in unrewarded_proposal
            .votes_for
            .into_iter()
            .chain(unrewarded_proposal.votes_against.into_iter())
        {
            let curr_stake = *account_stakes.get(&acc_id).unwrap_or(&0u128);
            account_stakes
                .try_insert(acc_id, curr_stake.saturating_add(stake))
                .expect("infallible");
        }

        match get_reward_allocation::<T>(&governance_config, n) {
            Ok(allocation) => {
                total_allocation = total_allocation.saturating_add(allocation);
            }
            Err(err) => {
                error!("could not get reward allocation for proposal {proposal_id}: {err:?}");
                continue;
            }
        }

        UnrewardedProposals::<T>::remove(proposal_id);
        n = n.saturating_add(1);
    }

    distribute_proposal_rewards::<T>(
        account_stakes,
        total_allocation,
        governance_config.max_proposal_reward_treasury_allocation,
    );
}

pub fn get_reward_allocation<T: crate::Config>(
    governance_config: &GovernanceConfiguration<T>,
    n: u16,
) -> Result<I92F36, DispatchError> {
    let treasury_address = DaoTreasuryAddress::<T>::get();
    let treasury_balance = <T as crate::Config>::Currency::free_balance(&treasury_address);
    let treasury_balance = I92F36::from_num(treasury_balance);

    let allocation_percentage = I92F36::from_num(
        governance_config
            .proposal_reward_treasury_allocation
            .deconstruct(),
    );
    let max_allocation =
        I92F36::from_num(governance_config.max_proposal_reward_treasury_allocation);

    let mut allocation = treasury_balance
        .checked_mul(allocation_percentage)
        .unwrap_or_default()
        .min(max_allocation);
    if n > 0 {
        let mut base = I92F36::from_num(1.5);
        let mut result = I92F36::from_num(1);
        let mut remaining = n;

        while remaining > 0 {
            if remaining % 2 == 1 {
                result = result.checked_mul(base).unwrap_or(result);
            }
            base = base.checked_mul(base).unwrap_or_default();
            remaining /= 2;
        }

        allocation = allocation.checked_div(result).unwrap_or(allocation);
    }
    Ok(allocation)
}

fn distribute_proposal_rewards<T: crate::Config>(
    account_stakes: BoundedBTreeMap<T::AccountId, BalanceOf<T>, ConstU32<{ u32::MAX }>>,
    total_allocation: I92F36,
    max_proposal_reward_treasury_allocation: BalanceOf<T>,
) {
    // This is just a sanity check, making sure we can never allocate more than the max
    if total_allocation > I92F36::from_num(max_proposal_reward_treasury_allocation) {
        error!("total allocation exceeds max proposal reward treasury allocation");
        return;
    }

    use polkadot_sdk::frame_support::sp_runtime::traits::IntegerSquareRoot;

    let dao_treasury_address = DaoTreasuryAddress::<T>::get();
    let account_sqrt_stakes: Vec<_> = account_stakes
        .into_iter()
        .map(|(acc_id, stake)| (acc_id, stake.integer_sqrt()))
        .collect();

    let total_stake: BalanceOf<T> = account_sqrt_stakes.iter().map(|(_, stake)| *stake).sum();
    let total_stake = I92F36::from_num(total_stake);

    for (acc_id, stake) in account_sqrt_stakes.into_iter() {
        let percentage = I92F36::from_num(stake)
            .checked_div(total_stake)
            .unwrap_or_default();

        let reward: BalanceOf<T> = total_allocation
            .checked_mul(percentage)
            .unwrap_or_default()
            .to_num();

        // Transfer the proposal reward to the accounts from treasury
        if let Err(err) = <T as crate::Config>::Currency::transfer(
            &dao_treasury_address,
            &acc_id,
            reward,
            ExistenceRequirement::AllowDeath,
        ) {
            error!("could not transfer proposal reward: {err:?}")
        }
    }
}
