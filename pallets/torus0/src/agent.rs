use crate::AccountIdOf;
use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    frame_support::{dispatch::DispatchResult, CloneNoBound},
    polkadot_sdk_frame::prelude::OriginFor,
    sp_runtime::{BoundedVec, Percent},
};
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;

#[derive(CloneNoBound, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Agent<T: crate::Config> {
    pub key: AccountIdOf<T>,
    pub name: BoundedVec<u8, T::MaxAgentNameLengthConstraint>,
    pub address: BoundedVec<u8, T::MaxAgentAddressLengthConstraint>,
    pub weight_factor: Percent,
}

pub fn register<T: crate::Config>(
    _origin: OriginFor<T>,
    _name: Vec<u8>,
    _address: Vec<u8>,
    _agent_key: AccountIdOf<T>,
    _metadata: Option<Vec<u8>>,
) -> DispatchResult {
    todo!()
}

pub fn deregister<T: crate::Config>(_origin: OriginFor<T>) -> DispatchResult {
    todo!()
}

pub fn update<T: crate::Config>(
    _origin: OriginFor<T>,
    _name: Vec<u8>,
    _address: Vec<u8>,
    _metadata: Option<Vec<u8>>,
    _staking_fee: Option<Percent>,
    _weight_control_fee: Option<Percent>,
) -> DispatchResult {
    todo!()
}
