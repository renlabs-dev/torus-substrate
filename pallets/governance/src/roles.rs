use pallet_permission0_api::{CuratorPermissions, Permission0CuratorApi};
use polkadot_sdk::{
    frame_election_provider_support::Get,
    frame_support::dispatch::DispatchResult,
    polkadot_sdk_frame::prelude::OriginFor,
    sp_runtime::{DispatchError, Percent},
};

use crate::{AccountIdOf, Allocators, Config, Error, Event, ensure};

/// Adds a new allocator to the network, checking wether it's already registered.
#[doc(hidden)]
pub fn add_allocator<T: Config>(key: AccountIdOf<T>) -> DispatchResult {
    ensure!(
        !Allocators::<T>::contains_key(&key),
        Error::<T>::AlreadyAllocator
    );
    Allocators::<T>::insert(key, ());
    Ok(())
}

/// Removes an existing allocator to the network, checking wether it's registered.
#[doc(hidden)]
pub fn remove_allocator<T: Config>(key: AccountIdOf<T>) -> DispatchResult {
    ensure!(
        Allocators::<T>::contains_key(&key),
        Error::<T>::NotAllocator
    );
    Allocators::<T>::remove(&key);
    Ok(())
}

/// Sets a penalty ratio for the given agent.
pub fn penalize_agent<T: Config>(
    origin: OriginFor<T>,
    agent_key: AccountIdOf<T>,
    percentage: u8,
) -> DispatchResult {
    let curator = <T as Config>::Permission0::ensure_curator_permission(
        origin,
        CuratorPermissions::PENALTY_CONTROL,
    )?;

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

    crate::Pallet::<T>::deposit_event(Event::PenaltyApplied {
        curator,
        agent: agent_key,
        penalty: percentage,
    });

    Ok(())
}

/// Returns error if the origin is not listed as an allocator.
pub fn ensure_allocator<T: Config>(key: &AccountIdOf<T>) -> DispatchResult {
    if !crate::Allocators::<T>::contains_key(key) {
        return Err(Error::<T>::NotAllocator.into());
    }

    Ok(())
}
