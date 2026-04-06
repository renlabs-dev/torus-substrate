//! Weight definitions for `pallet_authority_admin`.

use core::marker::PhantomData;

use polkadot_sdk::{
    frame_support::{
        traits::Get,
        weights::{Weight, constants::RocksDbWeight},
    },
    frame_system,
};

/// Weight functions needed for `pallet_authority_admin`.
pub trait WeightInfo {
    fn set_authorities() -> Weight;
}

/// Weights for `pallet_authority_admin` using the runtime database weights.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn set_authorities() -> Weight {
        Weight::from_parts(25_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
}

impl WeightInfo for () {
    fn set_authorities() -> Weight {
        Weight::from_parts(25_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(5_u64))
    }
}
