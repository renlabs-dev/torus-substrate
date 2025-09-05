use codec::{Decode, Encode, MaxEncodedLen};
use pallet_torus0_api::NamespacePath;
use polkadot_sdk::{
    frame_support::{CloneNoBound, DebugNoBound},
    sp_runtime::{BoundedBTreeMap, BoundedBTreeSet},
};
use scale_info::TypeInfo;

use crate::{Config, Permissions};

use super::PermissionId;

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
