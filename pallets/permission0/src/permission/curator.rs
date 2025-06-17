use bitflags::bitflags;
use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    frame_support::{CloneNoBound, DebugNoBound, EqNoBound, PartialEqNoBound},
    polkadot_sdk_frame::prelude::BlockNumberFor,
};
use scale_info::TypeInfo;

use crate::Config;

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
        /// Permission to toggle namespace creation
        const NAMESPACE_CREATION_TOGGLING = 0b0001_0000;
    }
}

#[derive(Encode, Decode, CloneNoBound, PartialEq, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub struct CuratorScope<T: Config> {
    pub flags: CuratorPermissions,
    pub cooldown: Option<BlockNumberFor<T>>,
}

impl<T: Config> CuratorScope<T> {
    pub fn has_permission(&self, permission: CuratorPermissions) -> bool {
        self.flags.contains(permission)
    }

    /// Checks for [`CuratorPermissions::APPLICATION_REVIEW`]
    pub fn can_review_applications(&self) -> bool {
        self.has_permission(CuratorPermissions::APPLICATION_REVIEW)
    }

    /// Checks for [`CuratorPermissions::WHITELIST_MANAGE`]
    pub fn can_manage_whitelist(&self) -> bool {
        self.has_permission(CuratorPermissions::WHITELIST_MANAGE)
    }

    /// Checks for [`CuratorPermissions::PENALTY_CONTROL`]
    pub fn can_control_penalties(&self) -> bool {
        self.has_permission(CuratorPermissions::PENALTY_CONTROL)
    }

    /// Checks for [`CuratorPermissions::NAMESPACE_CREATION_TOGGLING`]
    pub fn can_toggle_namespace_creation(&self) -> bool {
        self.has_permission(CuratorPermissions::NAMESPACE_CREATION_TOGGLING)
    }
}

impl<T: Config> CuratorScope<T> {
    /// Cleanup operations when permission is revoked or expired
    pub(crate) fn cleanup(
        &self,
        _permission_id: polkadot_sdk::sp_core::H256,
        _last_execution: &Option<crate::BlockNumberFor<T>>,
        _grantor: &T::AccountId,
    ) {
        // No special cleanup needed for curator permissions
    }
}
