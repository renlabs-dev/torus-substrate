use pallet_governance_api::GovernanceApi;
use pallet_permission0_api::Permission0EmissionApi;
use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_support::{
        dispatch::DispatchResult,
        storage::with_storage_layer,
        traits::{Currency, Imbalance},
        DebugNoBound,
    },
    polkadot_sdk_frame::prelude::BlockNumberFor,
    sp_core::Get,
    sp_runtime::{
        traits::Saturating, ArithmeticError, DispatchError, FixedU128, Percent, Perquintill,
    },
    sp_std::{
        borrow::Cow,
        collections::{btree_map::BTreeMap, btree_set::BTreeSet},
        vec,
        vec::Vec,
    },
    sp_tracing::{error, info},
};

use crate::{BalanceOf, Config, ConsensusMember, IncentivesRatio, NegativeImbalanceOf, Weights};

mod math;

pub(super) fn distribute_emission<T: Config>(block_number: BlockNumberFor<T>) {
    let emission = get_total_emission_per_block::<T>();

    let Ok(emission) = super::PendingEmission::<T>::try_mutate(|acc: &mut u128| {
        *acc = acc.checked_add(emission).ok_or(ArithmeticError::Overflow)?;
        Result::<_, DispatchError>::Ok(*acc)
    }) else {
        error!("Pending emissions overflowed, tried adding: {emission}");
        return;
    };

    let interval = <T::Torus>::reward_interval().into();
    let missing_blocks = block_number
        .into()
        .checked_rem(interval)
        .unwrap_or(interval);
    if !missing_blocks.is_zero() {
        return;
    }

    // The distribution is wrapped in a transaction.
    // If the consensus, for some reason, did not emit the entire pending emission,
    // it will go back to the accumulator and remove from total issuance.
    // If the closure returns Err, all changes are rolled back.
    let _res: DispatchResult = with_storage_layer(|| {
        let emission = T::Currency::issue(emission);
        info!(target: "consensus", "will emit {} tokens", emission.peek());

        let remaining = linear_rewards::<T>(emission);

        crate::PendingEmission::<T>::set(remaining.peek());

        info!(target: "consensus", "{} tokens returned to pending emissions.", remaining.peek());

        Ok(())
    });
}

#[doc(hidden)]
pub fn get_total_emission_per_block<T: Config>() -> BalanceOf<T> {
    let total_issuance = <T as Config>::Currency::total_issuance();
    let max_supply = T::MaxSupply::get();

    if total_issuance >= max_supply.get() {
        return 0;
    }

    let interval = T::HalvingInterval::get();
    let halving_count = total_issuance.checked_div(interval.get()).unwrap_or(0);
    let emission = T::BlockEmission::get() >> halving_count;

    let not_recycled =
        Percent::one().saturating_sub(crate::EmissionRecyclingPercentage::<T>::get());
    not_recycled.mul_floor(emission)
}

#[doc(hidden)]
#[derive(DebugNoBound, PartialEq, Eq)]
pub struct ConsensusMemberInput<T: Config> {
    pub agent_id: T::AccountId,
    pub validator_permit: bool,
    pub weights: Vec<(T::AccountId, FixedU128)>,
    pub stakes: Vec<(T::AccountId, u128)>,
    pub total_stake: u128,
    pub normalized_stake: FixedU128,
    pub delegating_to: Option<T::AccountId>,
    pub whitelisted: bool,
}

impl<T: Config> ConsensusMemberInput<T> {
    pub fn from_new_agent(agent_id: T::AccountId, whitelisted: bool) -> Self {
        Self {
            agent_id,
            validator_permit: Default::default(),
            weights: Default::default(),
            stakes: Default::default(),
            total_stake: Default::default(),
            normalized_stake: Default::default(),
            delegating_to: Default::default(),
            whitelisted,
        }
    }

    /// Creates a set of params for every agent registered to the network.
    pub fn all_members() -> BTreeMap<T::AccountId, ConsensusMemberInput<T>> {
        let min_validator_stake = <T::Torus>::min_validator_stake();

        let mut whitelisted_agents: BTreeSet<_> = <T::Torus>::agent_ids()
            .filter(<T::Governance>::is_whitelisted)
            .collect();
        let mut consensus_members: BTreeMap<_, _> = crate::ConsensusMembers::<T>::iter().collect();

        let mut inputs: Vec<_> = crate::WeightControlDelegation::<T>::iter()
            .map(|(delegator, recipient)| {
                let is_whitelisted = whitelisted_agents.remove(&delegator);
                consensus_members.remove(&delegator);

                let mut input = if let Some(delegatee_input) = consensus_members.get(&recipient) {
                    Self::from_agent(
                        delegator.clone(),
                        delegatee_input.weights.clone(),
                        min_validator_stake,
                    )
                } else {
                    Self::from_new_agent(delegator.clone(), is_whitelisted)
                };

                input.delegating_to = Some(recipient);

                (delegator, input)
            })
            .collect();

        inputs.extend(whitelisted_agents.into_iter().map(|agent_id| {
            let input = if let Some(member) = consensus_members.remove(&agent_id) {
                Self::from_agent(agent_id.clone(), member.weights, min_validator_stake)
            } else {
                Self::from_new_agent(agent_id.clone(), true)
            };

            (agent_id, input)
        }));

        inputs.extend(consensus_members.into_iter().map(|(agent_id, member)| {
            let input = Self::from_agent(agent_id.clone(), member.weights, min_validator_stake);
            (agent_id, input)
        }));

        let total_network_stake =
            FixedU128::from_inner(inputs.iter().map(|(_, member)| member.total_stake).sum());

        inputs.sort_unstable_by(|(_, a), (_, b)| {
            b.validator_permit
                .cmp(&a.validator_permit)
                .then(b.total_stake.cmp(&a.total_stake))
        });

        let max_validators = <T::Torus>::max_validators() as usize;
        for (idx, (_, input)) in inputs.iter_mut().enumerate() {
            if idx >= max_validators {
                input.validator_permit = false;
                input.weights.clear();
            }

            if total_network_stake != 0.into() {
                input.normalized_stake = FixedU128::from_inner(input.total_stake)
                    .const_checked_div(total_network_stake)
                    .unwrap_or_default();
            }
        }

        inputs.into_iter().collect()
    }

    /// Creates a set of consensus parameters for an agent.
    ///
    /// Calculates the total staked tokens and the normalized weights.
    pub fn from_agent(
        agent_id: T::AccountId,
        weights: Weights<T>,
        min_validator_stake: u128,
    ) -> ConsensusMemberInput<T> {
        let weight_factor =
            Percent::one().saturating_sub(<T::Torus>::weight_penalty_factor(&agent_id));

        let mut total_stake = 0;
        let stakes = <T::Torus>::staked_by(&agent_id)
            .into_iter()
            .map(|(id, stake)| {
                let stake = weight_factor.mul_floor(stake);

                total_stake = total_stake.saturating_add(stake);
                (id, stake)
            })
            .collect();

        let validator_permit = total_stake >= min_validator_stake && !weights.is_empty();

        let weights = if validator_permit {
            Self::prepare_weights(weights, &agent_id)
        } else {
            Default::default()
        };

        ConsensusMemberInput {
            whitelisted: <T::Torus>::is_agent_registered(&agent_id)
                && <T::Governance>::is_whitelisted(&agent_id),

            agent_id,
            validator_permit,
            weights,
            stakes,
            total_stake,
            normalized_stake: Default::default(),
            delegating_to: Default::default(),
        }
    }

    /// Removes self-weights, ensures the keys are registered to the consensus
    /// and normalizes it.
    pub fn prepare_weights(
        weights: Weights<T>,
        agent_id: &T::AccountId,
    ) -> Vec<(T::AccountId, FixedU128)> {
        let mut weights_sum = FixedU128::default();
        let mut weights: Vec<_> = weights
            .into_iter()
            .filter(|(id, _)| {
                id != agent_id
                    && (crate::ConsensusMembers::<T>::contains_key(id)
                        || (<T::Torus>::is_agent_registered(id)
                            && <T::Governance>::is_whitelisted(id)))
            })
            .map(|(id, weight)| {
                let weight = FixedU128::from_u32(weight as u32);
                weights_sum = weights_sum.saturating_add(weight);
                (id, weight)
            })
            .collect();

        if weights_sum > 0.into() {
            for (_, weight) in weights.iter_mut() {
                *weight = weight.const_checked_div(weights_sum).unwrap_or_default();
            }
        }

        weights
    }

    /// Normalizes the list of stakers to the agent, and adds the agent itself
    /// in case no stake was given.
    pub fn normalized_stakers(&self) -> Vec<(T::AccountId, Perquintill)> {
        self.stakes
            .iter()
            .map(|(k, v)| (k.clone(), Perquintill::from_rational(*v, self.total_stake)))
            .collect()
    }
}

#[must_use]
fn linear_rewards<T: Config>(mut emission: NegativeImbalanceOf<T>) -> NegativeImbalanceOf<T> {
    let treasury_fee = <T::Governance>::treasury_emission_fee();
    if !treasury_fee.is_zero() {
        let treasury_fee = treasury_fee.mul_floor(emission.peek());
        let treasury_fee = emission.extract(treasury_fee);
        T::Currency::resolve_creating(&<T::Governance>::dao_treasury_address(), treasury_fee);
    }

    let inputs = ConsensusMemberInput::<T>::all_members();

    let id_to_idx: BTreeMap<_, _> = inputs
        .keys()
        .cloned()
        .enumerate()
        .map(|(idx, id)| (id, idx))
        .collect();

    let mut weights: Vec<Vec<(usize, FixedU128)>> = vec![vec![]; inputs.len()];
    let mut stakes = vec![FixedU128::from_inner(0); inputs.len()];

    for ((input, weights), stake) in inputs.values().zip(&mut weights).zip(&mut stakes) {
        *stake = input.normalized_stake;
        *weights = input
            .weights
            .iter()
            .filter_map(|(id, weight)| {
                let idx = id_to_idx.get(id)?;
                Some((*idx, *weight))
            })
            .collect();
    }

    let ranks = math::matmul_sparse(&weights, &stakes, inputs.len());
    let incentives = math::normalize(ranks);

    let bonds_delta = math::row_hadamard_sparse(&weights, &stakes);
    let bonds_delta = math::col_normalize_sparse(bonds_delta, inputs.len());

    let dividends = math::matmul_transpose_sparse(&bonds_delta, &incentives);
    let dividends = math::normalize(dividends);

    let Emissions {
        mut dividends,
        incentives,
    } = compute_emissions::<T>(&mut emission, &stakes, incentives, dividends);

    for (idx, input) in inputs.values().enumerate() {
        let Some(delegating_to) = &input.delegating_to else {
            continue;
        };

        let Some(dividend) = dividends
            .get_mut(idx)
            .filter(|dividend| dividend.peek() > 0)
        else {
            continue;
        };

        let control_fee = <T::Torus>::weight_control_fee(delegating_to);
        let control_fee = control_fee.mul_floor(dividend.peek());
        let mut stake = dividend.extract(control_fee);

        if let Some(delegated_dividend) = id_to_idx
            .get(delegating_to)
            .and_then(|idx| dividends.get_mut(*idx))
        {
            delegated_dividend.subsume(stake);
        } else {
            // This is an impossible case, but if something changes in the future,
            // the code is here.
            <T::Permission0>::accumulate_emissions(
                delegating_to,
                &pallet_permission0_api::generate_root_stream_id(delegating_to),
                &mut stake,
            );

            let stake_num = stake.peek();
            T::Currency::resolve_creating(delegating_to, stake);
            let _ = <T::Torus>::stake_to(delegating_to, delegating_to, stake_num);
        }
    }

    let upscaled_incentives: Vec<_> = incentives
        .iter()
        .map(|i| FixedU128::from_inner(i.peek()))
        .collect();
    let upscaled_incentives = math::vec_max_upscale_to_u16(&upscaled_incentives);

    let upscaled_dividends: Vec<_> = dividends
        .iter()
        .map(|i| FixedU128::from_inner(i.peek()))
        .collect();
    let upscaled_dividends = math::vec_max_upscale_to_u16(&upscaled_dividends);

    for ((((input, incentive), mut dividend), upscaled_incentives), upscaled_dividends) in inputs
        .values()
        .zip(incentives)
        .zip(dividends)
        .zip(upscaled_incentives)
        .zip(upscaled_dividends)
    {
        let add_stake = |staker, mut amount: NegativeImbalanceOf<T>| {
            <T::Permission0>::accumulate_emissions(
                &staker,
                &pallet_permission0_api::generate_root_stream_id(&staker),
                &mut amount,
            );

            let raw_amount = amount.peek();

            T::Currency::resolve_creating(&staker, amount);
            if let Err(err) = <T::Torus>::stake_to(&staker, &input.agent_id, raw_amount) {
                error!("failed to stake {raw_amount} tokens to {staker:?}: {err:?}");
            }
        };

        if dividend.peek() != 0 {
            let fixed_dividend = dividend.peek();

            let stakers = input.normalized_stakers();
            let delegation_fee = <T::Torus>::staking_fee(&input.agent_id);
            for (staker, ratio) in stakers {
                let staker_dividend = ratio.mul_floor(fixed_dividend);
                let stake_fee = delegation_fee.mul_floor(staker_dividend);

                let stake = dividend.extract(staker_dividend.saturating_sub(stake_fee));

                add_stake(staker, stake);
            }
        }

        let remaining_emission = incentive.merge(dividend);
        if remaining_emission.peek() > 0 {
            add_stake(input.agent_id.clone(), remaining_emission);
        }

        if input.whitelisted {
            crate::ConsensusMembers::<T>::mutate(
                &input.agent_id,
                |member: &mut Option<ConsensusMember<T>>| {
                    let member = member.get_or_insert_with(Default::default);
                    member.last_incentives = upscaled_incentives;
                    member.last_dividends = upscaled_dividends;
                },
            );
        } else {
            crate::ConsensusMembers::<T>::remove(&input.agent_id);
            crate::WeightControlDelegation::<T>::remove(&input.agent_id);
        }
    }

    emission
}

struct Emissions<T: Config> {
    dividends: Vec<NegativeImbalanceOf<T>>,
    incentives: Vec<NegativeImbalanceOf<T>>,
}

fn compute_emissions<T: Config>(
    emission: &mut NegativeImbalanceOf<T>,
    stake: &[FixedU128],
    incentives: Vec<FixedU128>,
    dividends: Vec<FixedU128>,
) -> Emissions<T> {
    let combined_emission: Vec<_> = incentives
        .iter()
        .zip(dividends.iter())
        .map(|(incentive, dividend)| incentive.saturating_add(*dividend))
        .collect();
    let emission_sum = combined_emission
        .iter()
        .fold(FixedU128::default(), |acc, &e| acc.saturating_add(e));

    let normalized_incentives = math::normalize_with_sum(incentives, emission_sum);
    let normalized_dividends = if emission_sum == 0.into() {
        // When incentives and dividends are zero, the protocol still needs to issue tokens,
        // so it is distributed for all stake-holder agents.
        Cow::Borrowed(stake)
    } else {
        let dividends_emission = math::normalize_with_sum(dividends, emission_sum);
        Cow::Owned(dividends_emission)
    };

    let to_be_emitted = FixedU128::from_inner(emission.peek());

    let mut calculate_emissions = |v: &[FixedU128], to_be_emitted: FixedU128| {
        v.iter()
            .map(|&se| se.const_checked_mul(to_be_emitted).unwrap_or_default())
            .map(|amount| emission.extract(amount.into_inner()))
            .collect::<Vec<_>>()
    };

    let incentives_ratio = IncentivesRatio::<T>::get().deconstruct();

    let to_be_emitted = to_be_emitted.into_inner();
    let incentives_to_be_emitted;
    let dividends_to_be_emitted;

    if let Some(incentives_ratio) = incentives_ratio.checked_sub(50) {
        let incentives_percentage = Percent::from_parts(incentives_ratio.saturating_mul(2));
        let incentives = incentives_percentage.mul_floor(to_be_emitted);
        incentives_to_be_emitted = to_be_emitted.saturating_add(incentives);
        dividends_to_be_emitted = to_be_emitted.saturating_sub(incentives);
    } else if let Some(dividends_ratio) = 50u8.checked_sub(incentives_ratio) {
        let dividends_percentage = Percent::from_parts(dividends_ratio.saturating_mul(2));
        let dividends = dividends_percentage.mul_floor(to_be_emitted);
        dividends_to_be_emitted = to_be_emitted.saturating_add(dividends);
        incentives_to_be_emitted = to_be_emitted.saturating_sub(dividends);
    } else {
        // This is logically impossible at the time of writing.
        dividends_to_be_emitted = 0;
        incentives_to_be_emitted = 0;
        error!("MATH IS NOT MATHING. PLEASE CHECK ME, incentives_ratio = {incentives_ratio}");
    }

    let incentives = calculate_emissions(
        &normalized_incentives,
        FixedU128::from_inner(incentives_to_be_emitted),
    );
    let dividends = calculate_emissions(
        &normalized_dividends,
        FixedU128::from_inner(dividends_to_be_emitted),
    );

    Emissions {
        dividends,
        incentives,
    }
}
