use pallet_torus0::{Config, Pallet};
use polkadot_sdk::{frame_support::traits::Currency, frame_system::RawOrigin};
use test_utils::Test;

#[test]
fn test_balance_with_amount_greater_than_0() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;

        let _ = <Test as Config>::Currency::deposit_creating(&from, 2000);

        assert_eq!(<Test as Config>::Currency::total_balance(&from), 2000);
        assert_eq!(<Test as Config>::Currency::total_balance(&to), 0);

        assert_eq!(
            Pallet::<Test>::transfer_balance(RawOrigin::Signed(from).into(), to, 1000),
            Ok(())
        );

        assert_eq!(<Test as Config>::Currency::total_balance(&from), 1000);
        assert_eq!(<Test as Config>::Currency::total_balance(&to), 1000);
    });
}

#[test]
fn test_balance_with_amount_0() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;

        let _ = <Test as Config>::Currency::deposit_creating(&from, 2000);

        assert_eq!(<Test as Config>::Currency::total_balance(&from), 2000);
        assert_eq!(<Test as Config>::Currency::total_balance(&to), 0);

        assert_eq!(
            Pallet::<Test>::transfer_balance(RawOrigin::Signed(from).into(), to, 0),
            Err(pallet_torus0::Error::<Test>::InvalidAmount.into())
        );

        assert_eq!(<Test as Config>::Currency::total_balance(&from), 2000);
        assert_eq!(<Test as Config>::Currency::total_balance(&to), 0);
    });
}

#[test]
fn test_balance_with_amount_0_without_balance() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;

        assert_eq!(<Test as Config>::Currency::total_balance(&from), 0);
        assert_eq!(<Test as Config>::Currency::total_balance(&to), 0);

        assert_eq!(
            Pallet::<Test>::transfer_balance(RawOrigin::Signed(from).into(), to, 0),
            Err(pallet_torus0::Error::<Test>::InvalidAmount.into())
        );

        assert_eq!(<Test as Config>::Currency::total_balance(&from), 0);
        assert_eq!(<Test as Config>::Currency::total_balance(&to), 0);
    });
}

#[test]
fn test_balance_with_amount_greater_than_0_without_balance() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;

        assert_eq!(<Test as Config>::Currency::total_balance(&from), 0);
        assert_eq!(<Test as Config>::Currency::total_balance(&to), 0);

        assert_eq!(
            Pallet::<Test>::transfer_balance(RawOrigin::Signed(from).into(), to, 1000),
            Err(pallet_torus0::Error::<Test>::NotEnoughBalanceToTransfer.into())
        );

        assert_eq!(<Test as Config>::Currency::total_balance(&from), 0);
        assert_eq!(<Test as Config>::Currency::total_balance(&to), 0);
    });
}
