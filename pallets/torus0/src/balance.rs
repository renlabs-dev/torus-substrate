use polkadot_sdk::{
    frame_support::dispatch::DispatchResult, polkadot_sdk_frame::prelude::OriginFor,
};

use crate::{AccountIdOf, BalanceOf};

pub fn transfer_balance_multiple<T: crate::Config>(
    _origin: OriginFor<T>,
    _destination: AccountIdOf<T>,
    _amount: BalanceOf<T>,
) -> DispatchResult {
    todo!()
}
