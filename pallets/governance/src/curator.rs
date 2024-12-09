use polkadot_sdk::{
    frame_support::dispatch::DispatchResult, polkadot_sdk_frame::prelude::OriginFor,
};

use crate::AccountIdOf;

pub fn add_curator<T: crate::Config>(
    _origin: OriginFor<T>,
    _key: AccountIdOf<T>,
) -> DispatchResult {
    todo!()
}

pub fn remove_curator<T: crate::Config>(
    _origin: OriginFor<T>,
    _key: AccountIdOf<T>,
) -> DispatchResult {
    todo!()
}
