use pallet_governance_api::GovernanceApi;
use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_support::{dispatch::DispatchResult, ensure},
    frame_system::ensure_signed,
    polkadot_sdk_frame::prelude::OriginFor,
    sp_core::ConstU32,
    sp_runtime::BoundedVec,
    sp_std,
};

use crate::{ConsensusMember, ConsensusMembers};

pub fn set_weights<T: crate::Config>(
    origin: OriginFor<T>,
    mut weights: sp_std::vec::Vec<(T::AccountId, u16)>,
) -> DispatchResult {
    let acc_id = ensure_signed(origin)?;
    <T::Governance>::ensure_allocator(&acc_id)?;

    ensure!(
        !crate::WeightControlDelegation::<T>::contains_key(&acc_id),
        crate::Error::<T>::CannotSetWeightsWhileDelegating,
    );

    ensure!(
        <T::Torus>::is_agent_registered(&acc_id) && <T::Governance>::is_whitelisted(&acc_id),
        crate::Error::<T>::AgentIsNotRegistered
    );

    let total_stake: u128 = <T::Torus>::staked_by(&acc_id)
        .iter()
        .map(|(_, stake)| *stake)
        .sum();
    ensure!(
        total_stake >= <T::Torus>::min_validator_stake(),
        crate::Error::<T>::NotEnoughStakeToSetWeights
    );

    for (target, _) in &weights {
        ensure!(
            &acc_id != target,
            crate::Error::<T>::CannotSetWeightsForSelf
        );

        ensure!(
            <T::Torus>::is_agent_registered(target) && <T::Governance>::is_whitelisted(target),
            crate::Error::<T>::AgentIsNotRegistered
        );
    }

    weights.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    weights.dedup();

    let weights: BoundedVec<_, ConstU32<{ u32::MAX }>> =
        BoundedVec::try_from(weights).map_err(|_| crate::Error::<T>::WeightSetTooLarge)?;

    ConsensusMembers::<T>::mutate(&acc_id, |member: &mut Option<ConsensusMember<T>>| {
        let member = member.get_or_insert_with(Default::default);
        member.update_weights(weights);
    });

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::WeightsSet(acc_id));

    Ok(())
}

pub fn delegate_weight_control<T: crate::Config>(
    delegator: T::AccountId,
    delegatee: T::AccountId,
) -> DispatchResult {
    ensure!(
        delegator != delegatee,
        crate::Error::<T>::CannotDelegateWeightControlToSelf,
    );

    ensure!(
        <T::Torus>::is_agent_registered(&delegator) && <T::Governance>::is_whitelisted(&delegator),
        crate::Error::<T>::AgentIsNotRegistered
    );

    ensure!(
        <T::Torus>::is_agent_registered(&delegatee) && <T::Governance>::is_whitelisted(&delegatee),
        crate::Error::<T>::AgentIsNotRegistered
    );

    // At the current network stage, it only makes sense to delegate weight control
    // to allocators.
    <T::Governance>::ensure_allocator(&delegatee)?;

    crate::WeightControlDelegation::<T>::set(&delegator, Some(delegatee.clone()));

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::DelegatedWeightControl(
        delegator, delegatee,
    ));

    Ok(())
}

pub fn regain_weight_control<T: crate::Config>(origin: OriginFor<T>) -> DispatchResult {
    let acc_id = ensure_signed(origin)?;

    if <T::Governance>::ensure_allocator(&acc_id).is_err() {
        return Err(crate::Error::<T>::WeightControlNotEnabled.into());
    }

    crate::WeightControlDelegation::<T>::mutate(acc_id, |val| match val.take() {
        Some(_) => Ok(()),
        None => Err(crate::Error::<T>::AgentIsNotDelegating.into()),
    })
}
