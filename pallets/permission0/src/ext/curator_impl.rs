use crate::{
    generate_permission_id, pallet, update_permission_indices, Config, CuratorPermissions,
    CuratorScope, Error, Event, Pallet, PermissionContract, PermissionDuration, PermissionScope,
    Permissions, PermissionsByRecipient, RevocationTerms,
};

use pallet_permission0_api::{
    CuratorPermissions as ApiCuratorPermissions, Permission0CuratorApi,
    PermissionDuration as ApiPermissionDuration, PermissionId,
    RevocationTerms as ApiRevocationTerms,
};
use polkadot_sdk::frame_system::{ensure_root, ensure_signed_or_root};
use polkadot_sdk::sp_core::Get;
use polkadot_sdk::sp_runtime::traits::{AccountIdConversion, Saturating};
use polkadot_sdk::{
    frame_support::ensure,
    frame_system,
    polkadot_sdk_frame::prelude::{BlockNumberFor, OriginFor},
    sp_runtime::{DispatchError, DispatchResult},
};

impl<T: Config> Permission0CuratorApi<T::AccountId, OriginFor<T>, BlockNumberFor<T>>
    for pallet::Pallet<T>
{
    fn delegate_curator_permission(
        delegator: OriginFor<T>,
        recipient: T::AccountId,
        flags: ApiCuratorPermissions,
        cooldown: Option<BlockNumberFor<T>>,
        duration: ApiPermissionDuration<BlockNumberFor<T>>,
        revocation: ApiRevocationTerms<T::AccountId, BlockNumberFor<T>>,
    ) -> Result<PermissionId, DispatchError> {
        let duration = super::translate_duration::<T>(duration)?;
        let revocation = super::translate_revocation_terms::<T>(revocation)?;

        let flags = CuratorPermissions::from_bits_truncate(flags.bits());
        delegate_curator_permission_impl(
            delegator, recipient, flags, cooldown, duration, revocation,
        )
    }

    fn ensure_curator_permission(
        recipient: OriginFor<T>,
        flags: ApiCuratorPermissions,
    ) -> Result<T::AccountId, DispatchError> {
        let Some(recipient) = ensure_signed_or_root(recipient)? else {
            return Ok(T::PalletId::get().into_account_truncating());
        };

        let permissions = PermissionsByRecipient::<T>::get(&recipient);
        let Some((_, contract)) = permissions.into_iter().find_map(|permission_id| {
            let contract = Permissions::<T>::get(permission_id)?;

            if matches!(&contract.scope, PermissionScope::Curator(_)) {
                Some((permission_id, contract))
            } else {
                None
            }
        }) else {
            return Err(Error::<T>::PermissionNotFound.into());
        };

        let PermissionScope::Curator(scope) = &contract.scope else {
            return Err(Error::<T>::PermissionNotFound.into());
        };

        let flags = CuratorPermissions::from_bits_truncate(flags.bits());
        if !scope.has_permission(flags) {
            return Err(Error::<T>::PermissionNotFound.into());
        }

        if let Some(cooldown) = scope.cooldown {
            let now = <frame_system::Pallet<T>>::block_number();

            if contract
                .last_execution()
                .is_some_and(|last_execution| last_execution.saturating_add(cooldown) > now)
            {
                return Err(Error::<T>::PermissionInCooldown.into());
            }
        }

        Ok(recipient)
    }

    fn get_curator_permission(recipient: &T::AccountId) -> Option<PermissionId> {
        PermissionsByRecipient::<T>::get(recipient)
            .into_iter()
            .find_map(|permission_id| {
                let contract = Permissions::<T>::get(permission_id)?;

                if matches!(&contract.scope, PermissionScope::Curator(_)) {
                    Some(permission_id)
                } else {
                    None
                }
            })
    }
}

pub fn delegate_curator_permission_impl<T: Config>(
    delegator: OriginFor<T>,
    recipient: T::AccountId,
    mut flags: CuratorPermissions,
    cooldown: Option<BlockNumberFor<T>>,
    duration: PermissionDuration<T>,
    revocation: RevocationTerms<T>,
) -> Result<PermissionId, DispatchError> {
    ensure_root(delegator)?;

    // Root permission is not delegateable
    flags.remove(CuratorPermissions::ROOT);

    ensure!(!flags.is_empty(), Error::<T>::InvalidCuratorPermissions);

    let delegator = <T as Config>::PalletId::get().into_account_truncating();

    // We do not check for the ROOT curator permission at the moment.
    // This is mainly due to our use of a SUDO key at the moment.
    // Once we move away from centralized chain management, a ROOT curator
    // will be appointed by the system.

    for perm in PermissionsByRecipient::<T>::get(&recipient) {
        let Some(contract) = Permissions::<T>::get(perm) else {
            continue;
        };

        if matches!(&contract.scope, PermissionScope::Curator(_)) {
            return Err(Error::<T>::DuplicatePermission.into());
        }
    }

    let scope = PermissionScope::Curator(CuratorScope { flags, cooldown });
    let permission_id = generate_permission_id::<T>(&delegator, &recipient, &scope)?;

    let contract = PermissionContract::<T>::new(
        delegator,
        recipient,
        scope,
        duration,
        revocation,
        crate::EnforcementAuthority::None,
        1,
    );

    Permissions::<T>::insert(permission_id, &contract);
    update_permission_indices::<T>(&contract.delegator, &contract.recipient, permission_id)?;

    <Pallet<T>>::deposit_event(Event::PermissionDelegated {
        delegator: contract.delegator,
        recipient: contract.recipient,
        permission_id,
    });

    Ok(permission_id)
}

pub fn execute_permission_impl<T: Config>(permission_id: &PermissionId) -> DispatchResult {
    if let Some(mut contract) = Permissions::<T>::get(permission_id) {
        contract.tick_execution(<frame_system::Pallet<T>>::block_number())?;
        Permissions::<T>::insert(permission_id, &contract);
    }

    Ok(())
}
