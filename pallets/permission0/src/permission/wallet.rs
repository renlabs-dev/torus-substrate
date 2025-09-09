use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::frame_support::{CloneNoBound, DebugNoBound, EqNoBound, PartialEqNoBound};
use scale_info::TypeInfo;

use crate::Config;

#[derive(CloneNoBound, DebugNoBound, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct WalletScope<T: Config> {
    pub recipient: T::AccountId,
    pub r#type: WalletScopeType,
}

impl<T: Config> WalletScope<T> {
    /// Cleanup operations when permission is revoked or expired
    pub(crate) fn cleanup(
        &self,
        _permission_id: polkadot_sdk::sp_core::H256,
        _last_execution: &Option<crate::BlockNumberFor<T>>,
        _delegator: &T::AccountId,
    ) {
        // No actions to perform
    }
}

#[derive(CloneNoBound, DebugNoBound, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum WalletScopeType {
    Stake(WalletStake),
}

#[derive(
    CloneNoBound, DebugNoBound, Encode, Decode, MaxEncodedLen, TypeInfo, PartialEqNoBound, EqNoBound,
)]
pub struct WalletStake {
    /// If true, allows the recipient to perform transfer of stake between staked accounts.
    pub can_transfer_stake: bool,
    /// If true, this permission holds exclusive access to the delegator stake, meaning that
    /// the delegator has no right to perform operations over stake (including unstaking)
    /// while this permission is active.
    pub exclusive_stake_access: bool,
}
