#![cfg(feature = "runtime-benchmarks")]

use pallet_permission0_api::{
    CuratorPermissions, Permission0CuratorApi, PermissionDuration, RevocationTerms,
};
use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_benchmarking::{account, v2::*},
    frame_system::RawOrigin,
    sp_std::vec,
};

use crate::*;

fn create_application<T: Config>(module_key: &T::AccountId) {
    let config = crate::GlobalGovernanceConfig::<T>::get();
    let cost = config.agent_application_cost;
    let _ = <T as crate::Config>::Currency::deposit_creating(module_key, cost);

    let min_data_len = T::MinApplicationDataLength::get();
    let data = vec![0; min_data_len as usize];

    application::submit_application::<T>(module_key.clone(), module_key.clone(), data, false)
        .expect("failed to submit application");
}

fn curator<T: Config>() -> T::AccountId {
    let curator_id: T::AccountId = account("Curator", 10, 10);
    <<T as crate::Config>::Permission0 as Permission0CuratorApi<
        T::AccountId,
        OriginFor<T>,
        BlockNumberFor<T>,
    >>::grant_curator_permission(
        RawOrigin::Root.into(),
        curator_id.clone(),
        CuratorPermissions::all(),
        None,
        PermissionDuration::Indefinite,
        RevocationTerms::Irrevocable,
    )
    .unwrap();
    curator_id
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn add_allocator() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);

        #[extrinsic_call]
        add_allocator(RawOrigin::Root, module_key)
    }

    #[benchmark]
    fn remove_allocator() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        roles::add_allocator::<T>(module_key.clone()).expect("failed to add allocator");

        #[extrinsic_call]
        remove_allocator(RawOrigin::Root, module_key)
    }

    #[benchmark]
    fn add_to_whitelist() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        roles::add_allocator::<T>(module_key.clone()).expect("failed to add allocator");

        #[extrinsic_call]
        add_to_whitelist(RawOrigin::Signed(curator::<T>()), module_key)
    }

    #[benchmark]
    fn remove_from_whitelist() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        whitelist::add_to_whitelist::<T>(module_key.clone()).expect("failed to add to whitelist");

        #[extrinsic_call]
        remove_from_whitelist(RawOrigin::Signed(curator::<T>()), module_key)
    }

    #[benchmark]
    fn submit_application() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.agent_application_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

        let min_data_len = T::MinApplicationDataLength::get();
        let data = vec![0; min_data_len as usize];

        #[extrinsic_call]
        submit_application(
            RawOrigin::Signed(module_key.clone()),
            module_key.clone(),
            data,
            false,
        )
    }

    #[benchmark]
    fn accept_application() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        create_application::<T>(&module_key);

        #[extrinsic_call]
        accept_application(RawOrigin::Signed(curator::<T>()), 0)
    }

    #[benchmark]
    fn deny_application() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        create_application::<T>(&module_key);

        #[extrinsic_call]
        deny_application(RawOrigin::Signed(curator::<T>()), 0)
    }

    #[benchmark]
    fn penalize_agent() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);

        <pallet_torus0::Pallet<T> as Torus0Api<T::AccountId, BalanceOf<T>>>::force_register_agent(
            &module_key,
            vec![],
            vec![],
            vec![],
        )
        .expect("failed to register agent");

        #[extrinsic_call]
        penalize_agent(RawOrigin::Signed(curator::<T>()), module_key.clone(), 50)
    }

    #[benchmark]
    fn add_global_params_proposal() {
        let module_key: T::AccountId = account("Module", 0, 2);

        let params = proposal::GlobalParamsData {
            min_name_length: 2,
            max_name_length: (T::MaxAgentNameLengthConstraint::get() as u16).saturating_sub(1),
            min_weight_control_fee: 1,
            min_staking_fee: 1,
            dividends_participation_weight: Percent::zero(),
            namespace_pricing_config: T::DefaultNamespacePricingConfig::get(),
            proposal_cost: 0,
        };
        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

        #[extrinsic_call]
        add_global_params_proposal(RawOrigin::Signed(module_key), params, data)
    }

    #[benchmark]
    fn add_global_custom_proposal() {
        let module_key: T::AccountId = account("Module", 0, 2);

        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

        #[extrinsic_call]
        add_global_custom_proposal(RawOrigin::Signed(module_key), data)
    }

    #[benchmark]
    fn add_dao_treasury_transfer_proposal() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

        #[extrinsic_call]
        add_dao_treasury_transfer_proposal(
            RawOrigin::Signed(module_key),
            0,
            module_key.clone(),
            data,
        )
    }

    #[benchmark]
    fn vote_proposal() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

        proposal::add_global_custom_proposal::<T>(module_key.clone(), data)
            .expect("failed to create proposal");

        pallet_torus0::StakingTo::<T>::set(&module_key, &module_key, Some(1));

        voting::disable_delegation::<T>(module_key.clone()).expect("failed to disable delegation");

        #[extrinsic_call]
        vote_proposal(RawOrigin::Signed(module_key.clone()), 0, true)
    }

    #[benchmark]
    fn remove_vote_proposal() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

        proposal::add_global_custom_proposal::<T>(module_key.clone(), data)
            .expect("failed to create proposal");

        pallet_torus0::StakingTo::<T>::set(&module_key, &module_key, Some(1));

        voting::disable_delegation::<T>(module_key.clone()).expect("failed to disable delegation");

        voting::add_vote::<T>(module_key.clone(), 0, true).expect("failed to add vote");

        #[extrinsic_call]
        remove_vote_proposal(RawOrigin::Signed(module_key.clone()), 0)
    }

    #[benchmark]
    fn enable_vote_delegation() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);

        #[extrinsic_call]
        enable_vote_delegation(RawOrigin::Signed(module_key.clone()))
    }

    #[benchmark]
    fn disable_vote_delegation() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);

        #[extrinsic_call]
        disable_vote_delegation(RawOrigin::Signed(module_key.clone()))
    }

    #[benchmark]
    fn add_emission_proposal() {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

        #[extrinsic_call]
        add_emission_proposal(
            RawOrigin::Signed(module_key.clone()),
            Percent::from_parts(40),
            Percent::from_parts(40),
            Percent::from_parts(40),
            data,
        )
    }

    #[benchmark]
    fn toggle_agent_freezing() {
        #[extrinsic_call]
        toggle_agent_freezing(RawOrigin::Signed(curator::<T>()))
    }

    #[benchmark]
    fn toggle_namespace_freezing() {
        #[extrinsic_call]
        toggle_namespace_freezing(RawOrigin::Signed(curator::<T>()))
    }
}
