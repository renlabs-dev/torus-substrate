use polkadot_sdk::{
    frame_support::assert_err,
    sp_runtime::{Percent, traits::BadOrigin},
};
use test_utils::{RuntimeOrigin, Test, assert_ok, get_origin, new_test_ext};

#[test]
fn set_emission_params() {
    new_test_ext().execute_with(|| {
        assert_ok!(pallet_governance::Pallet::<Test>::set_emission_params(
            RuntimeOrigin::root(),
            Percent::from_percent(15),
            Percent::from_percent(15),
        ));

        assert_eq!(
            pallet_emission0::EmissionRecyclingPercentage::<Test>::get(),
            Percent::from_percent(15)
        );

        assert_eq!(
            pallet_governance::TreasuryEmissionFee::<Test>::get(),
            Percent::from_percent(15)
        );
    });
}

#[test]
fn set_emission_params_non_root() {
    new_test_ext().execute_with(|| {
        assert_err!(
            pallet_governance::Pallet::<Test>::set_emission_params(
                get_origin(0),
                Percent::from_percent(15),
                Percent::from_percent(15),
            ),
            BadOrigin
        );
    });
}
