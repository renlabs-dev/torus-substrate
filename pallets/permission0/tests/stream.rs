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

        assert_ok!(grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::Streams(streams),
            vec![(agent_0, u16::MAX)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(3));

        assert_err!(
            grant_emission_permission(
                agent_0,
                agent_1,
                pallet_permission0_api::EmissionAllocation::Streams(streams),
                vec![(agent_0, u16::MAX)],
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

        let permission_id = assert_ok!(grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::Streams(streams),
            vec![(agent_0, u16::MAX)],
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

        let permission_id = assert_ok!(grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::Streams(streams),
            vec![(agent_1, u16::MAX)],
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
    test_utils::new_test_ext().execute_with(|| {
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

        let permission_id = assert_ok!(grant_emission_permission(
            miner,
            val,
            pallet_permission0_api::EmissionAllocation::Streams(streams),
            vec![(val, u16::MAX)],
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
    test_utils::new_test_ext().execute_with(|| {
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

        let permission_id = assert_ok!(grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::Streams(streams),
            vec![(agent_0, u16::MAX)],
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
            pallet_permission0::Pallet::<Test>::update_emission_permission(
                get_origin(agent_2),
                permission_id,
                BoundedBTreeMap::new(),
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );
    });
}

#[test]
fn grantor_cannot_change_irrevocable_permission() {
    test_utils::new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::Streams(streams),
            vec![(agent_0, u16::MAX)],
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

        let mut new_targets = BoundedBTreeMap::new();
        new_targets.try_insert(agent_0, u16::MAX).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_emission_permission(
                get_origin(agent_0),
                permission_id,
                new_targets,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );
    });
}

#[test]
fn grantor_cannot_change_arbiter_permission() {
    test_utils::new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 1;
        register_empty_agent(agent_2);

        let agent_3 = 1;
        register_empty_agent(agent_3);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::Streams(streams),
            vec![(agent_0, u16::MAX)],
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

        let mut new_targets = BoundedBTreeMap::new();
        new_targets.try_insert(agent_0, u16::MAX).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_emission_permission(
                get_origin(agent_0),
                permission_id,
                new_targets,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );
    });
}

#[test]
fn grantor_cannot_change_permission_before_block() {
    test_utils::new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 1;
        register_empty_agent(agent_2);

        let agent_3 = 1;
        register_empty_agent(agent_3);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::Streams(streams),
            vec![(agent_0, u16::MAX)],
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

        let mut new_targets = BoundedBTreeMap::new();
        new_targets.try_insert(agent_0, u16::MAX).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_emission_permission(
                get_origin(agent_0),
                permission_id,
                new_targets,
                None,
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );

        step_block(6);

        let mut new_targets = BoundedBTreeMap::new();
        new_targets.try_insert(agent_0, u16::MAX).unwrap();

        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_emission_permission(
                get_origin(agent_0),
                permission_id,
                new_targets,
                None,
                None
            )
        );
    });
}

#[test]
fn grantee_can_only_change_targets() {
    test_utils::new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 1;
        register_empty_agent(agent_2);

        let agent_3 = 1;
        register_empty_agent(agent_3);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);

        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::Streams(streams),
            vec![(agent_0, u16::MAX)],
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

        let mut new_targets = BoundedBTreeMap::new();
        new_targets.try_insert(agent_0, u16::MAX).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_emission_permission(
                get_origin(agent_1),
                permission_id,
                new_targets,
                Some(BoundedBTreeMap::new()),
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );

        step_block(6);

        let mut new_targets = BoundedBTreeMap::new();
        new_targets.try_insert(agent_0, u16::MAX).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_emission_permission(
                get_origin(agent_1),
                permission_id,
                new_targets,
                None,
                Some(pallet_permission0::DistributionControl::Manual)
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );
    });
}

#[test]
fn updating_works() {
    test_utils::new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 1;
        register_empty_agent(agent_2);

        let agent_3 = 1;
        register_empty_agent(agent_3);

        add_balance(agent_0, as_tors(10) + 1);

        let stream = generate_root_stream_id(&agent_0);
        let mut streams = BTreeMap::new();
        streams.insert(stream, Percent::from_percent(100));

        let permission_id = assert_ok!(grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::Streams(streams.clone()),
            vec![(agent_0, u16::MAX)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::RevocableByGrantor,
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

        let mut new_targets = BoundedBTreeMap::new();
        new_targets.try_insert(agent_1, u16::MAX).unwrap();

        let new_stream = generate_root_stream_id(&agent_1);
        let mut new_streams = BTreeMap::new();
        new_streams.insert(new_stream, Percent::from_percent(100));

        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_emission_permission(
                get_origin(agent_0),
                permission_id,
                new_targets.clone(),
                Some(new_streams.clone().try_into().unwrap()),
                Some(pallet_permission0::DistributionControl::Interval(100))
            )
        );

        let PermissionScope::Emission(emission_scope) =
            pallet_permission0::Permissions::<Test>::get(permission_id)
                .unwrap()
                .scope
        else {
            panic!("unexpected permission scope");
        };

        assert_eq!(
            emission_scope.allocation,
            pallet_permission0::EmissionAllocation::Streams(new_streams.try_into().unwrap())
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

        assert_eq!(emission_scope.targets, new_targets);
    });
}

#[test]
fn update_prevents_overarching_update_when_grantor_is_grantee() {
    test_utils::new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 1;
        register_empty_agent(agent_2);

        let agent_3 = 1;
        register_empty_agent(agent_3);

        add_balance(agent_0, as_tors(10) + 1);

        let stream_id = generate_root_stream_id(&agent_0);
        let mut streams = BTreeMap::new();
        streams.insert(stream_id, Percent::from_percent(100));

        let permission_id = assert_ok!(grant_emission_permission(
            agent_0,
            agent_0,
            pallet_permission0_api::EmissionAllocation::Streams(streams.clone()),
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::RevocableAfter(5),
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        let mut new_targets = BoundedBTreeMap::new();
        new_targets.try_insert(agent_1, u16::MAX).unwrap();

        assert_err!(
            pallet_permission0::Pallet::<Test>::update_emission_permission(
                get_origin(agent_0),
                permission_id,
                new_targets.clone(),
                Some(BoundedBTreeMap::new()),
                None
            ),
            pallet_permission0::Error::<Test>::NotAuthorizedToEdit
        );

        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_emission_permission(
                get_origin(agent_0),
                permission_id,
                new_targets.clone(),
                None,
                None
            )
        );

        step_block(6);

        assert_ok!(
            pallet_permission0::Pallet::<Test>::update_emission_permission(
                get_origin(agent_0),
                permission_id,
                new_targets.clone(),
                Some(streams.clone().try_into().unwrap()),
                Some(pallet_permission0::DistributionControl::Interval(100))
            )
        );
    });
}
