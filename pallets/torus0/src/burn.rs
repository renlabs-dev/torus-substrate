use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    frame_election_provider_support::Get,
    frame_support::DebugNoBound,
    polkadot_sdk_frame::prelude::BlockNumberFor,
    sp_runtime::{traits::Saturating, FixedU128},
};
use scale_info::{prelude::marker::PhantomData, TypeInfo};

use crate::BalanceOf;

#[derive(Clone, TypeInfo, Decode, Encode, PartialEq, Eq, DebugNoBound, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct BurnConfiguration<T: crate::Config> {
    pub min_burn: BalanceOf<T>,
    pub max_burn: BalanceOf<T>,
    pub adjustment_alpha: u64,
    pub target_registrations_interval: BlockNumberFor<T>,
    pub target_registrations_per_interval: u16,
    pub max_registrations_per_interval: u16,
    pub _pd: PhantomData<T>,
}

impl<T> Default for BurnConfiguration<T>
where
    T: crate::Config,
{
    fn default() -> Self {
        Self {
            min_burn: T::DefaultMinBurn::get(),
            max_burn: T::DefaultMaxBurn::get(),
            adjustment_alpha: T::DefaultAdjustmentAlpha::get(),
            target_registrations_interval: T::DefaultTargetRegistrationsInterval::get(),
            target_registrations_per_interval: T::DefaultTargetRegistrationsPerInterval::get(),
            max_registrations_per_interval: T::DefaultMaxRegistrationsPerInterval::get(),
            _pd: Default::default(),
        }
    }
}

/// Adjusts registration burn for the current block.
///
/// The next burn is calculated by analyzing the last N
/// (`target_registrations_interval`) blocks and increases if the target
/// registrations per interval was reached.
pub fn adjust_burn<T: crate::Config>(current_block: u64) {
    let BurnConfiguration {
        min_burn,
        max_burn,
        adjustment_alpha,
        target_registrations_interval,
        target_registrations_per_interval,
        ..
    } = crate::BurnConfig::<T>::get();

    let target_registrations_interval: u64 = target_registrations_interval
        .into()
        .try_into()
        .expect("block number is 64 bits long");
    let current_burn = crate::Burn::<T>::get();
    let registrations_this_interval = crate::RegistrationsThisInterval::<T>::get();

    let reached_interval = current_block
        .checked_rem(target_registrations_interval)
        .is_some_and(|r| r == 0);

    if !reached_interval {
        return;
    }

    let updated_burn = FixedU128::from_inner(current_burn)
        .const_checked_mul(FixedU128::from_u32(
            registrations_this_interval.saturating_add(target_registrations_per_interval) as u32,
        ))
        .unwrap_or_default()
        .const_checked_div(FixedU128::from_u32(
            target_registrations_per_interval.saturating_mul(2) as u32,
        ))
        .unwrap_or_default();

    let alpha = FixedU128::from_inner(adjustment_alpha as u128)
        .const_checked_div(FixedU128::from_inner(u64::MAX as u128))
        .unwrap_or_else(|| FixedU128::from_inner(0));

    let next_value = alpha
        .const_checked_mul(FixedU128::from_inner(current_burn))
        .unwrap_or_else(|| FixedU128::from_inner(0))
        .saturating_add(
            FixedU128::from_u32(1)
                .saturating_sub(alpha)
                .const_checked_mul(updated_burn)
                .unwrap_or_else(|| FixedU128::from_inner(0)),
        );

    let new_burn = if next_value >= FixedU128::from_inner(max_burn) {
        max_burn
    } else if next_value <= FixedU128::from_inner(min_burn) {
        min_burn
    } else {
        next_value.into_inner()
    };

    crate::Burn::<T>::set(new_burn);
    crate::RegistrationsThisInterval::<T>::set(0);
}
