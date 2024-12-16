use polkadot_sdk::sp_std::collections::btree_map::BTreeMap;

use polkadot_sdk::{
    frame_support::dispatch::DispatchResult, polkadot_sdk_frame::prelude::OriginFor,
};

use crate::{AccountIdOf, BalanceOf};

pub fn add_stake<T: crate::Config>(
    _origin: OriginFor<T>,
    _agent_key: AccountIdOf<T>,
    _amount: BalanceOf<T>,
) -> DispatchResult {
    todo!()
}

pub fn remove_stake<T: crate::Config>(
    _origin: OriginFor<T>,
    _agent_key: AccountIdOf<T>,
    _amount: BalanceOf<T>,
) -> DispatchResult {
    todo!()
}

pub fn transfer_stake<T: crate::Config>(
    _origin: OriginFor<T>,
    _agent_key: AccountIdOf<T>,
    _new_agent_key: AccountIdOf<T>,
    _amount: BalanceOf<T>,
) -> DispatchResult {
    todo!()
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
