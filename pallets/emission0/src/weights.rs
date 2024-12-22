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
    weights: sp_std::vec::Vec<(T::AccountId, u16)>,
) -> DispatchResult {
    let acc_id = ensure_signed(origin)?;

    ensure!(
        weights.len() <= crate::MaxAllowedWeights::<T>::get() as usize,
        crate::Error::<T>::WeightSetTooLarge
    );

    for (target, _) in &weights {
        ensure!(
            &acc_id != target,
            crate::Error::<T>::CannotSetWeightsForSelf
        );

        ensure!(
            <T::Torus>::is_agent_registered(target),
            crate::Error::<T>::AgentDoesNotExist
        );
    }

    let weights: BoundedVec<_, ConstU32<{ u32::MAX }>> =
        BoundedVec::try_from(weights).map_err(|_| crate::Error::<T>::WeightSetTooLarge)?;

    ConsensusMembers::<T>::mutate(acc_id, |member: &mut Option<ConsensusMember<T>>| {
        let member = member.get_or_insert_with(Default::default);
        member.update_weights(weights);
    });

    Ok(())
}

pub fn delegate_weight_control<T: crate::Config>(
    origin: OriginFor<T>,
    target: T::AccountId,
) -> DispatchResult {
    let acc_id = ensure_signed(origin)?;
    crate::WeightControlDelegation::<T>::set(acc_id, Some(target));
    Ok(())
}

pub fn regain_weight_control<T: crate::Config>(origin: OriginFor<T>) -> DispatchResult {
    let acc_id = ensure_signed(origin)?;
    crate::WeightControlDelegation::<T>::remove(acc_id);
    Ok(())
}
