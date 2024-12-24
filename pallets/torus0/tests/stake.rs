use pallet_torus0::{Config, Pallet};
use polkadot_sdk::{frame_support::traits::Currency, frame_system::RawOrigin, sp_core::Get};
use test_utils::Test;

#[test]
fn test_add_stake_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let minimum_stake = <<Test as Config>::DefaultMinimumAllowedStake as Get<u128>>::get();

        assert_eq!(
            Pallet::<Test>::register_agent(
                RawOrigin::Signed(from).into(),
                from,
                "from".as_bytes().to_vec(),
                "from://idk".as_bytes().to_vec(),
                "idk".as_bytes().to_vec()
            ),
            Ok(())
        );

        assert_eq!(
            Pallet::<Test>::register_agent(
                RawOrigin::Signed(to).into(),
                to,
                "to".as_bytes().to_vec(),
                "to://idk".as_bytes().to_vec(),
                "idk".as_bytes().to_vec()
            ),
            Ok(())
        );

        let _ = <Test as Config>::Currency::deposit_creating(&from, minimum_stake * 2);

        assert_eq!(
            <Test as Config>::Currency::total_balance(&from),
            minimum_stake * 2
        );
        assert_eq!(<Test as Config>::Currency::total_balance(&to), 0);

        assert_eq!(
            Pallet::<Test>::add_stake(RawOrigin::Signed(from).into(), to, minimum_stake),
            Ok(())
        );
    });
}
