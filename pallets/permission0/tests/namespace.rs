#![allow(clippy::indexing_slicing, clippy::arithmetic_side_effects)]

use pallet_permission0::{
    CuratorPermissions, Error, Pallet, PermissionDuration, PermissionId, PermissionScope,
    Permissions, PermissionsByDelegator, PermissionsByRecipient, RevocationTerms,
};
use pallet_permission0_api::Permission0NamespacesApi;
use pallet_torus0_api::{NamespacePath, NamespacePathInner};
use polkadot_sdk::sp_core::{H256, TryCollect};
use polkadot_sdk::sp_runtime::DispatchError;
use polkadot_sdk::{
    frame_support::{BoundedBTreeMap, BoundedBTreeSet, assert_err, assert_ok},
    frame_system::RawOrigin,
};
use test_utils::*;

pub fn new_test_ext() -> polkadot_sdk::sp_io::TestExternalities {
    new_test_ext_with_block(100)
}

// Helper to get the most recent permission ID from events
fn get_last_delegated_permission_id(delegator: AccountId) -> PermissionId {
    System::events()
        .into_iter()
        .filter_map(|record| {
            if let RuntimeEvent::Permission0(pallet_permission0::Event::PermissionDelegated {
                delegator: event_delegator,
                permission_id,
            }) = record.event
            {
                if event_delegator == delegator {
                    Some(permission_id)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next_back() // Get most recent
        .expect("No PermissionDelegated event found")
}

// Helper to get all permission IDs for a delegator from events (in chronological order)
fn get_all_delegated_permission_ids(delegator: AccountId) -> Vec<PermissionId> {
    System::events()
        .into_iter()
        .filter_map(|record| {
            if let RuntimeEvent::Permission0(pallet_permission0::Event::PermissionDelegated {
                delegator: event_delegator,
                permission_id,
            }) = record.event
            {
                if event_delegator == delegator {
                    Some(permission_id)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

// Helper to get permission ID where a specific account is the recipient
// This checks the actual permission scope to find the recipient
fn get_permission_id_for_recipient(recipient: AccountId) -> PermissionId {
    // Look through all permissions to find one where this account is the recipient
    for (permission_id, contract) in Permissions::<Test>::iter() {
        match &contract.scope {
            PermissionScope::Curator(scope) if scope.recipient == recipient => {
                return permission_id;
            }
            PermissionScope::Namespace(scope) if scope.recipient == recipient => {
                return permission_id;
            }
            PermissionScope::Stream(scope) if scope.recipients.contains_key(&recipient) => {
                return permission_id;
            }
            _ => continue,
        }
    }
    panic!("No permission found for recipient: {recipient:?}");
}

fn register_agent(id: AccountId) {
    let name = match id {
        0 => "alice".to_string(),
        1 => "bob".to_string(),
        2 => "charlie".to_string(),
        3 => "dave".to_string(),
        4 => "eve".to_string(),
        5 => "ferdie".to_string(),
        _ => format!("agent-{id}"),
    };
    let name = name.as_bytes();

    Balances::force_set_balance(RawOrigin::Root.into(), id, u128::MAX).unwrap();
    Torus0::register_agent(get_origin(id), name.to_vec(), name.to_vec(), name.to_vec()).unwrap();
}

macro_rules! paths_map {
    ($ ( $key:expr => [$($value:expr),+] ),* $(,)?) => {
        {
            TryCollect::<$crate::BoundedBTreeMap<_, _, _>>::try_collect(
                vec![$((
                    $key,
                    TryCollect::<$crate::BoundedBTreeSet<_, _>>::try_collect(
                        vec![$($value),*].into_iter()
                    ).unwrap()
                    )),*
                ].into_iter()
            ).unwrap()
        }
    };
}

fn register_namespace(id: AccountId, name: &[u8]) -> NamespacePathInner {
    let bounded_namespace: NamespacePathInner = name.to_vec().try_into().unwrap();
    Torus0::create_namespace(get_origin(id), bounded_namespace.clone()).unwrap();
    bounded_namespace
}

#[test]
fn is_delegating_namespace_returns_false_for_no_permissions() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = alice();

        let path = NamespacePath::new_agent(b"agent.alice.compute").unwrap();
        assert!(!<Pallet<Test> as Permission0NamespacesApi<
            AccountId,
            NamespacePath,
        >>::is_delegating_namespace(&agent_0, &path));
    });
}

#[test]
fn is_delegating_namespace_returns_true_for_exact_match() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();

        let bounded_namespace = register_namespace(delegator, b"agent.alice.compute");

        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths.try_insert(None, namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));

        let query_path = NamespacePath::new_agent(b"agent.alice.compute").unwrap();
        assert!(<Pallet<Test> as Permission0NamespacesApi<
            AccountId,
            NamespacePath,
        >>::is_delegating_namespace(&delegator, &query_path));
    });
}

#[test]
fn is_delegating_namespace_returns_true_for_parent_child_relationship() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();

        let bounded_parent = register_namespace(delegator, b"agent.alice.compute");
        let bounded_child = register_namespace(delegator, b"agent.alice.compute.gpu.h100");

        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_child.clone()).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths.try_insert(None, namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));

        for path in [bounded_parent, bounded_child] {
            let path = NamespacePath::new_agent(&path).unwrap();
            assert!(<Pallet<Test> as Permission0NamespacesApi<
                AccountId,
                NamespacePath,
            >>::is_delegating_namespace(&delegator, &path));
        }
    });
}

#[test]
fn is_delegating_namespace_returns_true_for_child_parent_relationship() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();

        let bounded_parent = register_namespace(delegator, b"agent.alice.compute");

        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_parent.clone()).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths.try_insert(None, namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));

        for path in [bounded_parent.as_slice(), b"agent.alice.compute.gpu.h100"] {
            let path = NamespacePath::new_agent(path).unwrap();
            assert!(<Pallet<Test> as Permission0NamespacesApi<
                AccountId,
                NamespacePath,
            >>::is_delegating_namespace(&delegator, &path));
        }
    });
}

#[test]
fn is_delegating_namespace_returns_false_for_unrelated_paths() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();

        let bounded_compute = register_namespace(delegator, b"agent.alice.compute.gpu.h100");

        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_compute).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths.try_insert(None, namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));

        let storage_query = NamespacePath::new_agent(b"agent.alice.storage.ssd.100tb").unwrap();
        assert!(!<Pallet<Test> as Permission0NamespacesApi<
            AccountId,
            NamespacePath,
        >>::is_delegating_namespace(
            &delegator, &storage_query
        ));
    });
}

#[test]
fn is_delegating_namespace_handles_multiple_permissions() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient_1 = bob();
        let recipient_2 = charlie();

        let bounded_compute = register_namespace(delegator, b"agent.alice.compute");
        let bounded_storage = register_namespace(delegator, b"agent.alice.storage");

        let mut compute_namespace_set = BoundedBTreeSet::new();
        compute_namespace_set.try_insert(bounded_compute).unwrap();
        let mut compute_paths = BoundedBTreeMap::new();
        compute_paths
            .try_insert(None, compute_namespace_set)
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient_1,
            compute_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));

        let mut storage_namespace_set = BoundedBTreeSet::new();
        storage_namespace_set.try_insert(bounded_storage).unwrap();
        let mut storage_paths = BoundedBTreeMap::new();
        storage_paths
            .try_insert(None, storage_namespace_set)
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient_2,
            storage_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));

        for path in [b"agent.alice.compute.gpu", b"agent.alice.storage.ssd"] {
            let path = NamespacePath::new_agent(path).unwrap();
            assert!(<Pallet<Test> as Permission0NamespacesApi<
                AccountId,
                NamespacePath,
            >>::is_delegating_namespace(&delegator, &path));
        }

        let network_query = NamespacePath::new_agent(b"agent.alice.network.bandwidth").unwrap();
        assert!(!<Pallet<Test> as Permission0NamespacesApi<
            AccountId,
            NamespacePath,
        >>::is_delegating_namespace(
            &delegator, &network_query
        ));
    });
}

#[test]
fn delegate_namespace_permission_fails_for_unregistered_delegator() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let unregistered_delegator = 0;
        let recipient = bob();

        let namespace_path = b"agent.alice.compute".to_vec();
        let bounded_namespace: NamespacePathInner = namespace_path.try_into().unwrap();
        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths.try_insert(None, namespace_set).unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(unregistered_delegator),
                recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::NotRegisteredAgent
        );
    });
}

#[test]
fn delegate_namespace_permission_fails_for_unregistered_recipient() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let unregistered_recipient = 1;

        let namespace_path = b"agent.alice.compute".to_vec();
        let bounded_namespace: NamespacePathInner = namespace_path.try_into().unwrap();
        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths.try_insert(None, namespace_set).unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(delegator),
                unregistered_recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::NotRegisteredAgent
        );
    });
}

#[test]
fn delegate_namespace_permission_fails_for_nonexistent_namespace() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();

        let nonexistent_namespace = b"agent.alice.nonexistent".to_vec();
        let bounded_namespace: NamespacePathInner = nonexistent_namespace.try_into().unwrap();
        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths.try_insert(None, namespace_set).unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(delegator),
                recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::NamespaceDoesNotExist
        );
    });
}

#[test]
fn delegate_namespace_permission_fails_for_invalid_namespace_path() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();

        let invalid_namespace = b"invalid.path".to_vec();
        let bounded_namespace: NamespacePathInner = invalid_namespace.try_into().unwrap();
        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths.try_insert(None, namespace_set).unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(delegator),
                recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::NamespacePathIsInvalid
        );
    });
}

#[test]
fn delegate_namespace_permission_creates_permission_successfully() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();

        let bounded_namespace = register_namespace(delegator, b"agent.alice.compute");

        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths.try_insert(None, namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths.clone(),
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));

        let query_path = NamespacePath::new_agent(b"agent.alice.compute").unwrap();
        assert!(<Pallet<Test> as Permission0NamespacesApi<
            AccountId,
            NamespacePath,
        >>::is_delegating_namespace(&delegator, &query_path));
    });
}

#[test]
fn delegate_namespace_permission_handles_multiple_paths() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();

        let bounded_compute = register_namespace(delegator, b"agent.alice.compute");
        let bounded_storage = register_namespace(delegator, b"agent.alice.storage");

        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_compute).unwrap();
        namespace_set.try_insert(bounded_storage).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths.try_insert(None, namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));

        for path in [b"agent.alice.compute", b"agent.alice.storage"] {
            let path = NamespacePath::new_agent(path).unwrap();
            assert!(<Pallet<Test> as Permission0NamespacesApi<
                AccountId,
                NamespacePath,
            >>::is_delegating_namespace(&delegator, &path));
        }
    });
}

#[test]
fn delegate_namespace_permission_creates_correct_scope() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();

        let bounded_namespace = register_namespace(delegator, b"agent.alice.compute");

        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths.try_insert(None, namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));

        let permissions: Vec<_> = Permissions::<Test>::iter().collect();
        assert_eq!(permissions.len(), 1);

        let (_, permission) = permissions.first().unwrap();
        assert_eq!(permission.delegator, delegator);

        match &permission.scope {
            PermissionScope::Namespace(namespace_scope) => {
                assert_eq!(namespace_scope.recipient, recipient);
                assert_eq!(namespace_scope.paths.len(), 1);
                let expected_path = NamespacePath::new_agent(b"agent.alice.compute").unwrap();
                // Check that the path exists in the map under the None key (delegator's own namespace)
                let delegator_paths = namespace_scope.paths.get(&None).unwrap();
                assert!(delegator_paths.contains(&expected_path));
            }
            _ => panic!("Expected Namespace scope"),
        }
    });
}

#[test]
fn delegate_namespace_permission_fails_with_multiple_parent_permissions() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();

        let bounded_gpu = register_namespace(delegator, b"agent.alice.compute.gpu");
        let bounded_ssd = register_namespace(delegator, b"agent.alice.storage.ssd");

        let mut gpu_set = BoundedBTreeSet::new();
        gpu_set.try_insert(bounded_gpu).unwrap();
        let mut ssd_set = BoundedBTreeSet::new();
        ssd_set.try_insert(bounded_ssd).unwrap();

        let mut multi_parent_paths = BoundedBTreeMap::new();
        multi_parent_paths
            .try_insert(Some(H256::from([0; 32])), gpu_set)
            .unwrap();
        multi_parent_paths
            .try_insert(Some(H256::from([1; 32])), ssd_set)
            .unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(delegator),
                delegator,
                multi_parent_paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::MultiParentForbidden
        );
    });
}

#[test]
fn delegate_namespace_permission_fails_with_too_many_total_namespaces_across_parents() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();

        let mut parent_namespace_set = BoundedBTreeSet::new();
        for i in 0..6 {
            let namespace_name = format!("agent.alice.compute.gpu.{i}");
            let bounded_namespace = register_namespace(delegator, namespace_name.as_bytes());
            parent_namespace_set.try_insert(bounded_namespace).unwrap();
        }

        let mut parent_paths = BoundedBTreeMap::new();
        parent_paths
            .try_insert(None, parent_namespace_set.clone())
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            delegator,
            parent_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));
        let parent_permission_id = get_last_delegated_permission_id(delegator);

        let mut child_namespace_set = BoundedBTreeSet::new();
        for i in 0..6 {
            let namespace_name = format!("agent.alice.storage.ssd.{i}");
            let bounded_namespace = register_namespace(delegator, namespace_name.as_bytes());
            child_namespace_set.try_insert(bounded_namespace).unwrap();
        }

        let mut child_paths = BoundedBTreeMap::new();
        child_paths.try_insert(None, child_namespace_set).unwrap();
        child_paths
            .try_insert(Some(parent_permission_id), parent_namespace_set)
            .unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(delegator),
                delegator,
                child_paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::TooManyNamespaces
        );
    });
}

#[test]
fn delegate_namespace_permission_fails_with_nonexistent_parent_permission() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();

        let bounded_namespace = register_namespace(delegator, b"agent.alice.compute");
        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();

        let fake_permission_id = H256::from_low_u64_be(999);
        let mut paths = BoundedBTreeMap::new();
        paths
            .try_insert(Some(fake_permission_id), namespace_set)
            .unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(delegator),
                recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::ParentPermissionNotFound
        );
    });
}

#[test]
fn delegate_namespace_permission_fails_when_delegator_not_recipient_of_parent() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let original_delegator = alice();
        let wrong_delegator = bob();
        let recipient = charlie();

        // Create parent permission
        let bounded_namespace = register_namespace(original_delegator, b"agent.alice.compute");
        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();
        let mut parent_paths = BoundedBTreeMap::new();
        parent_paths.try_insert(None, namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(original_delegator),
            recipient, // recipient is the one who received the permission
            parent_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));
        let parent_permission_id = get_last_delegated_permission_id(original_delegator);

        // Try to use the parent permission from wrong_delegator (who is not the recipient)
        let bounded_gpu = register_namespace(original_delegator, b"agent.alice.compute.gpu");
        let mut gpu_set = BoundedBTreeSet::new();
        gpu_set.try_insert(bounded_gpu).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths
            .try_insert(Some(parent_permission_id), gpu_set)
            .unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(wrong_delegator),
                recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::NotPermissionRecipient
        );
    });
}

#[test]
fn delegate_namespace_permission_fails_when_parent_has_wrong_scope() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();

        // Create a curator permission (non-namespace scope)
        delegate_curator_permission(recipient, CuratorPermissions::all(), None);
        let curator_permission_id = get_permission_id_for_recipient(recipient);

        // Try to use the curator permission as parent for namespace permission
        let bounded_namespace = register_namespace(delegator, b"agent.alice.compute");
        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths
            .try_insert(Some(curator_permission_id), namespace_set)
            .unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(recipient),
                delegator,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::UnsupportedPermissionType
        );
    });
}

#[test]
fn delegate_namespace_permission_fails_when_exceeding_available_instances() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();
        let final_recipient = charlie();

        // Create parent permission with limited instances
        let bounded_namespace = register_namespace(delegator, b"agent.alice.compute");
        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();
        let mut parent_paths = BoundedBTreeMap::new();
        parent_paths.try_insert(None, namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            parent_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            2 // Only 2 instances available
        ));
        let parent_permission_id = get_last_delegated_permission_id(delegator);

        // Try to create child permission that requires more instances than available
        let bounded_gpu = register_namespace(delegator, b"agent.alice.compute.gpu");
        let mut gpu_set = BoundedBTreeSet::new();
        gpu_set.try_insert(bounded_gpu).unwrap();
        let mut paths = BoundedBTreeMap::new();
        paths
            .try_insert(Some(parent_permission_id), gpu_set)
            .unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(recipient),
                final_recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                3 // Requesting 3 instances but only 2 available
            ),
            Error::<Test>::NotEnoughInstances
        );
    });
}

#[test]
fn permission_contract_available_instances_reduces_with_children() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();
        let final_recipient = charlie();

        // Create parent permission with 5 instances
        let bounded_namespace = register_namespace(delegator, b"agent.alice.compute");
        let mut namespace_set = BoundedBTreeSet::new();
        namespace_set.try_insert(bounded_namespace).unwrap();
        let mut parent_paths = BoundedBTreeMap::new();
        parent_paths.try_insert(None, namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            parent_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            5 // 5 instances total
        ));
        let parent_permission_id = get_last_delegated_permission_id(delegator);

        // Verify initial available instances
        let parent_permission = Permissions::<Test>::get(parent_permission_id).unwrap();
        assert_eq!(parent_permission.available_instances().unwrap(), 5);

        // Create child permission with 2 instances
        let bounded_gpu = register_namespace(delegator, b"agent.alice.compute.gpu");
        let mut gpu_set = BoundedBTreeSet::new();
        gpu_set.try_insert(bounded_gpu).unwrap();
        let mut child_paths = BoundedBTreeMap::new();
        child_paths
            .try_insert(Some(parent_permission_id), gpu_set)
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(recipient),
            final_recipient,
            child_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            2 // Child uses 2 instances
        ));

        // Verify available instances reduced to 3 (5 - 2)
        let parent_permission = Permissions::<Test>::get(parent_permission_id).unwrap();
        assert_eq!(parent_permission.available_instances().unwrap(), 3);
    });
}

#[test]
fn delegate_granular_namespace_from_parent_permission_succeeds() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let alice = alice();
        let bob = bob();
        let charlie = charlie();

        let parent_compute = register_namespace(alice, b"agent.alice.compute");
        let granular_gpu = register_namespace(alice, b"agent.alice.compute.gpu");
        let very_granular_h100 = register_namespace(alice, b"agent.alice.compute.gpu.h100");

        let mut parent_namespace_set = BoundedBTreeSet::new();
        parent_namespace_set.try_insert(parent_compute).unwrap();
        let mut alice_paths = BoundedBTreeMap::new();
        alice_paths.try_insert(None, parent_namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(alice),
            bob,
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            5
        ));
        let alice_permission_id = get_last_delegated_permission_id(alice);

        // Bob (as intermediary) can delegate more granular namespaces that exist
        // This tests the behavior in resolve_paths where it checks:
        // 1. parent_path.is_parent_of(child)
        // 2. agent_name extraction from parent_path
        // 3. find_agent_by_name to get the agent
        // 4. namespace_exists check on the granular namespace
        let mut bob_granular_set = BoundedBTreeSet::new();
        bob_granular_set.try_insert(granular_gpu).unwrap();
        let mut bob_paths = BoundedBTreeMap::new();
        bob_paths
            .try_insert(Some(alice_permission_id), bob_granular_set)
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(bob),
            charlie,
            bob_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            2
        ));
        let bob_permission_id = get_last_delegated_permission_id(bob);

        // Charlie can further delegate even more granular namespaces
        let mut charlie_very_granular_set = BoundedBTreeSet::new();
        charlie_very_granular_set
            .try_insert(very_granular_h100)
            .unwrap();
        let mut charlie_paths = BoundedBTreeMap::new();
        charlie_paths
            .try_insert(Some(bob_permission_id), charlie_very_granular_set)
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(charlie),
            alice, // Full circle back to Alice
            charlie_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            1
        ));

        // Verify all permissions were created successfully
        assert_eq!(PermissionsByDelegator::<Test>::get(alice).len(), 1); // Alice's original
        assert_eq!(PermissionsByDelegator::<Test>::get(bob).len(), 1); // Bob's granular
        assert_eq!(PermissionsByDelegator::<Test>::get(charlie).len(), 1); // Charlie's very granular
    });
}

#[test]
fn delegate_granular_namespace_fails_when_granular_namespace_does_not_exist() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let alice = alice();
        let bob = bob();

        let parent_compute = register_namespace(alice, b"agent.alice.compute");
        // Note: We do NOT create agent.alice.compute.gpu namespace

        // Alice delegates broad permission to Bob
        let mut parent_namespace_set = BoundedBTreeSet::new();
        parent_namespace_set.try_insert(parent_compute).unwrap();
        let mut alice_paths = BoundedBTreeMap::new();
        alice_paths.try_insert(None, parent_namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(alice),
            bob,
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            5
        ));
        let alice_permission_id = get_last_delegated_permission_id(alice);

        // Bob tries to delegate granular namespace that doesn't exist
        // This should fail because namespace_exists check returns false
        let nonexistent_gpu = b"agent.alice.compute.gpu".to_vec().try_into().unwrap();
        let mut bob_granular_set = BoundedBTreeSet::new();
        bob_granular_set.try_insert(nonexistent_gpu).unwrap();
        let mut bob_paths = BoundedBTreeMap::new();
        bob_paths
            .try_insert(Some(alice_permission_id), bob_granular_set)
            .unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(bob),
                alice,
                bob_paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                2
            ),
            Error::<Test>::ParentPermissionNotFound // This error is thrown when matched_count != children.len()
        );
    });
}

#[test]
fn delegate_namespace_fails_when_child_not_granular_of_parent() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let alice = alice();
        let bob = bob();

        // Alice creates compute namespace and unrelated storage namespace
        let parent_compute = register_namespace(alice, b"agent.alice.compute");
        let unrelated_storage = register_namespace(alice, b"agent.alice.storage.ssd"); // Not granular of compute

        // Alice delegates compute permission to Bob
        let mut parent_namespace_set = BoundedBTreeSet::new();
        parent_namespace_set.try_insert(parent_compute).unwrap();
        let mut alice_paths = BoundedBTreeMap::new();
        alice_paths.try_insert(None, parent_namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(alice),
            bob,
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            5
        ));
        let alice_permission_id = get_last_delegated_permission_id(alice);

        // Bob tries to delegate storage namespace using compute parent permission
        // This should fail because storage is not granular of compute
        let mut bob_unrelated_set = BoundedBTreeSet::new();
        bob_unrelated_set.try_insert(unrelated_storage).unwrap();
        let mut bob_paths = BoundedBTreeMap::new();
        bob_paths
            .try_insert(Some(alice_permission_id), bob_unrelated_set)
            .unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(bob),
                alice,
                bob_paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                2
            ),
            Error::<Test>::ParentPermissionNotFound // matched_count will be 0, children.len() will be 1
        );
    });
}

#[test]
fn delegate_namespace_succeeds_with_exact_match_from_parent() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let alice = alice();
        let bob = bob();

        // Alice creates namespace
        let compute_gpu = register_namespace(alice, b"agent.alice.compute.gpu");

        // Alice delegates permission to Bob
        let mut parent_namespace_set = BoundedBTreeSet::new();
        parent_namespace_set
            .try_insert(compute_gpu.clone())
            .unwrap();
        let mut alice_paths = BoundedBTreeMap::new();
        alice_paths.try_insert(None, parent_namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(alice),
            bob,
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            5
        ));
        let alice_permission_id = get_last_delegated_permission_id(alice);

        // Bob can delegate the exact same namespace (exact match case in resolve_paths)
        let mut bob_exact_set = BoundedBTreeSet::new();
        bob_exact_set.try_insert(compute_gpu).unwrap();
        let mut bob_paths = BoundedBTreeMap::new();
        bob_paths
            .try_insert(Some(alice_permission_id), bob_exact_set)
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(bob),
            alice,
            bob_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            2
        ));

        // Verify both permissions exist
        assert_eq!(PermissionsByDelegator::<Test>::get(alice).len(), 1);
        assert_eq!(PermissionsByDelegator::<Test>::get(bob).len(), 1);
    });
}

#[test]
fn revoke_namespace_permission_cascades_through_multiple_levels() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let dave = dave();
        let eve = eve();

        // Create hierarchical namespaces
        let level1_compute = register_namespace(alice, b"agent.alice.compute");
        let level2_gpu = register_namespace(alice, b"agent.alice.compute.gpu");
        let level3_h100 = register_namespace(alice, b"agent.alice.compute.gpu.h100");
        let level4_cluster = register_namespace(alice, b"agent.alice.compute.gpu.h100.cluster01");

        // Level 1: Alice delegates to Bob (RevocableByDelegator for testing revocation)
        let mut alice_paths = BoundedBTreeMap::new();
        let mut alice_namespace_set = BoundedBTreeSet::new();
        alice_namespace_set.try_insert(level1_compute).unwrap();
        alice_paths.try_insert(None, alice_namespace_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(alice),
            bob,
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator, // Alice can revoke
            10                                     // Plenty of instances
        ));
        let alice_permission_id = get_last_delegated_permission_id(alice);

        // Level 2: Bob delegates to Charlie
        let mut bob_paths = BoundedBTreeMap::new();
        let mut bob_namespace_set = BoundedBTreeSet::new();
        bob_namespace_set.try_insert(level2_gpu).unwrap();
        bob_paths
            .try_insert(Some(alice_permission_id), bob_namespace_set)
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(bob),
            charlie,
            bob_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator, // Bob can revoke
            8
        ));
        let bob_permission_id = get_last_delegated_permission_id(bob);

        // Level 3: Charlie delegates to Dave
        let mut charlie_paths = BoundedBTreeMap::new();
        let mut charlie_namespace_set = BoundedBTreeSet::new();
        charlie_namespace_set.try_insert(level3_h100).unwrap();
        charlie_paths
            .try_insert(Some(bob_permission_id), charlie_namespace_set)
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(charlie),
            dave,
            charlie_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator, // Charlie can revoke
            5
        ));
        let charlie_permission_id = get_last_delegated_permission_id(charlie);

        // Level 4: Dave delegates to Eve (final level)
        let mut dave_paths = BoundedBTreeMap::new();
        let mut dave_namespace_set = BoundedBTreeSet::new();
        dave_namespace_set.try_insert(level4_cluster).unwrap();
        dave_paths
            .try_insert(Some(charlie_permission_id), dave_namespace_set)
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(dave),
            eve,
            dave_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator, // Dave can revoke
            2
        ));
        let dave_permission_id = get_last_delegated_permission_id(dave);

        // Verify the full delegation chain exists
        assert_eq!(PermissionsByDelegator::<Test>::get(alice).len(), 1);
        assert_eq!(PermissionsByDelegator::<Test>::get(bob).len(), 1);
        assert_eq!(PermissionsByDelegator::<Test>::get(charlie).len(), 1);
        assert_eq!(PermissionsByDelegator::<Test>::get(dave).len(), 1);

        // Verify parent-child relationships
        let alice_permission = Permissions::<Test>::get(alice_permission_id).unwrap();
        assert!(
            alice_permission
                .children()
                .unwrap()
                .contains(&bob_permission_id)
        );
        assert_eq!(alice_permission.available_instances().unwrap(), 2); // 10 - 8 = 2

        let bob_permission = Permissions::<Test>::get(bob_permission_id).unwrap();
        assert!(
            bob_permission
                .children()
                .unwrap()
                .contains(&charlie_permission_id)
        );
        assert_eq!(bob_permission.available_instances().unwrap(), 3); // 8 - 5 = 3

        let charlie_permission = Permissions::<Test>::get(charlie_permission_id).unwrap();
        assert!(
            charlie_permission
                .children()
                .unwrap()
                .contains(&dave_permission_id)
        );
        assert_eq!(charlie_permission.available_instances().unwrap(), 3); // 5 - 2 = 3

        let dave_permission = Permissions::<Test>::get(dave_permission_id).unwrap();
        assert_eq!(dave_permission.children().unwrap().len(), 0); // No children
        assert_eq!(dave_permission.available_instances().unwrap(), 2); // Full instances available

        // FIRST REVOCATION: Revoke the last permission (Dave's to Eve)
        // This should only affect Dave's permission, removing it from Charlie's children
        assert_ok!(Permission0::revoke_permission(
            get_origin(dave), // Dave revokes his own delegation
            dave_permission_id
        ));

        // Verify Dave's permission is gone
        assert!(!Permissions::<Test>::contains_key(dave_permission_id));
        assert_eq!(PermissionsByDelegator::<Test>::get(dave).len(), 0);

        // Verify Charlie's children count decreased
        let charlie_permission_after_dave_revoke =
            Permissions::<Test>::get(charlie_permission_id).unwrap();
        assert!(
            !charlie_permission_after_dave_revoke
                .children()
                .unwrap()
                .contains(&dave_permission_id)
        );
        assert_eq!(
            charlie_permission_after_dave_revoke
                .available_instances()
                .unwrap(),
            5
        ); // Back to full 5 instances

        // Verify other permissions still exist
        assert!(Permissions::<Test>::contains_key(alice_permission_id));
        assert!(Permissions::<Test>::contains_key(bob_permission_id));
        assert!(Permissions::<Test>::contains_key(charlie_permission_id));
        assert_eq!(PermissionsByDelegator::<Test>::get(alice).len(), 1);
        assert_eq!(PermissionsByDelegator::<Test>::get(bob).len(), 1);
        assert_eq!(PermissionsByDelegator::<Test>::get(charlie).len(), 1);

        // SECOND REVOCATION: Revoke the first permission (Alice's to Bob)
        // This should cascade and remove all remaining permissions
        assert_ok!(Permission0::revoke_permission(
            get_origin(alice), // Alice revokes her delegation to Bob
            alice_permission_id
        ));

        // Verify everything is erased correctly (cascade revocation)
        assert!(!Permissions::<Test>::contains_key(alice_permission_id));
        assert!(!Permissions::<Test>::contains_key(bob_permission_id));
        assert!(!Permissions::<Test>::contains_key(charlie_permission_id));

        // Verify all delegator indices are cleared
        assert_eq!(PermissionsByDelegator::<Test>::get(alice).len(), 0);
        assert_eq!(PermissionsByDelegator::<Test>::get(bob).len(), 0);
        assert_eq!(PermissionsByDelegator::<Test>::get(charlie).len(), 0);
        assert_eq!(PermissionsByDelegator::<Test>::get(dave).len(), 0);

        // Verify recipient indices are also cleared
        assert_eq!(PermissionsByRecipient::<Test>::get(bob).len(), 0);
        assert_eq!(PermissionsByRecipient::<Test>::get(charlie).len(), 0);
        assert_eq!(PermissionsByRecipient::<Test>::get(dave).len(), 0);
        assert_eq!(PermissionsByRecipient::<Test>::get(eve).len(), 0);
    });
}

#[test]
fn revoke_middle_permission_cascades_to_children_only() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let dave = dave();

        let level1 = register_namespace(alice, b"agent.alice.compute");
        let level2 = register_namespace(alice, b"agent.alice.compute.gpu");
        let level3 = register_namespace(alice, b"agent.alice.compute.gpu.h100");

        // Create 3-level chain: Alice -> Bob -> Charlie -> Dave
        let mut alice_paths = BoundedBTreeMap::new();
        let mut alice_set = BoundedBTreeSet::new();
        alice_set.try_insert(level1).unwrap();
        alice_paths.try_insert(None, alice_set).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(alice),
            bob,
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            10
        ));
        let alice_permission_id = get_last_delegated_permission_id(alice);

        let mut bob_paths = BoundedBTreeMap::new();
        let mut bob_set = BoundedBTreeSet::new();
        bob_set.try_insert(level2).unwrap();
        bob_paths
            .try_insert(Some(alice_permission_id), bob_set)
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(bob),
            charlie,
            bob_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            7
        ));
        let bob_permission_id = get_last_delegated_permission_id(bob);

        let mut charlie_paths = BoundedBTreeMap::new();
        let mut charlie_set = BoundedBTreeSet::new();
        charlie_set.try_insert(level3).unwrap();
        charlie_paths
            .try_insert(Some(bob_permission_id), charlie_set)
            .unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(charlie),
            dave,
            charlie_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            3
        ));
        let charlie_permission_id = get_last_delegated_permission_id(charlie);

        // Verify initial state
        assert_eq!(PermissionsByDelegator::<Test>::get(alice).len(), 1);
        assert_eq!(PermissionsByDelegator::<Test>::get(bob).len(), 1);
        assert_eq!(PermissionsByDelegator::<Test>::get(charlie).len(), 1);

        // Revoke Bob's permission (middle of chain)
        // This should cascade to Charlie and Dave, but leave Alice's permission intact
        assert_ok!(Permission0::revoke_permission(
            get_origin(bob),
            bob_permission_id
        ));

        // Verify Alice's permission still exists but Bob's children are gone
        assert!(Permissions::<Test>::contains_key(alice_permission_id));
        let alice_permission = Permissions::<Test>::get(alice_permission_id).unwrap();
        assert!(
            !alice_permission
                .children()
                .unwrap()
                .contains(&bob_permission_id)
        );
        assert_eq!(alice_permission.available_instances().unwrap(), 10); // Back to full instances

        // Verify Bob, Charlie, and Dave permissions are all gone (cascaded)
        assert!(!Permissions::<Test>::contains_key(bob_permission_id));
        assert!(!Permissions::<Test>::contains_key(charlie_permission_id));

        assert_eq!(PermissionsByDelegator::<Test>::get(bob).len(), 0);
        assert_eq!(PermissionsByDelegator::<Test>::get(charlie).len(), 0);

        // Alice still has her permission
        assert_eq!(PermissionsByDelegator::<Test>::get(alice).len(), 1);
    });
}

#[test]
fn revocation_terms_is_weaker_function_tests() {
    new_test_ext().execute_with(|| {
        // RevocableByDelegator is always weaker than anything
        assert!(RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::Irrevocable,
            &RevocationTerms::RevocableByDelegator
        ));
        assert!(RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::RevocableByDelegator,
            &RevocationTerms::RevocableByDelegator
        ));
        assert!(RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::RevocableAfter(100),
            &RevocationTerms::RevocableByDelegator
        ));
        assert!(RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::RevocableByArbiters {
                accounts: Default::default(),
                required_votes: 0
            },
            &RevocationTerms::RevocableByDelegator
        ));

        // RevocableAfter(a) vs RevocableAfter(b) is weaker if a >= b
        assert!(RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::RevocableAfter(100),
            &RevocationTerms::RevocableAfter(100)
        ));
        assert!(RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::RevocableAfter(200),
            &RevocationTerms::RevocableAfter(100)
        ));
        assert!(!RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::RevocableAfter(50),
            &RevocationTerms::RevocableAfter(100)
        ));

        // Irrevocable parent allows RevocableAfter child (weaker)
        assert!(RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::Irrevocable,
            &RevocationTerms::RevocableAfter(100)
        ));

        assert!(RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::Irrevocable,
            &RevocationTerms::Irrevocable
        ));

        // All other combinations are not weaker
        assert!(!RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::RevocableByDelegator,
            &RevocationTerms::Irrevocable
        ));
        assert!(!RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::RevocableAfter(100),
            &RevocationTerms::Irrevocable
        ));
        assert!(!RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::RevocableByArbiters {
                accounts: Default::default(),
                required_votes: 0
            },
            &RevocationTerms::Irrevocable
        ));

        // RevocableByDelegator to RevocableAfter is not weaker
        assert!(!RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::RevocableByDelegator,
            &RevocationTerms::RevocableAfter(100)
        ));

        // RevocableAfter to RevocableByArbiters is not weaker
        assert!(!RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::RevocableAfter(100),
            &RevocationTerms::RevocableByArbiters {
                accounts: Default::default(),
                required_votes: 0
            }
        ));

        // Irrevocable to RevocableByArbiters is not weaker
        assert!(!RevocationTerms::<Test>::is_weaker(
            &RevocationTerms::Irrevocable,
            &RevocationTerms::RevocableByArbiters {
                accounts: Default::default(),
                required_votes: 2
            }
        ));
    });
}

#[test]
fn delegate_namespace_permission_requires_weaker_revocation_terms() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let dave = dave();
        let eve = eve();

        let compute_namespace = register_namespace(alice, b"agent.alice.compute");
        let gpu_namespace = register_namespace(alice, b"agent.alice.compute.gpu");

        let alice_paths = paths_map!(None => [compute_namespace]);
        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(alice),
            bob,
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableAfter(200),
            5
        ));
        let alice_permission_id = get_last_delegated_permission_id(alice);

        let bob_paths = paths_map!(Some(alice_permission_id) => [gpu_namespace.clone()]);

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(bob),
                charlie,
                bob_paths.clone(),
                PermissionDuration::Indefinite,
                RevocationTerms::RevocableAfter(300),
                2
            ),
            Error::<Test>::RevocationTermsTooStrong
        );

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(bob),
                dave,
                bob_paths.clone(),
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                2
            ),
            Error::<Test>::RevocationTermsTooStrong
        );

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(bob),
            charlie,
            bob_paths.clone(),
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableAfter(100),
            2
        ));

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(bob),
            eve,
            bob_paths.clone(),
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            1
        ));
    });
}

#[test]
fn delegate_namespace_permission_irrevocable_parent_allows_revocable_after() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let alice = alice();
        let bob = bob();

        let alice_paths = paths_map!(None => [register_namespace(alice, b"agent.alice.compute")]);
        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(alice),
            bob,
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            3
        ));
        let alice_permission_id = get_last_delegated_permission_id(alice);

        let bob_paths = paths_map!(Some(alice_permission_id) => [register_namespace(alice, b"agent.alice.compute.gpu")]);
        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(bob),
            alice,
            bob_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableAfter(100),
            1
        ));

        assert_eq!(PermissionsByDelegator::<Test>::get(alice).len(), 1);
        assert_eq!(PermissionsByDelegator::<Test>::get(bob).len(), 1);
    });
}

#[test]
fn delegate_namespace_permission_fails_when_exceeding_depth_limit() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        for i in 0..7 {
            register_agent(i);
        }

        let level1 = register_namespace(0, b"agent.alice.compute");
        let level2 = register_namespace(0, b"agent.alice.compute.gpu");
        let level3 = register_namespace(0, b"agent.alice.compute.gpu.h100");
        let level4 = register_namespace(0, b"agent.alice.compute.gpu.h100.cluster");
        let level5 = register_namespace(0, b"agent.alice.compute.gpu.h100.cluster.node01");
        let level6 = register_namespace(0, b"agent.alice.compute.gpu.h100.cluster.node01.core01");

        let mut parent_permission_id = None;
        let namespaces = [level1, level2, level3, level4, level5.clone()];

        for (i, namespace) in namespaces.iter().enumerate() {
            let delegator = i as u64 as u32;
            let recipient = (i + 1) as u64 as u32;

            let paths = paths_map!(parent_permission_id => [namespace.clone()]);

            assert_ok!(Permission0::delegate_namespace_permission(
                get_origin(delegator),
                recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::RevocableByDelegator,
                10
            ));

            parent_permission_id = Some(get_last_delegated_permission_id(delegator));
        }

        for to_fail in [level5, level6] {
            let paths = paths_map!(parent_permission_id => [to_fail]);

            assert_err!(
                Permission0::delegate_namespace_permission(
                    get_origin(5),
                    6,
                    paths,
                    PermissionDuration::Indefinite,
                    RevocationTerms::RevocableByDelegator,
                    10
                ),
                Error::<Test>::DelegationDepthExceeded
            );
        }
    });
}

#[test]
fn update_namespace_permission_basic_validations() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();
        let not_delegator = charlie();

        let non_existent_id = PermissionId::from([0xFF; 32]);
        assert_err!(
            Permission0::update_namespace_permission(get_origin(delegator), non_existent_id, 10),
            Error::<Test>::PermissionNotFound
        );

        let paths = paths_map!(None => [register_namespace(delegator, b"agent.alice.compute")]);
        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            5
        ));
        let permission_id = get_last_delegated_permission_id(delegator);

        assert_err!(
            Permission0::update_namespace_permission(get_origin(not_delegator), permission_id, 10),
            Error::<Test>::NotAuthorizedToEdit
        );

        // if new instance count is the same as the old one, we succeed with no changes
        assert_ok!(Permission0::update_namespace_permission(
            get_origin(delegator),
            permission_id,
            5
        ));

        let permission = Permissions::<Test>::get(permission_id).unwrap();
        assert_eq!(permission.max_instances().unwrap_or_default(), 5);
    });
}

#[test]
fn update_namespace_permission_larger_instances() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient_1 = bob();
        let recipient_2 = charlie();

        let paths_1 = paths_map!(None => [register_namespace(delegator, b"agent.alice.compute")]);

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient_1,
            paths_1,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            5
        ));
        let no_parent_permission_id = get_last_delegated_permission_id(delegator);

        // can increase without limit when no parent
        assert_ok!(Permission0::update_namespace_permission(
            get_origin(delegator),
            no_parent_permission_id,
            100
        ));

        let permission = Permissions::<Test>::get(no_parent_permission_id).unwrap();
        assert_eq!(permission.max_instances().unwrap_or_default(), 100);

        let paths_2 = paths_map!(None => [register_namespace(delegator, b"agent.alice.storage")]);

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient_2,
            paths_2,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            20
        ));
        let parent_permission_id = get_all_delegated_permission_ids(delegator)[1];

        let child_paths = paths_map!(Some(parent_permission_id) => [register_namespace(delegator, b"agent.alice.storage.ssd")]);

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(recipient_2),
            delegator,
            child_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            5
        ));
        let child_permission_id = get_last_delegated_permission_id(recipient_2);

        let parent = Permissions::<Test>::get(parent_permission_id).unwrap();
        assert_eq!(parent.available_instances().unwrap(), 15);

        // try to increase child instances beyond parent's available
        assert_err!(
            Permission0::update_namespace_permission(
                get_origin(recipient_2),
                child_permission_id,
                16
            ),
            Error::<Test>::NotEnoughInstances
        );

        // should succeed with exactly parent's available instances
        assert_ok!(Permission0::update_namespace_permission(
            get_origin(recipient_2),
            child_permission_id,
            15
        ));

        let updated_child = Permissions::<Test>::get(child_permission_id).unwrap();
        assert_eq!(updated_child.max_instances().unwrap_or_default(), 15);

        let parent = Permissions::<Test>::get(parent_permission_id).unwrap();
        assert_eq!(parent.available_instances().unwrap(), 5);
    });
}

#[test]
fn update_namespace_permission_smaller_instances() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let delegator = alice();
        let recipient = bob();
        let child_recipient = charlie();

        let revocable_after_block = 100;
        let paths = paths_map!(None => [register_namespace(delegator, b"agent.alice.compute")]);
        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths.clone(),
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableAfter(revocable_after_block),
            10
        ));
        let parent_permission_id = get_last_delegated_permission_id(delegator);

        let child_paths = paths_map!(Some(parent_permission_id) => [register_namespace(delegator, b"agent.alice.compute.gpu")]);
        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(recipient),
            child_recipient,
            child_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            4
        ));

        let parent = Permissions::<Test>::get(parent_permission_id).unwrap();
        assert_eq!(parent.available_instances().unwrap(), 6);

        // reducing before revocable period should fail
        assert_err!(
            Permission0::update_namespace_permission(
                get_origin(delegator),
                parent_permission_id,
                5
            ),
            Error::<Test>::NotAuthorizedToEdit
        );

        System::set_block_number(revocable_after_block + 1);

        // reducing to lowest than used instances should fail
        assert_err!(
            Permission0::update_namespace_permission(
                get_origin(delegator),
                parent_permission_id,
                3
            ),
            Error::<Test>::NotEnoughInstances
        );

        assert_ok!(Permission0::update_namespace_permission(
            get_origin(delegator),
            parent_permission_id,
            4
        ));

        let updated = Permissions::<Test>::get(parent_permission_id).unwrap();
        assert_eq!(updated.max_instances().unwrap_or_default(), 4);
        assert_eq!(updated.available_instances().unwrap(), 0);

        let storage_paths = paths_map!(None => [register_namespace(delegator, b"agent.alice.storage")]);
        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            storage_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            20
        ));
        let irrevocable_permission_id = get_all_delegated_permission_ids(delegator)[1];

        // cannot reduce irrevocable permission
        assert_err!(
            Permission0::update_namespace_permission(
                get_origin(delegator),
                irrevocable_permission_id,
                10
            ),
            Error::<Test>::NotAuthorizedToEdit
        );

        let network_paths = paths_map!(None => [register_namespace(delegator, b"agent.alice.network")]);
        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            network_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
            15
        ));
        let revocable_permission_id = get_all_delegated_permission_ids(delegator)[2];

        assert_ok!(Permission0::update_namespace_permission(
            get_origin(delegator),
            revocable_permission_id,
            5
        ));

        let updated = Permissions::<Test>::get(revocable_permission_id).unwrap();
        assert_eq!(updated.max_instances().unwrap_or_default(), 5);
    });
}

#[test]
fn bulk_delegate_namespace_permission_fails_when_redelegation_exceeds_parent_instances() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let dave = dave();

        // Alice creates a namespace and delegates to Bob with 3 instances
        let namespace = register_namespace(alice, b"agent.alice.compute");
        let alice_paths = paths_map!(None => [namespace.clone()]);

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(alice),
            bob,
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            3 // Bob gets 3 instances
        ));

        let bob_permission_id = get_last_delegated_permission_id(alice);

        // Bob tries to redelegate to Charlie and Dave with 2 instances each (total 4)
        // This should fail because Bob only has 3 instances available
        let bob_paths = paths_map!(Some(bob_permission_id) => [namespace]);

        let mut recipients = BoundedBTreeSet::new();
        recipients.try_insert(charlie).unwrap();
        recipients.try_insert(dave).unwrap();

        // This should fail because 2 recipients × 2 instances = 4 instances needed,
        // but Bob only has 3 instances available
        assert_err!(
            Permission0::bulk_delegate_namespace_permission(
                get_origin(bob),
                recipients,
                bob_paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
                2 // 2 instances per recipient
            ),
            Error::<Test>::NotEnoughInstances
        );

        // Verify no permissions were created
        assert_eq!(
            PermissionsByRecipient::<Test>::get(charlie).len(),
            0,
            "Charlie should have no permissions"
        );
        assert_eq!(
            PermissionsByRecipient::<Test>::get(dave).len(),
            0,
            "Dave should have no permissions"
        );

        // Verify Bob's permission still has all instances available
        let bob_permission = Permissions::<Test>::get(bob_permission_id).unwrap();
        assert_eq!(bob_permission.available_instances().unwrap(), 3);
        assert_eq!(bob_permission.children().unwrap().len(), 0);
    });
}

#[test]
fn bulk_delegate_namespace_permission_succeeds_within_parent_instance_limit() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let dave = dave();

        // Alice creates a namespace and delegates to Bob with 4 instances
        let namespace = register_namespace(alice, b"agent.alice.compute");
        let alice_paths = paths_map!(None => [namespace.clone()]);

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(alice),
            bob,
            alice_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            4 // Bob gets 4 instances
        ));

        let bob_permission_id = get_last_delegated_permission_id(alice);

        // Bob redelegates to Charlie and Dave with 2 instances each (total 4)
        // This should succeed because Bob has exactly 4 instances available
        let bob_paths = paths_map!(Some(bob_permission_id) => [namespace]);

        let mut recipients = BoundedBTreeSet::new();
        recipients.try_insert(charlie).unwrap();
        recipients.try_insert(dave).unwrap();

        assert_ok!(Permission0::bulk_delegate_namespace_permission(
            get_origin(bob),
            recipients,
            bob_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
            2 // 2 instances per recipient
        ));

        // Verify permissions were created
        assert_eq!(
            PermissionsByRecipient::<Test>::get(charlie).len(),
            1,
            "Charlie should have 1 permission"
        );
        assert_eq!(
            PermissionsByRecipient::<Test>::get(dave).len(),
            1,
            "Dave should have 1 permission"
        );

        // Get the created permission IDs using helper function
        let charlie_permission_id = get_permission_id_for_recipient(charlie);
        let dave_permission_id = get_permission_id_for_recipient(dave);

        // Verify each permission has 2 instances
        let charlie_permission = Permissions::<Test>::get(charlie_permission_id).unwrap();
        assert_eq!(charlie_permission.max_instances().unwrap_or_default(), 2);

        let dave_permission = Permissions::<Test>::get(dave_permission_id).unwrap();
        assert_eq!(dave_permission.max_instances().unwrap_or_default(), 2);

        // Verify Bob's permission has no instances left available
        let bob_permission = Permissions::<Test>::get(bob_permission_id).unwrap();
        assert_eq!(bob_permission.available_instances().unwrap(), 0); // 4 - (2+2) = 0
        assert_eq!(bob_permission.children().unwrap().len(), 2);
        assert!(
            bob_permission
                .children()
                .unwrap()
                .contains(&charlie_permission_id)
        );
        assert!(
            bob_permission
                .children()
                .unwrap()
                .contains(&dave_permission_id)
        );
    });
}

#[test]
fn namespace_instance_overlap_logic_comprehensive_test() {
    new_test_ext().execute_with(|| {
        // Register all agents we'll need
        let alice = 0;
        register_agent(alice);
        let bob = 1;
        register_agent(bob);
        let charlie = 2;
        register_agent(charlie);
        let dave = 3;
        register_agent(dave);
        let eve = 4;
        register_agent(eve);
        let ferdie = 5;
        register_agent(ferdie);

        // Register all namespaces we'll need
        let compute = register_namespace(alice, b"agent.alice.compute");
        let cpu = register_namespace(alice, b"agent.alice.compute.cpu");
        let epyc4005 = register_namespace(alice, b"agent.alice.compute.cpu.epyc4005");
        let xeon = register_namespace(alice, b"agent.alice.compute.cpu.xeon");
        let gpu = register_namespace(alice, b"agent.alice.compute.gpu");
        let h100 = register_namespace(alice, b"agent.alice.compute.gpu.h100");
        let rtx4090 = register_namespace(alice, b"agent.alice.compute.gpu.rtx4090");
        let cluster1 = register_namespace(alice, b"agent.alice.compute.gpu.h100.cluster1");
        let cluster2 = register_namespace(alice, b"agent.alice.compute.gpu.h100.cluster2");
        let storage = register_namespace(alice, b"agent.alice.compute.storage");

        // Helper function that implements the overlap-based instance counting logic
        fn delegate(
            delegator: AccountId,
            recipient: AccountId,
            parent: Option<PermissionId>,
            path: &[&NamespacePathInner],
            instances: u32,
        ) -> Result<PermissionId, DispatchError> {
            step_block(1);

            let paths = TryCollect::<crate::BoundedBTreeMap<_, _, _>>::try_collect(
                vec![(
                    parent,
                    TryCollect::<crate::BoundedBTreeSet<_, _>>::try_collect(
                        path.iter().map(|p| (**p).clone()),
                    )
                    .unwrap(),
                )]
                .into_iter(),
            )
            .unwrap();

            Permission0::delegate_namespace_permission(
                get_origin(delegator),
                recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::RevocableByDelegator,
                instances,
            )?;

            Ok(get_last_delegated_permission_id(delegator))
        }

        // ========== TEST 1 INSTANCE ==========
        // With 1 instance, no overlapping paths can coexist
        let parent = delegate(alice, bob, None, &[&compute], 1).unwrap();

        delegate(bob, charlie, Some(parent), &[&gpu], 2)
            .expect_err("more instances than available");

        // Siblings can coexist (no overlap)
        delegate(bob, charlie, Some(parent), &[&cpu, &h100, &gpu], 1)
            .expect_err("gpu overlaps with h100");

        delegate(bob, charlie, Some(parent), &[&cpu], 1).unwrap();
        delegate(bob, charlie, Some(parent), &[&h100], 1).unwrap(); // OK: h100 is under gpu, not cpu

        // Parent-child overlaps are blocked
        delegate(bob, charlie, Some(parent), &[&gpu], 1).expect_err("gpu overlaps with h100");
        delegate(bob, dave, Some(parent), &[&epyc4005], 1).expect_err("epyc4005 overlaps with cpu");

        // But non-overlapping paths work
        delegate(bob, dave, Some(parent), &[&rtx4090], 1).unwrap(); // OK: sibling to h100

        // ========== TEST 2 INSTANCES ==========
        // With 2 instances, we can have 2 overlapping chains
        let parent = delegate(alice, bob, None, &[&compute], 2).unwrap();

        delegate(bob, charlie, Some(parent), &[&cpu], 1).unwrap();
        delegate(bob, charlie, Some(parent), &[&h100], 1).unwrap();

        // Can delegate same path twice (uses 2 instances)
        delegate(bob, dave, Some(parent), &[&h100], 1).unwrap(); // 2nd h100

        // Can delegate overlapping paths
        delegate(bob, eve, Some(parent), &[&epyc4005, &rtx4090, &h100], 1)
            .expect_err("3rd h100 needs 3 instances"); // Overlaps with cpu

        delegate(bob, eve, Some(parent), &[&epyc4005], 1).unwrap(); // Overlaps with cpu
        delegate(bob, eve, Some(parent), &[&rtx4090], 1).unwrap(); // Sibling to h100s

        // But 3rd h100 would fail
        delegate(bob, ferdie, Some(parent), &[&h100], 1).expect_err("3rd h100 needs 3 instances");

        // ========== TEST 3 INSTANCES ==========
        // Test 3a: Three identical paths
        let parent3a = delegate(alice, bob, None, &[&compute], 3).unwrap();

        delegate(bob, charlie, Some(parent3a), &[&h100], 1).unwrap(); // h100 #1
        delegate(bob, dave, Some(parent3a), &[&h100], 1).unwrap(); // h100 #2
        delegate(bob, eve, Some(parent3a), &[&h100], 1).unwrap(); // h100 #3
        delegate(bob, ferdie, Some(parent3a), &[&h100], 1).expect_err("4th h100 needs 4 instances");

        // Test 3b: Complex branching with overlaps
        let parent3b = delegate(alice, bob, None, &[&compute], 3).unwrap();

        delegate(bob, charlie, Some(parent3b), &[&cpu], 1).unwrap(); // CPU branch
        delegate(bob, charlie, Some(parent3b), &[&gpu], 1).unwrap(); // GPU branch (sibling)
        delegate(bob, dave, Some(parent3b), &[&epyc4005], 1).unwrap(); // Under CPU (1 overlap)
        delegate(bob, eve, Some(parent3b), &[&h100], 1).unwrap(); // Under GPU (1 overlap)
        delegate(bob, eve, Some(parent3b), &[&rtx4090], 1).unwrap(); // Under GPU (1 overlap)

        // xeon would work (only overlaps with cpu, using 1 instance)
        delegate(bob, ferdie, Some(parent3b), &[&xeon], 1).unwrap();

        // ========== TEST 4 INSTANCES ==========
        // Test 4a: Four identical paths
        let parent4a = delegate(alice, bob, None, &[&compute], 4).unwrap();

        delegate(bob, charlie, Some(parent4a), &[&h100], 1).unwrap(); // h100 #1
        delegate(bob, dave, Some(parent4a), &[&h100], 1).unwrap(); // h100 #2
        delegate(bob, eve, Some(parent4a), &[&h100], 1).unwrap(); // h100 #3
        delegate(bob, ferdie, Some(parent4a), &[&h100], 1).unwrap(); // h100 #4

        // Can't add 5th
        let parent4a_new = delegate(alice, charlie, None, &[&compute], 4).unwrap();
        delegate(charlie, alice, Some(parent4a_new), &[&h100], 1).unwrap();
        delegate(charlie, alice, Some(parent4a_new), &[&h100], 1).unwrap();
        delegate(charlie, alice, Some(parent4a_new), &[&h100], 1).unwrap();
        delegate(charlie, alice, Some(parent4a_new), &[&h100], 1).unwrap();
        delegate(charlie, bob, Some(parent4a_new), &[&h100], 1)
            .expect_err("5th h100 needs 5 instances");

        // Test 4b: Deep hierarchy with siblings
        let parent4b = delegate(alice, bob, None, &[&compute], 4).unwrap();

        // Build branches
        delegate(bob, charlie, Some(parent4b), &[&gpu], 1).unwrap(); // GPU root
        delegate(bob, dave, Some(parent4b), &[&h100], 1).unwrap(); // Under GPU (1 overlap)
        delegate(bob, eve, Some(parent4b), &[&cluster1], 1).unwrap(); // Under h100 (2 overlaps)
        delegate(bob, ferdie, Some(parent4b), &[&cluster2], 1).unwrap(); // Sibling to cluster1 (2 overlaps)

        delegate(bob, ferdie, Some(parent4b), &[&cluster1], 1).unwrap();
        delegate(bob, ferdie, Some(parent4b), &[&cluster1], 1)
            .expect_err("consumed all cluster1 instances");

        delegate(bob, ferdie, Some(parent4b), &[&cluster2], 1).unwrap();
        delegate(bob, ferdie, Some(parent4b), &[&cluster2], 1)
            .expect_err("consumed all cluster2 instances");

        // Can still add non-overlapping branches
        delegate(bob, charlie, Some(parent4b), &[&cpu], 1).unwrap(); // CPU branch (no overlap)
        delegate(bob, dave, Some(parent4b), &[&storage], 1).unwrap(); // Storage branch (no overlap)

        // rtx4090 works (only overlaps with gpu root)
        delegate(bob, eve, Some(parent4b), &[&rtx4090], 1).unwrap();

        // With 2 instances, we can have 2 overlapping chains
        let parent = delegate(alice, bob, None, &[&gpu, &h100], 2).unwrap();

        delegate(bob, charlie, Some(parent), &[&rtx4090, &h100], 1).unwrap();
        delegate(bob, charlie, Some(parent), &[&h100], 1).unwrap();
        delegate(bob, charlie, Some(parent), &[&h100], 1).expect_err("h100 instace pool depleted");
    });
}
