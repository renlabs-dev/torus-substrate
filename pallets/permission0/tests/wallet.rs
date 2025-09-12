use pallet_permission0::{
    PermissionDuration, PermissionId, PermissionScope, Permissions, RevocationTerms,
    ext::wallet_impl::WalletStakeOperation,
    permission::wallet::{WalletScopeType, WalletStake},
};
use polkadot_sdk::frame_support::{assert_err, assert_ok};
use test_utils::*;

pub fn new_test_ext() -> polkadot_sdk::sp_io::TestExternalities {
    new_test_ext_with_block(1)
}

fn setup_agents_with_stake() -> (AccountId, AccountId, AccountId) {
    zero_min_burn();

    let staker = 1;
    let validator1 = 2;
    let validator2 = 3;

    register_empty_agent(staker);
    register_empty_agent(validator1);
    register_empty_agent(validator2);

    add_balance(staker, as_tors(1000));

    assert_ok!(Torus0::add_stake(
        RuntimeOrigin::signed(staker),
        validator1,
        as_tors(100)
    ));

    (staker, validator1, validator2)
}

fn get_last_delegated_permission_id(delegator: AccountId) -> PermissionId {
    System::events()
        .into_iter()
        .filter_map(|record| {
            if let RuntimeEvent::Permission0(pallet_permission0::Event::PermissionDelegated {
                delegator: event_delegator,
                permission_id,
            }) = record.event
            {
                if event_delegator == delegator {
                    Some(permission_id)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next_back()
        .expect("No PermissionDelegated event found")
}

// ============ Delegation Rules Tests ============

#[test]
fn basic_stake_permission_delegation() {
    new_test_ext().execute_with(|| {
        let (staker, _, _) = setup_agents_with_stake();
        let recipient = 10;
        register_empty_agent(recipient);

        assert_err!(
            Permission0::delegate_wallet_stake_permission(
                RuntimeOrigin::signed(staker),
                staker,
                WalletStake {
                    can_transfer_stake: false,
                    exclusive_stake_access: false,
                },
                PermissionDuration::Indefinite,
                RevocationTerms::RevocableByDelegator,
            ),
            pallet_permission0::Error::<Test>::SelfPermissionNotAllowed
        );

        assert_ok!(Permission0::delegate_wallet_stake_permission(
            RuntimeOrigin::signed(staker),
            recipient,
            WalletStake {
                can_transfer_stake: false,
                exclusive_stake_access: false,
            },
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
        ));

        let permission_id = get_last_delegated_permission_id(staker);
        let permission = Permissions::<Test>::get(permission_id).unwrap();

        assert_eq!(permission.delegator, staker);
        if let PermissionScope::Wallet(wallet) = permission.scope {
            assert_eq!(wallet.recipient, recipient);
            #[allow(irrefutable_let_patterns)]
            if let WalletScopeType::Stake(stake) = wallet.r#type {
                assert!(!stake.can_transfer_stake);
                assert!(!stake.exclusive_stake_access);
            } else {
                panic!("Expected Stake type");
            }
        } else {
            panic!("Expected Wallet scope");
        }
    });
}

#[test]
fn exclusive_permission_blocks_new_delegations() {
    new_test_ext().execute_with(|| {
        let (staker, _, _) = setup_agents_with_stake();
        let recipient1 = 10;
        let recipient2 = 11;
        register_empty_agent(recipient1);
        register_empty_agent(recipient2);

        assert_ok!(Permission0::delegate_wallet_stake_permission(
            RuntimeOrigin::signed(staker),
            recipient1,
            WalletStake {
                can_transfer_stake: true,
                exclusive_stake_access: true,
            },
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
        ));

        assert_err!(
            Permission0::delegate_wallet_stake_permission(
                RuntimeOrigin::signed(staker),
                recipient2,
                WalletStake {
                    can_transfer_stake: false,
                    exclusive_stake_access: false,
                },
                PermissionDuration::Indefinite,
                RevocationTerms::RevocableByDelegator,
            ),
            pallet_permission0::Error::<Test>::DuplicatePermission
        );
    });
}

#[test]
fn cannot_delegate_exclusive_after_non_exclusive() {
    new_test_ext().execute_with(|| {
        let (staker, _, _) = setup_agents_with_stake();
        let recipient1 = 10;
        let recipient2 = 11;
        register_empty_agent(recipient1);
        register_empty_agent(recipient2);

        assert_ok!(Permission0::delegate_wallet_stake_permission(
            RuntimeOrigin::signed(staker),
            recipient1,
            WalletStake {
                can_transfer_stake: false,
                exclusive_stake_access: false,
            },
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
        ));

        assert_err!(
            Permission0::delegate_wallet_stake_permission(
                RuntimeOrigin::signed(staker),
                recipient2,
                WalletStake {
                    can_transfer_stake: true,
                    exclusive_stake_access: true,
                },
                PermissionDuration::Indefinite,
                RevocationTerms::RevocableByDelegator,
            ),
            pallet_permission0::Error::<Test>::DuplicatePermission
        );
    });
}

#[test]
fn multiple_non_exclusive_permissions_allowed() {
    new_test_ext().execute_with(|| {
        let (staker, _, _) = setup_agents_with_stake();
        let recipient1 = 10;
        let recipient2 = 11;
        register_empty_agent(recipient1);
        register_empty_agent(recipient2);

        // Delegate first non-exclusive stake permission
        assert_ok!(Permission0::delegate_wallet_stake_permission(
            RuntimeOrigin::signed(staker),
            recipient1,
            WalletStake {
                can_transfer_stake: false,
                exclusive_stake_access: false,
            },
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
        ));

        // Delegate second non-exclusive permission (should succeed)
        assert_ok!(Permission0::delegate_wallet_stake_permission(
            RuntimeOrigin::signed(staker),
            recipient2,
            WalletStake {
                can_transfer_stake: true,
                exclusive_stake_access: false,
            },
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
        ));

        // Verify both permissions exist
        let permissions: Vec<_> = pallet_permission0::PermissionsByDelegator::<Test>::get(staker)
            .into_iter()
            .collect();
        assert_eq!(permissions.len(), 2);
    });
}

// ============ Execution Tests ============

#[test]
fn unstake_operation_through_permission() {
    new_test_ext().execute_with(|| {
        let (staker, validator, _) = setup_agents_with_stake();
        let recipient = 10;
        register_empty_agent(recipient);

        assert_ok!(Permission0::delegate_wallet_stake_permission(
            RuntimeOrigin::signed(staker),
            recipient,
            WalletStake {
                can_transfer_stake: false,
                exclusive_stake_access: false,
            },
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
        ));

        let permission_id = get_last_delegated_permission_id(staker);

        assert_ok!(Permission0::execute_wallet_stake_permission(
            RuntimeOrigin::signed(recipient),
            permission_id,
            WalletStakeOperation::Unstake {
                staked: validator,
                amount: as_tors(50),
            }
        ));

        let stake_amount = pallet_torus0::StakingTo::<Test>::get(staker, validator).unwrap_or(0);
        assert_eq!(stake_amount, as_tors(50));
    });
}

#[test]
fn transfer_stake_operation_requires_permission() {
    new_test_ext().execute_with(|| {
        let (staker, validator1, validator2) = setup_agents_with_stake();
        let recipient = 10;
        register_empty_agent(recipient);

        assert_ok!(Permission0::delegate_wallet_stake_permission(
            RuntimeOrigin::signed(staker),
            recipient,
            WalletStake {
                can_transfer_stake: false,
                exclusive_stake_access: false,
            },
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
        ));
        let permission_id = get_last_delegated_permission_id(staker);

        assert_err!(
            Permission0::execute_wallet_stake_permission(
                RuntimeOrigin::signed(recipient),
                permission_id,
                WalletStakeOperation::Transfer {
                    from: validator1,
                    to: validator2,
                    amount: as_tors(30),
                }
            ),
            pallet_permission0::Error::<Test>::PermissionNotFound
        );

        assert_ok!(Permission0::delegate_wallet_stake_permission(
            RuntimeOrigin::signed(staker),
            recipient,
            WalletStake {
                can_transfer_stake: true,
                exclusive_stake_access: false,
            },
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
        ));
        let permission_id = get_last_delegated_permission_id(staker);

        assert_ok!(Permission0::execute_wallet_stake_permission(
            RuntimeOrigin::signed(recipient),
            permission_id,
            WalletStakeOperation::Transfer {
                from: validator1,
                to: validator2,
                amount: as_tors(30),
            }
        ));
    });
}

#[test]
fn transfer_stake_with_permission() {
    new_test_ext().execute_with(|| {
        let (staker, validator1, validator2) = setup_agents_with_stake();
        let recipient = 10;
        register_empty_agent(recipient);

        assert_ok!(Permission0::delegate_wallet_stake_permission(
            RuntimeOrigin::signed(staker),
            recipient,
            WalletStake {
                can_transfer_stake: true,
                exclusive_stake_access: false,
            },
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
        ));

        let permission_id = get_last_delegated_permission_id(staker);

        assert_ok!(Permission0::execute_wallet_stake_permission(
            RuntimeOrigin::signed(recipient),
            permission_id,
            WalletStakeOperation::Transfer {
                from: validator1,
                to: validator2,
                amount: as_tors(30),
            }
        ));

        let stake1 = pallet_torus0::StakingTo::<Test>::get(staker, validator1).unwrap_or(0);
        let stake2 = pallet_torus0::StakingTo::<Test>::get(staker, validator2).unwrap_or(0);
        assert_eq!(stake1, as_tors(70));
        assert_eq!(stake2, as_tors(30));
    });
}

#[test]
fn only_recipient_can_execute_permission() {
    new_test_ext().execute_with(|| {
        let (staker, validator, _) = setup_agents_with_stake();
        let recipient = 10;
        let other_user = 11;
        register_empty_agent(recipient);
        register_empty_agent(other_user);

        // Delegate stake permission
        assert_ok!(Permission0::delegate_wallet_stake_permission(
            RuntimeOrigin::signed(staker),
            recipient,
            WalletStake {
                can_transfer_stake: false,
                exclusive_stake_access: false,
            },
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
        ));

        let permission_id = get_last_delegated_permission_id(staker);

        // Try to execute with wrong account
        assert_err!(
            Permission0::execute_wallet_stake_permission(
                RuntimeOrigin::signed(other_user),
                permission_id,
                WalletStakeOperation::Unstake {
                    staked: validator,
                    amount: as_tors(50),
                }
            ),
            pallet_permission0::Error::<Test>::NotPermissionRecipient
        );

        // Execute with correct recipient
        assert_ok!(Permission0::execute_wallet_stake_permission(
            RuntimeOrigin::signed(recipient),
            permission_id,
            WalletStakeOperation::Unstake {
                staked: validator,
                amount: as_tors(50),
            }
        ));
    });
}

#[test]
fn permission_not_found_error() {
    new_test_ext().execute_with(|| {
        let (_, validator, _) = setup_agents_with_stake();
        let recipient = 10;
        register_empty_agent(recipient);

        let invalid_permission_id = [1u8; 32].into();

        assert_err!(
            Permission0::execute_wallet_stake_permission(
                RuntimeOrigin::signed(recipient),
                invalid_permission_id,
                WalletStakeOperation::Unstake {
                    staked: validator,
                    amount: as_tors(50),
                }
            ),
            pallet_permission0::Error::<Test>::PermissionNotFound
        );
    });
}

#[test]
fn revoke_wallet_permission() {
    new_test_ext().execute_with(|| {
        let (staker, validator, _) = setup_agents_with_stake();
        let recipient = 10;
        register_empty_agent(recipient);

        // Delegate stake permission
        assert_ok!(Permission0::delegate_wallet_stake_permission(
            RuntimeOrigin::signed(staker),
            recipient,
            WalletStake {
                can_transfer_stake: false,
                exclusive_stake_access: false,
            },
            PermissionDuration::Indefinite,
            RevocationTerms::RevocableByDelegator,
        ));

        let permission_id = get_last_delegated_permission_id(staker);

        // Revoke permission
        assert_ok!(Permission0::revoke_permission(
            RuntimeOrigin::signed(staker),
            permission_id
        ));

        // Try to use revoked permission
        assert_err!(
            Permission0::execute_wallet_stake_permission(
                RuntimeOrigin::signed(recipient),
                permission_id,
                WalletStakeOperation::Unstake {
                    staked: validator,
                    amount: as_tors(50),
                }
            ),
            pallet_permission0::Error::<Test>::PermissionNotFound
        );
    });
}
