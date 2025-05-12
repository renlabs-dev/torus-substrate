#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused)]

mod ext;

mod faucet;

pub(crate) use ext::*;

use crate::frame::traits::DispatchInfoOf;
use crate::frame::traits::SignedExtension;
pub use pallet::*;
use pallet_torus0_api::Torus0Api;
use polkadot_sdk::sp_runtime::Saturating;
use polkadot_sdk::{
    frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency},
    frame_system::pallet_prelude::OriginFor,
    polkadot_sdk_frame::{self as frame, traits::StaticLookup},
};
use scale_info::prelude::vec::Vec;

type AccountIdLookupOf<T> =
    <<T as polkadot_sdk::frame_system::Config>::Lookup as StaticLookup>::Source;

#[frame::pallet]
pub mod pallet {

    use frame::prelude::{ensure_none, BlockNumberFor};

    use super::*;

    #[cfg(feature = "testnet")]
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    #[cfg(not(feature = "testnet"))]
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    #[pallet::config(with_default)]
    pub trait Config: polkadot_sdk::frame_system::Config {
        #[pallet::no_default_bounds]
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        type Currency: Currency<Self::AccountId> + Send + Sync;

        type Torus: Torus0Api<
            Self::AccountId,
            <Self::Currency as Currency<Self::AccountId>>::Balance,
            <Self::Currency as Currency<Self::AccountId>>::NegativeImbalance,
        >;
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(_: TransactionSource, call: &Self::Call) -> TransactionValidity {
            #[cfg(not(feature = "testnet"))]
            {
                InvalidTransaction::Call.into()
            }

            #[cfg(feature = "testnet")]
            {
                #[allow(unused_variables)]
                let Call::faucet {
                    block_number,
                    nonce,
                    work,
                    key,
                } = call
                else {
                    return InvalidTransaction::Call.into();
                };

                let current_block = <polkadot_sdk::frame_system::Pallet<T>>::block_number();
                let target_block = BlockNumberFor::<T>::from(*block_number as u32);
                if current_block < target_block {
                    return InvalidTransaction::Future.into();
                }

                let key = T::Lookup::lookup(key.clone())?;

                let key_balance = T::Currency::free_balance(&key);
                let key_stake = <T::Torus>::sum_staking_to(&key);

                let total_worth: BalanceOf<T> = key_balance.saturating_add(key_stake);
                let threshold: BalanceOf<T> = 50_000_000_000_000_000_000_000u128
                    .try_into()
                    .unwrap_or_default();
                if total_worth >= threshold {
                    return InvalidTransaction::Custom(0).into();
                }

                ValidTransaction::with_tag_prefix("RunFaucet")
                    .priority(0)
                    .longevity(5)
                    .and_provides(key)
                    .propagate(true)
                    .build()
            }
        }
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[cfg(feature = "testnet")]
        #[pallet::call_index(1)]
        #[pallet::weight((
            Weight::from_parts(85_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(16))
            .saturating_add(T::DbWeight::get().writes(28)),
            DispatchClass::Operational,
            Pays::No
        ))]
        pub fn faucet(
            origin: OriginFor<T>,
            block_number: u64,
            nonce: u64,
            work: Vec<u8>,
            key: AccountIdLookupOf<T>,
        ) -> DispatchResult {
            ensure_none(origin)?;

            let key = T::Lookup::lookup(key)?;
            faucet::execute::<T>(key, block_number, nonce, work)
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        Faucet(AccountIdOf<T>, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidWorkBlock,
        InvalidDifficulty,
        InvalidSeal,
    }
}
