use crate::{
    Config, CuratorPermissions, CuratorScope, Error, Event, Pallet, PermissionContract,
    PermissionDuration, PermissionScope, Permissions, PermissionsByRecipient, RevocationTerms,
    generate_permission_id, pallet, update_permission_indices,
};

use pallet_permission0_api::{
    CuratorPermissions as ApiCuratorPermissions, Permission0CuratorApi, PermissionId,
};
use polkadot_sdk::frame_system::ensure_signed_or_root;
use polkadot_sdk::sp_core::Get;
use polkadot_sdk::sp_runtime::BoundedBTreeMap;
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
    fn ensure_curator_permission(
        recipient: OriginFor<T>,
        flags: ApiCuratorPermissions,
    ) -> Result<T::AccountId, DispatchError> {
        let Some(recipient) = ensure_signed_or_root(recipient)? else {
            return Ok(T::PalletId::get().into_account_truncating());
        };

        let flags = CuratorPermissions::from_bits_truncate(flags.bits());
        let permissions = PermissionsByRecipient::<T>::get(&recipient);
        let now = <frame_system::Pallet<T>>::block_number();

        let mut cur_error = Error::<T>::PermissionNotFound;
        for permission_id in permissions {
            let Some(contract) = Permissions::<T>::get(permission_id) else {
                continue;
            };

            let PermissionScope::Curator(scope) = &contract.scope else {
                continue;
            };

            if !scope.has_permission(flags) {
                continue;
            }

            if contract.available_instances() < 1 {
                if !matches!(cur_error, Error::<T>::PermissionInCooldown) {
                    cur_error = Error::<T>::NotEnoughInstances;
                }

                continue;
            }

            if let Some(cooldown) = scope.cooldown
                && let Some(last_execution) = contract.last_execution()
                && last_execution.saturating_add(cooldown) > now
            {
                cur_error = Error::<T>::PermissionInCooldown;
                continue;
            }

            return Ok(contract.recipient);
        }

        Err(cur_error.into())
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

    #[cfg(feature = "runtime-benchmarks")]
    fn force_curator(recipient: &T::AccountId, flags: ApiCuratorPermissions) {
        use polkadot_sdk::frame_system::RawOrigin;

        let mut map: BoundedBTreeMap<
            Option<PermissionId>,
            CuratorPermissions,
            T::MaxCuratorSubpermissionsPerPermission,
        > = BoundedBTreeMap::new();
        map.try_insert(None, CuratorPermissions::from_bits_truncate(flags.bits()))
            .unwrap();

        delegate_curator_permission_impl(
            RawOrigin::Root.into(),
            recipient.clone(),
            map,
            None,
            PermissionDuration::<T>::Indefinite,
            RevocationTerms::<T>::Irrevocable,
            1,
        )
        .unwrap();
    }
}

pub fn delegate_curator_permission_impl<T: Config>(
    delegator: OriginFor<T>,
    recipient: T::AccountId,
    mut flags: BoundedBTreeMap<
        Option<PermissionId>,
        CuratorPermissions,
        T::MaxCuratorSubpermissionsPerPermission,
    >,
    cooldown: Option<BlockNumberFor<T>>,
    duration: PermissionDuration<T>,
    revocation: RevocationTerms<T>,
    instances: u32,
) -> Result<PermissionId, DispatchError> {
    let signer = ensure_signed_or_root(delegator)?;

    let is_root = signer.is_none();
    let delegator =
        signer.unwrap_or_else(|| <T as Config>::PalletId::get().into_account_truncating());

    let mut parents = polkadot_sdk::sp_std::vec::Vec::with_capacity(flags.len());

    for (parent_pid, flags) in &mut flags {
        // Root permission is not delegateable
        flags.remove(CuratorPermissions::ROOT);

        ensure!(!flags.is_empty(), Error::<T>::InvalidCuratorPermissions);

        if let Some(parent_pid) = parent_pid {
            let Some(parent) = Permissions::<T>::get(parent_pid) else {
                return Err(Error::<T>::ParentPermissionNotFound.into());
            };

            ensure!(
                parent.recipient == delegator,
                Error::<T>::NotPermissionRecipient
            );

            let PermissionScope::Curator(scope) = &parent.scope else {
                return Err(Error::<T>::UnsupportedPermissionType.into());
            };

            ensure!(
                scope.has_permission(*flags),
                Error::<T>::InvalidCuratorPermissions
            );

            ensure!(
                instances <= parent.available_instances(),
                Error::<T>::NotEnoughInstances
            );

            ensure!(
                RevocationTerms::<T>::is_weaker(&parent.revocation, &revocation),
                Error::<T>::RevocationTermsTooStrong
            );

            parents.push(*parent_pid);
        } else {
            // We do not check for the ROOT curator permission at the moment.
            // This is mainly due to our use of a SUDO key at the moment.
            // Once we move away from centralized chain management, a ROOT curator
            // will be appointed by the system.

            ensure!(is_root, Error::<T>::NotPermissionRecipient);
        }
    }

    let scope = PermissionScope::Curator(CuratorScope { flags, cooldown });
    let permission_id = generate_permission_id::<T>(&delegator, &recipient, &scope)?;

    for parent in parents {
        Permissions::<T>::mutate_extant(parent, |parent| {
            parent.children.try_insert(permission_id).ok()
        })
        .ok_or(Error::<T>::TooManyChildren)?;
    }

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
