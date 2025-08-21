use crate::{
    Config, Error, Event, Pallet, PermissionContract, PermissionDuration, PermissionId,
    PermissionScope, Permissions, PermissionsByDelegator, RevocationTerms, generate_permission_id,
    permission::NamespaceScope, permission::add_permission_indices,
};
use pallet_permission0_api::Permission0NamespacesApi;
use pallet_torus0_api::{NamespacePath, NamespacePathInner, Torus0Api};
use polkadot_sdk::{
    frame_support::{dispatch::DispatchResult, ensure},
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

    ensure!(instances > 0, Error::<T>::NotEnoughInstances);

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

            let namespaces = if let Some(pid) = pid {
                let Some(parent) = Permissions::<T>::get(pid) else {
                    return Err(Error::<T>::ParentPermissionNotFound.into());
                };

                let PermissionScope::Namespace(namespace) = &parent.scope else {
                    return Err(Error::<T>::UnsupportedPermissionType.into());
                };

                ensure!(
                    namespace.recipient == delegator,
                    Error::<T>::NotPermissionRecipient
                );

                ensure!(
                    instances <= parent.available_instances(),
                    Error::<T>::NotEnoughInstances
                );

                ensure!(
                    RevocationTerms::<T>::is_weaker(&parent.revocation, &revocation),
                    Error::<T>::RevocationTermsTooStrong
                );

                parents.push(pid);

                resolve_paths::<T>(&delegator, Some((*pid, namespace)), paths)
            } else {
                resolve_paths::<T>(&delegator, None, paths)
            }?;

            Ok((*pid, namespaces))
        })
        .collect::<Result<BTreeMap<_, _>, DispatchError>>()?;

    let paths = paths
        .try_into()
        .map_err(|_| Error::<T>::TooManyNamespaces)?;

    let scope = PermissionScope::Namespace(NamespaceScope {
        recipient: recipient.clone(),
        paths,
    });
    let permission_id = generate_permission_id::<T>(&delegator, &scope)?;

    for parent in parents {
        Permissions::<T>::mutate_extant(parent, |parent| {
            parent.children.try_insert(permission_id).ok()
        })
        .ok_or(Error::<T>::TooManyChildren)?;
    }

    let contract = PermissionContract::<T>::new(
        delegator,
        scope,
        duration,
        revocation,
        crate::EnforcementAuthority::None,
        instances,
    );

    Permissions::<T>::insert(permission_id, &contract);
    add_permission_indices::<T>(
        &contract.delegator,
        core::iter::once(&recipient),
        permission_id,
    )?;

    <Pallet<T>>::deposit_event(Event::PermissionDelegated {
        delegator: contract.delegator,
        permission_id,
    });

    Ok(permission_id)
}

/// Maximum allowed delegation depth for namespaces
const MAX_DELEGATION_DEPTH: u32 = 5;

/// Check the delegation depth of a namespace by traversing up the permission chain
fn check_namespace_delegation_depth<T: Config>(
    namespace_path: &NamespacePath,
    parent_permission_id: Option<PermissionId>,
    current_depth: u32,
) -> Result<(), DispatchError> {
    if current_depth > MAX_DELEGATION_DEPTH {
        return Err(Error::<T>::DelegationDepthExceeded.into());
    }

    let Some(parent_id) = parent_permission_id else {
        return Ok(());
    };

    let Some(parent_permission) = Permissions::<T>::get(parent_id) else {
        return Err(Error::<T>::ParentPermissionNotFound.into());
    };

    let PermissionScope::Namespace(parent_scope) = &parent_permission.scope else {
        return Err(Error::<T>::UnsupportedPermissionType.into());
    };

    for (grandparent_id, namespace_set) in parent_scope.paths.iter() {
        if namespace_set.contains(namespace_path) {
            return check_namespace_delegation_depth::<T>(
                namespace_path,
                *grandparent_id,
                current_depth.saturating_add(1),
            );
        }

        for parent_namespace in namespace_set.iter() {
            if parent_namespace.is_parent_of(namespace_path) {
                return check_namespace_delegation_depth::<T>(
                    parent_namespace,
                    *grandparent_id,
                    current_depth.saturating_add(1),
                );
            }
        }
    }

    Ok(())
}

fn resolve_paths<T: Config>(
    delegator: &T::AccountId,
    parent: Option<(PermissionId, &NamespaceScope<T>)>,
    paths: &BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
) -> Result<BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>, DispatchError> {
    let children = paths.iter().map(|path| {
        NamespacePath::new_agent(path).map_err(|_| Error::<T>::NamespacePathIsInvalid.into())
    });

    let paths = if let Some((parent_pid, parent)) = parent {
        let children = children.collect::<Result<BTreeSet<_>, DispatchError>>()?;
        let matched_paths = parent
            .paths
            .values()
            .flat_map(|p_path| p_path.iter())
            .filter_map(|parent_path| {
                if children.contains(parent_path) {
                    Some(())
                } else {
                    let agent_name = parent_path.agent_name()?;
                    let child_path = children
                        .iter()
                        .find(|child| parent_path.is_parent_of(child))?;

                    let agent = T::Torus::find_agent_by_name(agent_name)?;
                    if !T::Torus::namespace_exists(&agent, child_path) {
                        return None;
                    }

                    Some(())
                }
            })
            .count();

        ensure!(
            matched_paths == children.len(),
            Error::<T>::ParentPermissionNotFound
        );

        for child_path in &children {
            check_namespace_delegation_depth::<T>(child_path, Some(parent_pid), 1)?;
        }

        children
    } else {
        children
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

pub(crate) fn update_namespace_permission<T: Config>(
    origin: OriginFor<T>,
    permission_id: PermissionId,
    max_instances: u32,
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    let permission = Permissions::<T>::get(permission_id);

    let Some(mut permission) = permission else {
        return Err(Error::<T>::PermissionNotFound.into());
    };

    ensure!(permission.delegator == who, Error::<T>::NotAuthorizedToEdit);

    let scope = permission.scope.clone();
    match &scope {
        PermissionScope::Namespace(namespace) => {
            if max_instances == permission.max_instances {
                return Ok(());
            } else if max_instances > permission.max_instances {
                for parent in namespace.paths.keys().copied().flatten() {
                    let Some(parent) = Permissions::<T>::get(parent) else {
                        continue;
                    };

                    ensure!(
                        max_instances <= parent.available_instances(),
                        Error::<T>::NotEnoughInstances
                    );
                }
            } else {
                ensure!(permission.is_updatable(), Error::<T>::NotAuthorizedToEdit);
                ensure!(
                    max_instances >= permission.used_instances(),
                    Error::<T>::NotEnoughInstances
                );
            }
        }
        _ => return Err(Error::<T>::NotEditable.into()),
    }

    permission.max_instances = max_instances;
    Permissions::<T>::set(permission_id, Some(permission));

    Ok(())
}
