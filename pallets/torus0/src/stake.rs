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
