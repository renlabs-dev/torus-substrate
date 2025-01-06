use crate::AccountIdOf;
use crate::{ensure, storage::StorageMap, Config, Error};
use polkadot_sdk::frame_election_provider_support::Get;
use polkadot_sdk::sp_runtime::{DispatchError, Percent};
use polkadot_sdk::{
    frame_support::dispatch::DispatchResult, frame_system::ensure_signed,
    polkadot_sdk_frame::prelude::OriginFor,
};

pub(super) fn manage_role<T: Config, M: StorageMap<AccountIdOf<T>, ()>>(
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

pub fn set_root_curator<T: Config>(key: AccountIdOf<T>) -> DispatchResult {
    crate::RootCurator::<T>::set(Some(key));
    Ok(())
}

pub fn remove_root_curator<T: Config>() -> DispatchResult {
    crate::RootCurator::<T>::set(None);
    Ok(())
}

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

pub fn ensure_root_curator<T: Config>(origin: OriginFor<T>) -> DispatchResult {
    let key: AccountIdOf<T> = ensure_signed(origin)?;
    if crate::RootCurator::<T>::get() != Some(key) {
        return Err(Error::<T>::NotRootCurator.into());
    }

    Ok(())
}

pub fn ensure_curator<T: Config>(origin: OriginFor<T>) -> DispatchResult {
    let key: AccountIdOf<T> = ensure_signed(origin)?;
    if !crate::Curators::<T>::contains_key(&key) && crate::RootCurator::<T>::get() != Some(key) {
        return Err(Error::<T>::NotCurator.into());
    }

    Ok(())
}

pub fn ensure_allocator<T: Config>(key: &AccountIdOf<T>) -> DispatchResult {
    if !crate::Allocators::<T>::contains_key(key) {
        return Err(Error::<T>::NotAllocator.into());
    }

    Ok(())
}
