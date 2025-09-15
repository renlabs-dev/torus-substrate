use codec::{Decode, Encode, MaxEncodedLen};
use pallet_torus0_api::NamespacePath;
use polkadot_sdk::{
    frame_support::{CloneNoBound, DebugNoBound, dispatch::DispatchResult},
    sp_runtime::{BoundedBTreeMap, BoundedBTreeSet},
    sp_std::vec::Vec,
};
use scale_info::TypeInfo;

use crate::{Config, Error, Permissions};

use super::{PermissionId, PermissionScope};

/// Scope for namespace permissions
#[derive(Encode, Decode, CloneNoBound, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub struct NamespaceScope<T: Config> {
    pub recipient: T::AccountId,
    /// Set of namespace paths this permission delegates access to
    pub paths: BoundedBTreeMap<
        Option<PermissionId>,
        BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>,
        T::MaxNamespacesPerPermission,
    >,
    /// Maximum number of instances of this permission
    pub max_instances: u32,
    /// Children permissions
    pub children: BoundedBTreeSet<PermissionId, T::MaxChildrenPerPermission>,
}

impl<T: Config> NamespaceScope<T> {
    fn redelegated_paths(
        &self,
        this_id: PermissionId,
    ) -> impl Iterator<Item = (NamespacePath, u32)> {
        self.children
            .iter()
            .filter_map(|c| {
                if let PermissionScope::Namespace(scope) = Permissions::<T>::get(c)?.scope {
                    Some(scope)
                } else {
                    None
                }
            })
            .flat_map(move |scope| {
                scope
                    .paths
                    .into_iter()
                    .filter(move |(pid, _)| pid.as_ref().is_some_and(|pid| pid == &this_id))
                    .flat_map(|(_, paths)| paths)
                    .map(move |path| (path, scope.max_instances))
            })
    }

    /// Checks that the provided paths are allowed to be delegated.
    /// This is true if enough instances are available and unused by
    /// siblings.
    ///
    /// Given a subpath, an instance is used if any parent or child
    /// paths from that subpath were delegated to a sibling permission.
    /// sibling subpaths do not consume an instance relative to this
    /// subpath.
    pub(crate) fn check_available_instances<'a>(
        &self,
        this_id: PermissionId,
        other: impl Iterator<Item = &'a NamespacePath>,
        instances: u32,
    ) -> DispatchResult {
        let mut redelegated_paths: Vec<_> = self.redelegated_paths(this_id).collect();

        for other in other {
            let mut available_instances = self.max_instances;

            for (redelegated_path, max_instances) in &redelegated_paths {
                if redelegated_path == other
                    || redelegated_path.is_parent_of(other)
                    || other.is_parent_of(redelegated_path)
                {
                    available_instances = available_instances.saturating_sub(*max_instances);
                    if available_instances < instances {
                        return Err(Error::<T>::NotEnoughInstances.into());
                    }
                }
            }

            // This is crucial, do not change.
            // When delegating multiple related paths in a single
            // call (e.g. .compute and .compute.gpu), we have to
            // account for these entries when calculating available
            // instances for the next entries (.compute consume
            // an instance used by .compute.gpu). To solve this
            // in a single call, we add it to the sibling path
            // list: the next entry will iterate this path as well.
            //
            // Earlier paths are siblings to later paths.
            redelegated_paths.push((other.clone(), instances));

            if available_instances < instances {
                return Err(Error::<T>::NotEnoughInstances.into());
            }
        }

        Ok(())
    }

    /// Count the number of available instances for a given path.
    pub fn available_instances(&self, this_id: PermissionId, path: &NamespacePath) -> u32 {
        let mut available_instances = self.max_instances;

        for (redelegated_path, max_instances) in self.redelegated_paths(this_id) {
            if &redelegated_path == path
                || redelegated_path.is_parent_of(path)
                || path.is_parent_of(&redelegated_path)
            {
                available_instances = available_instances.saturating_sub(max_instances);
                if available_instances == 0 {
                    return 0;
                }
            }
        }

        available_instances
    }

    /// Cleanup operations when permission is revoked or expired
    pub(super) fn cleanup(
        &self,
        permission_id: polkadot_sdk::sp_core::H256,
        _last_execution: &Option<crate::BlockNumberFor<T>>,
        _delegator: &T::AccountId,
    ) {
        for pid in self.paths.keys().cloned().flatten() {
            Permissions::<T>::mutate_extant(pid, |parent| {
                if let Some(children) = parent.children_mut() {
                    children.remove(&permission_id);
                }
            });
        }
    }
}
