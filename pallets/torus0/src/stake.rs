use polkadot_sdk::{
    frame_support::dispatch::DispatchResult, polkadot_sdk_frame::prelude::OriginFor,
    sp_std::vec::Vec,
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

pub fn add_stake_multiple<T: crate::Config>(
    _origin: OriginFor<T>,
    _agent_keys: Vec<AccountIdOf<T>>,
    _amounts: Vec<BalanceOf<T>>,
) -> DispatchResult {
    todo!()
}

pub fn remove_stake_multiple<T: crate::Config>(
    _origin: OriginFor<T>,
    _agent_keys: Vec<AccountIdOf<T>>,
    _amounts: Vec<BalanceOf<T>>,
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
