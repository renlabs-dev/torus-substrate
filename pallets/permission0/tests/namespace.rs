#![allow(clippy::indexing_slicing, clippy::arithmetic_side_effects)]

use pallet_permission0::{
    CuratorPermissions, Error, Pallet, PermissionDuration, PermissionId, PermissionScope,
    Permissions, PermissionsByDelegator, PermissionsByRecipient, RevocationTerms,
};
use pallet_permission0_api::Permission0NamespacesApi;
use pallet_torus0_api::{NamespacePath, NamespacePathInner};
use polkadot_sdk::sp_core::{H256, TryCollect};
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
        .last() // Get most recent
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
            PermissionScope::Emission(scope) if scope.recipients.contains_key(&recipient) => {
                return permission_id;
            }
            _ => continue,
        }
    }
    panic!("No permission found for recipient: {:?}", recipient);
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
        let agent_0 = 0;
        register_agent(agent_0);

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
        let delegator = 0;
        let recipient = 1;
        register_agent(delegator);
        register_agent(recipient);

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
        let delegator = 0;
        let recipient = 1;
        register_agent(delegator);
        register_agent(recipient);

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
        let delegator = 0;
        let recipient = 1;
        register_agent(delegator);
        register_agent(recipient);

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
        let delegator = 0;
        let recipient = 1;
        register_agent(delegator);
        register_agent(recipient);

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
        let delegator = 0;
        let recipient_1 = 1;
        let recipient_2 = 2;
        register_agent(delegator);
        register_agent(recipient_1);
        register_agent(recipient_2);

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
        let recipient = 1;
        register_agent(recipient);

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
        let delegator = 0;
        let unregistered_recipient = 1;
        register_agent(delegator);

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
        let delegator = 0;
        let recipient = 1;
        register_agent(delegator);
        register_agent(recipient);

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
        let delegator = 0;
        let recipient = 1;
        register_agent(delegator);
        register_agent(recipient);

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
        let delegator = 0;
        let recipient = 1;
        register_agent(delegator);
        register_agent(recipient);

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
        let delegator = 0;
        let recipient = 1;
        register_agent(delegator);
        register_agent(recipient);

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
        let delegator = 0;
        let recipient = 1;
        register_agent(delegator);
        register_agent(recipient);

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
        let delegator = 0;
        register_agent(delegator);

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
        let delegator = 0;
        register_agent(delegator);

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
        let delegator = 0;
        let recipient = 1;
        register_agent(delegator);
        register_agent(recipient);

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
        let original_delegator = 0;
        let wrong_delegator = 1;
        let recipient = 2;
        register_agent(original_delegator);
        register_agent(wrong_delegator);
        register_agent(recipient);

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
        let delegator = 0;
        let recipient = 1;
        register_agent(delegator);
        register_agent(recipient);

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
        let delegator = 0;
        let recipient = 1;
        let final_recipient = 2;
        register_agent(delegator);
        register_agent(recipient);
        register_agent(final_recipient);

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
        let delegator = 0;
        let recipient = 1;
        let final_recipient = 2;
        register_agent(delegator);
        register_agent(recipient);
        register_agent(final_recipient);

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
        assert_eq!(parent_permission.available_instances(), 5);

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
        assert_eq!(parent_permission.available_instances(), 3);
    });
}

#[test]
fn delegate_granular_namespace_from_parent_permission_succeeds() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let alice = 0;
        let bob = 1;
        let charlie = 2;

        register_agent(alice);
        register_agent(bob);
        register_agent(charlie);

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
        let alice = 0;
        let bob = 1;

        register_agent(alice);
        register_agent(bob);

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
        let alice = 0;
        let bob = 1;

        register_agent(alice);
        register_agent(bob);

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
        let alice = 0;
        let bob = 1;

        register_agent(alice);
        register_agent(bob);

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
        let alice = 0;
        let bob = 1;
        let charlie = 2;
        let dave = 3;
        let eve = 4;

        // Register all agents
        register_agent(alice);
        register_agent(bob);
        register_agent(charlie);
        register_agent(dave);
        register_agent(eve);

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
        assert!(alice_permission.children.contains(&bob_permission_id));
        assert_eq!(alice_permission.available_instances(), 2); // 10 - 8 = 2

        let bob_permission = Permissions::<Test>::get(bob_permission_id).unwrap();
        assert!(bob_permission.children.contains(&charlie_permission_id));
        assert_eq!(bob_permission.available_instances(), 3); // 8 - 5 = 3

        let charlie_permission = Permissions::<Test>::get(charlie_permission_id).unwrap();
        assert!(charlie_permission.children.contains(&dave_permission_id));
        assert_eq!(charlie_permission.available_instances(), 3); // 5 - 2 = 3

        let dave_permission = Permissions::<Test>::get(dave_permission_id).unwrap();
        assert_eq!(dave_permission.children.len(), 0); // No children
        assert_eq!(dave_permission.available_instances(), 2); // Full instances available

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
                .children
                .contains(&dave_permission_id)
        );
        assert_eq!(
            charlie_permission_after_dave_revoke.available_instances(),
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
        let alice = 0;
        let bob = 1;
        let charlie = 2;
        let dave = 3;

        register_agent(alice);
        register_agent(bob);
        register_agent(charlie);
        register_agent(dave);

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
        assert!(!alice_permission.children.contains(&bob_permission_id));
        assert_eq!(alice_permission.available_instances(), 10); // Back to full instances

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
        let alice = 0;
        let bob = 1;
        let charlie = 2;
        let dave = 3;
        let eve = 4;

        register_agent(alice);
        register_agent(bob);
        register_agent(charlie);
        register_agent(dave);
        register_agent(eve);

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
        let alice = 0;
        let bob = 1;

        register_agent(alice);
        register_agent(bob);

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
        let delegator = 0;
        let recipient = 1;
        let not_delegator = 2;
        register_agent(delegator);
        register_agent(recipient);
        register_agent(not_delegator);

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
        assert_eq!(permission.max_instances, 5);
    });
}

#[test]
fn update_namespace_permission_larger_instances() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let delegator = 0;
        let recipient_1 = 1;
        let recipient_2 = 2;
        register_agent(delegator);
        register_agent(recipient_1);
        register_agent(recipient_2);

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
        assert_eq!(permission.max_instances, 100);

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
        assert_eq!(parent.available_instances(), 15);

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
        assert_eq!(updated_child.max_instances, 15);

        let parent = Permissions::<Test>::get(parent_permission_id).unwrap();
        assert_eq!(parent.available_instances(), 5);
    });
}

#[test]
fn update_namespace_permission_smaller_instances() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let delegator = 0;
        let recipient = 1;
        let child_recipient = 2;
        register_agent(delegator);
        register_agent(recipient);
        register_agent(child_recipient);

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
        assert_eq!(parent.available_instances(), 6);

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
        assert_eq!(updated.max_instances, 4);
        assert_eq!(updated.available_instances(), 0);

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
        assert_eq!(updated.max_instances, 5);
    });
}
