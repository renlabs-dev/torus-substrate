use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::frame_support::dispatch::DispatchResult;
use polkadot_sdk::frame_support::DebugNoBound;
use polkadot_sdk::polkadot_sdk_frame::prelude::OriginFor;
use polkadot_sdk::sp_core::ConstU32;
use polkadot_sdk::sp_runtime::BoundedVec;
use polkadot_sdk::sp_std::vec::Vec;
use scale_info::TypeInfo;

#[derive(DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct AgentApplication<T: crate::Config> {
    pub id: u32,
    pub payer_key: T::AccountId,
    pub agent_key: T::AccountId,
    pub data: BoundedVec<u8, ConstU32<256>>,
    pub cost: u64,
    pub expires_at: u64,
}

pub fn submit_application<T: crate::Config>(
    _origin: OriginFor<T>,
    _agent_key: T::AccountId,
    _data: Vec<u8>,
) -> DispatchResult {
    todo!()
}

pub fn accept_application<T: crate::Config>(
    _origin: OriginFor<T>,
    _application_id: u32,
) -> DispatchResult {
    todo!()
}

pub fn deny_application<T: crate::Config>(
    _origin: OriginFor<T>,
    _application_id: u32,
) -> DispatchResult {
    todo!()
}
