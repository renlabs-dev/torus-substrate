#![allow(clippy::indexing_slicing)]

use std::collections::BTreeMap;

use pallet_permission0::permission::emission::StreamId;
use pallet_permission0::{EnforcementAuthority, EnforcementReferendum};
use pallet_permission0_api::{Permission0EmissionApi, generate_root_stream_id};
use polkadot_sdk::frame_support::{assert_err, traits::Currency};
use polkadot_sdk::sp_core::bounded_vec;
use polkadot_sdk::sp_runtime::Percent;
use test_utils::*;

fn stream_percentages(agent: AccountId, percentage: u8) -> BTreeMap<StreamId, Percent> {
    BTreeMap::from([(
        generate_root_stream_id(&agent),
        Percent::from_percent(percentage),
    )])
}

fn distribute_emission(agent: AccountId, amount: Balance) {
    let stream_id = generate_root_stream_id(&agent);
    let mut imbalance = Balances::issue(amount);
    Permission0::accumulate_emissions(&agent, &stream_id, &mut imbalance);
}

#[test]
fn set_enforcement_authority_by_delegator() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let delegator = 0;
        let recipient = 1;
        let controller = 2;

        register_empty_agent(delegator);
        register_empty_agent(recipient);
        register_empty_agent(controller);

        add_balance(delegator, as_tors(10) + 1);

        let permission_id = assert_ok!(delegate_emission_permission(
            delegator,
            vec![(recipient, u16::MAX)],
            pallet_permission0_api::EmissionAllocation::FixedAmount(as_tors(10)),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert_err!(
            pallet_permission0::Pallet::<Test>::set_enforcement_authority(
                get_origin(recipient),
                permission_id,
                EnforcementAuthority::ControlledBy {
                    controllers: bounded_vec![controller],
                    required_votes: 1
                }
            ),
            pallet_permission0::Error::<Test>::NotPermissionDelegator
        );

        assert_err!(
            pallet_permission0::Pallet::<Test>::set_enforcement_authority(
                get_origin(controller),
                permission_id,
                EnforcementAuthority::ControlledBy {
                    controllers: bounded_vec![controller],
                    required_votes: 1
                }
            ),
            pallet_permission0::Error::<Test>::NotPermissionDelegator
        );

        assert_ok!(
            pallet_permission0::Pallet::<Test>::set_enforcement_authority(
                get_origin(delegator),
                permission_id,
                EnforcementAuthority::ControlledBy {
                    controllers: bounded_vec![controller],
                    required_votes: 1
                }
            )
        );

        let contract = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();

        match contract.enforcement {
            pallet_permission0::EnforcementAuthority::ControlledBy {
                controllers,
                required_votes,
            } => {
                assert_eq!(controllers.len(), 1);
                assert_eq!(controllers[0], controller);
                assert_eq!(required_votes, 1);
            }
            _ => panic!("Enforcement authority not set correctly"),
        }
    });
}

#[test]
fn toggle_accumulation_by_controller() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let delegator = 0;
        let recipient = 1;
        let controller = 2;

        register_empty_agent(delegator);
        register_empty_agent(recipient);
        register_empty_agent(controller);

        add_balance(delegator, as_tors(100) + 1);

        let permission_id = assert_ok!(delegate_emission_permission(
            delegator,
            vec![(recipient, u16::MAX)],
            pallet_permission0_api::EmissionAllocation::Streams(stream_percentages(delegator, 100)),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::ControlledBy {
                controllers: vec![controller],
                required_votes: 1
            },
        ));

        distribute_emission(delegator, as_tors(10));

        assert_ok!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(controller),
                permission_id,
                false,
            )
        );

        let contract = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        if let pallet_permission0::PermissionScope::Emission(emission_scope) = contract.scope {
            assert!(!emission_scope.accumulating);
        }

        let balance_before = get_balance(recipient);
        distribute_emission(delegator, as_tors(10));

        assert_eq!(get_balance(recipient), balance_before);

        assert_ok!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(controller),
                permission_id,
                true,
            )
        );

        distribute_emission(delegator, as_tors(10));

        assert_ok!(pallet_permission0::Pallet::<Test>::execute_permission(
            get_origin(delegator),
            permission_id,
        ));

        assert!(get_balance(recipient) > balance_before);
    });
}

#[test]
fn unauthorized_account_cannot_toggle() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let delegator = 0;
        let recipient = 1;
        let controller = 2;
        let unauthorized = 3;

        register_empty_agent(delegator);
        register_empty_agent(recipient);
        register_empty_agent(controller);
        register_empty_agent(unauthorized);

        add_balance(delegator, as_tors(10) + 1);

        let permission_id = assert_ok!(delegate_emission_permission(
            delegator,
            vec![(recipient, u16::MAX)],
            pallet_permission0_api::EmissionAllocation::FixedAmount(as_tors(10)),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::ControlledBy {
                controllers: vec![controller],
                required_votes: 1
            },
        ));

        assert_err!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(unauthorized),
                permission_id,
                false,
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToToggle
        );

        assert_ok!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(delegator),
                permission_id,
                false,
            ),
        );
    });
}

#[test]
fn enforcement_execute_permission() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let delegator = 0;
        let recipient = 1;
        let controller = 2;

        register_empty_agent(delegator);
        register_empty_agent(recipient);
        register_empty_agent(controller);

        add_balance(delegator, as_tors(100) + 1);

        let permission_id = assert_ok!(delegate_emission_permission(
            delegator,
            vec![(recipient, u16::MAX)],
            pallet_permission0_api::EmissionAllocation::Streams(stream_percentages(delegator, 100)),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::ControlledBy {
                controllers: vec![controller],
                required_votes: 1
            },
        ));

        distribute_emission(delegator, as_tors(10));

        let balance_before = get_balance(recipient);

        assert_ok!(
            pallet_permission0::Pallet::<Test>::enforcement_execute_permission(
                get_origin(controller),
                permission_id,
            )
        );

        assert!(get_balance(recipient) > balance_before);
    });
}

#[test]
fn unauthorized_cannot_enforcement_execute() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let delegator = 0;
        let recipient = 1;
        let controller = 2;
        let unauthorized = 3;

        register_empty_agent(delegator);
        register_empty_agent(recipient);
        register_empty_agent(controller);
        register_empty_agent(unauthorized);

        add_balance(delegator, as_tors(100) + 1);

        let permission_id = assert_ok!(delegate_emission_permission(
            delegator,
            vec![(recipient, u16::MAX)],
            pallet_permission0_api::EmissionAllocation::Streams(stream_percentages(delegator, 100)),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::ControlledBy {
                controllers: vec![controller],
                required_votes: 1
            },
        ));

        distribute_emission(delegator, as_tors(10));

        assert_err!(
            pallet_permission0::Pallet::<Test>::enforcement_execute_permission(
                get_origin(unauthorized),
                permission_id,
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToToggle
        );
    });
}

#[test]
fn multi_controller_voting() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let delegator = 0;
        let recipient = 1;
        let controller1 = 2;
        let controller2 = 3;
        let controller3 = 4;

        register_empty_agent(delegator);
        register_empty_agent(recipient);
        register_empty_agent(controller1);
        register_empty_agent(controller2);
        register_empty_agent(controller3);

        add_balance(delegator, as_tors(100) + 1);

        let permission_id = assert_ok!(delegate_emission_permission(
            delegator,
            vec![(recipient, u16::MAX)],
            pallet_permission0_api::EmissionAllocation::Streams(stream_percentages(delegator, 100)),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::ControlledBy {
                controllers: vec![controller1, controller2, controller3],
                required_votes: 2
            },
        ));

        distribute_emission(delegator, as_tors(10));

        assert_ok!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(controller1),
                permission_id,
                false,
            )
        );

        let contract = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        if let pallet_permission0::PermissionScope::Emission(emission_scope) = contract.scope {
            assert!(emission_scope.accumulating);
        }

        assert_ok!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(controller2),
                permission_id,
                false,
            )
        );

        let contract = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        if let pallet_permission0::PermissionScope::Emission(emission_scope) = contract.scope {
            assert!(!emission_scope.accumulating);
        }

        distribute_emission(delegator, as_tors(10));

        assert_ok!(
            pallet_permission0::Pallet::<Test>::enforcement_execute_permission(
                get_origin(controller1),
                permission_id,
            )
        );

        let votes = pallet_permission0::EnforcementTracking::<Test>::get(
            permission_id,
            EnforcementReferendum::Execution,
        );
        assert_eq!(votes.len(), 1);

        let balance_before = get_balance(recipient);

        assert_ok!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(controller1),
                permission_id,
                true,
            )
        );
        assert_ok!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(controller2),
                permission_id,
                true,
            )
        );

        distribute_emission(delegator, as_tors(10));

        assert_ok!(
            pallet_permission0::Pallet::<Test>::enforcement_execute_permission(
                get_origin(controller1),
                permission_id,
            )
        );

        assert!(get_balance(recipient) == balance_before);

        assert_ok!(
            pallet_permission0::Pallet::<Test>::enforcement_execute_permission(
                get_origin(controller2),
                permission_id,
            )
        );

        assert!(get_balance(recipient) > balance_before);
    });
}

#[test]
fn enforcement_cannot_execute_non_manual_distribution() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let delegator = 0;
        let recipient = 1;
        let controller = 2;

        register_empty_agent(delegator);
        register_empty_agent(recipient);
        register_empty_agent(controller);

        add_balance(delegator, as_tors(100) + 1);

        let permission_id = assert_ok!(delegate_emission_permission(
            delegator,
            vec![(recipient, u16::MAX)],
            pallet_permission0_api::EmissionAllocation::Streams(stream_percentages(delegator, 100)),
            pallet_permission0_api::DistributionControl::Automatic(
                MinAutoDistributionThreshold::get()
            ),
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::ControlledBy {
                controllers: vec![controller],
                required_votes: 1
            },
        ));

        distribute_emission(delegator, as_tors(10));

        assert_err!(
            pallet_permission0::Pallet::<Test>::enforcement_execute_permission(
                get_origin(controller),
                permission_id,
            ),
            pallet_permission0::Error::<Test>::InvalidDistributionMethod
        );
    });
}
