use polkadot_sdk::sp_std::collections::btree_map::BTreeMap;

use crate::agent;
use crate::{AccountIdOf, BalanceOf};
use polkadot_sdk::frame_support::traits::{Currency, ExistenceRequirement, WithdrawReasons};
use polkadot_sdk::frame_support::{dispatch::DispatchResult, ensure};

pub fn add_stake<T: crate::Config>(
    key: AccountIdOf<T>,
    agent_key: AccountIdOf<T>,
    amount: BalanceOf<T>,
) -> DispatchResult {
    ensure!(
        amount >= crate::MinimumAllowedStake::<T>::get(),
        crate::Error::<T>::StakeTooSmall
    );

    ensure!(
        agent::exists::<T>(&agent_key),
        crate::Error::<T>::AgentDoesNotExist
    );

    let _ = <T as crate::Config>::Currency::withdraw(
        &key,
        amount,
        WithdrawReasons::TRANSFER,
        ExistenceRequirement::KeepAlive,
    )
    .map_err(|_| crate::Error::<T>::NotEnoughBalanceToStake)?;

    crate::StakingTo::<T>::mutate(&key, &agent_key, |stake| {
        *stake = Some(stake.unwrap_or(0).saturating_add(amount))
    });

    crate::StakedBy::<T>::mutate(&agent_key, &key, |stake| {
        *stake = Some(stake.unwrap_or(0).saturating_add(amount))
    });

    crate::TotalStake::<T>::mutate(|total_stake| *total_stake = total_stake.saturating_add(amount));

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::StakeAdded(key, agent_key, amount));

    Ok(())
}

pub fn remove_stake<T: crate::Config>(
    key: AccountIdOf<T>,
    agent_key: AccountIdOf<T>,
    amount: BalanceOf<T>,
) -> DispatchResult {
    ensure!(
        amount >= crate::MinimumAllowedStake::<T>::get(),
        crate::Error::<T>::StakeTooSmall
    );

    ensure!(
        agent::exists::<T>(&agent_key),
        crate::Error::<T>::AgentDoesNotExist
    );

    ensure!(
        crate::StakingTo::<T>::get(&key, &agent_key).unwrap_or(0) >= amount,
        crate::Error::<T>::NotEnoughStakeToWithdraw
    );

    crate::StakingTo::<T>::mutate(&key, &agent_key, |stake| {
        *stake = Some(stake.unwrap_or(0).saturating_sub(amount))
    });

    crate::StakedBy::<T>::mutate(&agent_key, &key, |stake| {
        *stake = Some(stake.unwrap_or(0).saturating_sub(amount))
    });

    crate::TotalStake::<T>::mutate(|total_stake| *total_stake = total_stake.saturating_sub(amount));

    let _ = <T as crate::Config>::Currency::deposit_creating(&key, amount);

    Ok(())
}

pub fn transfer_stake<T: crate::Config>(
    key: AccountIdOf<T>,
    agent_key: AccountIdOf<T>,
    new_agent_key: AccountIdOf<T>,
    amount: BalanceOf<T>,
) -> DispatchResult {
    remove_stake::<T>(key.clone(), agent_key, amount)?;
    add_stake::<T>(key, new_agent_key, amount)?;
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
pub fn sum_staked_by<T: crate::Config>(staked: &AccountIdOf<T>) -> BalanceOf<T> {
    crate::StakedBy::<T>::iter_prefix_values(staked).sum()
}
