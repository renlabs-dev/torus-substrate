// Helper function to grant a permission
fn grant_permission(
    grantor: AccountId,
    grantee: AccountId,
    percentage: u8,
    targets: Vec<(AccountId, u16)>,
    auto_threshold: Option<u128>,
) -> crate::PermissionId {
    let result = Permission0::grant_permission(
        RuntimeOrigin::signed(grantor),
        grantee,
        percentage,
        targets,
        auto_threshold,
    );
    assert_ok!(result);

    // Get the permission ID from the event
    let events = System::events();
    let permission_granted_event = events.last().unwrap().event.clone();

    if let RuntimeEvent::Permission0(Event::PermissionGranted { permission_id, .. }) =
        permission_granted_event
    {
        permission_id
    } else {
        panic!("Expected PermissionGranted event not found");
    }
}

#[test]
fn test_grant_permission() {
    new_test_ext().execute_with(|| {
        // Grant a permission from account 1 to account 2
        let targets = vec![(3, 1), (4, 2)]; // 3 gets 1/3, 4 gets 2/3
        let percentage = 30; // 30% of emissions

        let permission_id = grant_permission(1, 2, percentage, targets.clone(), None);

        // Check storage
        let permission = Permission0::permissions(permission_id).unwrap();
        assert_eq!(permission.grantor, 1);
        assert_eq!(permission.grantee, 2);
        assert_eq!(permission.percentage, percentage);

        // Check targets
        assert_eq!(permission.targets.len(), 2);
        assert_eq!(*permission.targets.get(&3).unwrap(), 1);
        assert_eq!(*permission.targets.get(&4).unwrap(), 2);

        // Check indices
        let by_participants = Permission0::permissions_by_participants((1, 2)).unwrap();
        assert_eq!(by_participants.len(), 1);
        assert_eq!(by_participants[0], permission_id);

        let by_grantor = Permission0::permissions_by_grantor(1).unwrap();
        assert_eq!(by_grantor.len(), 1);
        assert_eq!(by_grantor[0], permission_id);

        let by_grantee = Permission0::permissions_by_grantee(2).unwrap();
        assert_eq!(by_grantee.len(), 1);
        assert_eq!(by_grantee[0], permission_id);
    });
}

#[test]
fn test_grant_permission_exceeds_100_percent() {
    new_test_ext().execute_with(|| {
        // Grant a permission with 60%
        let targets1 = vec![(3, 1), (4, 1)];
        let permission_id1 = grant_permission(1, 2, 60, targets1, None);

        // Grant another permission with 50% - should fail
        let targets2 = vec![(3, 1), (4, 1)];
        assert_noop!(
            Permission0::grant_permission(RuntimeOrigin::signed(1), 3, 50, targets2, None),
            Error::<Test>::TotalAllocationExceeded
        );

        // Successful with 40%
        let targets3 = vec![(3, 1), (4, 1)];
        let permission_id2 = grant_permission(1, 3, 40, targets3, None);

        // Check both permissions exist
        assert!(Permission0::permission_exists(&permission_id1));
        assert!(Permission0::permission_exists(&permission_id2));
    });
}

#[test]
fn test_revoke_permission() {
    new_test_ext().execute_with(|| {
        // Grant a permission
        let targets = vec![(3, 1), (4, 1)];
        let permission_id = grant_permission(1, 2, 30, targets, None);

        // Revoke the permission
        assert_ok!(Permission0::revoke_permission(
            RuntimeOrigin::signed(1),
            permission_id
        ));

        // Permission should be gone
        assert!(!Permission0::permission_exists(&permission_id));

        // Indices should be cleaned up
        assert!(
            Permission0::permissions_by_participants((1, 2)).is_none()
                || Permission0::permissions_by_participants((1, 2))
                    .unwrap()
                    .is_empty()
        );
    });
}

#[test]
fn test_unauthorized_revoke() {
    new_test_ext().execute_with(|| {
        // Grant a permission
        let targets = vec![(3, 1), (4, 1)];
        let permission_id = grant_permission(1, 2, 30, targets, None);

        // Try to revoke from unauthorized account
        assert_noop!(
            Permission0::revoke_permission(RuntimeOrigin::signed(3), permission_id),
            Error::<Test>::NotAuthorizedToRevoke
        );

        // Grantee can also revoke
        assert_ok!(Permission0::revoke_permission(
            RuntimeOrigin::signed(2),
            permission_id
        ));
    });
}

#[test]
fn test_accumulate_and_manual_execute() {
    new_test_ext().execute_with(|| {
        // Grant a permission with manual distribution
        let targets = vec![(3, 1), (4, 2)]; // 1/3 to acc 3, 2/3 to acc 4
        let permission_id = grant_permission(1, 2, 30, targets, None);

        // Accumulate emissions
        let emission_amount = 1000;
        Permission0::accumulate_emissions(&1, emission_amount);

        // Check accumulated amount (30% of 1000 = 300)
        let accumulated = Permission0::get_accumulated_amount(&permission_id);
        assert_eq!(accumulated, 300);

        // Execute the permission
        assert_ok!(Permission0::execute_permission(
            RuntimeOrigin::signed(2),
            permission_id
        ));

        // Check balances are updated (100 to acc 3, 200 to acc 4)
        assert_eq!(Balances::free_balance(3), 1_000_000 + 100);
        assert_eq!(Balances::free_balance(4), 1_000_000 + 200);

        // Accumulated amount should be reset
        assert_eq!(Permission0::get_accumulated_amount(&permission_id), 0);
    });
}

#[test]
fn test_accumulate_and_auto_execute() {
    new_test_ext().execute_with(|| {
        // Grant a permission with auto distribution (threshold 200)
        let targets = vec![(3, 1), (4, 1)]; // 50/50 split
        let permission_id = grant_permission(1, 2, 30, targets, Some(200));

        // Accumulate emissions just below threshold
        Permission0::accumulate_emissions(&1, 500); // 30% of 500 = 150

        // Check accumulated amount
        let accumulated = Permission0::get_accumulated_amount(&permission_id);
        assert_eq!(accumulated, 150);

        // Accumulate more to trigger auto-distribution
        Permission0::accumulate_emissions(&1, 200); // 30% of 200 = 60, total now 210

        // Accumulated amount should be reset to 0 after auto-distribution
        assert_eq!(Permission0::get_accumulated_amount(&permission_id), 0);

        // Check balances (105 to each target)
        assert_eq!(Balances::free_balance(3), 1_000_000 + 105);
        assert_eq!(Balances::free_balance(4), 1_000_000 + 105);
    });
}

#[test]
fn test_manual_execute_permission() {
    new_test_ext().execute_with(|| {
        // Grant a permission with manual distribution
        let targets = vec![(3, 1), (4, 1)];
        let permission_id = grant_permission(1, 2, 30, targets, None);

        // Accumulate emissions
        Permission0::accumulate_emissions(&1, 1000);

        // Try to execute from wrong account
        assert_noop!(
            Permission0::execute_permission(RuntimeOrigin::signed(1), permission_id),
            Error::<Test>::NotPermissionGrantee
        );

        // Execute from correct account
        assert_ok!(Permission0::execute_permission(
            RuntimeOrigin::signed(2),
            permission_id
        ));

        // Check balances
        assert_eq!(Balances::free_balance(3), 1_000_000 + 150);
        assert_eq!(Balances::free_balance(4), 1_000_000 + 150);
    });
}

#[test]
fn test_cant_manually_execute_auto_permission() {
    new_test_ext().execute_with(|| {
        // Grant a permission with auto distribution
        let targets = vec![(3, 1), (4, 1)];
        let permission_id = grant_permission(1, 2, 30, targets, Some(500));

        // Accumulate emissions
        Permission0::accumulate_emissions(&1, 1000);

        // Try to manually execute auto-distribution permission
        assert_noop!(
            Permission0::execute_permission(RuntimeOrigin::signed(2), permission_id),
            Error::<Test>::NotPermissionGrantee
        );
    });
}
