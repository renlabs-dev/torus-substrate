use crate::frame::traits::ExistenceRequirement;
use crate::{whitelist, AccountIdOf, BalanceOf, Block};
use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::frame_election_provider_support::Get;
use polkadot_sdk::frame_support::dispatch::DispatchResult;
use polkadot_sdk::frame_support::traits::Currency;
use polkadot_sdk::frame_support::traits::WithdrawReasons;
use polkadot_sdk::frame_support::DebugNoBound;
use polkadot_sdk::sp_runtime::BoundedVec;
use polkadot_sdk::sp_std::vec::Vec;
use scale_info::TypeInfo;

#[derive(DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct AgentApplication<T: crate::Config> {
    pub id: u32,
    pub payer_key: AccountIdOf<T>,
    pub agent_key: AccountIdOf<T>,
    pub data: BoundedVec<u8, T::MaxApplicationDataLength>,
    pub cost: BalanceOf<T>,
    pub expires_at: Block,
}

pub fn submit_application<T: crate::Config>(
    payer: AccountIdOf<T>,
    agent_key: AccountIdOf<T>,
    data: Vec<u8>,
) -> DispatchResult {
    if whitelist::is_whitelisted::<T>(&agent_key) {
        return Err(crate::Error::<T>::AlreadyWhitelisted.into());
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

    let application_id: u32 = crate::AgentApplicationId::<T>::mutate(|id| {
        let last_id = *id;
        *id = id.saturating_add(1);
        last_id
    });

    let application = AgentApplication::<T> {
        id: application_id,
        payer_key: payer,
        agent_key,
        data: BoundedVec::truncate_from(data),
        cost,
        expires_at,
    };

    crate::AgentApplications::<T>::insert(application_id, application);
    crate::Pallet::<T>::deposit_event(crate::Event::<T>::ApplicationCreated(application_id));

    Ok(())
}

pub fn accept_application<T: crate::Config>(application_id: u32) -> DispatchResult {
    let application = crate::AgentApplications::<T>::get(application_id)
        .ok_or(crate::Error::<T>::ApplicationNotFound)?;

    crate::AgentApplications::<T>::remove(application.id);

    whitelist::add_to_whitelist::<T>(application.agent_key.clone())?;
    crate::Pallet::<T>::deposit_event(crate::Event::<T>::ApplicationAccepted(application.id));
    crate::Pallet::<T>::deposit_event(crate::Event::<T>::WhitelistAdded(application.agent_key));

    Ok(())
}

pub fn deny_application<T: crate::Config>(application_id: u32) -> DispatchResult {
    let application = crate::AgentApplications::<T>::get(application_id)
        .ok_or(crate::Error::<T>::ApplicationNotFound)?;

    crate::AgentApplications::<T>::remove(application.id);
    crate::Pallet::<T>::deposit_event(crate::Event::<T>::ApplicationDenied(application.id));

    Ok(())
}

pub(crate) fn remove_expired_applications<T: crate::Config>(current_block: Block) {
    for application in crate::AgentApplications::<T>::iter_values() {
        if current_block < application.expires_at {
            continue;
        }

        crate::AgentApplications::<T>::remove(application.id);
        crate::Pallet::<T>::deposit_event(crate::Event::<T>::ApplicationExpired(application.id));
    }
}
