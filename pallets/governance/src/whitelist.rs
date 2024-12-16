use polkadot_sdk::frame_support::dispatch::DispatchResult;

use crate::AccountIdOf;

pub fn add_to_whitelist<T: crate::Config>(key: AccountIdOf<T>) -> DispatchResult {
    if is_whitelisted::<T>(&key) {
        return Err(crate::Error::<T>::AlreadyWhitelisted.into());
    }

    crate::Whitelist::<T>::insert(key.clone(), ());
    crate::Pallet::<T>::deposit_event(crate::Event::<T>::WhitelistAdded(key));
    Ok(())
}

pub fn remove_from_whitelist<T: crate::Config>(key: AccountIdOf<T>) -> DispatchResult {
    if !is_whitelisted::<T>(&key) {
        return Err(crate::Error::<T>::NotWhitelisted.into());
    }

    crate::Whitelist::<T>::remove(&key);
    crate::Pallet::<T>::deposit_event(crate::Event::<T>::WhitelistRemoved(key));
    Ok(())
}

pub fn is_whitelisted<T: crate::Config>(key: &AccountIdOf<T>) -> bool {
    crate::Whitelist::<T>::contains_key(key)
}
