use crate::{
    generate_permission_id, permission::NamespaceScope, update_permission_indices, Config, Error,
    Event, Pallet, PermissionContract, PermissionDuration, PermissionId, PermissionScope,
    Permissions, PermissionsByGrantor, RevocationTerms,
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
    fn is_delegating_namespace(grantor: &T::AccountId, path: &NamespacePath) -> bool {
        PermissionsByGrantor::<T>::get(grantor).iter().any(|id| {
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

pub fn grant_namespace_permission_impl<T: Config>(
    grantor: OriginFor<T>,
    grantee: T::AccountId,
    paths: BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
    duration: PermissionDuration<T>,
    revocation: RevocationTerms<T>,
) -> Result<PermissionId, DispatchError> {
    let grantor = ensure_signed(grantor)?;

    let paths = paths
        .into_iter()
        .map(|path| {
            let path =
                NamespacePath::new_agent(&path).map_err(|_| Error::<T>::NamespacePathIsInvalid)?;
            ensure!(
                T::Torus::namespace_exists(&grantor, &path),
                Error::<T>::NamespaceDoesNotExist
            );

            Ok(path)
        })
        .collect::<Result<BTreeSet<NamespacePath>, DispatchError>>()?;
    let paths = paths
        .try_into()
        .map_err(|_| Error::<T>::NamespacePathIsInvalid)?;

    let scope = PermissionScope::Namespace(NamespaceScope { paths });
    let permission_id = generate_permission_id::<T>(&grantor, &grantee, &scope)?;

    let contract = PermissionContract {
        grantor,
        grantee,
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
    update_permission_indices::<T>(&contract.grantor, &contract.grantee, permission_id)?;

    <Pallet<T>>::deposit_event(Event::PermissionGranted {
        grantor: contract.grantor,
        grantee: contract.grantee,
        permission_id,
    });

    Ok(permission_id)
}
