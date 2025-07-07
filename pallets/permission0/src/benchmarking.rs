#![cfg(feature = "runtime-benchmarks")]

use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_benchmarking::{account, v2::*},
    frame_system::RawOrigin,
    sp_runtime::Percent,
};

use crate::*;

#[benchmarks]
mod benchmarks {
    use polkadot_sdk::{
        sp_core::TryCollect, sp_std::collections::btree_map::BTreeMap, sp_std::vec,
    };

    use super::*;

    macro_rules! bounded_btree_map {
        ($ ( $key:expr => $value:expr ),* $(,)?) => {
            {
                TryCollect::<$crate::BoundedBTreeMap<_, _, _>>::try_collect(
                    vec![$(($key, $value)),*].into_iter()
                ).unwrap()
            }
        };
    }

    #[benchmark]
    fn delegate_emission_permission() {
        let delegator: T::AccountId = account("Delegator", 0, 0);
        let recipient: T::AccountId = account("Recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        T::Torus::force_register_agent(&delegator, vec![], vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, vec![], vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = bounded_btree_map![target => 100];
        let distribution = DistributionControl::Manual;
        let duration = PermissionDuration::Indefinite;
        let revocation = RevocationTerms::RevocableByDelegator;
        let enforcement = EnforcementAuthority::None;

        #[extrinsic_call]
        delegate_emission_permission(
            RawOrigin::Signed(delegator),
            recipient,
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
        let delegator: T::AccountId = account("Delegator", 0, 0);
        let recipient: T::AccountId = account("Recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        // Register agents
        T::Torus::force_register_agent(&delegator, vec![], vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, vec![], vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        // Fund delegator
        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        // Create permission
        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = bounded_btree_map![target => 100];
        let permission_id = ext::emission_impl::delegate_emission_permission_impl::<T>(
            delegator.clone(),
            recipient,
            allocation,
            targets,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            EnforcementAuthority::None,
            None,
        )
        .expect("failed to delegate permission");

        #[extrinsic_call]
        revoke_permission(RawOrigin::Signed(delegator), permission_id)
    }

    #[benchmark]
    fn execute_permission() {
        let delegator: T::AccountId = account("Delegator", 0, 0);
        let recipient: T::AccountId = account("Recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        // Register agents
        T::Torus::force_register_agent(&delegator, vec![], vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, vec![], vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        // Fund delegator
        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        // Create permission with fixed amount allocation
        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = bounded_btree_map![target => 100];

        let permission_id = ext::emission_impl::delegate_emission_permission_impl::<T>(
            delegator.clone(),
            recipient,
            allocation,
            targets,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            EnforcementAuthority::None,
            None,
        )
        .expect("failed to delegate permission");

        AccumulatedStreamAmounts::<T>::set((&delegator, stream_id, permission_id), Some(amount));

        #[extrinsic_call]
        execute_permission(RawOrigin::Signed(delegator), permission_id)
    }

    #[benchmark]
    fn toggle_permission_accumulation() {
        let delegator: T::AccountId = account("Delegator", 0, 0);
        let recipient: T::AccountId = account("Recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        // Register agents
        T::Torus::force_register_agent(&delegator, vec![], vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, vec![], vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        // Fund delegator
        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        // Create permission with stream allocation
        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = bounded_btree_map![target => 100];

        let permission_id = ext::emission_impl::delegate_emission_permission_impl::<T>(
            delegator.clone(),
            recipient,
            allocation,
            targets,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            EnforcementAuthority::None,
            None,
        )
        .expect("failed to delegate permission");

        #[extrinsic_call]
        toggle_permission_accumulation(RawOrigin::Signed(delegator), permission_id, false)
    }

    #[benchmark]
    fn enforcement_execute_permission() {
        let delegator: T::AccountId = account("Delegator", 0, 0);
        let recipient: T::AccountId = account("Recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);
        let controller: T::AccountId = account("Controller", 3, 0);

        // Register agents
        T::Torus::force_register_agent(&delegator, vec![], vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, vec![], vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        // Fund delegator
        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        // Create permission with fixed amount allocation and enforcement authority
        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = bounded_btree_map![target => 100];
        let controllers = vec![controller.clone()].try_into().unwrap();

        let enforcement = EnforcementAuthority::ControlledBy {
            controllers,
            required_votes: 1,
        };

        let permission_id = ext::emission_impl::delegate_emission_permission_impl::<T>(
            delegator.clone(),
            recipient,
            allocation,
            targets,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            enforcement,
            None,
        )
        .expect("failed to delegate permission");

        AccumulatedStreamAmounts::<T>::set((&delegator, stream_id, permission_id), Some(amount));

        #[extrinsic_call]
        enforcement_execute_permission(RawOrigin::Signed(controller), permission_id)
    }

    #[benchmark]
    fn set_enforcement_authority() {
        let delegator: T::AccountId = account("Delegator", 0, 0);
        let recipient: T::AccountId = account("Recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);
        let controller1: T::AccountId = account("Controller1", 3, 0);
        let controller2: T::AccountId = account("Controller2", 4, 0);

        // Register agents
        T::Torus::force_register_agent(&delegator, vec![], vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, vec![], vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, vec![], vec![], vec![])
            .expect("failed to register target");

        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = EmissionAllocation::Streams(streams.try_into().unwrap());
        let targets = bounded_btree_map![target => 100];

        let permission_id = ext::emission_impl::delegate_emission_permission_impl::<T>(
            delegator.clone(),
            recipient,
            allocation,
            targets,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            EnforcementAuthority::None,
            None,
        )
        .expect("failed to delegate permission");

        // Prepare new controllers
        let controllers = vec![controller1, controller2].try_into().unwrap();
        let required_votes = 1u32;
        let enforcement = EnforcementAuthority::ControlledBy {
            controllers,
            required_votes,
        };

        #[extrinsic_call]
        set_enforcement_authority(RawOrigin::Signed(delegator), permission_id, enforcement)
    }

    #[benchmark]
    fn delegate_curator_permission() {
        let recipient: T::AccountId = account("Recipient", 1, 0);

        // Prepare parameters
        let flags = CuratorPermissions::APPLICATION_REVIEW.bits()
            | CuratorPermissions::WHITELIST_MANAGE.bits()
            | CuratorPermissions::PENALTY_CONTROL.bits();
        let cooldown = Some(10u32.into());
        let duration = PermissionDuration::Indefinite;
        let revocation = RevocationTerms::RevocableByDelegator;

        #[extrinsic_call]
        delegate_curator_permission(
            RawOrigin::Root,
            recipient,
            flags,
            cooldown,
            duration,
            revocation,
        )
    }
}
