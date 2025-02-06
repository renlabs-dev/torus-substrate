use crate::BalanceOf;
use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::frame_election_provider_support::Get;
use polkadot_sdk::frame_support::DebugNoBound;
use polkadot_sdk::polkadot_sdk_frame::prelude::BlockNumberFor;
use scale_info::prelude::marker::PhantomData;
use scale_info::TypeInfo;
use substrate_fixed::types::I110F18;

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

    let updated_burn: I110F18 = I110F18::from_num(current_burn)
        .checked_mul(I110F18::from_num(
            registrations_this_interval.saturating_add(target_registrations_per_interval),
        ))
        .unwrap_or_default()
        .checked_div(I110F18::from_num(
            target_registrations_per_interval.saturating_add(target_registrations_per_interval),
        ))
        .unwrap_or_default();

    let alpha: I110F18 = I110F18::from_num(adjustment_alpha)
        .checked_div(I110F18::from_num(u64::MAX))
        .unwrap_or_else(|| I110F18::from_num(0));

    let next_value: I110F18 = alpha
        .checked_mul(I110F18::from_num(current_burn))
        .unwrap_or_else(|| I110F18::from_num(0))
        .saturating_add(
            I110F18::from_num(1.0)
                .saturating_sub(alpha)
                .checked_mul(updated_burn)
                .unwrap_or_else(|| I110F18::from_num(0)),
        );

    let new_burn = if next_value >= I110F18::from_num(max_burn) {
        max_burn
    } else if next_value <= I110F18::from_num(min_burn) {
        min_burn
    } else {
        next_value.to_num::<BalanceOf<T>>()
    };

    crate::Burn::<T>::set(new_burn);
    crate::RegistrationsThisInterval::<T>::set(0);
}
