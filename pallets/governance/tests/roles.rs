use pallet_governance::{Curators, GlobalGovernanceConfig, Whitelist};
use polkadot_sdk::{frame_support::assert_err, sp_runtime::Percent};
use test_utils::*;

use crate::pallet_governance::Error;

fn register(account: AccountId, _unused: u16, module: AccountId, stake: u128) {
    if get_balance(account) < stake {
        add_balance(account, stake);
    }

    let _ = pallet_governance::whitelist::add_to_whitelist::<Test>(module);

    assert_ok!(pallet_torus0::agent::register::<Test>(
        module,
        module,
        b"agent".to_vec(),
        b"url".to_vec(),
        b"metadata".to_vec(),
    ));

    assert!(get_balance(account) >= stake);

    let min_stake: u128 = pallet_torus0::MinAllowedStake::<Test>::get();
    if stake >= min_stake {
        if get_balance(account) - stake < 1 {
            add_balance(account, 1);
        }
        assert_ok!(pallet_torus0::stake::add_stake::<Test>(
            account, module, stake
        ));
    }
}

#[test]
fn add_and_remove_curator() {
    new_test_ext().execute_with(|| {
        let curator_key = 0;

        assert_ok!(pallet_governance::Pallet::<Test>::add_curator(
            RuntimeOrigin::root(),
            curator_key
        ));

        assert!(pallet_governance::Curators::<Test>::contains_key(
            curator_key
        ));

        assert_ok!(pallet_governance::Pallet::<Test>::remove_curator(
            RuntimeOrigin::root(),
            curator_key,
        ));

        assert!(!pallet_governance::Curators::<Test>::contains_key(
            curator_key
        ));
    });
}

#[test]
fn add_and_remove_allocator() {
    new_test_ext().execute_with(|| {
        let allocator_key = 0;

        assert_ok!(pallet_governance::Pallet::<Test>::add_allocator(
            RuntimeOrigin::root(),
            allocator_key
        ));

        assert!(pallet_governance::Allocators::<Test>::contains_key(
            allocator_key
        ));

        assert_ok!(pallet_governance::Pallet::<Test>::remove_allocator(
            RuntimeOrigin::root(),
            allocator_key,
        ));

        assert!(!pallet_governance::Allocators::<Test>::contains_key(
            allocator_key
        ));
    });
}

#[test]
fn add_and_remove_from_whitelist() {
    new_test_ext().execute_with(|| {
        let curator_key = 0;
        let module_key = 1;
        Curators::<Test>::insert(curator_key, ());

        assert_ok!(pallet_governance::Pallet::<Test>::add_to_whitelist(
            get_origin(curator_key),
            module_key
        ));

        assert_err!(
            pallet_governance::Pallet::<Test>::add_to_whitelist(
                get_origin(curator_key),
                module_key
            ),
            Error::<Test>::AlreadyWhitelisted
        );

        assert!(pallet_governance::whitelist::is_whitelisted::<Test>(
            &module_key
        ));

        assert_ok!(pallet_governance::Pallet::<Test>::remove_from_whitelist(
            get_origin(curator_key),
            module_key
        ));

        assert_err!(
            pallet_governance::Pallet::<Test>::remove_from_whitelist(
                get_origin(curator_key),
                module_key
            ),
            Error::<Test>::NotWhitelisted
        );

        assert!(!pallet_governance::whitelist::is_whitelisted::<Test>(
            &module_key
        ));
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

        assert_err!(
            pallet_governance::Pallet::<Test>::remove_from_whitelist(
                get_origin(not_curator_key),
                module_key
            ),
            Error::<Test>::NotCurator
        );
    });
}

#[test]
fn cannot_remove_from_whitelist_if_remove_application_exists() {
    new_test_ext().execute_with(|| {
        let curator_key = 0;
        let module_key = 1;

        Curators::<Test>::insert(curator_key, ());

        Whitelist::<Test>::set(module_key, Some(()));

        let proposal_cost = GlobalGovernanceConfig::<Test>::get().agent_application_cost;
        let data = "test".as_bytes().to_vec();

        add_balance(module_key, proposal_cost + 1);

        pallet_governance::Pallet::<Test>::submit_application(
            get_origin(module_key),
            module_key,
            data,
            true,
        )
        .unwrap();

        assert_err!(
            pallet_governance::Pallet::<Test>::remove_from_whitelist(
                get_origin(curator_key),
                module_key
            ),
            Error::<Test>::ApplicationKeyAlreadyUsed
        );
    });
}

#[test]
fn penalize_agent_successfully() {
    new_test_ext().execute_with(|| {
        let curator_key = 0;
        let module_key = 1;

        Curators::<Test>::insert(curator_key, ());

        register(module_key, 0, module_key, to_nano(100));

        Whitelist::<Test>::set(module_key, Some(()));

        assert_ok!(pallet_governance::Pallet::<Test>::penalize_agent(
            get_origin(curator_key),
            module_key,
            1
        ));

        assert_eq!(
            pallet_torus0::Agents::<Test>::get(module_key)
                .unwrap()
                .weight_penalty_factor,
            Percent::from_percent(1)
        );
    });
}
