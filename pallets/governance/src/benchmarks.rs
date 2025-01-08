use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_benchmarking::{account, benchmarks},
    frame_system::{self, RawOrigin},
    sp_std::vec,
};

use crate::*;

fn create_application<T: Config>(module_key: &T::AccountId) {
    let config = crate::GlobalGovernanceConfig::<T>::get();
    let cost = config.agent_application_cost;
    let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

    let min_data_len = T::MinApplicationDataLength::get();
    let data = vec![0; min_data_len as usize];

    application::submit_application::<T>(module_key.clone(), module_key.clone(), data, false)
        .expect("failed to submit application");
}

benchmarks! {
    add_curator {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
    }: _(RawOrigin::Root, module_key)

    remove_curator {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        crate::roles::manage_role::<T, crate::Curators<T>>(module_key.clone(), true, crate::Error::<T>::AlreadyCurator)
            .expect("failed to add curator");
    }: _(RawOrigin::Root, module_key)

    add_allocator {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
    }: _(RawOrigin::Root, module_key)

    remove_allocator {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        crate::roles::manage_role::<T, crate::Allocators<T>>(module_key.clone(), true, crate::Error::<T>::AlreadyCurator)
            .expect("failed to add allocator");
    }: _(RawOrigin::Root, module_key)

    add_to_whitelist {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        crate::roles::manage_role::<T, crate::Curators<T>>(module_key.clone(), true, crate::Error::<T>::AlreadyCurator)
            .expect("failed to add curator");
    }: _(RawOrigin::Signed(module_key.clone()), module_key.clone())

    remove_from_whitelist {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        crate::roles::manage_role::<T, crate::Curators<T>>(module_key.clone(), true, crate::Error::<T>::AlreadyCurator)
            .expect("failed to add curator");
        whitelist::add_to_whitelist::<T>(module_key.clone())
            .expect("failed to add to whitelist");
    }: _(RawOrigin::Signed(module_key.clone()), module_key.clone())

    submit_application {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.agent_application_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(
            &module_key,
            cost,
        );

        let min_data_len = T::MinApplicationDataLength::get();
        let data = vec![0; min_data_len as usize];
    }: _(RawOrigin::Signed(module_key.clone()), module_key.clone(), data, false)

    accept_application {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);

        crate::roles::manage_role::<T, crate::Curators<T>>(module_key.clone(), true, crate::Error::<T>::AlreadyCurator)
            .expect("failed to add curator");

        create_application::<T>(&module_key);
    }: _(RawOrigin::Signed(module_key.clone()), 0)

    deny_application {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);

        crate::roles::manage_role::<T, crate::Curators<T>>(module_key.clone(), true, crate::Error::<T>::AlreadyCurator)
            .expect("failed to add curator");

        create_application::<T>(&module_key);
    }: _(RawOrigin::Signed(module_key.clone()), 0)

    revoke_application {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);

        crate::roles::manage_role::<T, crate::Curators<T>>(module_key.clone(), true, crate::Error::<T>::AlreadyCurator)
            .expect("failed to add curator");

        create_application::<T>(&module_key);
        application::accept_application::<T>(module_key.clone(), 0)
            .expect("failed to accept application");
    }: _(RawOrigin::Signed(module_key.clone()), 0)

    penalize_agent {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);

        crate::roles::manage_role::<T, crate::Curators<T>>(module_key.clone(), true, crate::Error::<T>::AlreadyCurator)
            .expect("failed to add curator");

        <pallet_torus0::Pallet::<T> as Torus0Api<
            T::AccountId,
            <<T as pallet_torus0::Config>::Currency as Currency<T::AccountId>>::Balance,
            <<T as pallet_torus0::Config>::Currency as Currency<T::AccountId>>::NegativeImbalance,
        >>::force_register_agent(
            &module_key,
            vec![],
            vec![],
            vec![],
        ).expect("failed to register agent");

    }: _(RawOrigin::Signed(module_key.clone()), module_key.clone(), 50)

    add_global_params_proposal {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);

        let params = proposal::GlobalParamsData {
            min_name_length: 2,
            max_name_length: T::MaxAgentNameLengthConstraint::get() as u16 - 1,
            max_allowed_agents: 1,
            max_allowed_weights: 1,
            min_stake_per_weight: 0,
            min_weight_control_fee: 1,
            min_staking_fee: 1,
            dividends_participation_weight: Percent::zero(),
            proposal_cost: 0,
        };
        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

    }:  _(RawOrigin::Signed(module_key.clone()), params, data)

    add_global_custom_proposal {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

    }:  _(RawOrigin::Signed(module_key.clone()), data)

    add_dao_treasury_transfer_proposal {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

    }:  _(RawOrigin::Signed(module_key.clone()), 0, module_key.clone(), data)

    vote_proposal {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

        proposal::add_global_custom_proposal::<T>(module_key.clone(), data)
            .expect("failed to create proposal");

        pallet_torus0::StakingTo::<T>::set(&module_key, &module_key, Some(1));

        voting::disable_delegation::<T>(module_key.clone())
            .expect("failed to disable delegation");

    }:  _(RawOrigin::Signed(module_key.clone()), 0, true)

    remove_vote_proposal {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

        proposal::add_global_custom_proposal::<T>(module_key.clone(), data)
            .expect("failed to create proposal");

        pallet_torus0::StakingTo::<T>::set(&module_key, &module_key, Some(1));

        voting::disable_delegation::<T>(module_key.clone())
            .expect("failed to disable delegation");

        voting::add_vote::<T>(module_key.clone(), 0, true)
            .expect("failed to add vote");

    }:  _(RawOrigin::Signed(module_key.clone()), 0)

    enable_vote_delegation {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
    }:  _(RawOrigin::Signed(module_key))

    disable_vote_delegation {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
    }:  _(RawOrigin::Signed(module_key))

    add_emission_proposal {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let data = vec![0];

        let config = crate::GlobalGovernanceConfig::<T>::get();
        let cost = config.proposal_cost;
        let _ = <T as crate::Config>::Currency::deposit_creating(&module_key, cost);

    }:  _(RawOrigin::Signed(module_key.clone()), Percent::from_parts(40), Percent::from_parts(40), data)

    set_root_curator {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
    }:  _(RawOrigin::Root, module_key)

    remove_root_curator {
    }:  _(RawOrigin::Root)


}
