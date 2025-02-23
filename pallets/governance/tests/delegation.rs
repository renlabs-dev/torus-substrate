use pallet_governance::NotDelegatingVotingPower;
use polkadot_sdk::frame_support::assert_ok;
use test_utils::{get_origin, new_test_ext, Test};

#[test]
fn delegates_voting_power_correctly() {
    new_test_ext().execute_with(|| {
        const MODULE: u32 = 0;

        assert_ok!(pallet_governance::Pallet::<Test>::enable_vote_delegation(
            get_origin(MODULE)
        ));

        assert!(!NotDelegatingVotingPower::<Test>::get().contains(&MODULE));
    });
}

#[test]
fn disable_voting_power_delegation_correctly() {
    new_test_ext().execute_with(|| {
        const MODULE: u32 = 0;

        assert_ok!(pallet_governance::Pallet::<Test>::enable_vote_delegation(
            get_origin(MODULE)
        ));

        assert_ok!(pallet_governance::Pallet::<Test>::disable_vote_delegation(
            get_origin(MODULE)
        ));

        assert!(NotDelegatingVotingPower::<Test>::get().contains(&MODULE));
    });
}
