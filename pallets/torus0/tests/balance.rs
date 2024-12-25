use pallet_torus0::{Error, Pallet};
use polkadot_sdk::{
    frame_support::{assert_err, traits::Currency},
    frame_system::RawOrigin,
};
use test_utils::{assert_ok, Balances, Test};

#[test]
fn balance_with_amount_greater_than_0() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;

        let _ = Balances::deposit_creating(&from, 2000);

        assert_eq!(Balances::total_balance(&from), 2000);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_ok!(pallet_torus0::balance::transfer_balance::<Test>(
            from, to, 1000
        ));

        assert_eq!(Balances::total_balance(&from), 1000);
        assert_eq!(Balances::total_balance(&to), 1000);
    });
}

#[test]
fn balance_with_amount_0() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;

        let _ = Balances::deposit_creating(&from, 2000);

        assert_eq!(Balances::total_balance(&from), 2000);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_err!(
            pallet_torus0::balance::transfer_balance::<Test>(from, to, 0),
            Error::<Test>::InvalidAmount,
        );

        assert_eq!(Balances::total_balance(&from), 2000);
        assert_eq!(Balances::total_balance(&to), 0);
    });
}

#[test]
fn balance_with_amount_0_without_balance() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;

        assert_eq!(Balances::total_balance(&from), 0);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_err!(
            pallet_torus0::balance::transfer_balance::<Test>(from, to, 0),
            Error::<Test>::InvalidAmount,
        );

        assert_eq!(Balances::total_balance(&from), 0);
        assert_eq!(Balances::total_balance(&to), 0);
    });
}

#[test]
fn balance_with_amount_greater_than_0_without_balance() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;

        assert_eq!(Balances::total_balance(&from), 0);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_err!(
            pallet_torus0::balance::transfer_balance::<Test>(from, to, 1000),
            Error::<Test>::NotEnoughBalanceToTransfer,
        );

        assert_eq!(Balances::total_balance(&from), 0);
        assert_eq!(Balances::total_balance(&to), 0);
    });
}
