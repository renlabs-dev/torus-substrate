#![cfg_attr(not(feature = "std"), no_std)]

use polkadot_sdk::frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

/// Weight functions for pallet_permission0
pub trait WeightInfo {
    fn grant_permission() -> Weight;
    fn revoke_permission() -> Weight;
    fn execute_permission() -> Weight;
}

/// Weights for pallet_permission0 using the Substrate node and recommended hardware.
impl WeightInfo for () {
    // Default weights - these should be calibrated properly in the future with benchmarking
    fn grant_permission() -> Weight {
        Weight::from_parts(100_000_000, 0)
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes(5))
    }

    fn revoke_permission() -> Weight {
        Weight::from_parts(75_000_000, 0)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(4))
    }

    fn execute_permission() -> Weight {
        Weight::from_parts(150_000_000, 0)
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(3))
    }
}
