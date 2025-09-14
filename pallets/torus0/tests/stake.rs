use pallet_torus0::{
    Error, MinAllowedStake, StakedBy, StakingTo, TotalStake, stake::STAKE_IDENTIFIER,
};
use polkadot_sdk::frame_support::{
    assert_err,
    traits::{Currency, NamedReservableCurrency},
};
use test_utils::{
    Balances, Permission0, Test, account, as_tors, assert_ok, get_origin, pallet_governance,
    pallet_permission0,
};

#[test]
fn add_stake_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let stake_to_add = MinAllowedStake::<Test>::get();
        let total_balance = stake_to_add * 2;
        let balance_after = stake_to_add;

        let from = account!(id = 0, bal = total_balance);
        let to = account!(id = 1, agent = "bob", whitelist);

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_ok!(pallet_torus0::Pallet::<Test>::add_stake(
            get_origin(from),
            to,
            stake_to_add
        ));

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(Balances::free_balance(from), balance_after);
        assert_eq!(
            Balances::reserved_balance_named(STAKE_IDENTIFIER, &from),
            balance_after
        );

        assert_eq!(StakingTo::<Test>::get(from, to), Some(stake_to_add));
        assert_eq!(StakedBy::<Test>::get(to, from), Some(stake_to_add));
        assert_eq!(TotalStake::<Test>::get(), stake_to_add);
    });
}

#[test]
fn transfer_stake_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let stake_to_add = MinAllowedStake::<Test>::get();
        let total_balance = stake_to_add * 2;
        let balance_after = stake_to_add;

        let staker = account!(id = 0, bal = total_balance);
        let old_staked = account!(id = 1, agent = "bob", whitelist);
        let new_staked = account!(id = 2, agent = "charlie", whitelist);

        assert_eq!(Balances::total_balance(&staker), total_balance);
        assert_eq!(Balances::total_balance(&old_staked), 0);

        assert_ok!(pallet_torus0::Pallet::<Test>::add_stake(
            get_origin(staker),
            old_staked,
            stake_to_add
        ));

        assert_eq!(Balances::total_balance(&staker), total_balance);
        assert_eq!(Balances::free_balance(staker), balance_after);
        assert_eq!(
            Balances::reserved_balance_named(STAKE_IDENTIFIER, &staker),
            stake_to_add
        );

        assert_eq!(
            StakingTo::<Test>::get(staker, old_staked),
            Some(stake_to_add)
        );
        assert_eq!(
            StakedBy::<Test>::get(old_staked, staker),
            Some(stake_to_add)
        );
        assert_eq!(TotalStake::<Test>::get(), stake_to_add);

        assert_ok!(pallet_torus0::Pallet::<Test>::transfer_stake(
            get_origin(staker),
            old_staked,
            new_staked,
            stake_to_add
        ));

        assert_eq!(Balances::total_balance(&staker), total_balance);
        assert_eq!(Balances::free_balance(staker), balance_after);
        assert_eq!(
            Balances::reserved_balance_named(STAKE_IDENTIFIER, &staker),
            stake_to_add
        );

        assert_eq!(StakingTo::<Test>::get(staker, old_staked), Some(0));
        assert_eq!(StakedBy::<Test>::get(old_staked, staker), Some(0));

        assert_eq!(
            StakingTo::<Test>::get(staker, new_staked),
            Some(stake_to_add)
        );
        assert_eq!(
            StakedBy::<Test>::get(new_staked, staker),
            Some(stake_to_add)
        );

        assert_eq!(TotalStake::<Test>::get(), stake_to_add);
    });
}

#[test]
fn add_stake_without_registering_the_agent() {
    test_utils::new_test_ext().execute_with(|| {
        let stake_to_add = MinAllowedStake::<Test>::get();
        let total_balance = stake_to_add * 2;

        let from = account!(id = 0, bal = total_balance);
        let to = account!(id = 1);

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
        let stake_to_add = MinAllowedStake::<Test>::get();
        let total_balance = stake_to_add * 2;

        let from = account!(id = 0, bal = total_balance);
        let to = account!(id = 1, agent = "bob", whitelist);

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
        let stake_to_add_and_remove = MinAllowedStake::<Test>::get();
        let total_balance = stake_to_add_and_remove * 2;

        let from = account!(id = 0, bal = total_balance);
        let to = account!(id = 1, agent = "bob", whitelist);
        assert_eq!(Balances::total_balance(&to), 0);

        assert_ok!(pallet_torus0::Pallet::<Test>::add_stake(
            get_origin(from),
            to,
            stake_to_add_and_remove
        ));

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(
            Balances::free_balance(from),
            total_balance - stake_to_add_and_remove
        );

        assert_ok!(pallet_torus0::Pallet::<Test>::remove_stake(
            get_origin(from),
            to,
            stake_to_add_and_remove
        ));

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(Balances::free_balance(from), total_balance);

        assert_eq!(StakingTo::<Test>::get(from, to), Some(0));
        assert_eq!(StakedBy::<Test>::get(to, from), Some(0));
        assert_eq!(TotalStake::<Test>::get(), 0);
    });
}

#[test]
fn remove_stake_with_deregistered_agent() {
    test_utils::new_test_ext().execute_with(|| {
        let stake = as_tors(500);
        let total_balance = stake * 2;

        let from = account!(id = 0, bal = total_balance);
        let to = account!(id = 1, agent = "bob", whitelist);

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(Balances::total_balance(&to), 0);
        assert_ok!(pallet_torus0::Pallet::<Test>::add_stake(
            get_origin(from),
            to,
            stake,
        ));

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(Balances::free_balance(from), total_balance / 2);

        assert_eq!(TotalStake::<Test>::get(), stake);
        assert_eq!(StakingTo::<Test>::get(from, to), Some(stake));
        assert_eq!(StakedBy::<Test>::get(to, from), Some(stake));

        assert_ok!(pallet_torus0::Pallet::<Test>::deregister_agent(get_origin(
            to
        )));

        assert_err!(
            pallet_torus0::Pallet::<Test>::remove_stake(get_origin(from), to, stake),
            Error::<Test>::AgentDoesNotExist,
        );

        assert_eq!(Balances::total_balance(&from), total_balance);
        assert_eq!(Balances::free_balance(from), total_balance);

        assert_eq!(StakingTo::<Test>::get(from, to), None);
        assert_eq!(StakedBy::<Test>::get(to, from), None);
        assert_eq!(TotalStake::<Test>::get(), 0);
    });
}

#[test]
fn exclusive_wallet_permission_blocks_stake_operations() {
    test_utils::new_test_ext().execute_with(|| {
        let stake_amount = MinAllowedStake::<Test>::get();
        let total_balance = stake_amount * 3;

        let staker = account!(id = 0, agent = "alice", bal = total_balance);
        let validator1 = account!(id = 1, agent = "bob", whitelist);
        let validator2 = account!(id = 2, agent = "charlie", whitelist);
        let recipient = account!(id = 3, agent = "dave", whitelist);

        assert_ok!(pallet_torus0::Pallet::<Test>::add_stake(
            get_origin(staker),
            validator1,
            stake_amount
        ));

        assert_ok!(Permission0::delegate_wallet_stake_permission(
            get_origin(staker),
            recipient,
            pallet_permission0::permission::wallet::WalletStake {
                can_transfer_stake: true,
                exclusive_stake_access: true,
            },
            pallet_permission0::PermissionDuration::Indefinite,
            pallet_permission0::RevocationTerms::RevocableByDelegator,
        ));

        assert_err!(
            pallet_torus0::Pallet::<Test>::remove_stake(
                get_origin(staker),
                validator1,
                stake_amount / 2
            ),
            Error::<Test>::StakeIsDelegated
        );

        assert_err!(
            pallet_torus0::Pallet::<Test>::transfer_stake(
                get_origin(staker),
                validator1,
                validator2,
                stake_amount / 2
            ),
            Error::<Test>::StakeIsDelegated
        );

        assert_eq!(
            StakingTo::<Test>::get(staker, validator1),
            Some(stake_amount)
        );
        assert_eq!(StakingTo::<Test>::get(staker, validator2), None);
    });
}

#[test]
fn non_exclusive_wallet_permission_allows_stake_operations() {
    test_utils::new_test_ext().execute_with(|| {
        let stake_amount = MinAllowedStake::<Test>::get();
        let total_balance = stake_amount * 3;

        let staker = account!(id = 0, agent = "alice", bal = total_balance);
        let validator1 = account!(id = 1, agent = "bob", whitelist);
        let validator2 = account!(id = 2, agent = "charlie", whitelist);
        let recipient = account!(id = 3, agent = "dave", whitelist);

        assert_ok!(pallet_torus0::Pallet::<Test>::add_stake(
            get_origin(staker),
            validator1,
            stake_amount
        ));

        assert_ok!(Permission0::delegate_wallet_stake_permission(
            get_origin(staker),
            recipient,
            pallet_permission0::permission::wallet::WalletStake {
                can_transfer_stake: false,
                exclusive_stake_access: false,
            },
            pallet_permission0::PermissionDuration::Indefinite,
            pallet_permission0::RevocationTerms::RevocableByDelegator,
        ));

        assert_ok!(pallet_torus0::Pallet::<Test>::remove_stake(
            get_origin(staker),
            validator1,
            stake_amount / 2
        ));

        assert_eq!(
            StakingTo::<Test>::get(staker, validator1),
            Some(stake_amount / 2)
        );

        assert_ok!(pallet_torus0::Pallet::<Test>::transfer_stake(
            get_origin(staker),
            validator1,
            validator2,
            stake_amount / 4
        ));

        assert_eq!(
            StakingTo::<Test>::get(staker, validator1),
            Some(stake_amount / 4)
        );
        assert_eq!(
            StakingTo::<Test>::get(staker, validator2),
            Some(stake_amount / 4)
        );
    });
}
