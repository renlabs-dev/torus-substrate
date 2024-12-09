#![cfg_attr(not(feature = "std"), no_std)]

mod weights;

use crate::frame::testing_prelude::DispatchResult;
use crate::frame::testing_prelude::OriginFor;
pub use pallet::*;
use polkadot_sdk::frame_support::pallet_prelude::{ValueQuery, *};
use polkadot_sdk::polkadot_sdk_frame as frame;

#[frame::pallet]
pub mod pallet {
    use frame::traits::ConstU64;

    use super::*;

    #[pallet::storage]
    pub type Weights<T> =
        StorageMap<_, Identity, u16, BoundedVec<(u16, u16), ConstU32<{ u32::MAX }>>>;

    #[pallet::storage]
    pub type PendingEmission<T> = StorageMap<_, Identity, u16, u64>;

    #[pallet::storage]
    pub type UnitEmission<T> = StorageValue<_, u64, ValueQuery, ConstU64<23148148148>>;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn set_weights_extrinsic(
            origin: OriginFor<T>,
            uids: Vec<u16>,
            weights: Vec<u16>,
        ) -> DispatchResult {
            weights::set_weights::<T>(origin, uids, weights)
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn delegate_weight_control_extrinsic(
            origin: OriginFor<T>,
            target: T::AccountId,
        ) -> DispatchResult {
            weights::delegate_weight_control::<T>(origin, target)
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn regain_weight_control_extrinsic(origin: OriginFor<T>) -> DispatchResult {
            weights::regain_weight_control::<T>(origin)
        }
    }
}
