use pallet_torus0::{Error, MinAllowedStake, StakedBy, StakingTo, TotalStake};
use polkadot_sdk::frame_support::{assert_err, traits::Currency};
use test_utils::{as_tors, assert_ok, get_origin, pallet_governance, Balances, Test};

#[test]
fn add_stake_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let stake_to_add = MinAllowedStake::<Test>::get();
        let total_balance = stake_to_add * 2;
        let balance_after = stake_to_add;

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(to));

        assert_ok!(pallet_torus0::agent::register::<Test>(
            to,
            to,
            "to".as_bytes().to_vec(),
            "to://idk".as_bytes().to_vec(),
            "idk".as_bytes().to_vec()
        ));

        let _ = Balances::deposit_creating(&from, total_balance);

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_ok!(pallet_torus0::Pallet::<Test>::add_stake(
            get_origin(from),
            to,
            stake_to_add
        ));

        assert_eq!(Balances::total_balance(&from), balance_after);
        assert_eq!(StakingTo::<Test>::get(from, to), Some(stake_to_add));
        assert_eq!(StakedBy::<Test>::get(to, from), Some(stake_to_add));
        assert_eq!(TotalStake::<Test>::get(), stake_to_add);
    });
}

#[test]
fn transfer_stake_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let transfer = 2;
        let stake_to_add = MinAllowedStake::<Test>::get();
        let total_balance = stake_to_add * 2;
        let balance_after = stake_to_add;

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(to));
        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            transfer
        ));

        assert_ok!(pallet_torus0::agent::register::<Test>(
            to,
            to,
            "to".as_bytes().to_vec(),
            "to://idk".as_bytes().to_vec(),
            "idk".as_bytes().to_vec()
        ));

        assert_ok!(pallet_torus0::agent::register::<Test>(
            transfer,
            transfer,
            "transfer".as_bytes().to_vec(),
            "transfer://idk".as_bytes().to_vec(),
            "idk".as_bytes().to_vec()
        ));

        let _ = Balances::deposit_creating(&from, total_balance);

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_ok!(pallet_torus0::Pallet::<Test>::add_stake(
            get_origin(from),
            to,
            stake_to_add
        ));

        assert_eq!(Balances::total_balance(&from), balance_after);
        assert_eq!(StakingTo::<Test>::get(from, to), Some(stake_to_add));
        assert_eq!(StakedBy::<Test>::get(to, from), Some(stake_to_add));
        assert_eq!(TotalStake::<Test>::get(), stake_to_add);

        assert_ok!(pallet_torus0::Pallet::<Test>::transfer_stake(
            get_origin(from),
            to,
            transfer,
            stake_to_add
        ));

        assert_eq!(StakingTo::<Test>::get(from, to), Some(0));
        assert_eq!(StakingTo::<Test>::get(from, transfer), Some(stake_to_add));
        assert_eq!(StakedBy::<Test>::get(to, from), Some(0));
        assert_eq!(StakedBy::<Test>::get(transfer, from), Some(stake_to_add));
        assert_eq!(TotalStake::<Test>::get(), stake_to_add);
    });
}

#[test]
fn add_stake_without_registering_the_agent() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let stake_to_add = MinAllowedStake::<Test>::get();
        let total_balance = stake_to_add * 2;

        let _ = Balances::deposit_creating(&from, total_balance);

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_err!(
            pallet_torus0::Pallet::<Test>::add_stake(get_origin(from), to, stake_to_add),
            Error::<Test>::AgentDoesNotExist
        );

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(StakingTo::<Test>::get(from, to), None);
        assert_eq!(StakedBy::<Test>::get(to, from), None);
        assert_eq!(TotalStake::<Test>::get(), 0);
    });
}

#[test]
fn add_stake_without_the_minimum_stake() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let stake_to_add = MinAllowedStake::<Test>::get();
        let total_balance = stake_to_add * 2;

        let _ = Balances::deposit_creating(&from, total_balance);

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_err!(
            pallet_torus0::Pallet::<Test>::add_stake(get_origin(from), to, stake_to_add - 1),
            Error::<Test>::StakeTooSmall
        );

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(StakingTo::<Test>::get(from, to), None);
        assert_eq!(StakedBy::<Test>::get(to, from), None);
        assert_eq!(TotalStake::<Test>::get(), 0);
    });
}

#[test]
fn remove_stake_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;
        let stake_to_add_and_remove = MinAllowedStake::<Test>::get();
        let total_balance = stake_to_add_and_remove * 2;

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(to));

        assert_ok!(pallet_torus0::agent::register::<Test>(
            to,
            to,
            "to".as_bytes().to_vec(),
            "to://idk".as_bytes().to_vec(),
            "idk".as_bytes().to_vec()
        ));

        let _ = Balances::deposit_creating(&from, total_balance);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_ok!(pallet_torus0::Pallet::<Test>::add_stake(
            get_origin(from),
            to,
            stake_to_add_and_remove
        ));

        assert_ok!(pallet_torus0::Pallet::<Test>::remove_stake(
            get_origin(from),
            to,
            stake_to_add_and_remove
        ));

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(StakingTo::<Test>::get(from, to), Some(0));
        assert_eq!(StakedBy::<Test>::get(to, from), Some(0));
        assert_eq!(TotalStake::<Test>::get(), 0);
    });
}

#[test]
fn remove_stake_with_unregistered_agent() {
    test_utils::new_test_ext().execute_with(|| {
        let from = 0;
        let to = 1;

        let stake = as_tors(500);
        let total_balance = stake * 2;

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(to));
        assert_ok!(pallet_torus0::Pallet::<Test>::register_agent(
            get_origin(to),
            to,
            b"to".to_vec(),
            b"to://idk".to_vec(),
            b"idk".to_vec()
        ));

        let _ = Balances::deposit_creating(&from, total_balance);

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(Balances::total_balance(&to), 0);
        assert_ok!(pallet_torus0::Pallet::<Test>::add_stake(
            get_origin(from),
            to,
            stake,
        ));

        assert_eq!(Balances::total_balance(&from), total_balance / 2);

        assert_eq!(TotalStake::<Test>::get(), stake);
        assert_eq!(StakingTo::<Test>::get(from, to), Some(stake));
        assert_eq!(StakedBy::<Test>::get(to, from), Some(stake));

        assert_ok!(pallet_torus0::Pallet::<Test>::unregister_agent(get_origin(
            to
        )));

        assert_err!(
            pallet_torus0::Pallet::<Test>::remove_stake(get_origin(from), to, stake),
            Error::<Test>::AgentDoesNotExist,
        );

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(StakingTo::<Test>::get(from, to), None);
        assert_eq!(StakedBy::<Test>::get(to, from), None);
        assert_eq!(TotalStake::<Test>::get(), 0);
    });
}
