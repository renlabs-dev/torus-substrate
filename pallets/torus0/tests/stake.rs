use pallet_torus0::{Config, Error, Pallet, StakedBy, StakingTo, TotalStake};
use polkadot_sdk::{frame_support::traits::Currency, frame_system::RawOrigin, sp_core::Get};
use test_utils::{Balances, Test};

#[test]
fn test_add_stake_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let minimum_stake = <<Test as Config>::DefaultMinimumAllowedStake as Get<u128>>::get();

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

        let _ = Balances::deposit_creating(&from, minimum_stake * 2);

        assert_eq!(Balances::total_balance(&from), minimum_stake * 2);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_eq!(
            Pallet::<Test>::add_stake(RawOrigin::Signed(from).into(), to, minimum_stake),
            Ok(())
        );

        assert_eq!(Balances::total_balance(&from), minimum_stake);
        assert_eq!(StakingTo::<Test>::get(&from, &to), Some(minimum_stake));
        assert_eq!(StakedBy::<Test>::get(&to, &from), Some(minimum_stake));
        assert_eq!(TotalStake::<Test>::get(), minimum_stake);
    });
}

#[test]
fn test_add_stake_without_registering_the_agent() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let minimum_stake = <<Test as Config>::DefaultMinimumAllowedStake as Get<u128>>::get();

        let _ = Balances::deposit_creating(&from, minimum_stake * 2);

        assert_eq!(Balances::total_balance(&from), minimum_stake * 2);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_eq!(
            Pallet::<Test>::add_stake(RawOrigin::Signed(from).into(), to, minimum_stake),
            Err(Error::<Test>::AgentDoesNotExist.into())
        );

        assert_eq!(Balances::total_balance(&from), minimum_stake * 2);
        assert_eq!(StakingTo::<Test>::get(&from, &to), None);
        assert_eq!(StakedBy::<Test>::get(&to, &from), None);
        assert_eq!(TotalStake::<Test>::get(), 0);
    });
}

#[test]
fn test_add_stake_without_the_minimum_stake() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let minimum_stake = <<Test as Config>::DefaultMinimumAllowedStake as Get<u128>>::get();

        let _ = Balances::deposit_creating(&from, minimum_stake * 2);

        assert_eq!(Balances::total_balance(&from), minimum_stake * 2);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_eq!(
            Pallet::<Test>::add_stake(RawOrigin::Signed(from).into(), to, minimum_stake - 1),
            Err(Error::<Test>::StakeTooSmall.into())
        );

        assert_eq!(Balances::total_balance(&from), minimum_stake * 2);
        assert_eq!(StakingTo::<Test>::get(&from, &to), None);
        assert_eq!(StakedBy::<Test>::get(&to, &from), None);
        assert_eq!(TotalStake::<Test>::get(), 0);
    });
}
