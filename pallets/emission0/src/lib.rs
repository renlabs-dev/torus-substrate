#![cfg_attr(not(feature = "std"), no_std)]

mod ext;
mod weights;

pub(crate) use ext::*;
pub use pallet::*;
use polkadot_sdk::frame_support::dispatch::DispatchResult;
use polkadot_sdk::frame_support::pallet_prelude::{ValueQuery, *};
use polkadot_sdk::frame_system::pallet_prelude::OriginFor;
use polkadot_sdk::polkadot_sdk_frame as frame;

#[frame::pallet(dev_mode)]
pub mod pallet {
    use frame::traits::{ConstU128, Currency};

    use super::*;

    #[pallet::storage]
    pub type Weights<T: Config> = StorageMap<
        _,
        Identity,
        AccountIdOf<T>,
        BoundedVec<(AccountIdOf<T>, u16), ConstU32<{ u32::MAX }>>,
    >;

    #[pallet::storage]
    pub type PendingEmission<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, BalanceOf<T>>;

    #[pallet::storage]
    pub type UnitEmission<T: Config> =
        StorageValue<_, BalanceOf<T>, ValueQuery, ConstU128<23148148148>>;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {
        type Currency: Currency<Self::AccountId, Balance = u128> + Send + Sync;
    }

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
            target: AccountIdOf<T>,
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
