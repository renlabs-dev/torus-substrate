#![allow(clippy::indexing_slicing)]

use std::array::from_fn;

use pallet_emission0::{
    distribute::{get_total_emission_per_block, ConsensusMemberInput},
    Config, ConsensusMember, ConsensusMembers, EmissionRecyclingPercentage, IncentivesRatio,
    PendingEmission, WeightControlDelegation,
};
use polkadot_sdk::{
    pallet_balances,
    sp_core::Get,
    sp_runtime::{BoundedVec, FixedU128, Perbill, Percent},
};
use test_utils::{
    add_balance, add_stake,
    pallet_governance::{Allocators, TreasuryEmissionFee},
    pallet_torus0::{
        stake::sum_staked_by, Agents, FeeConstraints, MaxAllowedValidators, MinAllowedStake,
        MinValidatorStake, StakedBy,
    },
    register_empty_agent, step_block, Test,
};

#[test]
fn total_emission_per_block_does_halving() {
    test_utils::new_test_ext().execute_with(|| {
        let block_emission = <<Test as Config>::BlockEmission as Get<u128>>::get();
        let halving_interval = <<Test as Config>::HalvingInterval as Get<u128>>::get();
        let max_supply = <<Test as Config>::MaxSupply as Get<u128>>::get();

        let recycling_percentage = EmissionRecyclingPercentage::<Test>::get();
        let halving_emission = |halving: u128| {
            let block_emission = block_emission >> halving;
            block_emission - recycling_percentage.mul_ceil(block_emission)
        };

        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(emissions, halving_emission(0));

        pallet_balances::TotalIssuance::<Test>::set(halving_interval - 1);
        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(emissions, halving_emission(0));

        pallet_balances::TotalIssuance::<Test>::set(halving_interval);
        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(emissions, halving_emission(1));

        pallet_balances::TotalIssuance::<Test>::set(halving_interval * 2);
        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(emissions, halving_emission(2));

        pallet_balances::TotalIssuance::<Test>::set(max_supply);
        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(emissions, 0);
    });
}

#[test]
fn pending_emission_accumulates_and_returns_when_network_is_empty() {
    test_utils::new_test_ext().execute_with(|| {
        EmissionRecyclingPercentage::<Test>::set(Percent::zero());

        assert_eq!(PendingEmission::<Test>::get(), 0);

        let emissions = get_total_emission_per_block::<Test>();

        step_block(1);
        assert_eq!(PendingEmission::<Test>::get(), emissions);

        step_block(1);
        assert_eq!(PendingEmission::<Test>::get(), emissions * 2);

        let after_treasury_fee = Percent::one() - TreasuryEmissionFee::<Test>::get();
        let emissions = after_treasury_fee.mul_floor(emissions * 100);

        step_block(98);
        assert_eq!(PendingEmission::<Test>::get(), emissions);

        PendingEmission::<Test>::set(u128::MAX);
        step_block(1);
        assert_eq!(PendingEmission::<Test>::get(), u128::MAX);
    });
}

#[test]
fn weights_are_filtered_and_normalized() {
    test_utils::new_test_ext().execute_with(|| {
        let mut member = ConsensusMember::<Test>::default();
        member.update_weights(BoundedVec::truncate_from(vec![
            (0, 0),  // self-weight is discarded
            (1, 10), // unregistered agent still in members
            (2, 20), // new agent not in members
            (3, 40), // unknown agent
        ]));

        ConsensusMembers::<Test>::set(0, Some(member.clone()));
        ConsensusMembers::<Test>::set(1, Some(Default::default()));

        register_empty_agent(2);

        let weights = ConsensusMemberInput::<Test>::prepare_weights(member.weights, &0);
        assert_eq!(
            weights,
            vec![
                (1, FixedU128::from_rational(1, 3)),
                (2, FixedU128::from_rational(1, 3) * 2.into())
            ]
        )
    });
}

#[test]
fn creates_member_input_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let mut member = ConsensusMember::<Test>::default();

        let input = ConsensusMemberInput::<Test>::from_agent(0, member.weights.clone(), 0);
        assert_eq!(
            input,
            ConsensusMemberInput {
                agent_id: 0,
                validator_permit: false,
                weights: vec![],
                stakes: vec![],
                total_stake: 0,
                normalized_stake: FixedU128::from_inner(0),
                delegating_to: None,
                registered: false
            }
        );

        register_empty_agent(0);

        let input = ConsensusMemberInput::<Test>::from_agent(0, member.weights.clone(), 0);
        assert!(input.registered);

        StakedBy::<Test>::set(0, 1, Some(10));
        StakedBy::<Test>::set(0, 2, Some(20));

        ConsensusMembers::<Test>::set(1, Some(Default::default()));

        member.update_weights(BoundedVec::truncate_from(vec![(0, 0), (1, 10)]));

        let input = ConsensusMemberInput::<Test>::from_agent(0, member.weights.clone(), 15);
        assert_eq!(input.total_stake, 30);
        assert!(input.validator_permit);
        assert_eq!(input.weights.len(), 1);
    });
}

#[test]
fn creates_list_of_all_member_inputs_for_rewards() {
    test_utils::new_test_ext().execute_with(|| {
        let validator = 0;
        let new = 1;
        let delegating_registered = 2;
        let delegating_unregistered = 3;
        let delegating_unknown = 4;
        let miner = 5;
        let staker = 6;

        let stake = MinValidatorStake::<Test>::get();

        for id in [validator, new, miner, delegating_registered] {
            register_empty_agent(id);
        }

        for id in [miner, delegating_registered, delegating_unregistered] {
            ConsensusMembers::<Test>::set(id, Some(Default::default()));
        }

        let mut member = ConsensusMember::<Test>::default();
        member.update_weights(BoundedVec::truncate_from(vec![(miner, 10)]));
        ConsensusMembers::<Test>::set(validator, Some(member));
        StakedBy::<Test>::set(validator, staker, Some(stake * 3));

        StakedBy::<Test>::set(delegating_registered, staker, Some(stake));

        for id in [
            delegating_registered,
            delegating_unregistered,
            delegating_unknown,
        ] {
            WeightControlDelegation::<Test>::set(id, Some(validator));
        }

        let members = ConsensusMemberInput::<Test>::all_members();
        assert_eq!(members.len(), 6);

        assert_eq!(
            members[&validator],
            ConsensusMemberInput {
                agent_id: validator,
                validator_permit: true,
                weights: vec![(miner, FixedU128::from_u32(1))],
                stakes: vec![(staker, stake * 3)],
                total_stake: stake * 3,
                normalized_stake: FixedU128::from_float(0.75f64),
                delegating_to: None,
                registered: true,
            }
        );

        assert_eq!(
            members[&new],
            ConsensusMemberInput {
                agent_id: new,
                validator_permit: false,
                weights: vec![],
                stakes: vec![],
                total_stake: 0,
                normalized_stake: FixedU128::from_inner(0),
                delegating_to: None,
                registered: true,
            }
        );

        assert_eq!(
            members[&delegating_registered],
            ConsensusMemberInput {
                agent_id: delegating_registered,
                validator_permit: true,
                weights: vec![(miner, FixedU128::from_u32(1))],
                stakes: vec![(staker, stake)],
                total_stake: stake,
                normalized_stake: FixedU128::from_float(0.25f64),
                delegating_to: Some(validator),
                registered: true,
            }
        );

        assert_eq!(
            members[&delegating_unregistered],
            ConsensusMemberInput {
                agent_id: delegating_unregistered,
                validator_permit: false,
                weights: vec![],
                stakes: vec![],
                total_stake: 0,
                normalized_stake: FixedU128::from_inner(0),
                delegating_to: Some(validator),
                registered: false,
            }
        );

        assert_eq!(
            members[&delegating_unknown],
            ConsensusMemberInput {
                agent_id: delegating_unknown,
                validator_permit: false,
                weights: vec![],
                stakes: vec![],
                total_stake: 0,
                normalized_stake: FixedU128::from_inner(0),
                delegating_to: Some(validator),
                registered: false,
            }
        );

        assert_eq!(
            members[&miner],
            ConsensusMemberInput {
                agent_id: miner,
                validator_permit: false,
                weights: vec![],
                stakes: vec![],
                total_stake: 0,
                normalized_stake: FixedU128::from_inner(0),
                delegating_to: None,
                registered: true,
            }
        );
    });
}

#[test]
fn validator_permits_are_capped() {
    test_utils::new_test_ext().execute_with(|| {
        let min_validator_stake = MinValidatorStake::<Test>::get();

        let validators: [u32; 5] = from_fn(|i| i as u32);

        for (idx, &id) in validators.iter().rev().enumerate() {
            let mut member = ConsensusMember::<Test>::default();
            member.update_weights(BoundedVec::truncate_from(vec![(
                (id + 1) % validators.len() as u32, // avoid self weights
                10,
            )]));
            ConsensusMembers::<Test>::set(id, Some(member));
            StakedBy::<Test>::set(id, id, Some(min_validator_stake * idx as u128));
        }

        MaxAllowedValidators::<Test>::set(3);
        let members = ConsensusMemberInput::<Test>::all_members();
        assert_eq!(members.len(), 5);

        for id in &validators[..3] {
            let member = &members[id];
            assert!(member.validator_permit);
            assert!(!member.weights.is_empty());
        }

        for id in &validators[3..] {
            let member = &members[id];
            assert!(!member.validator_permit);
            assert!(member.weights.is_empty());
        }
    });
}

#[test]
fn deregister_old_agents_and_registers_new() {
    test_utils::new_test_ext().execute_with(|| {
        ConsensusMembers::<Test>::set(0, Some(Default::default()));
        ConsensusMembers::<Test>::set(1, Some(Default::default()));

        WeightControlDelegation::<Test>::set(1, Some(0));

        for id in [1, 2] {
            register_empty_agent(id);
        }

        step_block(100);

        assert!(!ConsensusMembers::<Test>::contains_key(0));

        assert!(ConsensusMembers::<Test>::contains_key(1));
        assert!(WeightControlDelegation::<Test>::contains_key(1));
        assert!(ConsensusMembers::<Test>::contains_key(2));
    });
}

#[test]
fn pays_dividends_and_incentives() {
    test_utils::new_test_ext().execute_with(|| {
        EmissionRecyclingPercentage::<Test>::set(Percent::zero());
        TreasuryEmissionFee::<Test>::set(Percent::zero());

        let min_stake = MinAllowedStake::<Test>::get();
        MinValidatorStake::<Test>::set(min_stake);

        let mut member = ConsensusMember::<Test>::default();
        member.update_weights(BoundedVec::truncate_from(vec![(1, 10), (2, 30)]));

        ConsensusMembers::<Test>::set(0, Some(member));
        ConsensusMembers::<Test>::set(1, Some(Default::default()));
        ConsensusMembers::<Test>::set(2, Some(Default::default()));

        add_stake(0, 0, 0);
        add_balance(1, 1);
        add_balance(2, 1);

        for id in [0, 1, 2] {
            register_empty_agent(id);
        }

        step_block(100);

        let total_emission = get_total_emission_per_block::<Test>() * 100;
        let mut sum = 0;

        let stake = StakedBy::<Test>::get(0, 0).unwrap_or_default();
        assert_eq!(stake - min_stake, total_emission / 2);
        sum += stake;

        let stake = StakedBy::<Test>::get(1, 1).unwrap_or_default();
        assert_eq!(stake, (total_emission / 2) / 4);
        sum += stake;

        let stake = StakedBy::<Test>::get(2, 2).unwrap_or_default();
        assert_eq!(stake, ((total_emission / 2) / 4) * 3);
        sum += stake;

        sum -= min_stake;

        assert_eq!(PendingEmission::<Test>::get(), 0);
        assert_eq!(sum, get_total_emission_per_block::<Test>() * 100);
    });
}

#[test]
fn pays_dividends_to_stakers() {
    test_utils::new_test_ext().execute_with(|| {
        let (min_validator_stake, _) = set_emissions_params();

        let validator = 0;
        let miner = 1;

        let staking_fee = Percent::from_float(0.25);

        let mut member = ConsensusMember::<Test>::default();
        member.update_weights(BoundedVec::truncate_from(vec![(miner, 1)]));

        ConsensusMembers::<Test>::set(validator, Some(member));
        ConsensusMembers::<Test>::set(miner, Some(Default::default()));

        for id in [validator, miner] {
            register_empty_agent(id);
        }

        Agents::<Test>::mutate_extant(validator, |agent| agent.fees.staking_fee = staking_fee);

        let stakers = [validator, 2, 3, 4];
        for (idx, id) in stakers.iter().enumerate() {
            let stake = (idx + 1) as u128 * min_validator_stake;
            add_stake(*id, validator, stake);
        }

        step_block(100);

        let total_emission = get_total_emission_per_block::<Test>() * 100;
        let mut sum = 0;

        let dividends = total_emission / 2;
        let incentives = total_emission / 2;

        assert_eq!(sum_staked_by::<Test>(&miner), incentives);
        sum += incentives;

        let stake_parts = (stakers.len() * (stakers.len() + 1) / 2) as u128;
        let stake_part_value = dividends / stake_parts;

        let mut validator_dividends = 0;
        let mut expected_validator_dividends = 0;
        let mut total_payed_fee = 0;

        for (idx, id) in stakers.iter().enumerate() {
            let stake_parts = (idx + 1) as u128;
            let pre_staked = stake_parts * min_validator_stake;

            let dividend = StakedBy::<Test>::get(validator, id).unwrap_or_default() - pre_staked;
            let expected_dividend = stake_parts * stake_part_value;

            let payed_fee = staking_fee.mul_floor(expected_dividend);

            if *id == validator {
                validator_dividends = dividend;
                expected_validator_dividends = expected_dividend;
            } else {
                let expected_stake = expected_dividend - payed_fee;
                assert_eq!(dividend, expected_stake);

                total_payed_fee += payed_fee;
            }

            sum += dividend;
        }

        assert_eq!(
            validator_dividends,
            expected_validator_dividends + total_payed_fee
        );

        assert_eq!(PendingEmission::<Test>::get(), 0);
        assert_eq!(sum, get_total_emission_per_block::<Test>() * 100);

        let validator = ConsensusMembers::<Test>::get(validator).unwrap();
        assert_eq!(validator.last_dividends, u16::MAX);
        assert_eq!(validator.last_incentives, 0);
        let miner = ConsensusMembers::<Test>::get(miner).unwrap();
        assert_eq!(miner.last_incentives, u16::MAX);
        assert_eq!(miner.last_dividends, 0);
    });
}

#[test]
fn pays_weight_control_fee_and_dividends_to_stakers() {
    test_utils::new_test_ext().execute_with(|| {
        let (min_validator_stake, weight_control_fee) = set_emissions_params();

        let val_1 = 0;
        let val_2 = 1;

        let miner = 2;

        let mut member = ConsensusMember::<Test>::default();
        member.update_weights(BoundedVec::truncate_from(vec![(miner, 1)]));

        ConsensusMembers::<Test>::set(val_1, Some(member));
        ConsensusMembers::<Test>::set(val_2, Some(Default::default()));
        ConsensusMembers::<Test>::set(miner, Some(Default::default()));

        for id in [val_1, val_2, miner] {
            register_empty_agent(id);
        }

        Allocators::<Test>::set(val_1, Some(()));

        pallet_emission0::weight_control::delegate_weight_control::<Test>(val_2, val_1)
            .expect("failed to delegate weight control");

        let val_1_staker = 3;
        add_stake(val_1_staker, val_1, min_validator_stake);

        let val_2_staker = 4;
        add_stake(val_2_staker, val_2, min_validator_stake);

        step_block(100);

        let total_dividends = (get_total_emission_per_block::<Test>() * 100) / 2;

        let mut val_1_stake = total_dividends / 2;
        let mut val_2_stake = total_dividends / 2;

        let val_2_weight_control_fee = weight_control_fee.mul_floor(val_2_stake);

        val_1_stake += val_2_weight_control_fee;
        val_2_stake -= val_2_weight_control_fee;

        assert_eq!(
            StakedBy::<Test>::get(val_1, val_1_staker).unwrap_or_default() - min_validator_stake,
            val_1_stake
        );
        assert_eq!(
            StakedBy::<Test>::get(val_2, val_2_staker).unwrap_or_default() - min_validator_stake,
            val_2_stake
        );

        let val_1 = ConsensusMembers::<Test>::get(val_1).unwrap();
        assert_eq!(val_1.last_dividends, u16::MAX);

        let factor =
            Perbill::from_rational(val_1_stake.min(val_2_stake), val_1_stake.max(val_2_stake));
        let val_2 = ConsensusMembers::<Test>::get(val_2).unwrap();
        assert_eq!(
            val_2.last_dividends,
            factor.mul_floor(u16::MAX as u32) as u16
        );
    });
}

#[test]
fn pays_according_to_incentives_ratio() {
    for ratio in 0..=100 {
        test_utils::new_test_ext().execute_with(|| {
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

            step_block(100);

            let total_emission = get_total_emission_per_block::<Test>() * 100;
            let total_incentives = incentives_ratio.mul_floor(total_emission);
            let dividends_ratio = Percent::one() - incentives_ratio;
            let total_dividends = dividends_ratio.mul_floor(total_emission);

            assert_eq!(
                StakedBy::<Test>::get(val, val).unwrap_or_default() - min_validator_stake,
                total_dividends
            );
            assert_eq!(
                StakedBy::<Test>::get(miner, miner).unwrap_or_default() - min_validator_stake,
                total_incentives
            );

            let val = ConsensusMembers::<Test>::get(val).unwrap();
            assert_eq!(val.last_dividends, if ratio == 100 { 0 } else { u16::MAX });

            let miner = ConsensusMembers::<Test>::get(miner).unwrap();
            assert_eq!(miner.last_incentives, if ratio == 0 { 0 } else { u16::MAX },);
        });
    }
}

#[test]
fn pays_weight_delegation_fee_to_validators_without_permits() {
    test_utils::new_test_ext().execute_with(|| {
        let (min_validator_stake, weight_control_fee) = set_emissions_params();

        let val_1 = 0;
        let val_2 = 1;

        let miner = 2;

        let mut member = ConsensusMember::<Test>::default();
        member.update_weights(BoundedVec::truncate_from(vec![(miner, 1)]));

        ConsensusMembers::<Test>::set(val_1, Some(member));
        ConsensusMembers::<Test>::set(val_2, Some(Default::default()));
        ConsensusMembers::<Test>::set(miner, Some(Default::default()));

        for id in [val_1, val_2, miner] {
            register_empty_agent(id);
        }

        Allocators::<Test>::set(val_1, Some(()));

        pallet_emission0::weight_control::delegate_weight_control::<Test>(val_2, val_1)
            .expect("failed to delegate weight control");

        add_stake(val_2, val_2, min_validator_stake);

        step_block(100);

        let total_dividends = (get_total_emission_per_block::<Test>() * 100) / 2;

        assert_eq!(
            StakedBy::<Test>::get(val_1, val_1).unwrap_or_default(),
            weight_control_fee.mul_floor(total_dividends)
        );

        assert_eq!(
            StakedBy::<Test>::get(val_2, val_2).unwrap_or_default() - min_validator_stake,
            total_dividends - weight_control_fee.mul_floor(total_dividends)
        );
    });
}

#[test]
fn ignores_agents_delegating_to_non_existing_agents() {
    test_utils::new_test_ext().execute_with(|| {
        let (min_validator_stake, _) = set_emissions_params();

        let val = 0;
        let miner = 1;
        let non_existing_val = 2;

        ConsensusMembers::<Test>::set(val, Some(Default::default()));
        ConsensusMembers::<Test>::set(miner, Some(Default::default()));

        for id in [val, miner] {
            register_empty_agent(id);
        }

        WeightControlDelegation::<Test>::set(val, Some(non_existing_val));

        add_stake(val, val, min_validator_stake);

        step_block(100);

        assert_eq!(
            StakedBy::<Test>::get(val, val).unwrap_or_default() - min_validator_stake,
            0
        );
        assert_eq!(StakedBy::<Test>::get(miner, miner).unwrap_or_default(), 0);

        let validator = ConsensusMembers::<Test>::get(val).unwrap();
        assert_eq!(validator.last_dividends, 0);
        let miner = ConsensusMembers::<Test>::get(miner).unwrap();
        assert_eq!(miner.last_incentives, 0);
    });
}

fn set_emissions_params() -> (u128, Percent) {
    EmissionRecyclingPercentage::<Test>::set(Percent::zero());
    TreasuryEmissionFee::<Test>::set(Percent::zero());

    let weight_control_fee = Percent::from_parts(25);
    FeeConstraints::<Test>::mutate(|constraints| {
        constraints.min_staking_fee = Percent::zero();
        constraints.min_weight_control_fee = weight_control_fee;
    });

    let min_validator_stake = 1;
    MinValidatorStake::<Test>::set(min_validator_stake);
    MinAllowedStake::<Test>::set(min_validator_stake);

    (min_validator_stake, weight_control_fee)
}
