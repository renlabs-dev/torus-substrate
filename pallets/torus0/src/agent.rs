use crate::AccountIdOf;
use codec::{Decode, Encode, MaxEncodedLen};
use pallet_governance_api::GovernanceApi;
use polkadot_sdk::frame_election_provider_support::Get;
use polkadot_sdk::sp_runtime::DispatchError;
use polkadot_sdk::{
    frame_support::{dispatch::DispatchResult, ensure, CloneNoBound},
    sp_runtime::{BoundedVec, Percent},
};
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;

#[derive(CloneNoBound, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Agent<T: crate::Config> {
    pub key: AccountIdOf<T>,
    pub name: BoundedVec<u8, T::MaxAgentNameLengthConstraint>,
    pub url: BoundedVec<u8, T::MaxAgentUrlLengthConstraint>,
    pub metadata: BoundedVec<u8, T::MaxAgentMetadataLengthConstraint>,
    pub weight_penalty_factor: Percent,
}

pub fn register<T: crate::Config>(
    agent_key: AccountIdOf<T>,
    name: Vec<u8>,
    url: Vec<u8>,
    metadata: Vec<u8>,
) -> DispatchResult {
    ensure!(
        !exists::<T>(&agent_key),
        crate::Error::<T>::AgentAlreadyRegistered
    );

    // TODO: Take pruning scores into consideration
    ensure!(
        crate::Agents::<T>::iter().count() < crate::MaxAllowedAgents::<T>::get() as usize,
        crate::Error::<T>::MaxAllowedAgents
    );

    ensure!(
        crate::Agents::<T>::iter().count() < crate::MaxAllowedAgents::<T>::get() as usize,
        crate::Error::<T>::MaxAllowedAgents
    );

    ensure!(
        crate::RegistrationsThisBlock::<T>::get() < crate::MaxRegistrationsPerBlock::<T>::get(),
        crate::Error::<T>::TooManyAgentRegistrationsThisBlock
    );

    let burn_config = crate::BurnConfig::<T>::get();
    ensure!(
        crate::RegistrationsThisInterval::<T>::get() < burn_config.max_registrations_per_interval,
        crate::Error::<T>::TooManyAgentRegistrationsThisBlock
    );

    ensure!(
        <T::Governance>::is_whitelisted(&agent_key),
        crate::Error::<T>::AgentKeyNotWhitelisted
    );

    validate_agent_name::<T>(&name[..])?;
    validate_agent_url::<T>(&url[..])?;
    validate_agent_metadata::<T>(&metadata[..])?;

    crate::Agents::<T>::insert(
        agent_key.clone(),
        Agent {
            key: agent_key.clone(),
            name: BoundedVec::truncate_from(name),
            url: BoundedVec::truncate_from(url),
            metadata: BoundedVec::truncate_from(metadata),
            weight_penalty_factor: Percent::from_percent(100),
        },
    );

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::AgentRegistered(agent_key));

    Ok(())
}

pub fn unregister<T: crate::Config>(agent_key: AccountIdOf<T>) -> DispatchResult {
    ensure!(
        exists::<T>(&agent_key),
        crate::Error::<T>::AgentDoesNotExist
    );

    crate::Agents::<T>::remove(&agent_key);
    crate::stake::clear_key::<T>(&agent_key)?;

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::AgentUnregistered(agent_key));

    Ok(())
}

pub fn update<T: crate::Config>(
    agent_key: AccountIdOf<T>,
    name: Vec<u8>,
    url: Vec<u8>,
    metadata: Option<Vec<u8>>,
    staking_fee: Option<Percent>,
    weight_control_fee: Option<Percent>,
) -> DispatchResult {
    ensure!(
        exists::<T>(&agent_key),
        crate::Error::<T>::AgentDoesNotExist
    );

    crate::Agents::<T>::try_mutate(&agent_key, |agent| {
        let Some(agent) = agent else {
            return Err(crate::Error::<T>::AgentDoesNotExist.into());
        };

        validate_agent_name::<T>(&name[..])?;
        agent.name = BoundedVec::truncate_from(name);

        validate_agent_url::<T>(&url[..])?;
        agent.url = BoundedVec::truncate_from(url);

        if let Some(metadata) = metadata {
            validate_agent_metadata::<T>(&metadata[..])?;
            agent.metadata = BoundedVec::truncate_from(metadata);
        }

        Ok::<(), DispatchError>(())
    })?;

    if staking_fee.is_none() && weight_control_fee.is_none() {
        return Ok(());
    }

    crate::Fee::<T>::try_mutate(&agent_key, |fee| {
        let constraints = crate::FeeConstraints::<T>::get();

        if let Some(staking_fee) = staking_fee {
            ensure!(
                staking_fee >= constraints.min_staking_fee,
                crate::Error::<T>::InvalidStakingFee
            );

            fee.staking_fee = staking_fee;
        }

        if let Some(weight_control_fee) = weight_control_fee {
            ensure!(
                weight_control_fee >= constraints.min_weight_control_fee,
                crate::Error::<T>::InvalidWeightControlFee
            );

            fee.weight_control_fee = weight_control_fee;
        }

        Ok::<(), DispatchError>(())
    })?;

    Ok(())
}

pub fn exists<T: crate::Config>(key: &AccountIdOf<T>) -> bool {
    crate::Agents::<T>::contains_key(key)
}

fn validate_agent_name<T: crate::Config>(bytes: &[u8]) -> DispatchResult {
    let len: u32 = bytes
        .len()
        .try_into()
        .map_err(|_| crate::Error::<T>::AgentNameTooLong)?;

    ensure!(
        len >= crate::MinNameLength::<T>::get() as u32,
        crate::Error::<T>::AgentNameTooShort
    );

    ensure!(
        len <= (crate::MaxNameLength::<T>::get() as u32)
            .min(T::MaxAgentNameLengthConstraint::get()),
        crate::Error::<T>::AgentNameTooLong
    );

    ensure!(
        core::str::from_utf8(bytes).is_ok(),
        crate::Error::<T>::InvalidAgentName
    );

    Ok(())
}

fn validate_agent_url<T: crate::Config>(bytes: &[u8]) -> DispatchResult {
    let len: u32 = bytes
        .len()
        .try_into()
        .map_err(|_| crate::Error::<T>::AgentUrlTooLong)?;

    ensure!(len > 0, crate::Error::<T>::AgentNameTooShort);

    ensure!(
        len <= (crate::MaxNameLength::<T>::get() as u32)
            .min(T::MaxAgentNameLengthConstraint::get()),
        crate::Error::<T>::AgentUrlTooShort
    );

    ensure!(
        core::str::from_utf8(bytes).is_ok(),
        crate::Error::<T>::InvalidAgentUrl
    );

    Ok(())
}

fn validate_agent_metadata<T: crate::Config>(bytes: &[u8]) -> DispatchResult {
    let len: u32 = bytes
        .len()
        .try_into()
        .map_err(|_| crate::Error::<T>::AgentMetadataTooLong)?;

    ensure!(len > 0, crate::Error::<T>::AgentMetadataTooShort);

    ensure!(
        len <= T::MaxAgentMetadataLengthConstraint::get(),
        crate::Error::<T>::AgentMetadataTooLong
    );

    ensure!(
        core::str::from_utf8(bytes).is_ok(),
        crate::Error::<T>::InvalidAgentMetadata
    );

    Ok(())
}
