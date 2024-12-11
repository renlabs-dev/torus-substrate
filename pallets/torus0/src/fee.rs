use core::marker::PhantomData;

use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::frame_election_provider_support::Get;
use polkadot_sdk::{frame_support::DebugNoBound, sp_runtime::Percent};
use scale_info::TypeInfo;

#[derive(DebugNoBound, Decode, Encode, MaxEncodedLen, PartialEq, Eq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ValidatorFeeConstraints<T: crate::Config> {
    pub min_stake_delegation_fee: Percent,
    pub min_weight_control_fee: Percent,
    pub _pd: PhantomData<T>,
}

impl<T: crate::Config> Default for ValidatorFeeConstraints<T> {
    fn default() -> Self {
        Self {
            min_stake_delegation_fee: Percent::from_percent(T::DefaultMinStakingFee::get()),
            min_weight_control_fee: Percent::from_percent(T::DefaultMinWeightControlFee::get()),
            _pd: PhantomData,
        }
    }
}

#[derive(DebugNoBound, Decode, Encode, MaxEncodedLen, PartialEq, Eq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ValidatorFee<T: crate::Config> {
    pub stake_delegation_fee: Percent,
    pub weight_control_fee: Percent,
    pub _pd: PhantomData<T>,
}

impl<T: crate::Config> Default for ValidatorFee<T> {
    fn default() -> Self {
        Self {
            stake_delegation_fee: Percent::from_percent(T::DefaultMinStakingFee::get()),
            weight_control_fee: Percent::from_percent(T::DefaultMinWeightControlFee::get()),
            _pd: PhantomData,
        }
    }
}
