use polkadot_sdk::{
    frame_support::dispatch::DispatchResult, frame_system::ensure_signed,
    polkadot_sdk_frame::prelude::OriginFor, sp_core::ConstU32, sp_runtime::BoundedVec, sp_std,
};

use crate::{ConsensusMember, ConsensusMembers};

pub fn set_weights<T: crate::Config>(
    origin: OriginFor<T>,
    weights: sp_std::vec::Vec<(T::AccountId, u16)>,
) -> DispatchResult {
    let acc_id = ensure_signed(origin)?;

    // TODO: validate against self weight
    // TODO: validate that targets exist

    let weights: BoundedVec<_, ConstU32<{ u32::MAX }>> =
        BoundedVec::try_from(weights).map_err(|_| crate::Error::<T>::WeightSetTooLarge)?;

    ConsensusMembers::<T>::mutate(acc_id, |member: &mut Option<ConsensusMember<T>>| {
        let member = member.get_or_insert_with(Default::default);
        member.update_weights(weights);
    });

    todo!()
}

pub fn delegate_weight_control<T: crate::Config>(
    _origin: OriginFor<T>,
    _target: T::AccountId,
) -> DispatchResult {
    todo!()
}

pub fn regain_weight_control<T: crate::Config>(_origin: OriginFor<T>) -> DispatchResult {
    todo!()
}
