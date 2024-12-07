use polkadot_sdk::{
    frame_support::dispatch::DispatchResult, polkadot_sdk_frame::prelude::OriginFor,
};

pub fn set_weights<T: crate::Config>(
    _origin: OriginFor<T>,
    _uids: Vec<u16>,
    _weights: Vec<u16>,
) -> DispatchResult {
    todo!()
}

pub fn delegate_weight_control<T: crate::Config>(
    _origin: OriginFor<T>,
    _target: T::AccountId,
) -> DispatchResult {
    todo!()
}

pub fn regain_weight_control<T: crate::Config>(_origin: OriginFor<T>) -> DispatchResult {
    todo!()
}
