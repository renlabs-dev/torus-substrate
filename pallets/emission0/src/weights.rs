use crate::{ConsensusMember, ConsensusMembers};
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

pub fn set_weights<T: crate::Config>(
    origin: OriginFor<T>,
    mut weights: sp_std::vec::Vec<(T::AccountId, u16)>,
) -> DispatchResult {
    let acc_id = ensure_signed(origin)?;

    <T::Governance>::ensure_allocator(&acc_id)?;

    ensure!(
        !crate::WeightControlDelegation::<T>::contains_key(&acc_id),
        crate::Error::<T>::CannotSetWeightsWhileDelegating
    );

    ensure!(
        weights.len() <= crate::MaxAllowedWeights::<T>::get() as usize,
        crate::Error::<T>::WeightSetTooLarge
    );

    ensure!(
        <T::Torus>::is_agent_registered(&acc_id),
        crate::Error::<T>::AgentIsNotRegistered
    );

    let total_stake: u128 = <T::Torus>::staked_by(&acc_id)
        .iter()
        .map(|(_, stake)| *stake)
        .sum();
    let min_stake_for_weights = crate::MinStakePerWeight::<T>::get()
        .checked_mul(weights.len() as u128)
        .unwrap_or_default();

    ensure!(
        total_stake >= min_stake_for_weights,
        crate::Error::<T>::NotEnoughStakeToSetWeights
    );

    for (target, _) in &weights {
        ensure!(
            &acc_id != target,
            crate::Error::<T>::CannotSetWeightsForSelf
        );

        ensure!(
            <T::Torus>::is_agent_registered(target),
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
    origin: OriginFor<T>,
    target: T::AccountId,
) -> DispatchResult {
    let acc_id = ensure_signed(origin)?;

    ensure!(
        acc_id != target,
        crate::Error::<T>::CannotDelegateWeightControlToSelf,
    );

    ensure!(
        <T::Torus>::is_agent_registered(&acc_id),
        crate::Error::<T>::AgentIsNotRegistered
    );

    ensure!(
        <T::Torus>::is_agent_registered(&target),
        crate::Error::<T>::AgentIsNotRegistered
    );

    crate::WeightControlDelegation::<T>::set(&acc_id, Some(target.clone()));

    crate::Pallet::<T>::deposit_event(crate::Event::<T>::DelegatedWeightControl(acc_id, target));

    Ok(())
}
