use crate::{
    generate_permission_id, permission::NamespaceScope, update_permission_indices, Config, Error,
    Event, Pallet, PermissionContract, PermissionDuration, PermissionId, PermissionScope,
    Permissions, PermissionsByDelegator, RevocationTerms,
};
use pallet_permission0_api::Permission0NamespacesApi;
use pallet_torus0_api::{NamespacePath, NamespacePathInner, Torus0Api};
use polkadot_sdk::{
    frame_support::ensure,
    frame_system::{self, ensure_signed},
    polkadot_sdk_frame::prelude::OriginFor,
    sp_runtime::{BoundedBTreeSet, DispatchError},
    sp_std::collections::btree_set::BTreeSet,
};

impl<T: Config> Permission0NamespacesApi<T::AccountId, NamespacePath> for Pallet<T> {
    fn is_delegating_namespace(delegator: &T::AccountId, path: &NamespacePath) -> bool {
        PermissionsByDelegator::<T>::get(delegator)
            .iter()
            .any(|id| {
                Permissions::<T>::get(id)
                    .filter(|permission| {
                        if let PermissionScope::Namespace(scope) = &permission.scope {
                            for p in &scope.paths {
                                if p == path || path.is_parent_of(p) || p.is_parent_of(path) {
                                    return true;
                                }
                            }
                        }

                        false
                    })
                    .is_some()
            })
    }
}

pub fn delegate_namespace_permission_impl<T: Config>(
    delegator: OriginFor<T>,
    recipient: T::AccountId,
    paths: BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
    duration: PermissionDuration<T>,
    revocation: RevocationTerms<T>,
) -> Result<PermissionId, DispatchError> {
    let delegator = ensure_signed(delegator)?;

    ensure!(
        T::Torus::is_agent_registered(&delegator),
        Error::<T>::NotRegisteredAgent
    );
    ensure!(
        T::Torus::is_agent_registered(&recipient),
        Error::<T>::NotRegisteredAgent
    );

    let paths = paths
        .into_iter()
        .map(|path| {
            let path =
                NamespacePath::new_agent(&path).map_err(|_| Error::<T>::NamespacePathIsInvalid)?;
            ensure!(
                T::Torus::namespace_exists(&delegator, &path),
                Error::<T>::NamespaceDoesNotExist
            );

            Ok(path)
        })
        .collect::<Result<BTreeSet<NamespacePath>, DispatchError>>()?;
    let paths = paths
        .try_into()
        .map_err(|_| Error::<T>::NamespacePathIsInvalid)?;

    let scope = PermissionScope::Namespace(NamespaceScope { paths });
    let permission_id = generate_permission_id::<T>(&delegator, &recipient, &scope)?;

    let contract = PermissionContract {
        delegator,
        recipient,
        scope,
        duration,
        revocation,
        enforcement: crate::EnforcementAuthority::None,
        last_execution: None,
        execution_count: 0,
        // Will change once we have a ROOT curator.
        parent: None,
        created_at: <frame_system::Pallet<T>>::block_number(),
    };

    Permissions::<T>::insert(permission_id, &contract);
    update_permission_indices::<T>(&contract.delegator, &contract.recipient, permission_id)?;

    <Pallet<T>>::deposit_event(Event::Permissiondelegated {
        delegator: contract.delegator,
        recipient: contract.recipient,
        permission_id,
    });

    Ok(permission_id)
}
