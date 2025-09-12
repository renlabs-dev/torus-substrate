use codec::{Decode, Encode, MaxEncodedLen};
use pallet_permission0_api::Permission0WalletApi;
use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_support::{
        CloneNoBound, DebugNoBound, EqNoBound, PartialEqNoBound, dispatch::DispatchResult, ensure,
    },
    frame_system::ensure_signed,
    polkadot_sdk_frame::prelude::OriginFor,
};
use scale_info::TypeInfo;

use crate::{
    BalanceOf, Config, Error, Event, Pallet, PermissionContract, PermissionDuration, PermissionId,
    PermissionScope, Permissions, PermissionsByDelegator, RevocationTerms, generate_permission_id,
    permission::{
        add_permission_indices,
        wallet::{WalletScope, WalletScopeType, WalletStake},
    },
};

impl<T: Config> Permission0WalletApi<T::AccountId> for Pallet<T> {
    fn find_active_wallet_permission(
        delegator: &T::AccountId,
    ) -> impl Iterator<
        Item = (
            PermissionId,
            pallet_permission0_api::WalletPermission<T::AccountId>,
        ),
    > {
        PermissionsByDelegator::<T>::get(delegator)
            .into_iter()
            .filter_map(|pid| {
                let permission = Permissions::<T>::get(pid)?;
                let PermissionScope::Wallet(wallet) = permission.scope else {
                    return None;
                };

                Some((
                    pid,
                    pallet_permission0_api::WalletPermission {
                        recipient: wallet.recipient,
                        r#type: match wallet.r#type {
                            WalletScopeType::Stake(stake) => {
                                pallet_permission0_api::WalletScopeType::Stake {
                                    can_transfer_stake: stake.can_transfer_stake,
                                    exclusive_stake_access: stake.exclusive_stake_access,
                                }
                            }
                        },
                    },
                ))
            })
    }
}
pub(crate) fn delegate_wallet_stake_permission<T: Config>(
    origin: OriginFor<T>,
    recipient: T::AccountId,
    stake_details: WalletStake,
    duration: PermissionDuration<T>,
    revocation: RevocationTerms<T>,
) -> DispatchResult {
    let delegator = ensure_signed(origin)?;
    ensure!(delegator != recipient, Error::<T>::SelfPermissionNotAllowed);

    for (_, perm) in Pallet::<T>::find_active_wallet_permission(&delegator) {
        if stake_details.exclusive_stake_access
            || matches!(
                perm.r#type,
                pallet_permission0_api::WalletScopeType::Stake {
                    exclusive_stake_access: true,
                    ..
                }
            )
        {
            return Err(Error::<T>::DuplicatePermission.into());
        }
    }

    let scope = PermissionScope::Wallet(WalletScope {
        recipient: recipient.clone(),
        r#type: WalletScopeType::Stake(stake_details),
    });
    let permission_id = generate_permission_id::<T>(&delegator, &scope)?;

    let contract = PermissionContract::<T>::new(
        delegator,
        scope,
        duration,
        revocation,
        crate::EnforcementAuthority::None,
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

    Ok(())
}

pub(crate) fn execute_wallet_stake_permission<T: Config>(
    caller: OriginFor<T>,
    permission_id: PermissionId,
    op: WalletStakeOperation<T>,
) -> DispatchResult {
    let caller = ensure_signed(caller)?;
    let Some(permission) = Permissions::<T>::get(permission_id) else {
        return Err(Error::<T>::PermissionNotFound.into());
    };
    let PermissionScope::Wallet(wallet) = &permission.scope else {
        return Err(Error::<T>::UnsupportedPermissionType.into());
    };
    #[allow(irrefutable_let_patterns)]
    let WalletScopeType::Stake(stake) = &wallet.r#type else {
        return Err(Error::<T>::UnsupportedPermissionType.into());
    };

    ensure!(
        caller == wallet.recipient,
        Error::<T>::NotPermissionRecipient
    );

    let staker = &permission.delegator;

    match op {
        WalletStakeOperation::Unstake { staked, amount } => {
            <T::Torus>::remove_stake(staker, &staked, amount)?;
        }
        WalletStakeOperation::Transfer { from, to, amount } => {
            ensure!(stake.can_transfer_stake, Error::<T>::PermissionNotFound);
            <T::Torus>::transfer_stake(staker, &from, &to, amount)?;
        }
    }

    Ok(())
}

#[derive(
    CloneNoBound, DebugNoBound, Encode, Decode, MaxEncodedLen, TypeInfo, PartialEqNoBound, EqNoBound,
)]
#[scale_info(skip_type_params(T))]
pub enum WalletStakeOperation<T: Config> {
    /// Unstakes the balance from the staked account, yielding control of the
    /// balance back to the delegator.
    Unstake {
        staked: T::AccountId,
        amount: BalanceOf<T>,
    },
    /// Transfers stake from one staked agent to another staked agent,
    /// related to the `transfer_stake` extrinsic in Torus0.
    Transfer {
        from: T::AccountId,
        to: T::AccountId,
        amount: BalanceOf<T>,
    },
}
