use pallet_torus0::{Config, Error, Pallet, StakedBy, StakingTo, TotalStake};
use polkadot_sdk::{
    frame_support::{assert_err, traits::Currency},
    frame_system::RawOrigin,
    sp_core::Get,
};
use test_utils::{assert_ok, Balances, Test};

#[test]
fn test_add_stake_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let minimum_stake = <<Test as Config>::DefaultMinimumAllowedStake as Get<u128>>::get();

        assert_ok!(Pallet::<Test>::register_agent(
            RawOrigin::Signed(to).into(),
            to,
            "to".as_bytes().to_vec(),
            "to://idk".as_bytes().to_vec(),
            "idk".as_bytes().to_vec()
        ));

        let _ = Balances::deposit_creating(&from, minimum_stake * 2);

        assert_eq!(Balances::total_balance(&from), minimum_stake * 2);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_ok!(Pallet::<Test>::add_stake(
            RawOrigin::Signed(from).into(),
            to,
            minimum_stake
        ));

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

        assert_err!(
            Pallet::<Test>::add_stake(RawOrigin::Signed(from).into(), to, minimum_stake),
            Error::<Test>::AgentDoesNotExist
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

        assert_err!(
            Pallet::<Test>::add_stake(RawOrigin::Signed(from).into(), to, minimum_stake - 1),
            Error::<Test>::StakeTooSmall
        );

        assert_eq!(Balances::total_balance(&from), minimum_stake * 2);
        assert_eq!(StakingTo::<Test>::get(&from, &to), None);
        assert_eq!(StakedBy::<Test>::get(&to, &from), None);
        assert_eq!(TotalStake::<Test>::get(), 0);
    });
}

#[test]
fn test_remove_stake_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let minimum_stake = <<Test as Config>::DefaultMinimumAllowedStake as Get<u128>>::get();

        assert_ok!(Pallet::<Test>::register_agent(
            RawOrigin::Signed(to).into(),
            to,
            "to".as_bytes().to_vec(),
            "to://idk".as_bytes().to_vec(),
            "idk".as_bytes().to_vec()
        ));

        let _ = Balances::deposit_creating(&from, minimum_stake * 2);

        assert_eq!(Balances::total_balance(&to), 0);

        assert_ok!(pallet_torus0::stake::add_stake::<Test>(
            from,
            to,
            minimum_stake
        ));

        assert_ok!(Pallet::<Test>::remove_stake(
            RawOrigin::Signed(from).into(),
            to,
            minimum_stake
        ));

        assert_eq!(Balances::total_balance(&from), minimum_stake * 2);
        assert_eq!(StakingTo::<Test>::get(&from, &to), Some(0));
        assert_eq!(StakedBy::<Test>::get(&to, &from), Some(0));
        assert_eq!(TotalStake::<Test>::get(), 0);
    });
}

#[test]
fn test_remove_stake_with_less_than_required_amount() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let minimum_stake = <<Test as Config>::DefaultMinimumAllowedStake as Get<u128>>::get();

        assert_ok!(Pallet::<Test>::register_agent(
            RawOrigin::Signed(to).into(),
            to,
            "to".as_bytes().to_vec(),
            "to://idk".as_bytes().to_vec(),
            "idk".as_bytes().to_vec()
        ));

        let _ = Balances::deposit_creating(&from, minimum_stake * 2);

        assert_eq!(Balances::total_balance(&to), 0);

        assert_ok!(pallet_torus0::stake::add_stake::<Test>(
            from,
            to,
            minimum_stake
        ));

        assert_err!(
            Pallet::<Test>::remove_stake(RawOrigin::Signed(from).into(), to, minimum_stake - 1),
            Error::<Test>::StakeTooSmall,
        );

        assert_eq!(Balances::total_balance(&from), minimum_stake);
        assert_eq!(StakingTo::<Test>::get(&from, &to), Some(minimum_stake));
        assert_eq!(StakedBy::<Test>::get(&to, &from), Some(minimum_stake));
        assert_eq!(TotalStake::<Test>::get(), minimum_stake);
    });
}

#[test]
fn test_remove_stake_with_unregistered_agent() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let minimum_stake = <<Test as Config>::DefaultMinimumAllowedStake as Get<u128>>::get();

        assert_ok!(Pallet::<Test>::register_agent(
            RawOrigin::Signed(to).into(),
            to,
            "to".as_bytes().to_vec(),
            "to://idk".as_bytes().to_vec(),
            "idk".as_bytes().to_vec()
        ));

        let _ = Balances::deposit_creating(&from, minimum_stake * 2);

        assert_eq!(Balances::total_balance(&to), 0);

        assert_ok!(pallet_torus0::stake::add_stake::<Test>(
            from,
            to,
            minimum_stake
        ));
        assert_ok!(pallet_torus0::agent::unregister::<Test>(to));

        assert_err!(
            Pallet::<Test>::remove_stake(RawOrigin::Signed(from).into(), to, minimum_stake),
            Error::<Test>::AgentDoesNotExist,
        );

        assert_eq!(Balances::total_balance(&from), minimum_stake);
        assert_eq!(StakingTo::<Test>::get(&from, &to), None);
        assert_eq!(StakedBy::<Test>::get(&to, &from), None);
        assert_eq!(TotalStake::<Test>::get(), minimum_stake);
    });
}
