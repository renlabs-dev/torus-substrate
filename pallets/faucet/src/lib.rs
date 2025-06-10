//! # Faucet Pallet
//!
//! A pallet that allows users to get test tokens on the testnet by performing proof-of-work.
//!
//! ## Overview
//!
//! The Faucet pallet provides a mechanism for users to acquire test tokens on the testnet
//! through a proof-of-work challenge. This enables developers to test their applications
//! on the testnet without requiring real tokens.
//!
//! The pallet is only enabled on testnets via the `testnet` feature flag and is disabled
//! on production networks to prevent token generation outside of the normal emission schedule.
//!
//! ## Features
//!
//! - **Proof-of-Work**: Requires users to perform computational work to prevent abuse
//! - **Token Distribution**: Provides a fixed amount of tokens to users who complete the challenge
//! - **Rate Limiting**: Prevents wealthy accounts from repeatedly requesting tokens
//! - **Recent Block Verification**: Ensures that proof-of-work was done recently
//!
//! ## Interface
//!
//! ### Extrinsics
//!
//! * `faucet` - Submit proof-of-work to receive test tokens
//!
//! ## Usage
//!
//! To request tokens from the faucet, users need to:
//!
//! 1. Get the hash of a recent block (within the last 3 blocks)
//! 2. Generate a proof-of-work based on block hash, account ID, and a nonce
//! 3. Submit the proof to the faucet extrinsic
//!
//! If the proof is valid and the account's total balance (including staked tokens) is below
//! the threshold, the account will receive the configured amount of test tokens.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused)]

mod ext;

pub mod faucet;

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

/// Type for looking up accounts by their address/pubkey
type AccountIdLookupOf<T> =
    <<T as polkadot_sdk::frame_system::Config>::Lookup as StaticLookup>::Source;

#[frame::pallet]
pub mod pallet {

    use frame::prelude::{ensure_none, BlockNumberFor};

    use super::*;

    /// Storage version for the pallet
    #[cfg(feature = "testnet")]
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    /// Storage version for the pallet (production networks)
    #[cfg(not(feature = "testnet"))]
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    /// Configuration trait for the faucet pallet
    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        /// The currency mechanism, used for token distribution
        type Currency: Currency<Self::AccountId> + Send + Sync;

        /// The Torus interface for accessing network functions
        type Torus: Torus0Api<Self::AccountId, BalanceOf<Self>>;
    }

    /// Implementation of the unsigned transaction validation for the faucet pallet
    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        /// Validates an unsigned faucet transaction
        ///
        /// Returns:
        /// - On production networks: Always invalid
        /// - On testnets: Invalid if:
        ///   - The call is not to the faucet extrinsic
        ///   - The block number is in the future
        ///   - The account already has enough funds (free balance + staked)
        fn validate_unsigned(_: TransactionSource, call: &Self::Call) -> TransactionValidity {
            // Disable faucet functionality on production networks (non-testnet builds)
            #[cfg(not(feature = "testnet"))]
            {
                InvalidTransaction::Call.into()
            }

            // Enable faucet functionality only on testnet builds
            #[cfg(feature = "testnet")]
            {
                // Extract parameters from the call
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

                // Ensure the block number is not in the future
                let current_block = <polkadot_sdk::frame_system::Pallet<T>>::block_number();
                let target_block = BlockNumberFor::<T>::from(*block_number as u32);
                if current_block < target_block {
                    return InvalidTransaction::Future.into();
                }

                // Lookup the account from the provided key
                let key = T::Lookup::lookup(key.clone())?;

                // Check if the account already has sufficient funds
                // We consider both free balance and staked tokens
                let key_balance = T::Currency::free_balance(&key);
                let key_stake = <T::Torus>::sum_staking_to(&key);

                // Threshold for rejecting faucet requests (50,000 tokens)
                let total_worth: BalanceOf<T> = key_balance.saturating_add(key_stake);
                let threshold: BalanceOf<T> = 50_000_000_000_000_000_000_000u128
                    .try_into()
                    .unwrap_or_default();
                if total_worth >= threshold {
                    return InvalidTransaction::Custom(0).into();
                }

                // Build a valid transaction if all checks pass
                ValidTransaction::with_tag_prefix("RunFaucet")
                    .priority(0)
                    .longevity(5)
                    .and_provides(key)
                    .propagate(true)
                    .build()
            }
        }
    }

    /// The main pallet struct
    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Callable functions for the faucet pallet
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Request tokens from the faucet by performing proof of work
        ///
        /// This extrinsic is only available on testnets. It requires the user to perform
        /// proof-of-work by finding a nonce that, when combined with a recent block hash
        /// and the user's account ID, produces a hash that meets the difficulty requirement.
        ///
        /// The account must have a total balance (free + staked) below the threshold to be eligible.
        ///
        /// # Parameters
        /// * `origin` - Must be None (unsigned)
        /// * `block_number` - A recent block number (within 3 blocks)
        /// * `nonce` - A value that makes the resulting hash meet the difficulty requirement
        /// * `work` - The hash result of the proof of work
        /// * `key` - The account ID that will receive the tokens
        ///
        /// # Weight
        /// * Read operations: 16
        /// * Write operations: 28
        /// * Does not pay fees
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
            // Ensure the call is unsigned (from None origin)
            ensure_none(origin)?;

            // Look up the account ID and execute the faucet logic
            let key = T::Lookup::lookup(key)?;
            faucet::execute::<T>(key, block_number, nonce, work)
        }
    }

    /// Events emitted by the faucet pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Tokens were successfully distributed by the faucet
        /// [account, amount]
        Faucet(AccountIdOf<T>, BalanceOf<T>),
    }

    /// Errors that can occur in the faucet pallet
    #[pallet::error]
    pub enum Error<T> {
        /// The block number provided is invalid (too old or in the future)
        InvalidWorkBlock,
        /// The proof-of-work does not meet the required difficulty
        InvalidDifficulty,
        /// The seal (hash) is invalid for the given parameters
        InvalidSeal,
    }
}
