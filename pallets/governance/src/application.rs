use crate::frame::traits::ExistenceRequirement;
use crate::{whitelist, AccountIdOf, AgentApplications, BalanceOf};
use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::frame_election_provider_support::Get;
use polkadot_sdk::frame_support::dispatch::DispatchResult;
use polkadot_sdk::frame_support::traits::Currency;
use polkadot_sdk::frame_support::traits::WithdrawReasons;
use polkadot_sdk::frame_support::{ensure, CloneNoBound, DebugNoBound};
use polkadot_sdk::polkadot_sdk_frame::prelude::BlockNumberFor;
use polkadot_sdk::sp_runtime::{BoundedVec, DispatchError};
use polkadot_sdk::sp_std::vec::Vec;
use scale_info::TypeInfo;

#[derive(CloneNoBound, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct AgentApplication<T: crate::Config> {
    pub id: u32,
    pub payer_key: AccountIdOf<T>,
    pub agent_key: AccountIdOf<T>,
    pub data: BoundedVec<u8, T::MaxApplicationDataLength>,
    pub cost: BalanceOf<T>,
    pub expires_at: BlockNumberFor<T>,
    pub action: ApplicationAction,
    pub status: ApplicationStatus<T>,
}

impl<T: crate::Config> AgentApplication<T> {
    fn revoke(&mut self, revoked_by: T::AccountId) -> DispatchResult {
        ensure!(
            self.action == ApplicationAction::Add,
            crate::Error::<T>::CannotRevokeRemoveApplication,
        );

        let previously_accepted_by = match &self.status {
            ApplicationStatus::Resolved {
                accepted: true,
                resolved_by,
            } => resolved_by,
            _ => return Err(crate::Error::<T>::CannotRevokeUnresolvedApplication.into()),
        };

        self.status = ApplicationStatus::Revoked {
            previously_accepted_by: previously_accepted_by.clone(),
            revoked_by,
        };

        whitelist::remove_from_whitelist::<T>(self.agent_key.clone())?;

        crate::Pallet::<T>::deposit_event(crate::Event::<T>::ApplicationRevoked(self.id));

        Ok(())
    }
}

#[derive(CloneNoBound, DebugNoBound, Decode, Encode, TypeInfo, MaxEncodedLen, PartialEq, Eq)]
pub enum ApplicationAction {
    Add,
    Remove,
}

#[derive(CloneNoBound, DebugNoBound, Decode, Encode, TypeInfo, MaxEncodedLen, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub enum ApplicationStatus<T: crate::Config> {
    Open,
    Resolved {
        accepted: bool,
        resolved_by: AccountIdOf<T>,
    },
    Revoked {
        previously_accepted_by: AccountIdOf<T>,
        revoked_by: AccountIdOf<T>,
    },
    Expired,
}

pub fn submit_application<T: crate::Config>(
    payer: AccountIdOf<T>,
    agent_key: AccountIdOf<T>,
    data: Vec<u8>,
    removing: bool,
) -> DispatchResult {
    let action = if removing {
        ApplicationAction::Remove
    } else {
        ApplicationAction::Add
    };

    if action == ApplicationAction::Add && whitelist::is_whitelisted::<T>(&agent_key) {
        return Err(crate::Error::<T>::AlreadyWhitelisted.into());
    } else if action == ApplicationAction::Remove && !whitelist::is_whitelisted::<T>(&agent_key) {
        return Err(crate::Error::<T>::NotWhitelisted.into());
    }

    if exists_for_agent_key::<T>(&agent_key, &action) {
        return Err(crate::Error::<T>::ApplicationKeyAlreadyUsed.into());
    }

    let config = crate::GlobalGovernanceConfig::<T>::get();
    let cost = config.agent_application_cost;

    let _ = <T as crate::Config>::Currency::withdraw(
        &payer,
        cost,
        WithdrawReasons::except(WithdrawReasons::TIP),
        ExistenceRequirement::AllowDeath,
    )
    .map_err(|_| crate::Error::<T>::NotEnoughBalanceToApply)?;

    let data_len: u32 = data
        .len()
        .try_into()
        .map_err(|_| crate::Error::<T>::InvalidApplicationDataLength)?;

    let data_range = T::MinApplicationDataLength::get()..T::MaxApplicationDataLength::get();
    if !data_range.contains(&data_len) {
        return Err(crate::Error::<T>::InvalidApplicationDataLength.into());
    }

    let current_block = <polkadot_sdk::frame_system::Pallet<T>>::block_number();

    let expires_at = current_block + config.agent_application_expiration;

    let next_id = AgentApplications::<T>::iter()
        .count()
        .try_into()
        .map_err(|_| crate::Error::<T>::InternalError)?;

    let application = AgentApplication::<T> {
        id: next_id,
        payer_key: payer,
        agent_key,
        data: BoundedVec::truncate_from(data),
        cost,
        expires_at,
        status: ApplicationStatus::Open,
        action,
    };

    crate::AgentApplications::<T>::insert(next_id, application);
    crate::Pallet::<T>::deposit_event(crate::Event::<T>::ApplicationCreated(next_id));

    Ok(())
}

pub fn accept_application<T: crate::Config>(
    resolver: T::AccountId,
    application_id: u32,
) -> DispatchResult {
    let application = crate::AgentApplications::<T>::mutate(application_id, |app| {
        if let Some(app) = app {
            ensure!(
                app.status == ApplicationStatus::Open,
                crate::Error::<T>::ApplicationNotPending
            );

            app.status = ApplicationStatus::Resolved {
                accepted: true,
                resolved_by: resolver,
            };

            Ok(app.clone())
        } else {
            Err(DispatchError::from(crate::Error::<T>::ApplicationNotFound))
        }
    })?;

    let _ = match application.action {
        ApplicationAction::Add => whitelist::add_to_whitelist::<T>(application.agent_key.clone()),
        ApplicationAction::Remove => {
            whitelist::remove_from_whitelist::<T>(application.agent_key.clone())
        }
    };

    // Pay the application cost back to the applicant
    let _ =
        <T as crate::Config>::Currency::deposit_creating(&application.payer_key, application.cost);

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::ApplicationAccepted(application.id));

    Ok(())
}

pub fn deny_application<T: crate::Config>(
    resolved_by: T::AccountId,
    application_id: u32,
) -> DispatchResult {
    let application = crate::AgentApplications::<T>::mutate(application_id, |app| {
        if let Some(app) = app {
            ensure!(
                app.status == ApplicationStatus::Open,
                crate::Error::<T>::ApplicationNotPending
            );

            app.status = ApplicationStatus::Resolved {
                accepted: false,
                resolved_by,
            };

            Ok(app.clone())
        } else {
            Err(DispatchError::from(crate::Error::<T>::ApplicationNotFound))
        }
    })?;

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::ApplicationDenied(application.id));

    Ok(())
}

pub fn revoke_application<T: crate::Config>(
    revoked_by: T::AccountId,
    application_id: u32,
) -> DispatchResult {
    crate::AgentApplications::<T>::mutate(application_id, |app| {
        if let Some(app) = app {
            app.revoke(revoked_by)?;
            Ok(app.clone())
        } else {
            Err(DispatchError::from(crate::Error::<T>::ApplicationNotFound))
        }
    })?;

    Ok(())
}

pub(crate) fn resolve_expired_applications<T: crate::Config>(current_block: BlockNumberFor<T>) {
    for application in crate::AgentApplications::<T>::iter_values() {
        if current_block < application.expires_at {
            continue;
        }

        crate::AgentApplications::<T>::mutate(application.id, |application| {
            if let Some(app) = application {
                app.status = ApplicationStatus::Expired;
            }
        });

        crate::Pallet::<T>::deposit_event(crate::Event::<T>::ApplicationExpired(application.id));
    }
}

pub(crate) fn exists_for_agent_key<T: crate::Config>(
    key: &AccountIdOf<T>,
    action: &ApplicationAction,
) -> bool {
    crate::AgentApplications::<T>::iter().any(|(_, application)| {
        application.agent_key == *key
            && application.status == ApplicationStatus::Open
            && application.action == *action
    })
}
