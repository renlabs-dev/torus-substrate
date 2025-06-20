use codec::{Decode, Encode, MaxEncodedLen};
use pallet_emission0_api::Emission0Api;
use pallet_governance_api::GovernanceApi;
use pallet_torus0_api::{NamespacePath, NAMESPACE_AGENT_PREFIX};
use polkadot_sdk::{
    frame_election_provider_support::Get,
    frame_support::{
        dispatch::DispatchResult,
        ensure,
        traits::{Currency, ExistenceRequirement},
        DebugNoBound,
    },
    polkadot_sdk_frame::prelude::BlockNumberFor,
    sp_runtime::{traits::Saturating, BoundedVec, DispatchError, Percent},
    sp_tracing::{debug_span, warn},
};
use scale_info::{prelude::vec::Vec, TypeInfo};

use crate::AccountIdOf;

/// Agents are one of the primitives in the Torus ecosystem which are bounded
/// to modules in off-chain environment. They can receive weights by the
/// allocators.
///
/// Agent registration needs approval from a curator. Registration applications
/// are submitter at dao.torus.network.
#[derive(DebugNoBound, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Agent<T: crate::Config> {
    /// The key that bounds the agent to the module
    pub key: AccountIdOf<T>,
    pub name: BoundedVec<u8, T::MaxAgentNameLengthConstraint>,
    pub url: BoundedVec<u8, T::MaxAgentUrlLengthConstraint>,
    pub metadata: BoundedVec<u8, T::MaxAgentMetadataLengthConstraint>,
    /// Penalities acts on agent's incentives and dividends of users who set
    /// weights on them.
    pub weight_penalty_factor: Percent,
    pub registration_block: BlockNumberFor<T>,
    pub fees: crate::fee::ValidatorFee<T>,
    pub last_update_block: BlockNumberFor<T>,
}

/// Register an agent to the given key, payed by the payer key.
///
/// If the network is full, this function will drop enough agents until there's
/// at least one slot (see [`find_agent_to_prune`]). Fails if no agents were
/// eligible for pruning.
///
/// Registration fee is stored as [`crate::Burn`].
pub fn register<T: crate::Config>(
    payer: AccountIdOf<T>,
    agent_key: AccountIdOf<T>,
    name: Vec<u8>,
    url: Vec<u8>,
    metadata: Vec<u8>,
) -> DispatchResult {
    let span = debug_span!("register", agent.key = ?agent_key);
    let _guard = span.enter();

    ensure!(
        <T as crate::pallet::Config>::Governance::can_register_agent(&payer),
        crate::pallet::Error::<T>::AgentsFrozen
    );

    ensure!(
        !exists::<T>(&agent_key),
        crate::Error::<T>::AgentAlreadyRegistered
    );

    ensure!(
        crate::RegistrationsThisBlock::<T>::get() < crate::MaxRegistrationsPerBlock::<T>::get(),
        crate::Error::<T>::TooManyAgentRegistrationsThisBlock
    );

    let burn_config = crate::BurnConfig::<T>::get();
    ensure!(
        crate::RegistrationsThisInterval::<T>::get() < burn_config.max_registrations_per_interval,
        crate::Error::<T>::TooManyAgentRegistrationsThisInterval
    );

    let namespace_path: Vec<_> = [NAMESPACE_AGENT_PREFIX, &name].concat();
    let namespace_path = NamespacePath::new_agent(&namespace_path).map_err(|err| {
        warn!("{agent_key:?} tried using invalid name: {err:?}");
        crate::Error::<T>::InvalidNamespacePath
    })?;

    validate_agent_url::<T>(&url[..])?;
    validate_agent_metadata::<T>(&metadata[..])?;

    let burn = crate::Burn::<T>::get();

    // Registration cost is sent to treasury
    <T as crate::Config>::Currency::transfer(
        &payer,
        &<T as crate::Config>::Governance::dao_treasury_address(),
        burn,
        ExistenceRequirement::AllowDeath,
    )
    .map_err(|_| crate::Error::<T>::NotEnoughBalanceToRegisterAgent)?;

    let registration_block = <polkadot_sdk::frame_system::Pallet<T>>::block_number();
    crate::Agents::<T>::insert(
        agent_key.clone(),
        Agent {
            key: agent_key.clone(),
            name: BoundedVec::truncate_from(name),
            url: BoundedVec::truncate_from(url),
            metadata: BoundedVec::truncate_from(metadata),
            weight_penalty_factor: Percent::from_percent(0),
            registration_block,
            fees: Default::default(),
            last_update_block: registration_block,
        },
    );

    crate::namespace::create_namespace::<T>(
        crate::namespace::NamespaceOwnership::Account(agent_key.clone()),
        namespace_path,
    )?;

    crate::RegistrationsThisBlock::<T>::mutate(|value| value.saturating_add(1));
    crate::RegistrationsThisInterval::<T>::mutate(|value| value.saturating_add(1));

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::AgentRegistered(agent_key.clone()));

    if let Some(allocator) = <T::Governance>::get_allocators().next() {
        let _ = <T::Emission>::delegate_weight_control(&agent_key, &allocator);
    } else {
        polkadot_sdk::sp_tracing::warn!("no allocators available to delegate to for {agent_key:?}");
    }

    Ok(())
}

/// Unregister an agent key from the network, erasing all its data and removing
/// stakers.
pub fn unregister<T: crate::Config>(agent_key: AccountIdOf<T>) -> DispatchResult {
    let span = debug_span!("unregister", agent.key = ?agent_key);
    let _guard = span.enter();

    let agent = crate::Agents::<T>::get(&agent_key).ok_or(crate::Error::<T>::AgentDoesNotExist)?;

    let namespace_path: Vec<_> = [NAMESPACE_AGENT_PREFIX, &agent.name].concat();
    let namespace_path = NamespacePath::new_agent(&namespace_path)
        .map_err(|_| crate::Error::<T>::InvalidNamespacePath)?;
    crate::namespace::delete_namespace::<T>(
        crate::namespace::NamespaceOwnership::Account(agent_key.clone()),
        namespace_path,
    )?;

    crate::Agents::<T>::remove(&agent_key);
    crate::stake::clear_key::<T>(&agent_key)?;

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::AgentUnregistered(agent_key));

    Ok(())
}

/// Updates the metadata of an existing agent.
pub fn update<T: crate::Config>(
    agent_key: AccountIdOf<T>,
    url: Vec<u8>,
    metadata: Option<Vec<u8>>,
    staking_fee: Option<Percent>,
    weight_control_fee: Option<Percent>,
) -> DispatchResult {
    let span = debug_span!("update", agent.key = ?agent_key);
    let _guard = span.enter();

    crate::Agents::<T>::try_mutate(&agent_key, |agent| {
        let Some(agent) = agent else {
            return Err(crate::Error::<T>::AgentDoesNotExist.into());
        };

        if is_in_update_cooldown::<T>(&agent_key)? {
            return Err(crate::Error::<T>::AgentUpdateOnCooldown.into());
        }

        validate_agent_url::<T>(&url[..])?;
        agent.url = BoundedVec::truncate_from(url);

        if let Some(metadata) = metadata {
            validate_agent_metadata::<T>(&metadata[..])?;
            agent.metadata = BoundedVec::truncate_from(metadata);
        }

        let constraints = crate::FeeConstraints::<T>::get();

        if let Some(staking_fee) = staking_fee {
            ensure!(
                staking_fee >= constraints.min_staking_fee,
                crate::Error::<T>::InvalidStakingFee
            );

            agent.fees.staking_fee = staking_fee;
        }

        if let Some(weight_control_fee) = weight_control_fee {
            ensure!(
                weight_control_fee >= constraints.min_weight_control_fee,
                crate::Error::<T>::InvalidWeightControlFee
            );

            agent.fees.weight_control_fee = weight_control_fee;
        }

        Ok::<(), DispatchError>(())
    })?;

    set_in_cooldown::<T>(&agent_key)?;
    crate::Pallet::<T>::deposit_event(crate::Event::<T>::AgentUpdated(agent_key));

    Ok(())
}

fn is_in_update_cooldown<T: crate::Config>(key: &AccountIdOf<T>) -> Result<bool, DispatchError> {
    let current_block = <polkadot_sdk::frame_system::Pallet<T>>::block_number();
    let cooldown = crate::AgentUpdateCooldown::<T>::get();

    let last_update = crate::Agents::<T>::get(key)
        .ok_or(crate::Error::<T>::AgentDoesNotExist)?
        .last_update_block;

    Ok(last_update.saturating_add(cooldown) > current_block)
}

fn set_in_cooldown<T: crate::Config>(key: &AccountIdOf<T>) -> DispatchResult {
    crate::Agents::<T>::mutate(key, |agent| {
        let Some(agent) = agent else {
            return Err(crate::Error::<T>::AgentDoesNotExist.into());
        };

        agent.last_update_block = <polkadot_sdk::frame_system::Pallet<T>>::block_number();

        Ok(())
    })
}

pub fn exists<T: crate::Config>(key: &AccountIdOf<T>) -> bool {
    crate::Agents::<T>::contains_key(key)
}

fn validate_agent_url<T: crate::Config>(bytes: &[u8]) -> DispatchResult {
    let len: u32 = bytes
        .len()
        .try_into()
        .map_err(|_| crate::Error::<T>::AgentUrlTooLong)?;

    ensure!(len > 0, crate::Error::<T>::AgentUrlTooShort);

    ensure!(
        len <= (crate::MaxAgentUrlLength::<T>::get() as u32)
            .min(T::MaxAgentUrlLengthConstraint::get()),
        crate::Error::<T>::AgentUrlTooLong
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

#[doc(hidden)]
pub enum PruningStrategy {
    /// Finds the agent producing least dividends and incentives to
    /// the network that is older than the current immunity period.
    LeastProductive,
    /// Like [`PruningStrategy::LeastProductive`] but ignoring the immunity
    /// period.
    IgnoreImmunity,
}
