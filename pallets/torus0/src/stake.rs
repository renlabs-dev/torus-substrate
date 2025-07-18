use polkadot_sdk::{
    frame_support::{
        dispatch::DispatchResult,
        ensure,
        traits::{Currency, ExistenceRequirement, Imbalance, WithdrawReasons},
    },
    sp_std::{collections::btree_map::BTreeMap, vec::Vec},
};

use crate::agent;
use crate::{AccountIdOf, BalanceOf};

/// Stakes `amount` tokens from `staker` to `staked` by withdrawing the tokens
/// and adding them to the [`crate::StakingTo`] and [`crate::StakedBy`] maps.
pub fn add_stake<T: crate::Config>(
    staker: AccountIdOf<T>,
    staked: AccountIdOf<T>,
    amount: BalanceOf<T>,
) -> DispatchResult {
    ensure!(
        agent::exists::<T>(&staked),
        crate::Error::<T>::AgentDoesNotExist
    );

    let _ = <T as crate::Config>::Currency::withdraw(
        &staker,
        amount,
        WithdrawReasons::TRANSFER,
        ExistenceRequirement::AllowDeath,
    )
    .map_err(|_| crate::Error::<T>::NotEnoughBalanceToStake)?;

    crate::StakedBy::<T>::mutate(&staked, &staker, |stake| {
        *stake = Some(stake.unwrap_or(0).saturating_add(amount))
    });

    crate::StakingTo::<T>::mutate(&staker, &staked, |stake| {
        *stake = Some(stake.unwrap_or(0).saturating_add(amount))
    });

    crate::TotalStake::<T>::mutate(|total_stake| *total_stake = total_stake.saturating_add(amount));

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::StakeAdded(staker, staked, amount));

    Ok(())
}

/// Withdraws stake from an agent and gives it back to the staker.
pub fn remove_stake<T: crate::Config>(
    staker: AccountIdOf<T>,
    staked: AccountIdOf<T>,
    amount: BalanceOf<T>,
) -> DispatchResult {
    ensure!(
        agent::exists::<T>(&staked),
        crate::Error::<T>::AgentDoesNotExist
    );

    ensure!(
        crate::StakingTo::<T>::get(&staker, &staked).unwrap_or(0) >= amount,
        crate::Error::<T>::NotEnoughStakeToWithdraw
    );

    remove_stake0::<T>(staker, staked, amount, true);

    Ok(())
}

fn remove_stake0<T: crate::Config>(
    staker: AccountIdOf<T>,
    staked: AccountIdOf<T>,
    amount: BalanceOf<T>,
    keep: bool,
) {
    let Some(stake) = crate::StakingTo::<T>::get(&staker, &staked) else {
        return;
    };

    let mut stake = T::Currency::issue(stake);
    let retrieved = stake.extract(amount);

    let new_stake = if keep || stake.peek() > 0 {
        Some(stake.peek())
    } else {
        None
    };

    crate::StakingTo::<T>::set(&staker, &staked, new_stake);
    crate::StakedBy::<T>::set(&staked, &staker, new_stake);
    crate::TotalStake::<T>::mutate(|total_stake| {
        *total_stake = total_stake.saturating_sub(retrieved.peek())
    });

    let retrieved_value = retrieved.peek();
    <T as crate::Config>::Currency::resolve_creating(&staker, retrieved);

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::StakeRemoved(
        staker,
        staked,
        retrieved_value,
    ));
}

/// Transfers stake from an account to another (see [`remove_stake`],
/// [`add_stake`]).
pub fn transfer_stake<T: crate::Config>(
    staker: AccountIdOf<T>,
    old_staked: AccountIdOf<T>,
    new_staked: AccountIdOf<T>,
    amount: BalanceOf<T>,
) -> DispatchResult {
    remove_stake::<T>(staker.clone(), old_staked, amount)?;
    add_stake::<T>(staker, new_staked, amount)?;
    Ok(())
}

/// Usually called when de-registering an agent, removes all stakes on a given
/// key.
pub(crate) fn clear_key<T: crate::Config>(key: &AccountIdOf<T>) -> DispatchResult {
    let stakes: Vec<_> = crate::StakingTo::<T>::iter().collect();
    for (staker, staked, amount) in stakes {
        if &staker == key || &staked == key {
            remove_stake0::<T>(staker, staked, amount, false);
        }
    }

    Ok(())
}

#[inline]
pub fn sum_staking_to<T: crate::Config>(staker: &AccountIdOf<T>) -> BalanceOf<T> {
    crate::StakingTo::<T>::iter_prefix_values(staker).sum()
}

#[inline]
pub fn get_staking_to_vector<T: crate::Config>(
    staker: &AccountIdOf<T>,
) -> BTreeMap<T::AccountId, BalanceOf<T>> {
    crate::StakingTo::<T>::iter_prefix(staker).collect()
}

#[inline]
pub fn get_staked_by_vector<T: crate::Config>(
    staked: &AccountIdOf<T>,
) -> Vec<(T::AccountId, BalanceOf<T>)> {
    crate::StakedBy::<T>::iter_prefix(staked).collect()
}

#[inline]
pub fn sum_staked_by<T: crate::Config>(staked: &AccountIdOf<T>) -> BalanceOf<T> {
    crate::StakedBy::<T>::iter_prefix_values(staked).sum()
}
