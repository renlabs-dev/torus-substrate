use math::normalize;
use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_support::{
        dispatch::DispatchResult,
        storage::with_storage_layer,
        traits::{Currency, Imbalance},
    },
    polkadot_sdk_frame::prelude::BlockNumberFor,
    sc_telemetry::log,
    sp_core::Get,
    sp_runtime::{ArithmeticError, DispatchError},
    sp_std::{borrow::Cow, collections::btree_map::BTreeMap, vec, vec::Vec},
};
use substrate_fixed::{traits::FromFixed, types::I64F64};

use crate::{BalanceOf, Config, ConsensusMember};

mod math;

pub(super) fn distribute_emission<T: Config>(block_number: BlockNumberFor<T>) {
    let emission = get_total_emission_per_block::<T>();

    let Ok(emission) = super::PendingEmission::<T>::try_mutate(|acc: &mut u128| {
        *acc = acc.checked_add(emission).ok_or(ArithmeticError::Overflow)?;
        Result::<_, DispatchError>::Ok(*acc)
    }) else {
        log::error!("Pending emissions overflowed, tried adding: {emission}");
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
        log::info!(target: "consensus", "will emit {} tokens", emission.peek());

        let remaining = linear_rewards::<T>(emission);

        crate::PendingEmission::<T>::set(remaining.peek());

        log::info!(target: "consensus", "{} tokens returned to pending emissions.", remaining.peek());

        Ok(())
    });
}

// TODO: test
fn get_total_emission_per_block<T: Config>() -> BalanceOf<T> {
    let total_issuance = <T as Config>::Currency::total_issuance();
    let max_supply = T::MaxSupply::get();

    if total_issuance >= max_supply.get() {
        return 0;
    }

    let interval = T::HalvingInterval::get();
    let halving_count = total_issuance.saturating_div(interval.get());
    T::BlockEmission::get() >> halving_count
}

struct ConsensusMemberInput<T: Config> {
    agent_id: T::AccountId,
    validator_permit: bool,
    weights: Vec<(T::AccountId, I64F64)>,
    stakes: Vec<(T::AccountId, I64F64)>,
    total_stake: I64F64,
    normalized_stake: I64F64,
}

impl<T: Config> ConsensusMemberInput<T> {
    /// Creates a set of params for every agent registered to the network.
    fn all_members() -> BTreeMap<T::AccountId, ConsensusMemberInput<T>> {
        let min_allowed_stake = <T::Torus>::min_allowed_stake();

        let mut total_network_stake = I64F64::default();
        let mut inputs: Vec<_> = crate::ConsensusMembers::<T>::iter()
            .map(|(agent_id, member)| {
                let params = Self::from_agent(agent_id.clone(), member, min_allowed_stake);
                total_network_stake = total_network_stake.saturating_add(params.total_stake);

                (agent_id, params)
            })
            .collect();

        inputs.sort_unstable_by(|(_, a), (_, b)| {
            a.validator_permit
                .cmp(&b.validator_permit)
                .then(a.total_stake.cmp(&b.total_stake))
        });

        let max_validators = <T::Torus>::max_validators() as usize;
        for (idx, (_, input)) in inputs.iter_mut().enumerate() {
            if idx >= max_validators {
                input.validator_permit = false;
                input.weights.clear();
            }

            if total_network_stake != I64F64::from_num(0) {
                input.normalized_stake = input.total_stake.saturating_div(total_network_stake)
            }
        }

        inputs.into_iter().collect()
    }

    // TODO: test
    /// Creates a set of consensus parameters for an agent.
    ///
    /// Calculates the total staked tokens and the normalized weights.
    fn from_agent(
        agent_id: T::AccountId,
        member: ConsensusMember<T>,
        min_allowed_stake: u128,
    ) -> ConsensusMemberInput<T> {
        let mut total_stake = I64F64::default();
        let stakes = <T::Torus>::stakes_on(&agent_id)
            .into_iter()
            .map(|(id, stake)| {
                let stake = I64F64::from_num(stake);
                total_stake = total_stake.saturating_add(stake);
                (id, stake)
            })
            .collect();

        let validator_permit = total_stake >= min_allowed_stake && !member.weights.is_empty();

        let weights = validator_permit
            .then(|| Self::prepare_weights(member, &agent_id))
            .unwrap_or_default();

        ConsensusMemberInput {
            agent_id,
            validator_permit,
            weights,
            stakes,
            total_stake,
            normalized_stake: Default::default(),
        }
    }

    // TODO: test
    /// Removes self-weights, ensures the keys are registered to the consensus and normalizes it.
    fn prepare_weights(
        member: ConsensusMember<T>,
        agent_id: &T::AccountId,
    ) -> Vec<(T::AccountId, I64F64)> {
        let mut weights_sum = I64F64::default();
        let mut weights: Vec<_> = member
            .weights
            .into_iter()
            .filter(|(id, _)| id != agent_id && crate::ConsensusMembers::<T>::contains_key(id))
            .map(|(id, weight)| {
                let weight = I64F64::from_num(weight);
                weights_sum = weights_sum.saturating_add(weight);
                (id, weight)
            })
            .collect();

        if weights_sum > I64F64::from_num(0) {
            for (_, weight) in weights.iter_mut() {
                *weight = weight.saturating_div(weights_sum);
            }
        }

        weights
    }

    /// Normalizes the list of stakers to the agent, and adds the agent itself in case no stake was given.
    fn normalized_stakers(&self) -> Vec<(T::AccountId, I64F64)> {
        if self.total_stake == I64F64::default() {
            vec![(self.agent_id.clone(), I64F64::default())]
        } else {
            self.stakes
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.checked_div(self.total_stake).unwrap_or_default(),
                    )
                })
                .collect()
        }
    }
}

#[must_use]
fn linear_rewards<T: Config>(
    mut emission: <T::Currency as Currency<T::AccountId>>::NegativeImbalance,
) -> <T::Currency as Currency<T::AccountId>>::NegativeImbalance {
    let inputs = ConsensusMemberInput::<T>::all_members();
    let id_to_idx: BTreeMap<_, _> = inputs
        .keys()
        .cloned()
        .enumerate()
        .map(|(idx, id)| (id, idx))
        .collect();

    let mut weights: Vec<Vec<(usize, I64F64)>> = vec![vec![]; inputs.len()];
    let mut stakes = vec![I64F64::from_num(0); inputs.len()];

    for ((params, weights), stake) in inputs.values().zip(&mut weights).zip(&mut stakes) {
        *stake = params.normalized_stake;
        *weights = params
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

    let bonds_delta = math::row_hadamard_sparse(weights.as_ref(), stakes.as_ref());
    let bonds_delta = math::col_normalize_sparse(bonds_delta, inputs.len());

    let dividends = math::matmul_transpose_sparse(&bonds_delta, &incentives);
    let dividends = math::normalize(dividends);

    let calculate_active_stake = || {
        let stakes = inputs
            .values()
            .map(|input| {
                if input.validator_permit {
                    input.total_stake
                } else {
                    I64F64::default()
                }
            })
            .collect();

        normalize(stakes)
    };

    let Emissions {
        dividends,
        incentives,
        normalized_emissions,
    } = compute_emissions::<T>(
        &mut emission,
        calculate_active_stake,
        &stakes,
        incentives,
        dividends,
    );

    let pruning_scores = math::vec_max_upscale_to_u16(&normalized_emissions);

    for (((input, incentive), mut dividend), pruning_score) in inputs
        .values()
        .zip(incentives)
        .zip(dividends)
        .zip(pruning_scores)
    {
        let is_agent_registered = <T::Torus>::is_agent_registered(&input.agent_id);

        let add_stake = |staker, amount| {
            let amount = if is_agent_registered {
                match <T::Torus>::stake_to(&staker, &input.agent_id, amount) {
                    Ok(()) => return,
                    Err(amount) => amount,
                }
            } else {
                amount
            };

            T::Currency::resolve_creating(&staker, amount);
        };

        if dividend.peek() != 0 {
            let stakers = input.normalized_stakers();
            let delegation_fee = <T::Torus>::staking_fee(&input.agent_id);

            let fixed_dividend = I64F64::from_num(dividend.peek());
            for (staker, ratio) in stakers {
                if staker == input.agent_id {
                    continue;
                }

                let Some(staker_dividend) = fixed_dividend.checked_mul(ratio).map(u128::from_fixed)
                else {
                    log::warn!(
                        "failed to calculate dividend for {} on {}",
                        staker,
                        input.agent_id
                    );

                    continue;
                };

                let stake_fee = delegation_fee.mul_floor(staker_dividend);
                let stake = dividend.extract(staker_dividend.saturating_sub(stake_fee));

                add_stake(staker, stake);
            }
        }

        let remaining_emission = incentive.merge(dividend);
        if remaining_emission.peek() > 0 {
            add_stake(input.agent_id.clone(), remaining_emission);
        }

        if !is_agent_registered {
            crate::ConsensusMembers::<T>::remove(&input.agent_id);
        } else {
            crate::ConsensusMembers::<T>::mutate_extant(&input.agent_id, |member| {
                member.pruning_score = pruning_score;
            })
        }
    }

    emission
}

struct Emissions<T: Config> {
    dividends: Vec<<T::Currency as Currency<T::AccountId>>::NegativeImbalance>,
    incentives: Vec<<T::Currency as Currency<T::AccountId>>::NegativeImbalance>,
    normalized_emissions: Vec<I64F64>,
}

fn compute_emissions<'a, T: Config>(
    emission: &mut <T::Currency as Currency<T::AccountId>>::NegativeImbalance,
    compute_active_stake: impl Fn() -> Vec<I64F64>,
    stake: &'a [I64F64],
    incentives: Vec<I64F64>,
    dividends: Vec<I64F64>,
) -> Emissions<T> {
    let combined_emission: Vec<_> = incentives
        .iter()
        .zip(dividends.iter())
        .map(|(incentive, dividend)| incentive.saturating_add(*dividend))
        .collect();
    let emission_sum: I64F64 = combined_emission.iter().sum();

    let normalized_incentives = math::normalize_with_sum(incentives, emission_sum);
    let normalized_dividends: Cow<'a, [I64F64]>;

    let to_be_emitted = I64F64::from_num(emission.peek());

    // Only used to track emission in storage.
    let combined_emissions;
    let calculate_normalized_emissions = |emissions: &[I64F64]| {
        emissions
            .iter()
            .map(|combined| combined.checked_mul(to_be_emitted).unwrap_or_default())
            .collect()
    };

    // If emission is zero, replace emission with normalized stake.
    if emission_sum == I64F64::from_num(0) {
        let active_stake = compute_active_stake();

        if math::is_zero(&active_stake) {
            combined_emissions = calculate_normalized_emissions(stake);
            normalized_dividends = Cow::Borrowed(stake);
        } else {
            combined_emissions = calculate_normalized_emissions(&active_stake);
            normalized_dividends = Cow::Owned(active_stake);
        }
    } else {
        let dividends_emission = math::normalize_with_sum(dividends, emission_sum);
        normalized_dividends = Cow::Owned(dividends_emission);

        combined_emissions = calculate_normalized_emissions(&math::normalize(combined_emission));
    }

    let mut calculate_emissions = |v: &[I64F64]| {
        v.iter()
            .map(|&se| se.checked_mul(to_be_emitted).unwrap_or_default())
            .map(|amount| emission.extract(amount.to_num()))
            .collect::<Vec<_>>()
    };

    let incentives = calculate_emissions(&normalized_incentives);
    let dividends = calculate_emissions(&normalized_dividends);

    Emissions {
        dividends,
        incentives,
        normalized_emissions: normalize(combined_emissions),
    }
}
