use crate::AccountIdOf;
use polkadot_sdk::frame_election_provider_support::Get;
use polkadot_sdk::sp_runtime::{DispatchError, Percent};
use polkadot_sdk::{
    frame_support::dispatch::DispatchResult, frame_system::ensure_signed,
    polkadot_sdk_frame::prelude::OriginFor,
};

pub fn add_curator<T: crate::Config>(key: AccountIdOf<T>) -> DispatchResult {
    if crate::Curators::<T>::contains_key(&key) {
        return Err(crate::Error::<T>::AlreadyCurator.into());
    }

    crate::Curators::<T>::insert(key, ());
    Ok(())
}

pub fn remove_curator<T: crate::Config>(key: AccountIdOf<T>) -> DispatchResult {
    if !crate::Curators::<T>::contains_key(&key) {
        return Err(crate::Error::<T>::NotCurator.into());
    }

    crate::Curators::<T>::remove(&key);
    Ok(())
}

pub fn penalize_agent<T: crate::Config>(
    agent_key: AccountIdOf<T>,
    percentage: u8,
) -> DispatchResult {
    if percentage > T::MaxPenaltyPercentage::get() {
        return Err(crate::Error::<T>::InvalidPenaltyPercentage.into());
    }

    pallet_torus0::Agents::<T>::try_mutate(&agent_key, |agent| {
        let Some(agent) = agent else {
            return Err(crate::Error::<T>::AgentNotFound.into());
        };

        agent.weight_penalty_factor = Percent::from_percent(100u8.saturating_sub(percentage));

        Ok::<(), DispatchError>(())
    })?;

    Ok(())
}

pub fn ensure_curator<T: crate::Config>(origin: OriginFor<T>) -> DispatchResult {
    let key: AccountIdOf<T> = ensure_signed(origin)?;
    if !crate::Curators::<T>::contains_key(key) {
        return Err(crate::Error::<T>::NotCurator.into());
    }

    Ok(())
}
