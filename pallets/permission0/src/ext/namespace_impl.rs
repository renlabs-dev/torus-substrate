use crate::{
    generate_permission_id, permission::NamespaceScope, update_permission_indices, Config, Error,
    Event, Pallet, PermissionContract, PermissionDuration, PermissionId, PermissionScope,
    Permissions, PermissionsByDelegator, RevocationTerms,
};
use pallet_permission0_api::Permission0NamespacesApi;
use pallet_torus0_api::{NamespacePath, NamespacePathInner, Torus0Api};
use polkadot_sdk::{
    frame_support::ensure,
    frame_system::ensure_signed,
    polkadot_sdk_frame::prelude::OriginFor,
    sp_core::Get,
    sp_runtime::{BoundedBTreeMap, BoundedBTreeSet, DispatchError},
    sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet},
};

impl<T: Config> Permission0NamespacesApi<T::AccountId, NamespacePath> for Pallet<T> {
    fn is_delegating_namespace(delegator: &T::AccountId, path: &NamespacePath) -> bool {
        PermissionsByDelegator::<T>::get(delegator)
            .iter()
            .any(|id| {
                Permissions::<T>::get(id)
                    .filter(|permission| {
                        if let PermissionScope::Namespace(scope) = &permission.scope {
                            for other in scope.paths.values().flat_map(|k| k.iter()) {
                                if other == path
                                    || path.is_parent_of(other)
                                    || other.is_parent_of(path)
                                {
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
    paths: BoundedBTreeMap<
        Option<PermissionId>,
        BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
        T::MaxNamespacesPerPermission,
    >,
    duration: PermissionDuration<T>,
    revocation: RevocationTerms<T>,
    instances: u32,
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

    let total_multi_parent = paths.keys().filter(|p| p.is_some()).count();
    ensure!(total_multi_parent <= 1, Error::<T>::MultiParentForbidden);

    let mut total_paths = 0usize;
    let mut parents = polkadot_sdk::sp_std::vec::Vec::with_capacity(paths.len());

    let paths = paths
        .iter()
        .map(|(pid, paths)| {
            total_paths = total_paths.saturating_add(paths.len());
            ensure!(
                total_paths as u32 <= T::MaxNamespacesPerPermission::get(),
                Error::<T>::TooManyNamespaces
            );

            let permission = pid.and_then(|pid| Some((pid, Permissions::<T>::get(pid)?)));
            let namespaces = if let Some((pid, permission)) = permission {
                ensure!(
                    permission.recipient == delegator,
                    Error::<T>::NotPermissionRecipient
                );

                let PermissionScope::Namespace(namespace) = &permission.scope else {
                    return Err(Error::<T>::UnsupportedPermissionType.into());
                };

                ensure!(
                    instances <= permission.available_instances(),
                    Error::<T>::NotEnoughInstances
                );

                parents.push(pid);

                resolve_paths::<T>(&delegator, Some(&namespace), &paths)
            } else {
                resolve_paths::<T>(&delegator, None, &paths)
            }?;

            Ok((*pid, namespaces))
        })
        .collect::<Result<BTreeMap<_, _>, DispatchError>>()?;

    let paths = paths
        .try_into()
        .map_err(|_| Error::<T>::TooManyNamespaces)?;

    let scope = PermissionScope::Namespace(NamespaceScope { paths });
    let permission_id = generate_permission_id::<T>(&delegator, &recipient, &scope)?;

    for parent in parents {
        Permissions::<T>::mutate_extant(parent, |parent| {
            parent.children.try_insert(permission_id).ok()
        })
        .ok_or_else(|| Error::<T>::TooManyChildren)?;
    }

    let contract = PermissionContract::<T>::new(
        delegator,
        recipient,
        scope,
        duration,
        revocation,
        crate::EnforcementAuthority::None,
        instances,
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

fn resolve_paths<T: Config>(
    delegator: &T::AccountId,
    parent: Option<&NamespaceScope<T>>,
    paths: &BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
) -> Result<BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>, DispatchError> {
    let paths = paths.iter().map(|path| {
        NamespacePath::new_agent(&path).map_err(|_| Error::<T>::NamespacePathIsInvalid.into())
    });

    let paths = if let Some(parent) = &parent {
        let paths = paths.collect::<Result<BTreeSet<_>, DispatchError>>()?;
        let matched_count = parent
            .paths
            .values()
            .flat_map(|parent| parent.iter())
            .filter(|p| paths.contains(*p))
            .count();

        ensure!(
            matched_count == paths.len(),
            Error::<T>::ParentPermissionNotFound
        );

        paths
    } else {
        paths
            .map(|path| {
                path.and_then(|path| {
                    if T::Torus::namespace_exists(delegator, &path) {
                        Ok(path)
                    } else {
                        Err(Error::<T>::NamespaceDoesNotExist.into())
                    }
                })
            })
            .collect::<Result<BTreeSet<_>, DispatchError>>()?
    };

    paths
        .try_into()
        .map_err(|_| Error::<T>::TooManyNamespaces.into())
}
