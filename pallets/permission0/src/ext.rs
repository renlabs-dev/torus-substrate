use crate::permission::EnforcementReferendum;
use crate::{
    Config, EnforcementAuthority, EnforcementTracking, Error, Event, Pallet, PermissionDuration,
    PermissionId, PermissionScope, Permissions, RevocationTerms, pallet,
};
use pallet_permission0_api::{
    EnforcementAuthority as ApiEnforcementAuthority, Permission0Api,
    PermissionDuration as ApiPermissionDuration, RevocationTerms as ApiRevocationTerms,
};
use polkadot_sdk::{
    frame_support::ensure,
    frame_system::ensure_signed_or_root,
    polkadot_sdk_frame::prelude::{BlockNumberFor, OriginFor},
    sp_runtime::{DispatchError, DispatchResult},
};

pub mod curator_impl;
pub mod emission_impl;
pub mod namespace_impl;

/// Implementation of the Permission0Api trait to be used externally
impl<T: Config> Permission0Api<OriginFor<T>> for pallet::Pallet<T> {
    fn permission_exists(id: &PermissionId) -> bool {
        Permissions::<T>::contains_key(id)
    }

    fn revoke_permission(who: OriginFor<T>, permission_id: &PermissionId) -> DispatchResult {
        revoke_permission_impl::<T>(who, permission_id)
    }

    fn execute_permission(who: OriginFor<T>, permission_id: &PermissionId) -> DispatchResult {
        execute_permission_impl::<T>(who, permission_id)
    }
}

fn translate_duration<T: Config>(
    duration: ApiPermissionDuration<BlockNumberFor<T>>,
) -> Result<PermissionDuration<T>, DispatchError> {
    Ok(match duration {
        ApiPermissionDuration::UntilBlock(block) => {
            let current_block = <polkadot_sdk::frame_system::Pallet<T>>::block_number();
            ensure!(block > current_block, Error::<T>::InvalidInterval);

            PermissionDuration::UntilBlock(block)
        }
        ApiPermissionDuration::Indefinite => PermissionDuration::Indefinite,
    })
}

fn translate_revocation_terms<T: Config>(
    revocation: ApiRevocationTerms<T::AccountId, BlockNumberFor<T>>,
) -> Result<RevocationTerms<T>, DispatchError> {
    let revocation = match revocation {
        ApiRevocationTerms::Irrevocable => RevocationTerms::Irrevocable,
        ApiRevocationTerms::RevocableByDelegator => RevocationTerms::RevocableByDelegator,
        ApiRevocationTerms::RevocableByArbiters {
            accounts,
            required_votes,
        } => {
            ensure!(required_votes > 0, Error::<T>::InvalidNumberOfRevokers);
            ensure!(!accounts.is_empty(), Error::<T>::InvalidNumberOfRevokers);

            ensure!(
                required_votes as usize <= accounts.len(),
                Error::<T>::InvalidNumberOfRevokers
            );

            RevocationTerms::RevocableByArbiters {
                accounts: accounts
                    .try_into()
                    .map_err(|_| crate::Error::<T>::TooManyRevokers)?,
                required_votes,
            }
        }
        ApiRevocationTerms::RevocableAfter(block) => {
            let current_block = <polkadot_sdk::frame_system::Pallet<T>>::block_number();
            ensure!(block > current_block, Error::<T>::InvalidInterval);

            RevocationTerms::RevocableAfter(block)
        }
    };

    Ok(revocation)
}

fn translate_enforcement_authority<T: Config>(
    enforcement: ApiEnforcementAuthority<T::AccountId>,
) -> Result<EnforcementAuthority<T>, DispatchError> {
    let enforcement = match enforcement {
        ApiEnforcementAuthority::None => EnforcementAuthority::None,
        ApiEnforcementAuthority::ControlledBy {
            controllers,
            required_votes,
        } => {
            ensure!(required_votes > 0, Error::<T>::InvalidNumberOfControllers);
            ensure!(
                !controllers.is_empty(),
                Error::<T>::InvalidNumberOfControllers
            );

            ensure!(
                required_votes as usize <= controllers.len(),
                Error::<T>::InvalidNumberOfControllers
            );

            EnforcementAuthority::ControlledBy {
                controllers: controllers
                    .try_into()
                    .map_err(|_| crate::Error::<T>::TooManyControllers)?,
                required_votes,
            }
        }
    };

    Ok(enforcement)
}

/// Revoke a permission implementation
pub(crate) fn revoke_permission_impl<T: Config>(
    origin: OriginFor<T>,
    permission_id: &PermissionId,
) -> DispatchResult {
    let contract = Permissions::<T>::get(permission_id).ok_or(Error::<T>::PermissionNotFound)?;
    contract.revoke(origin, *permission_id)
}

/// Execute a permission implementation
pub(crate) fn execute_permission_impl<T: Config>(
    who: OriginFor<T>,
    permission_id: &PermissionId,
) -> DispatchResult {
    let who = ensure_signed_or_root(who)?;

    let contract = Permissions::<T>::get(permission_id).ok_or(Error::<T>::PermissionNotFound)?;

    let delegator = contract.delegator.clone();

    ensure!(
        who.is_none() || who.as_ref() == Some(&delegator),
        Error::<T>::NotPermissionDelegator
    );

    match &contract.scope {
        PermissionScope::Emission(emission_scope) => {
            emission_impl::execute_permission_impl(permission_id, &contract, emission_scope)
        }
        PermissionScope::Curator(_) => curator_impl::execute_permission_impl::<T>(permission_id),
        PermissionScope::Namespace(_) => Ok(()),
    }
}

/// Execute a permission through enforcement authority
pub fn enforcement_execute_permission_impl<T: Config>(
    origin: OriginFor<T>,
    permission_id: PermissionId,
) -> DispatchResult {
    let who = ensure_signed_or_root(origin)?;

    let contract = Permissions::<T>::get(permission_id).ok_or(Error::<T>::PermissionNotFound)?;

    // If not root, check enforcement authority
    if let Some(who) = &who {
        match &contract.enforcement {
            EnforcementAuthority::None => {
                return Err(Error::<T>::NotAuthorizedToToggle.into());
            }
            EnforcementAuthority::ControlledBy {
                controllers,
                required_votes,
            } => {
                ensure!(controllers.contains(who), Error::<T>::NotAuthorizedToToggle);

                let referendum = EnforcementReferendum::Execution;
                let votes = EnforcementTracking::<T>::get(permission_id, &referendum)
                    .into_iter()
                    .filter(|id| id != who)
                    .filter(|id| controllers.contains(id))
                    .count();

                if votes.saturating_add(1) < *required_votes as usize {
                    return EnforcementTracking::<T>::mutate(
                        permission_id,
                        referendum.clone(),
                        |votes| {
                            votes
                                .try_insert(who.clone())
                                .map_err(|_| Error::<T>::TooManyControllers)?;

                            <Pallet<T>>::deposit_event(Event::EnforcementVoteCast {
                                permission_id,
                                voter: who.clone(),
                                referendum,
                            });

                            Ok(())
                        },
                    );
                }
            }
        }
    }

    match &contract.scope {
        PermissionScope::Emission(emission_scope) => {
            emission_impl::execute_permission_impl(&permission_id, &contract, emission_scope)?
        }
        PermissionScope::Curator(_) => {
            return curator_impl::execute_permission_impl::<T>(&permission_id);
        }
        PermissionScope::Namespace(_) => return Ok(()),
    }

    EnforcementTracking::<T>::remove(permission_id, EnforcementReferendum::Execution);

    <Pallet<T>>::deposit_event(Event::PermissionEnforcementExecuted {
        permission_id,
        executed_by: who,
    });

    Ok(())
}
