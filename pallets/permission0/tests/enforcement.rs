use std::collections::BTreeMap;

use pallet_permission0::permission::emission::StreamId;
use pallet_permission0::EnforcementReferendum;
use pallet_permission0_api::{generate_root_stream_id, Permission0Api};
use polkadot_sdk::frame_support::{assert_err, traits::Currency};
use polkadot_sdk::sp_runtime::Percent;
use test_utils::{assert_ok, *};

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
fn set_enforcement_authority_by_grantor() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let grantor = 0;
        let grantee = 1;
        let controller = 2;

        register_empty_agent(grantor);
        register_empty_agent(grantee);
        register_empty_agent(controller);

        add_balance(grantor, to_nano(10) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            grantor,
            grantee,
            pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
            vec![(grantee, u16::MAX)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert_err!(
            pallet_permission0::Pallet::<Test>::set_enforcement_authority(
                get_origin(grantee),
                permission_id,
                vec![controller],
                1,
            ),
            pallet_permission0::Error::<Test>::NotPermissionGrantor
        );

        assert_err!(
            pallet_permission0::Pallet::<Test>::set_enforcement_authority(
                get_origin(controller),
                permission_id,
                vec![controller],
                1,
            ),
            pallet_permission0::Error::<Test>::NotPermissionGrantor
        );

        assert_ok!(
            pallet_permission0::Pallet::<Test>::set_enforcement_authority(
                get_origin(grantor),
                permission_id,
                vec![controller],
                1,
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
        let grantor = 0;
        let grantee = 1;
        let controller = 2;

        register_empty_agent(grantor);
        register_empty_agent(grantee);
        register_empty_agent(controller);

        add_balance(grantor, to_nano(100) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            grantor,
            grantee,
            pallet_permission0_api::EmissionAllocation::Streams(stream_percentages(grantor, 100)),
            vec![(grantee, u16::MAX)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::ControlledBy {
                controllers: vec![controller],
                required_votes: 1
            },
        ));

        distribute_emission(grantor, to_nano(10));

        assert_ok!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(controller),
                permission_id,
                false,
            )
        );

        let contract = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        match contract.scope {
            pallet_permission0::PermissionScope::Emission(emission_scope) => {
                assert!(!emission_scope.accumulating);
            }
            _ => {}
        }

        let balance_before = get_balance(grantee);
        distribute_emission(grantor, to_nano(10));

        assert_eq!(get_balance(grantee), balance_before);

        assert_ok!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(controller),
                permission_id,
                true,
            )
        );

        distribute_emission(grantor, to_nano(10));

        assert_ok!(pallet_permission0::Pallet::<Test>::execute_permission(
            get_origin(grantor),
            permission_id,
        ));

        assert!(get_balance(grantee) > balance_before);
    });
}

#[test]
fn unauthorized_account_cannot_toggle() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let grantor = 0;
        let grantee = 1;
        let controller = 2;
        let unauthorized = 3;

        register_empty_agent(grantor);
        register_empty_agent(grantee);
        register_empty_agent(controller);
        register_empty_agent(unauthorized);

        add_balance(grantor, to_nano(10) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            grantor,
            grantee,
            pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
            vec![(grantee, u16::MAX)],
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
                get_origin(grantor),
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
        let grantor = 0;
        let grantee = 1;
        let controller = 2;

        register_empty_agent(grantor);
        register_empty_agent(grantee);
        register_empty_agent(controller);

        add_balance(grantor, to_nano(100) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            grantor,
            grantee,
            pallet_permission0_api::EmissionAllocation::Streams(stream_percentages(grantor, 100)),
            vec![(grantee, u16::MAX)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::ControlledBy {
                controllers: vec![controller],
                required_votes: 1
            },
        ));

        distribute_emission(grantor, to_nano(10));

        let balance_before = get_balance(grantee);

        assert_ok!(
            pallet_permission0::Pallet::<Test>::enforcement_execute_permission(
                get_origin(controller),
                permission_id,
            )
        );

        assert!(get_balance(grantee) > balance_before);
    });
}

#[test]
fn unauthorized_cannot_enforcement_execute() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let grantor = 0;
        let grantee = 1;
        let controller = 2;
        let unauthorized = 3;

        register_empty_agent(grantor);
        register_empty_agent(grantee);
        register_empty_agent(controller);
        register_empty_agent(unauthorized);

        add_balance(grantor, to_nano(100) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            grantor,
            grantee,
            pallet_permission0_api::EmissionAllocation::Streams(stream_percentages(grantor, 100)),
            vec![(grantee, u16::MAX)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::ControlledBy {
                controllers: vec![controller],
                required_votes: 1
            },
        ));

        distribute_emission(grantor, to_nano(10));

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
        let grantor = 0;
        let grantee = 1;
        let controller1 = 2;
        let controller2 = 3;
        let controller3 = 4;

        register_empty_agent(grantor);
        register_empty_agent(grantee);
        register_empty_agent(controller1);
        register_empty_agent(controller2);
        register_empty_agent(controller3);

        add_balance(grantor, to_nano(100) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            grantor,
            grantee,
            pallet_permission0_api::EmissionAllocation::Streams(stream_percentages(grantor, 100)),
            vec![(grantee, u16::MAX)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::ControlledBy {
                controllers: vec![controller1, controller2, controller3],
                required_votes: 2
            },
        ));

        distribute_emission(grantor, to_nano(10));

        assert_ok!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(controller1),
                permission_id,
                false,
            )
        );

        let contract = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        match contract.scope {
            pallet_permission0::PermissionScope::Emission(emission_scope) => {
                assert!(emission_scope.accumulating);
            }
            _ => {}
        }

        assert_ok!(
            pallet_permission0::Pallet::<Test>::toggle_permission_accumulation(
                get_origin(controller2),
                permission_id,
                false,
            )
        );

        let contract = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        match contract.scope {
            pallet_permission0::PermissionScope::Emission(emission_scope) => {
                assert!(!emission_scope.accumulating);
            }
            _ => {}
        }

        distribute_emission(grantor, to_nano(10));

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

        let balance_before = get_balance(grantee);

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

        distribute_emission(grantor, to_nano(10));

        assert_ok!(
            pallet_permission0::Pallet::<Test>::enforcement_execute_permission(
                get_origin(controller1),
                permission_id,
            )
        );

        assert!(get_balance(grantee) == balance_before);

        assert_ok!(
            pallet_permission0::Pallet::<Test>::enforcement_execute_permission(
                get_origin(controller2),
                permission_id,
            )
        );

        assert!(get_balance(grantee) > balance_before);
    });
}

#[test]
fn enforcement_cannot_execute_non_manual_distribution() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let grantor = 0;
        let grantee = 1;
        let controller = 2;

        register_empty_agent(grantor);
        register_empty_agent(grantee);
        register_empty_agent(controller);

        add_balance(grantor, to_nano(100) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            grantor,
            grantee,
            pallet_permission0_api::EmissionAllocation::Streams(stream_percentages(grantor, 100)),
            vec![(grantee, u16::MAX)],
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

        distribute_emission(grantor, to_nano(10));

        assert_err!(
            pallet_permission0::Pallet::<Test>::enforcement_execute_permission(
                get_origin(controller),
                permission_id,
            ),
            pallet_permission0::Error::<Test>::InvalidDistributionMethod
        );
    });
}
