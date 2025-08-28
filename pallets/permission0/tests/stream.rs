use std::collections::BTreeMap;

use pallet_emission0::distribute::get_total_emission_per_block;
use pallet_permission0::{AccumulatedStreamAmounts, PermissionScope};
use pallet_permission0_api::generate_root_stream_id;
use polkadot_sdk::{
    frame_support::assert_err,
    sp_runtime::{BoundedBTreeMap, BoundedVec, Percent},
};
use test_utils::{
    pallet_emission0::{
        ConsensusMember, ConsensusMembers, EmissionRecyclingPercentage, IncentivesRatio,
    },
    pallet_governance::TreasuryEmissionFee,
    pallet_torus0::{FeeConstraints, MinAllowedStake, MinValidatorStake, StakedBy},
    *,
};

pub fn new_test_ext() -> polkadot_sdk::sp_io::TestExternalities {
    new_test_ext_with_block(1)
}

// Helper to get the most recent permission ID from events
fn get_last_delegated_permission_id(delegator: AccountId) -> pallet_permission0::PermissionId {
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
        .next_back() // Get most recent
        .expect("No PermissionDelegated event found")
}

#[test]
fn stream_fails_if_overflow() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(98));

        assert_ok!(delegate_stream_permission(
            agent_0,
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::StreamAllocation::Streams(streams),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(3));

        assert_err!(
            delegate_stream_permission(
                agent_0,
                vec![(agent_1, u16::MAX)],
                pallet_permission0_api::StreamAllocation::Streams(streams),
                pallet_permission0_api::DistributionControl::Manual,
                pallet_permission0_api::PermissionDuration::Indefinite,
                pallet_permission0_api::RevocationTerms::Irrevocable,
                pallet_permission0_api::EnforcementAuthority::None,
            ),
            pallet_permission0::Error::<Test>::TotalAllocationExceeded
        );
    });
}

#[test]
fn stream_creates() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(delegate_stream_permission(
            agent_0,
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::StreamAllocation::Streams(streams),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));

        assert!(AccumulatedStreamAmounts::<Test>::contains_key((
            &agent_0,
            &stream_id,
            &permission_id
        )));
    });
}

#[test]
fn stream_manual_executes() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        EmissionRecyclingPercentage::<Test>::set(Percent::zero());
        TreasuryEmissionFee::<Test>::set(Percent::zero());

        let min_stake = MinAllowedStake::<Test>::get();
        MinValidatorStake::<Test>::set(min_stake);

        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(delegate_stream_permission(
            agent_0,
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::StreamAllocation::Streams(streams),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        AccumulatedStreamAmounts::<Test>::set(
            (agent_0, stream_id, permission_id),
            Some(as_tors(10)),
        );

        assert_ok!(Permission0::execute_permission(
            get_origin(agent_0),
            permission_id
        ));

        assert_eq!(get_balance(agent_1), as_tors(10));
    });
}

#[test]
fn stream_accumulates_and_executes_at_threshold() {
    test_utils::new_test_ext_with_block(100).execute_with(|| {
        let ratio = 10;
        let (min_validator_stake, _) = set_emissions_params();

        let incentives_ratio = Percent::from_parts(ratio);
        IncentivesRatio::<Test>::set(incentives_ratio);

        let val = 0;
        let miner = 1;

        let mut member = ConsensusMember::<Test>::default();
        member.update_weights(BoundedVec::truncate_from(vec![(miner, 1)]));

        ConsensusMembers::<Test>::set(val, Some(member));
        ConsensusMembers::<Test>::set(miner, Some(Default::default()));

        add_stake(val, val, min_validator_stake);
        add_stake(miner, miner, min_validator_stake);

        let stream_id = generate_root_stream_id(&miner);

        let total_emission = get_total_emission_per_block::<Test>() * 100;
        let total_incentives = incentives_ratio.mul_floor(total_emission);
        let dividends_ratio = Percent::one() - incentives_ratio;
        let total_dividends = dividends_ratio.mul_floor(total_emission);
        assert_eq!(total_emission, total_incentives + total_dividends);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(delegate_stream_permission(
            miner,
            vec![(val, u16::MAX)],
            pallet_permission0_api::StreamAllocation::Streams(streams),
            pallet_permission0_api::DistributionControl::Automatic(total_incentives),
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        step_block(120);

        assert_eq!(
            StakedBy::<Test>::get(val, val).unwrap_or_default() - min_validator_stake
                + get_balance(val)
                - min_validator_stake,
            total_dividends + total_incentives
        );
        assert_eq!(
            StakedBy::<Test>::get(miner, miner).unwrap_or_default() - min_validator_stake,
            0
        );

        // Should still exist
        assert_eq!(
            AccumulatedStreamAmounts::<Test>::get((miner, stream_id, permission_id)),
            Some(0)
        );
    });
}

fn set_emissions_params() -> (u128, Percent) {
    EmissionRecyclingPercentage::<Test>::set(Percent::zero());
    TreasuryEmissionFee::<Test>::set(Percent::zero());

    let weight_control_fee = Percent::from_parts(0);
    FeeConstraints::<Test>::mutate(|constraints| {
        constraints.min_staking_fee = Percent::zero();
        constraints.min_weight_control_fee = weight_control_fee;
    });

    let min_validator_stake = 1;
    MinValidatorStake::<Test>::set(min_validator_stake);
    MinAllowedStake::<Test>::set(min_validator_stake);

    (min_validator_stake, weight_control_fee)
}

#[test]
fn random_cannot_change_permission() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(delegate_stream_permission(
            agent_0,
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::StreamAllocation::Streams(streams),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));

        assert!(AccumulatedStreamAmounts::<Test>::contains_key((
            &agent_0,
            &stream_id,
            &permission_id
        )));

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(agent_2),
                permission_id,
                Some(BoundedBTreeMap::new()),
                None,
                None,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );
    });
}

#[test]
fn delegator_cannot_change_irrevocable_permission() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(delegate_stream_permission(
            agent_0,
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::StreamAllocation::Streams(streams),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));

        assert!(AccumulatedStreamAmounts::<Test>::contains_key((
            &agent_0,
            &stream_id,
            &permission_id
        )));

        let mut new_recipients = BoundedBTreeMap::new();
        new_recipients.try_insert(agent_1, u16::MAX).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(agent_0),
                permission_id,
                Some(new_recipients),
                None,
                None,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );
    });
}

#[test]
fn delegator_cannot_change_arbiter_permission() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        let agent_3 = 3;
        register_empty_agent(agent_3);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(delegate_stream_permission(
            agent_0,
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::StreamAllocation::Streams(streams),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::RevocableByArbiters {
                accounts: vec![agent_2, agent_3],
                required_votes: 2
            },
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));

        assert!(AccumulatedStreamAmounts::<Test>::contains_key((
            &agent_0,
            &stream_id,
            &permission_id
        )));

        let mut new_recipients = BoundedBTreeMap::new();
        new_recipients.try_insert(agent_1, u16::MAX).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(agent_0),
                permission_id,
                Some(new_recipients),
                None,
                None,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );
    });
}

#[test]
fn delegator_cannot_change_permission_before_block() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        let agent_3 = 3;
        register_empty_agent(agent_3);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(delegate_stream_permission(
            agent_0,
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::StreamAllocation::Streams(streams),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::RevocableAfter(5),
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));

        assert!(AccumulatedStreamAmounts::<Test>::contains_key((
            &agent_0,
            &stream_id,
            &permission_id
        )));

        let mut new_recipients = BoundedBTreeMap::new();
        new_recipients.try_insert(agent_2, u16::MAX).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(agent_0),
                permission_id,
                Some(new_recipients),
                None,
                None,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );

        step_block(6);

        let mut new_recipients = BoundedBTreeMap::new();
        new_recipients.try_insert(agent_2, u16::MAX).unwrap();

        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(agent_0),
                permission_id,
                Some(new_recipients),
                None,
                None,
                None,
                None
            )
        );
    });
}

#[test]
fn recipient_can_only_change_recipients() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        let agent_3 = 3;
        register_empty_agent(agent_3);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(delegate_stream_permission(
            agent_0,
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::StreamAllocation::Streams(streams),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::RevocableAfter(5),
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));

        assert!(AccumulatedStreamAmounts::<Test>::contains_key((
            &agent_0,
            &stream_id,
            &permission_id
        )));

        let mut new_recipients = BoundedBTreeMap::new();
        new_recipients.try_insert(agent_2, u16::MAX).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(agent_1),
                permission_id,
                Some(new_recipients),
                Some(BoundedBTreeMap::new()),
                None,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );

        step_block(6);

        let mut new_recipients = BoundedBTreeMap::new();
        new_recipients.try_insert(agent_2, u16::MAX).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(agent_1),
                permission_id,
                Some(new_recipients),
                None,
                Some(pallet_permission0::DistributionControl::Manual),
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );
    });
}

#[test]
fn updating_works() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        let agent_3 = 3;
        register_empty_agent(agent_3);

        add_balance(agent_0, as_tors(10) + 1);

        let stream = generate_root_stream_id(&agent_0);
        let mut streams = BTreeMap::new();
        streams.insert(stream, Percent::from_percent(100));

        let permission_id = assert_ok!(delegate_stream_permission(
            agent_0,
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::StreamAllocation::Streams(streams.clone()),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::RevocableByDelegator,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));

        assert!(AccumulatedStreamAmounts::<Test>::contains_key((
            &agent_0,
            &stream,
            &permission_id
        )));

        let mut new_recipients = BoundedBTreeMap::new();
        new_recipients.try_insert(agent_1, u16::MAX).unwrap();

        let new_stream = generate_root_stream_id(&agent_1);
        let mut new_streams = BTreeMap::new();
        new_streams.insert(new_stream, Percent::from_percent(100));

        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(agent_0),
                permission_id,
                Some(new_recipients.clone()),
                Some(new_streams.clone().try_into().unwrap()),
                Some(pallet_permission0::DistributionControl::Interval(100)),
                None,
                None
            )
        );

        let PermissionScope::Stream(emission_scope) =
            pallet_permission0::Permissions::<Test>::get(permission_id)
                .unwrap()
                .scope
        else {
            panic!("unexpected permission scope");
        };

        assert_eq!(
            emission_scope.allocation,
            pallet_permission0::StreamAllocation::Streams(new_streams.try_into().unwrap())
        );

        assert!(!AccumulatedStreamAmounts::<Test>::contains_key((
            &agent_0,
            &stream,
            &permission_id
        )));

        assert!(AccumulatedStreamAmounts::<Test>::contains_key((
            &agent_0,
            new_stream,
            &permission_id,
        )));

        assert_eq!(
            emission_scope.distribution,
            pallet_permission0::DistributionControl::Interval(100)
        );

        assert_eq!(emission_scope.recipients, new_recipients);
    });
}

#[test]
fn recipient_revocation_removes_single_recipient_from_multiple() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        let agent_3 = 3;
        register_empty_agent(agent_3);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);
        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(delegate_stream_permission(
            agent_0,
            vec![(agent_1, u16::MAX / 2), (agent_2, u16::MAX / 2)],
            pallet_permission0_api::StreamAllocation::Streams(streams),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        let permission = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        let pallet_permission0::PermissionScope::Stream(scope) = permission.scope else {
            panic!("Expected emission scope");
        };
        assert_eq!(scope.recipients.len(), 2);
        assert!(scope.recipients.contains_key(&agent_1));
        assert!(scope.recipients.contains_key(&agent_2));

        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_1)
                .contains(&permission_id)
        );
        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_2)
                .contains(&permission_id)
        );

        assert_ok!(pallet_permission0::Pallet::<Test>::revoke_permission(
            get_origin(agent_1),
            permission_id
        ));

        let permission = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        let pallet_permission0::PermissionScope::Stream(scope) = permission.scope else {
            panic!("Expected emission scope");
        };
        assert_eq!(scope.recipients.len(), 1);
        assert!(!scope.recipients.contains_key(&agent_1));
        assert!(scope.recipients.contains_key(&agent_2));

        // Verify indices updated correctly
        assert!(
            !pallet_permission0::PermissionsByRecipient::<Test>::get(agent_1)
                .contains(&permission_id)
        );
        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_2)
                .contains(&permission_id)
        );

        // Agent_2 revokes their part (should delete entire permission)
        assert_ok!(pallet_permission0::Pallet::<Test>::revoke_permission(
            get_origin(agent_2),
            permission_id
        ));

        // Permission should be completely deleted
        assert!(!pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));
        assert!(
            !pallet_permission0::PermissionsByRecipient::<Test>::get(agent_2)
                .contains(&permission_id)
        );
    });
}

#[test]
fn recipient_revocation_deletes_permission_when_single_recipient() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);
        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(delegate_stream_permission(
            agent_0,
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::StreamAllocation::Streams(streams),
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));
        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_1)
                .contains(&permission_id)
        );

        // Agent_1 revokes (should delete entire permission since it's the only recipient)
        assert_ok!(pallet_permission0::Pallet::<Test>::revoke_permission(
            get_origin(agent_1),
            permission_id
        ));

        // Permission should be completely deleted
        assert!(!pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));
        assert!(
            !pallet_permission0::PermissionsByRecipient::<Test>::get(agent_1)
                .contains(&permission_id)
        );
    });
}

#[test]
fn weight_setter_can_only_modify_weights_not_keys() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        let weight_setter = 3;
        register_empty_agent(weight_setter);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);
        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let mut recipients = BoundedBTreeMap::new();
        recipients.try_insert(agent_1, u16::MAX / 2).unwrap();
        recipients.try_insert(agent_2, u16::MAX / 2).unwrap();

        assert_ok!(
            pallet_permission0::Pallet::<Test>::delegate_stream_permission(
                get_origin(agent_0),
                recipients,
                pallet_permission0::StreamAllocation::Streams(streams.try_into().unwrap()),
                pallet_permission0::DistributionControl::Manual,
                pallet_permission0::PermissionDuration::Indefinite,
                pallet_permission0::RevocationTerms::RevocableByDelegator,
                pallet_permission0::EnforcementAuthority::None,
                None,                // recipient_manager
                Some(weight_setter), // weight_setter
            )
        );

        let permission_id = get_last_delegated_permission_id(agent_0);

        // Weight setter can modify weights of existing recipients
        let mut updated_recipients = BoundedBTreeMap::new();
        updated_recipients
            .try_insert(agent_1, u16::MAX / 4)
            .unwrap();
        updated_recipients
            .try_insert(agent_2, (u16::MAX / 4) * 3)
            .unwrap();

        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(weight_setter),
                permission_id,
                Some(updated_recipients),
                None,
                None,
                None,
                None
            )
        );

        // Verify weights were updated
        let permission = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        let pallet_permission0::PermissionScope::Stream(scope) = permission.scope else {
            panic!("Expected emission scope");
        };
        assert_eq!(*scope.recipients.get(&agent_1).unwrap(), u16::MAX / 4);
        assert_eq!(*scope.recipients.get(&agent_2).unwrap(), (u16::MAX / 4) * 3);

        // Weight setter CANNOT add new recipients (changing keys)
        let agent_3 = 4;
        register_empty_agent(agent_3);
        let mut new_recipients = BoundedBTreeMap::new();
        new_recipients.try_insert(agent_1, u16::MAX / 3).unwrap();
        new_recipients.try_insert(agent_3, u16::MAX / 3).unwrap(); // New key

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(weight_setter),
                permission_id,
                Some(new_recipients),
                None,
                None,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );

        // Weight setter CANNOT remove recipients (changing keys)
        let mut reduced_recipients = BoundedBTreeMap::new();
        reduced_recipients.try_insert(agent_1, u16::MAX).unwrap();
        // Missing agent_2

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(weight_setter),
                permission_id,
                Some(reduced_recipients),
                None,
                None,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );

        // Weight setter CANNOT modify other parameters
        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(weight_setter),
                permission_id,
                None,
                Some(BoundedBTreeMap::new()),
                None,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );
    });
}

#[test]
fn recipient_manager_can_modify_keys_and_weights() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        let recipient_manager = 3;
        register_empty_agent(recipient_manager);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);
        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let mut recipients = BoundedBTreeMap::new();
        recipients.try_insert(agent_1, u16::MAX).unwrap();

        assert_ok!(
            pallet_permission0::Pallet::<Test>::delegate_stream_permission(
                get_origin(agent_0),
                recipients,
                pallet_permission0::StreamAllocation::Streams(streams.try_into().unwrap()),
                pallet_permission0::DistributionControl::Manual,
                pallet_permission0::PermissionDuration::Indefinite,
                pallet_permission0::RevocationTerms::RevocableByDelegator,
                pallet_permission0::EnforcementAuthority::None,
                Some(recipient_manager), // recipient_manager
                None,                    // weight_setter
            )
        );

        let permission_id = get_last_delegated_permission_id(agent_0);

        // Recipient manager can add new recipients
        let mut new_recipients = BoundedBTreeMap::new();
        new_recipients.try_insert(agent_1, u16::MAX / 2).unwrap();
        new_recipients.try_insert(agent_2, u16::MAX / 2).unwrap();

        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(recipient_manager),
                permission_id,
                Some(new_recipients.clone()),
                None,
                None,
                None,
                None
            )
        );

        // Verify recipients were updated and indices are consistent
        let permission = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        let pallet_permission0::PermissionScope::Stream(scope) = permission.scope else {
            panic!("Expected emission scope");
        };
        assert_eq!(scope.recipients, new_recipients);
        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_2)
                .contains(&permission_id)
        );

        // Recipient manager can remove recipients
        let mut reduced_recipients = BoundedBTreeMap::new();
        reduced_recipients.try_insert(agent_2, u16::MAX).unwrap();

        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(recipient_manager),
                permission_id,
                Some(reduced_recipients.clone()),
                None,
                None,
                None,
                None
            )
        );

        // Verify recipient was removed and indices updated
        let permission = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        let pallet_permission0::PermissionScope::Stream(scope) = permission.scope else {
            panic!("Expected emission scope");
        };
        assert_eq!(scope.recipients, reduced_recipients);
        assert!(
            !pallet_permission0::PermissionsByRecipient::<Test>::get(agent_1)
                .contains(&permission_id)
        );
        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_2)
                .contains(&permission_id)
        );

        // Recipient manager CANNOT modify other parameters
        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(recipient_manager),
                permission_id,
                None,
                Some(BoundedBTreeMap::new()),
                None,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );
    });
}

#[test]
fn manager_modifications_only_allowed_after_revocable_period() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        let recipient_manager = 3;
        register_empty_agent(recipient_manager);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);
        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let mut recipients = BoundedBTreeMap::new();
        recipients.try_insert(agent_1, u16::MAX).unwrap();

        assert_ok!(
            pallet_permission0::Pallet::<Test>::delegate_stream_permission(
                get_origin(agent_0),
                recipients,
                pallet_permission0::StreamAllocation::Streams(streams.try_into().unwrap()),
                pallet_permission0::DistributionControl::Manual,
                pallet_permission0::PermissionDuration::Indefinite,
                pallet_permission0::RevocationTerms::RevocableAfter(5),
                pallet_permission0::EnforcementAuthority::None,
                Some(recipient_manager),
                None,
            )
        );

        let permission_id = get_last_delegated_permission_id(agent_0);

        // Manager cannot modify before revocable period
        let mut new_recipients = BoundedBTreeMap::new();
        new_recipients.try_insert(agent_1, u16::MAX / 2).unwrap();
        new_recipients.try_insert(agent_2, u16::MAX / 2).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(recipient_manager),
                permission_id,
                Some(new_recipients.clone()),
                None,
                None,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );

        // Wait for revocable period
        step_block(6);

        // Now manager can modify
        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(recipient_manager),
                permission_id,
                Some(new_recipients.clone()),
                None,
                None,
                None,
                None
            )
        );

        // Verify changes were applied
        let permission = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        let pallet_permission0::PermissionScope::Stream(scope) = permission.scope else {
            panic!("Expected emission scope");
        };
        assert_eq!(scope.recipients, new_recipients);
    });
}

#[test]
fn delegator_can_assign_and_remove_managers() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let weight_setter = 2;
        register_empty_agent(weight_setter);

        let recipient_manager = 3;
        register_empty_agent(recipient_manager);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);
        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let mut recipients = BoundedBTreeMap::new();
        recipients.try_insert(agent_1, u16::MAX).unwrap();

        assert_ok!(
            pallet_permission0::Pallet::<Test>::delegate_stream_permission(
                get_origin(agent_0),
                recipients,
                pallet_permission0::StreamAllocation::Streams(streams.try_into().unwrap()),
                pallet_permission0::DistributionControl::Manual,
                pallet_permission0::PermissionDuration::Indefinite,
                pallet_permission0::RevocationTerms::RevocableByDelegator,
                pallet_permission0::EnforcementAuthority::None,
                None, // weight_setter
                None, // recipient_manager
            )
        );

        let permission_id = get_last_delegated_permission_id(agent_0);

        // Delegator can assign managers
        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(agent_0),
                permission_id,
                None,
                None,
                None,
                Some(Some(recipient_manager)),
                Some(Some(weight_setter)),
            )
        );

        // Verify managers were assigned
        let permission = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        let pallet_permission0::PermissionScope::Stream(scope) = permission.scope else {
            panic!("Expected emission scope");
        };
        assert!(scope.weight_setters.contains(&weight_setter));
        assert!(scope.recipient_managers.contains(&recipient_manager));

        // Now managers can operate within their permissions
        let mut new_recipients = BoundedBTreeMap::new();
        new_recipients.try_insert(agent_1, u16::MAX / 2).unwrap();

        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(weight_setter),
                permission_id,
                Some(new_recipients.clone()),
                None,
                None,
                None,
                None
            )
        );

        // Delegator can remove managers using Some(None)
        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(agent_0),
                permission_id,
                None,
                None,
                None,
                Some(None), // Remove weight_setter
                Some(None)  // Remove recipient_manager
            )
        );

        // Verify managers were removed
        let permission = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        let pallet_permission0::PermissionScope::Stream(scope) = permission.scope else {
            panic!("Expected emission scope");
        };
        assert!(scope.weight_setters.len() == 1 && scope.weight_setters.contains(&agent_0));
        assert!(scope.recipient_managers.len() == 1 && scope.recipient_managers.contains(&agent_0));

        // Former managers should no longer have access
        assert_err!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(weight_setter),
                permission_id,
                Some(new_recipients),
                None,
                None,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );
    });
}

#[test]
fn index_consistency_during_complex_recipient_updates() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        let agent_3 = 3;
        register_empty_agent(agent_3);

        let agent_4 = 4;
        register_empty_agent(agent_4);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);
        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let mut recipients = BoundedBTreeMap::new();
        recipients.try_insert(agent_1, u16::MAX / 3).unwrap();
        recipients.try_insert(agent_2, u16::MAX / 3).unwrap();
        recipients.try_insert(agent_3, u16::MAX / 3).unwrap();

        assert_ok!(
            pallet_permission0::Pallet::<Test>::delegate_stream_permission(
                get_origin(agent_0),
                recipients,
                pallet_permission0::StreamAllocation::Streams(streams.try_into().unwrap()),
                pallet_permission0::DistributionControl::Manual,
                pallet_permission0::PermissionDuration::Indefinite,
                pallet_permission0::RevocationTerms::RevocableByDelegator,
                pallet_permission0::EnforcementAuthority::None,
                None,
                None,
            )
        );

        let permission_id = get_last_delegated_permission_id(agent_0);

        // Verify initial indices
        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_1)
                .contains(&permission_id)
        );
        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_2)
                .contains(&permission_id)
        );
        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_3)
                .contains(&permission_id)
        );
        assert!(
            !pallet_permission0::PermissionsByRecipient::<Test>::get(agent_4)
                .contains(&permission_id)
        );

        // Complex update: remove agent_2, keep agent_1 with new weight, add agent_4
        let mut updated_recipients = BoundedBTreeMap::new();
        updated_recipients
            .try_insert(agent_1, u16::MAX / 2)
            .unwrap(); // Keep with different weight
        updated_recipients
            .try_insert(agent_3, u16::MAX / 4)
            .unwrap(); // Keep with different weight  
        updated_recipients
            .try_insert(agent_4, u16::MAX / 4)
            .unwrap(); // Add new

        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_stream_permission(
                get_origin(agent_0),
                permission_id,
                Some(updated_recipients.clone()),
                None,
                None,
                None,
                None
            )
        );

        // Verify permission state is correct
        let permission = pallet_permission0::Permissions::<Test>::get(permission_id).unwrap();
        let pallet_permission0::PermissionScope::Stream(scope) = permission.scope else {
            panic!("Expected emission scope");
        };
        assert_eq!(scope.recipients, updated_recipients);

        // Verify ALL indices are consistent
        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_1)
                .contains(&permission_id)
        ); // Still there
        assert!(
            !pallet_permission0::PermissionsByRecipient::<Test>::get(agent_2)
                .contains(&permission_id)
        ); // Removed
        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_3)
                .contains(&permission_id)
        ); // Still there
        assert!(
            pallet_permission0::PermissionsByRecipient::<Test>::get(agent_4)
                .contains(&permission_id)
        ); // Added

        // Verify delegator indices are also consistent
        let delegator_permissions =
            pallet_permission0::PermissionsByDelegator::<Test>::get(agent_0);
        assert!(delegator_permissions.contains(&permission_id));

        // Verify participants indices include all current participants
        let participants_1 =
            pallet_permission0::PermissionsByParticipants::<Test>::get((&agent_0, &agent_1));
        let participants_2 =
            pallet_permission0::PermissionsByParticipants::<Test>::get((&agent_0, &agent_2));
        let participants_3 =
            pallet_permission0::PermissionsByParticipants::<Test>::get((&agent_0, &agent_3));
        let participants_4 =
            pallet_permission0::PermissionsByParticipants::<Test>::get((&agent_0, &agent_4));

        assert!(participants_1.contains(&permission_id)); // Still there
        assert!(!participants_2.contains(&permission_id)); // Removed
        assert!(participants_3.contains(&permission_id)); // Still there
        assert!(participants_4.contains(&permission_id)); // Added
    });
}

#[test]
fn cannot_create_empty_recipients_for_irrevocable_permissions() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);
        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(50));

        assert_err!(
            pallet_permission0::Pallet::<Test>::delegate_stream_permission(
                get_origin(agent_0),
                Default::default(),
                pallet_permission0::StreamAllocation::Streams(streams.clone().try_into().unwrap()),
                pallet_permission0::DistributionControl::Manual,
                pallet_permission0::PermissionDuration::Indefinite,
                pallet_permission0::RevocationTerms::Irrevocable,
                pallet_permission0::EnforcementAuthority::None,
                None,
                None,
            ),
            pallet_permission0::Error::<Test>::NoRecipientsSpecified
        );

        assert_err!(
            pallet_permission0::Pallet::<Test>::delegate_stream_permission(
                get_origin(agent_0),
                Default::default(),
                pallet_permission0::StreamAllocation::Streams(streams.clone().try_into().unwrap()),
                pallet_permission0::DistributionControl::Manual,
                pallet_permission0::PermissionDuration::Indefinite,
                pallet_permission0::RevocationTerms::RevocableAfter(10),
                pallet_permission0::EnforcementAuthority::None,
                None,
                None,
            ),
            pallet_permission0::Error::<Test>::NoRecipientsSpecified
        );

        assert_ok!(
            pallet_permission0::Pallet::<Test>::delegate_stream_permission(
                get_origin(agent_0),
                Default::default(),
                pallet_permission0::StreamAllocation::Streams(streams.try_into().unwrap()),
                pallet_permission0::DistributionControl::Manual,
                pallet_permission0::PermissionDuration::Indefinite,
                pallet_permission0::RevocationTerms::RevocableByDelegator,
                pallet_permission0::EnforcementAuthority::None,
                None,
                None,
            )
        );
    });
}
