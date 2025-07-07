#![cfg(feature = "runtime-benchmarks")]

use pallet_governance_api::GovernanceApi;
use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_benchmarking::{account, v2::*},
    frame_system::RawOrigin,
    sp_std::vec,
};

use crate::*;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_weights() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let module_key2: T::AccountId = account("ModuleKey2", 0, 3);

        <T::Governance>::set_allocator(&module_key2);

        <T::Torus>::force_register_agent(&module_key, vec![], vec![], vec![])
            .expect("failed to register agent");
        <T::Torus>::force_register_agent(&module_key2, vec![], vec![], vec![])
            .expect("failed to register agent");

        <T::Governance>::force_set_whitelisted(&module_key);
        <T::Governance>::force_set_whitelisted(&module_key2);

        <T::Governance>::set_allocator(&module_key2);
        let _ = <T::Currency>::deposit_creating(&module_key2, <T::Torus>::min_validator_stake());
        <T::Torus>::force_set_stake(
            &module_key2,
            &module_key2,
            <T::Torus>::min_validator_stake(),
        )
        .unwrap();

        let weights = vec![(module_key, 10)];

        #[extrinsic_call]
        set_weights(RawOrigin::Signed(module_key2), weights)
    }

    #[benchmark]
    fn delegate_weight_control() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let module_key2: T::AccountId = account("ModuleKey2", 0, 3);

        <T::Torus>::force_register_agent(&module_key, vec![], vec![], vec![])
            .expect("failed to register agent");
        <T::Torus>::force_register_agent(&module_key2, vec![], vec![], vec![])
            .expect("failed to register agent");

        <T::Governance>::force_set_whitelisted(&module_key);
        <T::Governance>::force_set_whitelisted(&module_key2);

        <T::Governance>::set_allocator(&module_key2);

        #[extrinsic_call]
        delegate_weight_control(RawOrigin::Signed(module_key), module_key2.clone())
    }

    #[benchmark]
    fn regain_weight_control() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let module_key2: T::AccountId = account("ModuleKey2", 0, 3);

        <T::Torus>::force_register_agent(&module_key, vec![], vec![], vec![])
            .expect("failed to register agent");
        <T::Torus>::force_register_agent(&module_key2, vec![], vec![], vec![])
            .expect("failed to register agent");

        <T::Governance>::force_set_whitelisted(&module_key);
        <T::Governance>::force_set_whitelisted(&module_key2);

        <T::Governance>::set_allocator(&module_key);
        <T::Governance>::set_allocator(&module_key2);

        Pallet::<T>::delegate_weight_control(
            RawOrigin::Signed(module_key.clone()).into(),
            module_key2.clone(),
        )
        .expect("failed to delegate weight control");

        #[extrinsic_call]
        regain_weight_control(RawOrigin::Signed(module_key))
    }
}
