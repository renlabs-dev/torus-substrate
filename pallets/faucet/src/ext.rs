//! Type aliases for the faucet pallet
//!
//! This module contains type definitions used throughout the faucet pallet code.

use polkadot_sdk::frame_support::traits::Currency;

/// Type alias for the Balance type used in the pallet
///
/// This represents the token amount type in the system, derived from the Currency configuration.
pub(super) type BalanceOf<T> = <<T as crate::Config>::Currency as Currency<
    <T as polkadot_sdk::frame_system::Config>::AccountId,
>>::Balance;

/// Type alias for the AccountId type used in the pallet
///
/// This represents the account identifier type in the system.
pub(super) type AccountIdOf<T> = <T as polkadot_sdk::frame_system::Config>::AccountId;
