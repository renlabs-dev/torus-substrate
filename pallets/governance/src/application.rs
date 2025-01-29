use crate::frame::traits::ExistenceRequirement;
use crate::{whitelist, AccountIdOf, AgentApplications, BalanceOf, Block};
use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::frame_election_provider_support::Get;
use polkadot_sdk::frame_support::dispatch::DispatchResult;
use polkadot_sdk::frame_support::traits::Currency;
use polkadot_sdk::frame_support::traits::WithdrawReasons;
use polkadot_sdk::frame_support::DebugNoBound;
use polkadot_sdk::sp_runtime::BoundedVec;
use polkadot_sdk::sp_std::vec::Vec;
use scale_info::TypeInfo;

/// Decentralized autonomous organization application, it's used to do agent
/// operations on the network, like creating or removing, and needs to be approved
/// by other peers.
#[derive(CloneNoBound, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct AgentApplication<T: crate::Config> {
    pub id: u32,
    pub payer_key: AccountIdOf<T>,
    pub agent_key: AccountIdOf<T>,
    pub data: BoundedVec<u8, T::MaxApplicationDataLength>,
    pub cost: BalanceOf<T>,
    /// The exact block when the agent will be deleted/expired
    pub expires_at: BlockNumberFor<T>,
    pub action: ApplicationAction,
    pub status: ApplicationStatus,
}

/// DAO avaliable actions on the network
#[derive(DebugNoBound, Decode, Encode, TypeInfo, MaxEncodedLen, PartialEq, Eq)]
pub enum ApplicationAction {
    Add,
    Remove,
}

#[derive(DebugNoBound, Decode, Encode, TypeInfo, MaxEncodedLen, PartialEq, Eq)]
pub enum ApplicationStatus {
    Open,
    Resolved { accepted: bool },
    Expired,
}

impl<T: crate::Config> AgentApplication<T> {
    /// Returns true if the application is in the Open state, i.e. not Expired
    /// or Resolved, meaning it can be acted upon.
    pub fn is_open(&self) -> bool {
        matches!(self.status, ApplicationStatus::Open)
    }
}

/// Create DAO application for the _agent_, if it's not whitelisted yet. It will withdraw
/// a fee from the payer account for either creating or deleting an agent.
pub fn submit_application<T: crate::Config>(
    payer: AccountIdOf<T>,
    agent_key: AccountIdOf<T>,
    data: Vec<u8>,
    removing: bool,
) -> DispatchResult {
    if !removing && whitelist::is_whitelisted::<T>(&agent_key) {
        return Err(crate::Error::<T>::AlreadyWhitelisted.into());
    } else if removing && !whitelist::is_whitelisted::<T>(&agent_key) {
        return Err(crate::Error::<T>::NotWhitelisted.into());
    }

    let action = if removing {
        ApplicationAction::Remove
    } else {
        ApplicationAction::Add
    };

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

    let current_block: u64 =
        TryInto::try_into(<polkadot_sdk::frame_system::Pallet<T>>::block_number())
            .ok()
            .expect("blockchain will not exceed 2^64 blocks; QED.");

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

/// Accept DAO application, either creating or deleting it from the whitelist. The application
/// must be open and exist on the chain.
pub fn accept_application<T: crate::Config>(application_id: u32) -> DispatchResult {
    let application = crate::AgentApplications::<T>::get(application_id)
        .ok_or(crate::Error::<T>::ApplicationNotFound)?;

    if !application.is_open() {
        return Err(crate::Error::<T>::ApplicationNotOpen.into());
    }

    match application.action {
        ApplicationAction::Add => {
            crate::Whitelist::<T>::insert(application.agent_key.clone(), ());
            crate::Pallet::<T>::deposit_event(crate::Event::<T>::WhitelistAdded(
                application.agent_key,
            ));
        }
        ApplicationAction::Remove => {
            crate::Whitelist::<T>::remove(&application.agent_key);
            crate::Pallet::<T>::deposit_event(crate::Event::<T>::WhitelistRemoved(
                application.agent_key,
            ));
        }
    }

    crate::AgentApplications::<T>::mutate(application_id, |application| {
        if let Some(app) = application {
            app.status = ApplicationStatus::Resolved { accepted: true };
        }
    });

    // Pay the application cost back to the applicant
    // TODO: should this value be used?
    let _ =
        <T as crate::Config>::Currency::deposit_creating(&application.payer_key, application.cost);

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::ApplicationAccepted(application.id));

    Ok(())
}

/// Deny DAO application, deleting it from the whitelist. The application must be open and
/// exist on the chain.
pub fn deny_application<T: crate::Config>(application_id: u32) -> DispatchResult {
    let application = crate::AgentApplications::<T>::get(application_id)
        .ok_or(crate::Error::<T>::ApplicationNotFound)?;

    if !application.is_open() {
        return Err(crate::Error::<T>::ApplicationNotOpen.into());
    }

    crate::AgentApplications::<T>::mutate(application_id, |application| {
        if let Some(app) = application {
            app.status = ApplicationStatus::Resolved { accepted: false };
        }
    });

    // Application cost is discarded

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::ApplicationDenied(application.id));

    Ok(())
}

/// Reverts the curator application on the chain, if it's not an application creation.
pub(crate) fn resolve_expired_applications<T: crate::Config>(current_block: Block) {
    for application in crate::AgentApplications::<T>::iter_values() {
        // Skip if not expired yet or if not in Open status
        if current_block < application.expires_at
            || !matches!(application.status, ApplicationStatus::Open)
        {
            continue;
        }

        crate::AgentApplications::<T>::mutate(application.id, |application| {
            if let Some(app) = application {
                if matches!(app.status, ApplicationStatus::Open) {
                    app.status = ApplicationStatus::Expired;
                }
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
