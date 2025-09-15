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
    use codec::alloc::string::ToString;
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
    fn delegate_stream_permission() {
        let delegator: T::AccountId = account("delegator", 0, 0);
        let recipient: T::AccountId = account("recipient", 1, 0);

        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");

        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = StreamAllocation::Streams(streams.try_into().unwrap());
        let recipients = bounded_btree_map![recipient => 100];
        let distribution = DistributionControl::Manual;
        let duration = PermissionDuration::Indefinite;
        let revocation = RevocationTerms::RevocableByDelegator;
        let enforcement = EnforcementAuthority::None;

        #[extrinsic_call]
        delegate_stream_permission(
            RawOrigin::Signed(delegator),
            recipients,
            allocation,
            distribution,
            duration,
            revocation,
            enforcement,
            None,
            None,
        )
    }

    #[benchmark]
    fn revoke_permission() {
        let delegator: T::AccountId = account("delegator", 0, 0);
        let recipient: T::AccountId = account("recipient", 1, 0);

        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");

        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = StreamAllocation::Streams(streams.try_into().unwrap());
        let recipients = bounded_btree_map![recipient => 100];
        let permission_id = ext::stream_impl::delegate_stream_permission_impl::<T>(
            delegator.clone(),
            recipients,
            allocation,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            EnforcementAuthority::None,
            None,
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

        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");

        // Fund delegator
        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        // Create permission with fixed amount allocation
        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = StreamAllocation::Streams(streams.try_into().unwrap());
        let recipients = bounded_btree_map![recipient => 100];

        let permission_id = ext::stream_impl::delegate_stream_permission_impl::<T>(
            delegator.clone(),
            recipients,
            allocation,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            EnforcementAuthority::None,
            None,
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

        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");

        // Fund delegator
        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        // Create permission with stream allocation
        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = StreamAllocation::Streams(streams.try_into().unwrap());
        let recipients = bounded_btree_map![recipient => 100];

        let permission_id = ext::stream_impl::delegate_stream_permission_impl::<T>(
            delegator.clone(),
            recipients,
            allocation,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            EnforcementAuthority::None,
            None,
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
        let controller: T::AccountId = account("Controller", 3, 0);

        // Register agents
        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");

        // Fund delegator
        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        // Create permission with fixed amount allocation and enforcement authority
        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = StreamAllocation::Streams(streams.try_into().unwrap());
        let recipients = bounded_btree_map![recipient => 100];
        let controllers = vec![controller.clone()].try_into().unwrap();

        let enforcement = EnforcementAuthority::ControlledBy {
            controllers,
            required_votes: 1,
        };

        let permission_id = ext::stream_impl::delegate_stream_permission_impl::<T>(
            delegator.clone(),
            recipients,
            allocation,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            enforcement,
            None,
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
        let controller1: T::AccountId = account("Controller1", 3, 0);
        let controller2: T::AccountId = account("Controller2", 4, 0);

        T::Torus::force_register_agent(&delegator, b"delegator".to_vec(), vec![], vec![])
            .expect("failed to register delegator");
        T::Torus::force_register_agent(&recipient, b"recipient".to_vec(), vec![], vec![])
            .expect("failed to register recipient");

        let amount = 10_000_000u32.into();
        let _ = <T::Currency>::deposit_creating(&delegator, amount);

        let stream_id: StreamId = [0; 32].into();
        let streams = BTreeMap::from([(stream_id, Percent::from_percent(30))]);
        let allocation = StreamAllocation::Streams(streams.try_into().unwrap());
        let recipients = bounded_btree_map![recipient => 100];

        let permission_id = ext::stream_impl::delegate_stream_permission_impl::<T>(
            delegator.clone(),
            recipients,
            allocation,
            DistributionControl::Manual,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            EnforcementAuthority::None,
            None,
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

        let alice_paths_set = BTreeSet::from([
            b"agent.alice.network".to_vec().try_into().unwrap(),
            b"agent.alice.compute".to_vec().try_into().unwrap(),
        ]);
        let alice_paths_bounded: BoundedBTreeSet<_, _> = alice_paths_set
            .try_into()
            .expect("failed to create bounded set");

        let mut alice_paths = BoundedBTreeMap::new();
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

        let alice_n = &[
            b"agent.alice.n.01",
            b"agent.alice.n.02",
            b"agent.alice.n.03",
            b"agent.alice.n.04",
            b"agent.alice.n.05",
            b"agent.alice.n.06",
            b"agent.alice.n.07",
            b"agent.alice.n.08",
            b"agent.alice.n.09",
            b"agent.alice.n.10",
            b"agent.alice.n.11",
            b"agent.alice.n.12",
            b"agent.alice.n.13",
            b"agent.alice.n.14",
            b"agent.alice.n.15",
        ];
        for namespace_bytes in alice_n {
            T::Torus::force_register_namespace(&alice, namespace_bytes.to_vec())
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

        let alice_bob_set = BTreeSet::from([
            b"agent.alice.compute".to_vec().try_into().unwrap(),
            b"agent.alice.network".to_vec().try_into().unwrap(),
            b"agent.alice.storage".to_vec().try_into().unwrap(),
            b"agent.alice.n".to_vec().try_into().unwrap(),
            b"agent.alice.n".to_vec().try_into().unwrap(),
        ]);
        let alice_bob_set: BoundedBTreeSet<_, _> = alice_bob_set
            .try_into()
            .expect("failed to create bounded set");

        let mut alice_paths = BoundedBTreeMap::new();
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

        let bob_charlie_set = BTreeSet::from([
            b"agent.alice.compute.gpu".to_vec().try_into().unwrap(),
            b"agent.alice.network.subnet1".to_vec().try_into().unwrap(),
            b"agent.alice.n".to_vec().try_into().unwrap(),
        ]);
        let bob_charlie_set: BoundedBTreeSet<_, _> = bob_charlie_set
            .try_into()
            .expect("failed to create bounded set");

        let mut bob_paths = BoundedBTreeMap::new();
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

        for (i, path) in alice_n.iter().enumerate() {
            let acc: T::AccountId = account("acc", 100.saturating_add(i as u32), 0);
            let mut acc_name = "acc".to_string();
            acc_name.push_str(&i.to_string());

            T::Torus::force_register_agent(&acc, acc_name.as_bytes().to_vec(), vec![], vec![])
                .expect("failed to register acc");

            let set = BTreeSet::from([path.to_vec().try_into().unwrap()]);
            let set: BoundedBTreeSet<_, _> = set.try_into().expect("failed to create bounded set");
            let mut paths = BoundedBTreeMap::new();
            paths
                .try_insert(Some(bob_permission_id), set)
                .expect("failed to insert bob paths");

            ext::namespace_impl::delegate_namespace_permission_impl::<T>(
                RawOrigin::Signed(charlie.clone()).into(),
                acc.clone(),
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::RevocableByDelegator,
                1,
            )
            .expect("failed to create bob->charlie permission");
        }

        let charlie_dave_set = BTreeSet::from([
            b"agent.alice.compute.gpu.h100".to_vec().try_into().unwrap(),
            b"agent.alice.network.subnet1.fast"
                .to_vec()
                .try_into()
                .unwrap(),
        ]);

        let charlie_dave_set: BoundedBTreeSet<_, _> = charlie_dave_set
            .try_into()
            .expect("failed to create bounded set");

        let mut charlie_paths = BoundedBTreeMap::new();
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
