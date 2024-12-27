use crate::pallet_governance::Error;
use pallet_governance::Curators;
use polkadot_sdk::frame_support::assert_err;
use test_utils::*;

#[test]
fn user_is_added_to_whitelist() {
    new_test_ext().execute_with(|| {
        let curator_key = 0;
        let module_key = 1;
        Curators::<Test>::insert(curator_key, ());

        assert_ok!(pallet_governance::Pallet::<Test>::add_to_whitelist(
            get_origin(curator_key),
            module_key
        ));

        assert!(pallet_governance::whitelist::is_whitelisted::<Test>(
            &module_key
        ))
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
