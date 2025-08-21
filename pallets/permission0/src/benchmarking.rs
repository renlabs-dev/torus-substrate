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
        let delegator: T::AccountId = account("delegator", 0, 0);
        let recipient: T::AccountId = account("recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, b"target".to_vec(), vec![], vec![])
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
        let delegator: T::AccountId = account("delegator", 0, 0);
        let recipient: T::AccountId = account("recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, b"target".to_vec(), vec![], vec![])
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

        #[extrinsic_call]
        revoke_permission(RawOrigin::Signed(delegator), permission_id)
    }

    #[benchmark]
    fn execute_permission() {
        let delegator: T::AccountId = account("delegator", 0, 0);
        let recipient: T::AccountId = account("recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, b"target".to_vec(), vec![], vec![])
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
        let delegator: T::AccountId = account("delegator", 0, 0);
        let recipient: T::AccountId = account("recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);

        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, b"target".to_vec(), vec![], vec![])
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
        let delegator: T::AccountId = account("delegator", 0, 0);
        let recipient: T::AccountId = account("recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);
        let controller: T::AccountId = account("Controller", 3, 0);

        // Register agents
        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, b"target".to_vec(), vec![], vec![])
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
        let delegator: T::AccountId = account("delegator", 0, 0);
        let recipient: T::AccountId = account("recipient", 1, 0);
        let target: T::AccountId = account("Target", 2, 0);
        let controller1: T::AccountId = account("Controller1", 3, 0);
        let controller2: T::AccountId = account("Controller2", 4, 0);

        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");
        T::Torus::force_register_agent(&target, b"target".to_vec(), vec![], vec![])
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
        let recipient: T::AccountId = account("recipient", 1, 0);

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
            bounded_btree_map!(None => flags),
            cooldown,
            duration,
            revocation,
            1,
        )
    }

    #[benchmark]
    fn update_namespace_permission() {
        use pallet_torus0_api::NamespacePathInner;
        use polkadot_sdk::sp_std::collections::btree_set::BTreeSet;

        let alice: T::AccountId = account("alice", 0, 0);
        let bob: T::AccountId = account("bob", 1, 0);

        T::Torus::force_register_agent(&alice, b"alice".to_vec(), vec![], vec![])
            .expect("failed to register alice");
        T::Torus::force_register_agent(&bob, b"bob".to_vec(), vec![], vec![])
            .expect("failed to register bob");

        for namespace_bytes in [
            b"agent.alice.network".to_vec(),
            b"agent.alice.compute".to_vec(),
            b"agent.alice.storage".to_vec(),
        ] {
            T::Torus::force_register_namespace(&alice, namespace_bytes.clone())
                .expect("failed to register namespace");
        }

        let mut alice_paths_set: BTreeSet<NamespacePathInner> = BTreeSet::new();
        alice_paths_set.insert(b"agent.alice.network".to_vec().try_into().unwrap());
        alice_paths_set.insert(b"agent.alice.compute".to_vec().try_into().unwrap());
        let alice_paths_bounded: BoundedBTreeSet<
            NamespacePathInner,
            T::MaxNamespacesPerPermission,
        > = alice_paths_set
            .try_into()
            .expect("failed to create bounded set");

        let mut alice_paths: BoundedBTreeMap<
            Option<PermissionId>,
            BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
            T::MaxNamespacesPerPermission,
        > = BoundedBTreeMap::new();
        alice_paths
            .try_insert(None, alice_paths_bounded)
            .expect("failed to insert alice paths");

        let permission_id = ext::namespace_impl::delegate_namespace_permission_impl::<T>(
            RawOrigin::Signed(alice.clone()).into(),
            bob.clone(),
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            20,
        )
        .expect("failed to create alice->bob permission");

        #[extrinsic_call]
        update_namespace_permission(RawOrigin::Signed(alice), permission_id, 10)
    }

    #[benchmark]
    fn delegate_namespace_permission() {
        use pallet_torus0_api::NamespacePathInner;
        use polkadot_sdk::sp_std::collections::btree_set::BTreeSet;

        let alice: T::AccountId = account("alice", 0, 0);
        let bob: T::AccountId = account("bob", 1, 0);
        let charlie: T::AccountId = account("charlie", 2, 0);
        let dave: T::AccountId = account("dave", 3, 0);
        let eve: T::AccountId = account("eve", 4, 0);

        T::Torus::force_register_agent(&alice, b"alice".to_vec(), vec![], vec![])
            .expect("failed to register alice");
        T::Torus::force_register_agent(&bob, b"bob".to_vec(), vec![], vec![])
            .expect("failed to register bob");
        T::Torus::force_register_agent(&charlie, b"charlie".to_vec(), vec![], vec![])
            .expect("failed to register charlie");
        T::Torus::force_register_agent(&dave, b"dave".to_vec(), vec![], vec![])
            .expect("failed to register dave");
        T::Torus::force_register_agent(&eve, b"eve".to_vec(), vec![], vec![])
            .expect("failed to register eve");

        // Create and register many namespaces with varying depths to simulate pollution
        // These will be registered but not used in the actual delegation chain
        for namespace_bytes in [
            b"agent.alice.network".to_vec(),
            b"agent.alice.network.subnet1".to_vec(),
            b"agent.alice.network.subnet1.fast".to_vec(),
            b"agent.alice.network.subnet2".to_vec(),
            b"agent.alice.storage".to_vec(),
            b"agent.alice.storage.ssd".to_vec(),
            b"agent.alice.storage.hdd".to_vec(),
            b"agent.alice.database".to_vec(),
            b"agent.alice.database.mysql".to_vec(),
            b"agent.alice.database.postgres".to_vec(),
            b"agent.alice.compute".to_vec(),
            b"agent.alice.compute.gpu".to_vec(),
            b"agent.alice.compute.gpu.h100".to_vec(),
        ] {
            T::Torus::force_register_namespace(&alice, namespace_bytes.clone())
                .expect("failed to register pollution namespace");
        }

        for namespace_bytes in [
            b"agent.bob.compute".to_vec(),
            b"agent.bob.compute.cpu".to_vec(),
            b"agent.bob.compute.memory".to_vec(),
        ] {
            T::Torus::force_register_namespace(&bob, namespace_bytes.clone())
                .expect("failed to register pollution namespace");
        }

        for namespace_bytes in [
            b"agent.charlie.services".to_vec(),
            b"agent.charlie.services.api".to_vec(),
            b"agent.charlie.services.web".to_vec(),
        ] {
            T::Torus::force_register_namespace(&charlie, namespace_bytes.clone())
                .expect("failed to register pollution namespace");
        }

        let mut alice_bob_set: BTreeSet<NamespacePathInner> = BTreeSet::new();
        alice_bob_set.insert(b"agent.alice.compute".to_vec().try_into().unwrap());
        alice_bob_set.insert(b"agent.alice.network".to_vec().try_into().unwrap());
        alice_bob_set.insert(b"agent.alice.storage".to_vec().try_into().unwrap());
        let alice_bob_set: BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission> =
            alice_bob_set
                .try_into()
                .expect("failed to create bounded set");

        let mut alice_paths: BoundedBTreeMap<
            Option<PermissionId>,
            BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
            T::MaxNamespacesPerPermission,
        > = BoundedBTreeMap::new();
        alice_paths
            .try_insert(None, alice_bob_set)
            .expect("failed to insert alice paths");

        let alice_permission_id = ext::namespace_impl::delegate_namespace_permission_impl::<T>(
            RawOrigin::Signed(alice.clone()).into(),
            bob.clone(),
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            20,
        )
        .expect("failed to create alice->bob permission");

        let mut bob_charlie_set: BTreeSet<NamespacePathInner> = BTreeSet::new();
        bob_charlie_set.insert(b"agent.alice.compute.gpu".to_vec().try_into().unwrap());
        bob_charlie_set.insert(b"agent.alice.network.subnet1".to_vec().try_into().unwrap());
        let bob_charlie_set: BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission> =
            bob_charlie_set
                .try_into()
                .expect("failed to create bounded set");

        let mut bob_paths: BoundedBTreeMap<
            Option<PermissionId>,
            BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
            T::MaxNamespacesPerPermission,
        > = BoundedBTreeMap::new();
        bob_paths
            .try_insert(Some(alice_permission_id), bob_charlie_set)
            .expect("failed to insert bob paths");

        let bob_permission_id = ext::namespace_impl::delegate_namespace_permission_impl::<T>(
            RawOrigin::Signed(bob.clone()).into(),
            charlie.clone(),
            bob_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            15,
        )
        .expect("failed to create bob->charlie permission");

        let mut charlie_dave_set: BTreeSet<NamespacePathInner> = BTreeSet::new();
        charlie_dave_set.insert(b"agent.alice.compute.gpu.h100".to_vec().try_into().unwrap());
        charlie_dave_set.insert(
            b"agent.alice.network.subnet1.fast"
                .to_vec()
                .try_into()
                .unwrap(),
        );

        let charlie_dave_set: BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission> =
            charlie_dave_set
                .try_into()
                .expect("failed to create bounded set");

        let mut charlie_paths: BoundedBTreeMap<
            Option<PermissionId>,
            BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
            T::MaxNamespacesPerPermission,
        > = BoundedBTreeMap::new();
        charlie_paths
            .try_insert(Some(bob_permission_id), charlie_dave_set)
            .expect("failed to insert charlie paths");

        #[extrinsic_call]
        delegate_namespace_permission(
            RawOrigin::Signed(charlie),
            dave,
            charlie_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            10, // Final instance count
        )
    }
}
