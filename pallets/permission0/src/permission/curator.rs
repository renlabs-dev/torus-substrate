use bitflags::bitflags;
use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    frame_support::{CloneNoBound, DebugNoBound, EqNoBound, PartialEqNoBound},
    polkadot_sdk_frame::prelude::BlockNumberFor,
    sp_runtime::BoundedBTreeMap,
};
use scale_info::TypeInfo;

use crate::{Config, Permissions};

use super::PermissionId;

#[derive(
    CloneNoBound,
    Copy,
    DebugNoBound,
    Encode,
    Decode,
    EqNoBound,
    PartialEqNoBound,
    TypeInfo,
    MaxEncodedLen,
)]
pub struct CuratorPermissions(u32);

bitflags! {
    impl CuratorPermissions: u32 {
        /// Able to appoint other curators. Though not used at the moment,
        /// it will be valuable when we remove the SUDO key/multisig.
        const ROOT                        = 0b0000_0001;
        /// Permission to review and process agent applications
        const APPLICATION_REVIEW          = 0b0000_0010;
        /// Permission to manage the whitelist (add/remove accounts)
        const WHITELIST_MANAGE            = 0b0000_0100;
        /// Permission to apply penalty factors to agents
        const PENALTY_CONTROL             = 0b0000_1000;
        /// Permission to toggle agent freezing
        const AGENT_FREEZING_TOGGLING     = 0b0001_0000;
        /// Permission to toggle namespace freezing
        const NAMESPACE_FREEZING_TOGGLING = 0b0010_0000;
    }
}

#[derive(Encode, Decode, CloneNoBound, PartialEq, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub struct CuratorScope<T: Config> {
    pub recipient: T::AccountId,
    pub flags: BoundedBTreeMap<
        Option<PermissionId>,
        CuratorPermissions,
        T::MaxCuratorSubpermissionsPerPermission,
    >,
    pub cooldown: Option<BlockNumberFor<T>>,
}

impl<T: Config> CuratorScope<T> {
    pub fn has_permission(&self, permission: CuratorPermissions) -> bool {
        self.flags.iter().any(|(_, p)| p.contains(permission))
    }
}

impl<T: Config> CuratorScope<T> {
    /// Cleanup operations when permission is revoked or expired
    pub(crate) fn cleanup(
        &self,
        permission_id: polkadot_sdk::sp_core::H256,
        _last_execution: &Option<crate::BlockNumberFor<T>>,
        _delegator: &T::AccountId,
    ) {
        for pid in self.flags.keys().cloned().flatten() {
            Permissions::<T>::mutate_extant(pid, |parent| {
                parent.children.remove(&permission_id);
            });
        }
    }
}
