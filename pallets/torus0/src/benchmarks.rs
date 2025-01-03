use pallet_governance_api::GovernanceApi;
use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_benchmarking::{account, benchmarks},
    frame_system::{self, RawOrigin},
    sp_std::vec,
};

use crate::*;

fn mock_agent<T: Config>(id: &T::AccountId) {
    <Pallet<T> as Torus0Api<
        T::AccountId,
        <<T as Config>::Currency as Currency<T::AccountId>>::Balance,
        <<T as Config>::Currency as Currency<T::AccountId>>::NegativeImbalance,
    >>::force_register_agent(id, vec![], vec![], vec![])
    .expect("failed to register agent");

    let min_allowed_stake = MinAllowedStake::<T>::get();
    let _ = T::Currency::deposit_creating(id, min_allowed_stake);
}

benchmarks! {
    add_stake {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        mock_agent::<T>(&module_key);

        let min_allowed_stake = MinAllowedStake::<T>::get();
    }: _(RawOrigin::Signed(module_key.clone()), module_key.clone(), min_allowed_stake)

    remove_stake {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        mock_agent::<T>(&module_key);

        let min_allowed_stake = MinAllowedStake::<T>::get();
        stake::add_stake::<T>(module_key.clone(), module_key.clone(), min_allowed_stake)
            .expect("failed to stake");
    }: _(RawOrigin::Signed(module_key.clone()), module_key.clone(), min_allowed_stake)

    transfer_stake {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        mock_agent::<T>(&module_key);

        let module_key2: T::AccountId = account("ModuleKey2", 1, 2);
        mock_agent::<T>(&module_key2);

        let min_allowed_stake = MinAllowedStake::<T>::get();
        stake::add_stake::<T>(module_key.clone(), module_key.clone(), min_allowed_stake)
            .expect("failed to stake");
    }: _(RawOrigin::Signed(module_key.clone()), module_key.clone(), module_key2.clone(), min_allowed_stake)

    register_agent {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        <T::Governance>::set_whitelisted(&module_key);

        let name = vec![b'0'; MinNameLength::<T>::get() as usize];
        let url = vec![b'0'; MinNameLength::<T>::get() as usize];
        let metadata= vec![b'0'; T::MaxAgentMetadataLengthConstraint::get() as usize];

    }: _(RawOrigin::Signed(module_key.clone()), module_key.clone(), name, url, metadata)

    unregister_agent {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        mock_agent::<T>(&module_key);
    }: _(RawOrigin::Signed(module_key))

    update_agent {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        mock_agent::<T>(&module_key);

        let name = vec![b'0'; MinNameLength::<T>::get() as usize];
        let url = vec![b'0'; MinNameLength::<T>::get() as usize];
        let metadata= vec![b'0'; T::MaxAgentMetadataLengthConstraint::get() as usize];

        let constraints = FeeConstraints::<T>::get();
    }: _(RawOrigin::Signed(module_key), name, url, Some(metadata), Some(constraints.min_staking_fee), Some(constraints.min_weight_control_fee))
}
