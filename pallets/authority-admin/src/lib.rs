#![cfg_attr(not(feature = "std"), no_std)]

pub mod benchmarking;
pub mod weights;

pub use pallet::*;
pub use weights::*;

use frame::prelude::ensure;
use polkadot_sdk::{
    frame_support::{
        dispatch::DispatchResult, pallet_prelude::*, storage_alias, traits::EnsureOrigin,
    },
    frame_system::pallet_prelude::OriginFor,
    pallet_aura, pallet_grandpa, polkadot_sdk_frame as frame,
    sp_consensus_grandpa::{AuthorityList, SetId},
    sp_runtime::traits::Zero,
};
use scale_info::prelude::vec::Vec;

#[storage_alias(pallet_name)]
type CurrentSetId<T: Config> = StorageValue<pallet_grandpa::Pallet<T>, SetId, ValueQuery>;

/// Updates the GRANDPA set id for runtimes that do not use pallet_session.
pub fn increment_grandpa_set_id<T: Config>() -> DispatchResult {
    CurrentSetId::<T>::try_mutate(|current_set_id| {
        let Some(next_set_id) = current_set_id.checked_add(1) else {
            return Err(Error::<T>::GrandpaSetIdOverflow.into());
        };

        *current_set_id = next_set_id;

        Ok(())
    })
}

/// Schedules a zero-delay GRANDPA change and updates Aura immediately.
pub fn set_authorities<T: Config>(
    aura_authorities: Vec<<T as pallet_aura::Config>::AuthorityId>,
    grandpa_authorities: AuthorityList,
) -> DispatchResult {
    let aura_authorities =
        BoundedVec::<_, <T as pallet_aura::Config>::MaxAuthorities>::try_from(aura_authorities)
            .map_err(|_| Error::<T>::TooManyAuraAuthorities)?;

    ensure!(
        !aura_authorities.is_empty(),
        Error::<T>::EmptyAuraAuthorities
    );
    ensure!(
        !grandpa_authorities.is_empty(),
        Error::<T>::EmptyGrandpaAuthorities
    );
    ensure!(
        grandpa_authorities
            .iter()
            .all(|(_, authority_weight)| *authority_weight > 0),
        Error::<T>::InvalidGrandpaAuthorityWeight
    );

    let grandpa_authority_count: u32 = grandpa_authorities
        .len()
        .try_into()
        .map_err(|_| Error::<T>::TooManyGrandpaAuthorities)?;

    ensure!(
        grandpa_authority_count <= <T as pallet_grandpa::Config>::MaxAuthorities::get(),
        Error::<T>::TooManyGrandpaAuthorities
    );
    ensure!(
        pallet_grandpa::Pallet::<T>::pending_change().is_none(),
        Error::<T>::GrandpaAuthorityChangePending
    );

    pallet_grandpa::Pallet::<T>::schedule_change(grandpa_authorities, Zero::zero(), None)?;
    increment_grandpa_set_id::<T>()?;

    let aura_authority_count: u32 = aura_authorities
        .len()
        .try_into()
        .map_err(|_| Error::<T>::TooManyAuraAuthorities)?;

    pallet_aura::Pallet::<T>::change_authorities(aura_authorities);

    Pallet::<T>::deposit_event(Event::<T>::AuthoritySetChanged {
        aura_authority_count,
        grandpa_authority_count,
    });

    Ok(())
}

#[frame::pallet]
pub mod pallet {
    use super::*;

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    #[pallet::config]
    pub trait Config:
        polkadot_sdk::frame_system::Config + pallet_aura::Config + pallet_grandpa::Config
    {
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        /// Origin allowed to rotate consensus authorities.
        type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Updates the active Aura authority set and schedules the matching
        /// GRANDPA authority set with zero delay.
        #[pallet::call_index(0)]
        #[pallet::weight((<T as Config>::WeightInfo::set_authorities(), DispatchClass::Operational, Pays::No))]
        pub fn set_authorities(
            origin: OriginFor<T>,
            aura_authorities: Vec<<T as pallet_aura::Config>::AuthorityId>,
            grandpa_authorities: AuthorityList,
        ) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin)?;
            crate::set_authorities::<T>(aura_authorities, grandpa_authorities)
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Consensus authority sets were updated.
        AuthoritySetChanged {
            aura_authority_count: u32,
            grandpa_authority_count: u32,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Aura authority set cannot be empty.
        EmptyAuraAuthorities,
        /// GRANDPA authority set cannot be empty.
        EmptyGrandpaAuthorities,
        /// Aura authority set exceeds the configured maximum.
        TooManyAuraAuthorities,
        /// GRANDPA authority set exceeds the configured maximum.
        TooManyGrandpaAuthorities,
        /// GRANDPA authority weights must be greater than zero.
        InvalidGrandpaAuthorityWeight,
        /// A GRANDPA authority change is already pending.
        GrandpaAuthorityChangePending,
        /// GRANDPA set id overflowed while scheduling the new authority set.
        GrandpaSetIdOverflow,
    }
}
