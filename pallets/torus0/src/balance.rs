use polkadot_sdk::{
    frame_support::dispatch::DispatchResult, polkadot_sdk_frame::prelude::OriginFor,
};
use scale_info::prelude::vec::Vec;

use crate::{AccountIdOf, BalanceOf};

pub fn transfer_balance_multiple<T: crate::Config>(
    _origin: OriginFor<T>,
    _destinations: Vec<AccountIdOf<T>>,
    _amounts: Vec<BalanceOf<T>>,
) -> DispatchResult {
    todo!()
}
