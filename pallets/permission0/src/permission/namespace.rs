use codec::{Decode, Encode, MaxEncodedLen};
use pallet_torus0_api::NamespacePath;
use polkadot_sdk::{
    frame_support::{CloneNoBound, DebugNoBound, dispatch::DispatchResult},
    sp_runtime::{BoundedBTreeMap, BoundedBTreeSet},
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
    /// Checks that the provided paths are allowed to be delegated.
    /// This is true if enough instances are available and unused by
    /// sibilings.
    ///
    /// Given a subpath, an instance is used if any parent or child
    /// paths from that subpath were delegated to a sibiling permission.
    /// Sibiling subpaths do not consume an instance relative to this
    /// subpath.
    pub(crate) fn check_available_instances<'a>(
        &self,
        this_id: PermissionId,
        other: impl Iterator<Item = &'a NamespacePath>,
        instances: u32,
    ) -> DispatchResult {
        let mut sibiling_paths = self
            .children
            .iter()
            .filter_map(|c| {
                if let PermissionScope::Namespace(sibiling) = Permissions::<T>::get(c)?.scope {
                    Some(sibiling)
                } else {
                    None
                }
            })
            .flat_map(|sibiling| {
                sibiling
                    .paths
                    .into_iter()
                    .filter(|(pid, _)| pid.as_ref().is_some_and(|pid| pid == &this_id))
                    .flat_map(|(_, paths)| paths)
                    .map(move |path| (path, sibiling.max_instances))
            })
            .collect::<polkadot_sdk::sp_std::vec::Vec<_>>();

        for other in other {
            let mut used_instances = 0u32;

            for (sibiling_path, max_instances) in &sibiling_paths {
                if sibiling_path == other
                    || sibiling_path.is_parent_of(other)
                    || other.is_parent_of(sibiling_path)
                {
                    used_instances = used_instances.saturating_add(*max_instances);
                }
            }

            // This is crucial, do not change.
            // When delegating multiple related paths in a single
            // call (e.g. .compute and .compute.gpu), we have to
            // account for these entries when calculating available
            // instances for the next entries (.compute consume
            // an instance used by .compute.gpu). To solve this
            // in a single call, we add it to the sibiling path
            // list: the next entry will iterate this path as well.
            //
            // Earlier paths are sibilings to later paths.
            sibiling_paths.push((other.clone(), instances));

            if self.max_instances.saturating_sub(used_instances) < instances {
                return Err(Error::<T>::NotEnoughInstances.into());
            }
        }

        Ok(())
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
