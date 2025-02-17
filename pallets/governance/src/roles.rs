use polkadot_sdk::{
    frame_election_provider_support::Get,
    frame_support::dispatch::DispatchResult,
    frame_system::ensure_signed,
    polkadot_sdk_frame::prelude::OriginFor,
    sp_runtime::{DispatchError, Percent},
};

use crate::{ensure, storage::StorageMap, AccountIdOf, Config, Error};

/// Generic function used to manage the Curator and Allocator maps, which behave
/// similarly.
#[doc(hidden)]
pub fn manage_role<T: Config, M: StorageMap<AccountIdOf<T>, ()>>(
    key: AccountIdOf<T>,
    is_add: bool,
    error: Error<T>,
) -> DispatchResult {
    ensure!(M::contains_key(&key) != is_add, error);
    if is_add {
        M::insert(key, ())
    } else {
        M::remove(&key)
    }
    Ok(())
}

/// Sets a penalty ratio for the given agent.
pub fn penalize_agent<T: Config>(agent_key: AccountIdOf<T>, percentage: u8) -> DispatchResult {
    let percentage = Percent::from_parts(percentage);
    if percentage > T::MaxPenaltyPercentage::get() {
        return Err(Error::<T>::InvalidPenaltyPercentage.into());
    }

    pallet_torus0::Agents::<T>::try_mutate(&agent_key, |agent| {
        let Some(agent) = agent else {
            return Err(Error::<T>::AgentNotFound.into());
        };

        agent.weight_penalty_factor = percentage;

        Ok::<(), DispatchError>(())
    })?;

    Ok(())
}

/// Returns error if the origin is not listed as a curator.
pub fn ensure_curator<T: Config>(origin: OriginFor<T>) -> DispatchResult {
    let key: AccountIdOf<T> = ensure_signed(origin)?;
    if !crate::Curators::<T>::contains_key(key) {
        return Err(Error::<T>::NotCurator.into());
    }

    Ok(())
}

/// Returns error if the origin is not listed as an allocator.
pub fn ensure_allocator<T: Config>(key: &AccountIdOf<T>) -> DispatchResult {
    if !crate::Allocators::<T>::contains_key(key) {
        return Err(Error::<T>::NotAllocator.into());
    }

    Ok(())
}
