use polkadot_sdk::{
    frame_support::dispatch::DispatchResult, polkadot_sdk_frame::prelude::OriginFor,
};

pub fn add_vote<T: crate::Config>(
    _origin: OriginFor<T>,
    _proposal_id: u64,
    _agree: bool,
) -> DispatchResult {
    todo!()
}

pub fn remove_vote<T: crate::Config>(_origin: OriginFor<T>, _proposal_id: u64) -> DispatchResult {
    todo!()
}

pub fn enable_delegation<T: crate::Config>(_origin: OriginFor<T>) -> DispatchResult {
    todo!()
}

pub fn disable_delegation<T: crate::Config>(_origin: OriginFor<T>) -> DispatchResult {
    todo!()
}
