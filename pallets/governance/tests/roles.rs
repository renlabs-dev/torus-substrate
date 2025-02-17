use pallet_governance::Curators;
use polkadot_sdk::frame_support::assert_err;
use test_utils::*;

use crate::pallet_governance::Error;

#[test]
fn add_and_remove_curator() {
    new_test_ext().execute_with(|| {
        let curator_key = 0;

        assert_ok!(pallet_governance::Pallet::<Test>::add_curator(
            RuntimeOrigin::root(),
            curator_key
        ));

        assert!(pallet_governance::Curators::<Test>::contains_key(
            curator_key
        ));

        assert_ok!(pallet_governance::Pallet::<Test>::remove_curator(
            RuntimeOrigin::root(),
            curator_key,
        ));

        assert!(!pallet_governance::Curators::<Test>::contains_key(
            curator_key
        ));
    });
}

#[test]
fn add_and_remove_allocator() {
    new_test_ext().execute_with(|| {
        let allocator_key = 0;

        assert_ok!(pallet_governance::Pallet::<Test>::add_allocator(
            RuntimeOrigin::root(),
            allocator_key
        ));

        assert!(pallet_governance::Allocators::<Test>::contains_key(
            allocator_key
        ));

        assert_ok!(pallet_governance::Pallet::<Test>::remove_allocator(
            RuntimeOrigin::root(),
            allocator_key,
        ));

        assert!(!pallet_governance::Allocators::<Test>::contains_key(
            allocator_key
        ));
    });
}

#[test]
fn add_and_remove_from_whitelist() {
    new_test_ext().execute_with(|| {
        let curator_key = 0;
        let module_key = 1;
        Curators::<Test>::insert(curator_key, ());

        assert_ok!(pallet_governance::Pallet::<Test>::add_to_whitelist(
            get_origin(curator_key),
            module_key
        ));

        assert_err!(
            pallet_governance::Pallet::<Test>::add_to_whitelist(
                get_origin(curator_key),
                module_key
            ),
            Error::<Test>::AlreadyWhitelisted
        );

        assert!(pallet_governance::whitelist::is_whitelisted::<Test>(
            &module_key
        ));

        assert_ok!(pallet_governance::Pallet::<Test>::remove_from_whitelist(
            get_origin(curator_key),
            module_key
        ));

        assert_err!(
            pallet_governance::Pallet::<Test>::remove_from_whitelist(
                get_origin(curator_key),
                module_key
            ),
            Error::<Test>::NotWhitelisted
        );

        assert!(!pallet_governance::whitelist::is_whitelisted::<Test>(
            &module_key
        ));
    });
}

#[test]
fn only_curators_can_whitelist() {
    new_test_ext().execute_with(|| {
        let not_curator_key = 0;
        let module_key = 1;

        assert_err!(
            pallet_governance::Pallet::<Test>::add_to_whitelist(
                get_origin(not_curator_key),
                module_key
            ),
            Error::<Test>::NotCurator
        );
    });
}
