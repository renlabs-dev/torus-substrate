use pallet_permission0::{
    Error, Pallet, PermissionDuration, PermissionScope, Permissions, RevocationTerms,
};
use pallet_permission0_api::Permission0NamespacesApi;
use pallet_torus0_api::{NamespacePath, NamespacePathInner};
use polkadot_sdk::{
    frame_support::{assert_err, assert_ok, BoundedBTreeSet},
    frame_system::RawOrigin,
};
use test_utils::*;

fn register_agent(id: AccountId) {
    let name = match id {
        0 => &b"alice"[..],
        1 => &b"bob"[..],
        _ => &b"charlie"[..],
    };

    Balances::force_set_balance(RawOrigin::Root.into(), id, u128::MAX).unwrap();
    Torus0::register_agent(get_origin(id), name.to_vec(), name.to_vec(), name.to_vec()).unwrap();
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

        let mut paths = BoundedBTreeSet::new();
        paths.try_insert(bounded_namespace).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
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

        let mut paths = BoundedBTreeSet::new();
        paths.try_insert(bounded_child.clone()).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
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

        let mut paths = BoundedBTreeSet::new();
        paths.try_insert(bounded_parent.clone()).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
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

        let mut paths = BoundedBTreeSet::new();
        paths.try_insert(bounded_compute).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
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

        let mut compute_paths = BoundedBTreeSet::new();
        compute_paths.try_insert(bounded_compute).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient_1,
            compute_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
        ));

        let mut storage_paths = BoundedBTreeSet::new();
        storage_paths.try_insert(bounded_storage).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient_2,
            storage_paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
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
        let mut paths = BoundedBTreeSet::new();
        paths.try_insert(bounded_namespace).unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(unregistered_delegator),
                recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
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
        let mut paths = BoundedBTreeSet::new();
        paths.try_insert(bounded_namespace).unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(delegator),
                unregistered_recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
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
        let mut paths = BoundedBTreeSet::new();
        paths.try_insert(bounded_namespace).unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(delegator),
                recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
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
        let mut paths = BoundedBTreeSet::new();
        paths.try_insert(bounded_namespace).unwrap();

        assert_err!(
            Permission0::delegate_namespace_permission(
                get_origin(delegator),
                recipient,
                paths,
                PermissionDuration::Indefinite,
                RevocationTerms::Irrevocable,
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

        let mut paths = BoundedBTreeSet::new();
        paths.try_insert(bounded_namespace).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths.clone(),
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
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

        let mut paths = BoundedBTreeSet::new();
        paths.try_insert(bounded_compute).unwrap();
        paths.try_insert(bounded_storage).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
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

        let mut paths = BoundedBTreeSet::new();
        paths.try_insert(bounded_namespace).unwrap();

        assert_ok!(Permission0::delegate_namespace_permission(
            get_origin(delegator),
            recipient,
            paths,
            PermissionDuration::Indefinite,
            RevocationTerms::Irrevocable,
        ));

        let permissions: Vec<_> = Permissions::<Test>::iter().collect();
        assert_eq!(permissions.len(), 1);

        let (_, permission) = permissions.first().unwrap();
        assert_eq!(permission.delegator, delegator);
        assert_eq!(permission.recipient, recipient);

        match &permission.scope {
            PermissionScope::Namespace(namespace_scope) => {
                assert_eq!(namespace_scope.paths.len(), 1);
                let expected_path = NamespacePath::new_agent(b"agent.alice.compute").unwrap();
                assert!(namespace_scope.paths.contains(&expected_path));
            }
            _ => panic!("Expected Namespace scope"),
        }
    });
}
