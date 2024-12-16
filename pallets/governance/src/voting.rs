use crate::{proposal::ProposalStatus, AccountIdOf, Error, Event, Proposals};
use crate::{GovernanceConfiguration, NotDelegatingVotingPower};
use polkadot_sdk::sp_std::{collections::btree_set::BTreeSet, vec::Vec};
use polkadot_sdk::{frame_support::traits::tokens::Balance, sp_runtime::SaturatedConversion};
use polkadot_sdk::{
    frame_support::{dispatch::DispatchResult, ensure},
    polkadot_sdk_frame::prelude::OriginFor,
};
use polkadot_sdk::{
    frame_support::{dispatch::DispatchResult, ensure, storage::with_storage_layer},
    polkadot_sdk_frame::prelude::OriginFor,
    sc_telemetry::log,
    sp_core::ConstU32,
    sp_runtime::{BoundedBTreeMap, DispatchError, Percent},
};
use substrate_fixed::types::I92F36;

pub fn add_vote<T: crate::Config>(
    voter: AccountIdOf<T>,
    proposal_id: u64,
    agree: bool,
) -> DispatchResult {
    let Some(mut proposal) = Proposals::<T>::get(proposal_id) else {
        return Err(Error::<T>::ProposalNotFound.into());
    };

    let crate::proposal::ProposalStatus::Open {
        votes_for,
        votes_against,
        ..
    } = &mut proposal.status
    else {
        return Err(Error::<T>::ProposalClosed.into());
    };

    ensure!(
        !votes_for.contains(&voter) && !votes_against.contains(&voter),
        crate::Error::<T>::AlreadyVoted
    );

    let voter_delegated_stake = pallet_torus0::stake::sum_staked_by::<T>(&voter);
    let voter_owned_stake = pallet_torus0::stake::sum_staking_to::<T>(&voter);

    ensure!(
        voter_delegated_stake > 0 || voter_owned_stake > 0,
        crate::Error::<T>::InsufficientStake
    );

    if !crate::NotDelegatingVotingPower::<T>::get().contains(&voter) && voter_delegated_stake == 0 {
        return Err(Error::<T>::VoterIsDelegatingVotingPower.into());
    }

    if agree {
        votes_for
            .try_insert(voter.clone())
            .map_err(|_| Error::<T>::InternalError)?;
    } else {
        votes_against
            .try_insert(voter.clone())
            .map_err(|_| Error::<T>::InternalError)?;
    }

    Proposals::<T>::insert(proposal.id, proposal);
    crate::Pallet::<T>::deposit_event(Event::<T>::ProposalVoted(proposal_id, voter, agree));
    Ok(())
}

pub fn remove_vote<T: crate::Config>(voter: AccountIdOf<T>, proposal_id: u64) -> DispatchResult {
    let Ok(mut proposal) = Proposals::<T>::try_get(proposal_id) else {
        return Err(Error::<T>::ProposalNotFound.into());
    };

    let ProposalStatus::Open {
        votes_for,
        votes_against,
        ..
    } = &mut proposal.status
    else {
        return Err(Error::<T>::ProposalClosed.into());
    };

    let removed = votes_for.remove(&voter) || votes_against.remove(&voter);

    // Check if the voter has actually voted on the proposal
    ensure!(removed, crate::Error::<T>::NotVoted);

    // Update the proposal in storage
    Proposals::<T>::insert(proposal.id, proposal);
    crate::Pallet::<T>::deposit_event(Event::<T>::ProposalVoteUnregistered(proposal_id, voter));
    Ok(())
}

pub fn enable_delegation<T: crate::Config>(_origin: OriginFor<T>) -> DispatchResult {
    todo!()
}

pub fn disable_delegation<T: crate::Config>(_origin: OriginFor<T>) -> DispatchResult {
    todo!()
}
