#![cfg(feature = "runtime-benchmarks")]

use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_benchmarking::{account, v2::*},
    frame_system::RawOrigin,
    sp_runtime::Percent,
    sp_std::vec,
};

use crate::*;

#[benchmarks]
mod benchmarks {
    use polkadot_sdk::sp_std::collections::btree_map::BTreeMap;

    use super::*;

    #[benchmark]
    fn grant_emission_permission() {
        let grantor: T::AccountId = account("Grantor", 0, 0);
        let grantee: T::AccountId = account("Grantee", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        T::Torus::force_register_agent(&grantor, vec![], vec![], vec![])
            .expect("failed to register grantor");
        T::Torus::force_register_agent(&grantee, vec![], vec![], vec![])
            .expect("failed to register grantee");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&grantor, amount);

        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = vec![(target, 100)];
        let distribution = DistributionControl::Manual;
        let duration = PermissionDuration::Indefinite;
        let revocation = RevocationTerms::RevocableByGrantor;
        let enforcement = EnforcementAuthority::None;

        #[extrinsic_call]
        grant_emission_permission(
            RawOrigin::Signed(grantor),
            grantee,
            allocation,
            targets,
            distribution,
            duration,
            revocation,
            enforcement,
        )
    }

    #[benchmark]
    fn revoke_permission() {
        let grantor: T::AccountId = account("Grantor", 0, 0);
        let grantee: T::AccountId = account("Grantee", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        // Register agents
        T::Torus::force_register_agent(&grantor, vec![], vec![], vec![])
            .expect("failed to register grantor");
        T::Torus::force_register_agent(&grantee, vec![], vec![], vec![])
            .expect("failed to register grantee");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        // Fund grantor
        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&grantor, amount);

        // Create permission
        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = vec![(target, 100)];
        let permission_id = ext::emission_impl::grant_emission_permission_impl::<T>(
            grantor.clone(),
            grantee,
            allocation,
            targets,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByGrantor,
            EnforcementAuthority::None,
            None,
        )
        .expect("failed to grant permission");

        #[extrinsic_call]
        revoke_permission(RawOrigin::Signed(grantor), permission_id)
    }

    #[benchmark]
    fn execute_permission() {
        let grantor: T::AccountId = account("Grantor", 0, 0);
        let grantee: T::AccountId = account("Grantee", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        // Register agents
        T::Torus::force_register_agent(&grantor, vec![], vec![], vec![])
            .expect("failed to register grantor");
        T::Torus::force_register_agent(&grantee, vec![], vec![], vec![])
            .expect("failed to register grantee");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        // Fund grantor
        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&grantor, amount);

        // Create permission with fixed amount allocation
        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = vec![(target, 100)];

        let permission_id = ext::emission_impl::grant_emission_permission_impl::<T>(
            grantor.clone(),
            grantee,
            allocation,
            targets,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByGrantor,
            EnforcementAuthority::None,
            None,
        )
        .expect("failed to grant permission");

        AccumulatedStreamAmounts::<T>::set((&grantor, stream_id, permission_id), Some(amount));

        #[extrinsic_call]
        execute_permission(RawOrigin::Signed(grantor), permission_id)
    }

    #[benchmark]
    fn toggle_permission_accumulation() {
        let grantor: T::AccountId = account("Grantor", 0, 0);
        let grantee: T::AccountId = account("Grantee", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        // Register agents
        T::Torus::force_register_agent(&grantor, vec![], vec![], vec![])
            .expect("failed to register grantor");
        T::Torus::force_register_agent(&grantee, vec![], vec![], vec![])
            .expect("failed to register grantee");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        // Fund grantor
        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&grantor, amount);

        // Create permission with stream allocation
        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = vec![(target, 100)];

        let permission_id = ext::emission_impl::grant_emission_permission_impl::<T>(
            grantor.clone(),
            grantee,
            allocation,
            targets,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByGrantor,
            EnforcementAuthority::None,
            None,
        )
        .expect("failed to grant permission");

        #[extrinsic_call]
        toggle_permission_accumulation(RawOrigin::Signed(grantor), permission_id, false)
    }

    #[benchmark]
    fn enforcement_execute_permission() {
        let grantor: T::AccountId = account("Grantor", 0, 0);
        let grantee: T::AccountId = account("Grantee", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);
        let controller: T::AccountId = account("Controller", 3, 0);

        // Register agents
        T::Torus::force_register_agent(&grantor, vec![], vec![], vec![])
            .expect("failed to register grantor");
        T::Torus::force_register_agent(&grantee, vec![], vec![], vec![])
            .expect("failed to register grantee");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        // Fund grantor
        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&grantor, amount);

        // Create permission with fixed amount allocation and enforcement authority
        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = vec![(target, 100)];
        let controllers = vec![controller.clone()].try_into().unwrap();

        let enforcement = EnforcementAuthority::ControlledBy {
            controllers,
            required_votes: 1,
        };

        let permission_id = ext::emission_impl::grant_emission_permission_impl::<T>(
            grantor.clone(),
            grantee,
            allocation,
            targets,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByGrantor,
            enforcement,
            None,
        )
        .expect("failed to grant permission");

        AccumulatedStreamAmounts::<T>::set((&grantor, stream_id, permission_id), Some(amount));

        #[extrinsic_call]
        enforcement_execute_permission(RawOrigin::Signed(controller), permission_id)
    }

    #[benchmark]
    fn set_enforcement_authority() {
        let grantor: T::AccountId = account("Grantor", 0, 0);
        let grantee: T::AccountId = account("Grantee", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);
        let controller1: T::AccountId = account("Controller1", 3, 0);
        let controller2: T::AccountId = account("Controller2", 4, 0);

        // Register agents
        T::Torus::force_register_agent(&grantor, vec![], vec![], vec![])
            .expect("failed to register grantor");
        T::Torus::force_register_agent(&grantee, vec![], vec![], vec![])
            .expect("failed to register grantee");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&grantor, amount);

        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = vec![(target, 100)];

        let permission_id = ext::emission_impl::grant_emission_permission_impl::<T>(
            grantor.clone(),
            grantee,
            allocation,
            targets,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByGrantor,
            EnforcementAuthority::None,
            None,
        )
        .expect("failed to grant permission");

        // Prepare new controllers
        let controllers = vec![controller1, controller2];
        let required_votes = 1u32;

        #[extrinsic_call]
        set_enforcement_authority(
            RawOrigin::Signed(grantor),
            permission_id,
            controllers,
            required_votes,
        )
    }

    #[benchmark]
    fn grant_curator_permission() {
        let grantee: T::AccountId = account("Grantee", 1, 0);

        // Prepare parameters
        let flags = CuratorPermissions::APPLICATION_REVIEW.bits()
            | CuratorPermissions::WHITELIST_MANAGE.bits()
            | CuratorPermissions::PENALTY_CONTROL.bits();
        let cooldown = Some(10u32.into());
        let duration = PermissionDuration::Indefinite;
        let revocation = RevocationTerms::RevocableByGrantor;

        #[extrinsic_call]
        grant_curator_permission(
            RawOrigin::Root,
            grantee,
            flags,
            cooldown,
            duration,
            revocation,
        )
    }
}
