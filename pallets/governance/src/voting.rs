use polkadot_sdk::frame_support::{dispatch::DispatchResult, ensure};

use crate::{AccountIdOf, Error, Event, Proposals, proposal::ProposalStatus};

/// Casts a vote on behalf of a voter.
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

/// Removes the casted vote.
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

/// Gives voting power delegation to the delegator's staked agents.
pub fn enable_delegation<T: crate::Config>(delegator: AccountIdOf<T>) -> DispatchResult {
    crate::NotDelegatingVotingPower::<T>::mutate(|delegators| {
        delegators.remove(&delegator);
    });

    Ok(())
}

/// Removes voting power delegation to the delegator's staked agents.
pub fn disable_delegation<T: crate::Config>(delegator: AccountIdOf<T>) -> DispatchResult {
    crate::NotDelegatingVotingPower::<T>::mutate(|delegators| {
        delegators
            .try_insert(delegator.clone())
            .map(|_| ())
            .map_err(|_| Error::<T>::InternalError.into())
    })
}
