use core::marker::PhantomData;

use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::frame_election_provider_support::Get;
use polkadot_sdk::{frame_support::DebugNoBound, sp_runtime::Percent};
use scale_info::TypeInfo;

#[derive(DebugNoBound, Decode, Encode, MaxEncodedLen, PartialEq, Eq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ValidatorFeeConstraints<T: crate::Config> {
    pub min_staking_fee: Percent,
    pub min_weight_control_fee: Percent,
    pub _pd: PhantomData<T>,
}

impl<T: crate::Config> Default for ValidatorFeeConstraints<T> {
    fn default() -> Self {
        Self {
            min_staking_fee: Percent::from_percent(T::DefaultMinStakingFee::get()),
            min_weight_control_fee: Percent::from_percent(T::DefaultMinWeightControlFee::get()),
            _pd: PhantomData,
        }
    }
}

#[derive(DebugNoBound, Decode, Encode, MaxEncodedLen, PartialEq, Eq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ValidatorFee<T: crate::Config> {
    pub staking_fee: Percent,
    pub weight_control_fee: Percent,
    pub _pd: PhantomData<T>,
}

impl<T: crate::Config> Default for ValidatorFee<T> {
    fn default() -> Self {
        let fee_constraints = crate::FeeConstraints::<T>::get();

        Self {
            staking_fee: fee_constraints.min_staking_fee,
            weight_control_fee: fee_constraints.min_weight_control_fee,
            _pd: PhantomData,
        }
    }
}
