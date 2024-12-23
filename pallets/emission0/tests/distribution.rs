use pallet_emission0::{
    distribute::{get_total_emission_per_block, ConsensusMemberInput},
    Config, ConsensusMember, ConsensusMembers, PendingEmission, WeightControlDelegation,
};
use polkadot_sdk::{pallet_balances, sp_core::Get, sp_runtime::BoundedVec};
use substrate_fixed::{traits::ToFixed, types::I96F32};
use test_utils::{
    pallet_torus0::{self, agent::Agent},
    step_block, Test,
};

#[test]
fn total_emission_per_block_does_halving() {
    test_utils::new_test_ext().execute_with(|| {
        let block_emission = <<Test as Config>::BlockEmission as Get<u128>>::get();
        let halving_interval = <<Test as Config>::HalvingInterval as Get<u128>>::get();
        let max_supply = <<Test as Config>::MaxSupply as Get<u128>>::get();

        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(emissions, block_emission);

        pallet_balances::TotalIssuance::<Test>::set(halving_interval - 1);
        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(emissions, block_emission);

        pallet_balances::TotalIssuance::<Test>::set(halving_interval);
        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(emissions, block_emission >> 1);

        pallet_balances::TotalIssuance::<Test>::set(halving_interval * 2);
        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(emissions, block_emission >> 2);

        pallet_balances::TotalIssuance::<Test>::set(max_supply);
        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(emissions, 0);
    });
}

#[test]
fn pending_emission_accumulates_and_returns_when_network_is_empty() {
    test_utils::new_test_ext().execute_with(|| {
        assert_eq!(PendingEmission::<Test>::get(), 0);

        step_block(1);

        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(PendingEmission::<Test>::get(), emissions);

        step_block(1);

        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(PendingEmission::<Test>::get(), emissions * 2);

        step_block(98);

        let emissions = get_total_emission_per_block::<Test>();
        assert_eq!(PendingEmission::<Test>::get(), emissions * 100);
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

        pallet_torus0::Agents::<Test>::set(
            2,
            Some(Agent {
                key: 2,
                name: Default::default(),
                url: Default::default(),
                metadata: Default::default(),
                weight_factor: Default::default(),
            }),
        );

        let weights = ConsensusMemberInput::<Test>::prepare_weights(member, &0);
        assert_eq!(
            weights,
            vec![
                (1, 0.3333333333f64.to_fixed()),
                (2, 0.6666666665f64.to_fixed())
            ]
        )
    });
}

#[test]
fn creates_member_input_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let mut member = ConsensusMember::<Test>::default();

        let input = ConsensusMemberInput::<Test>::from_agent(0, member.clone(), 0);
        assert_eq!(
            input,
            ConsensusMemberInput {
                agent_id: 0,
                validator_permit: false,
                weights: vec![],
                stakes: vec![],
                total_stake: 0.to_fixed(),
                normalized_stake: 0.to_fixed(),
                delegating_to: None,
                registered: false
            }
        );

        pallet_torus0::Agents::<Test>::set(
            0,
            Some(Agent {
                key: 0,
                name: Default::default(),
                url: Default::default(),
                metadata: Default::default(),
                weight_factor: Default::default(),
            }),
        );

        let input = ConsensusMemberInput::<Test>::from_agent(0, member.clone(), 0);
        assert!(input.registered);

        pallet_torus0::StakedBy::<Test>::set(0, 1, Some(10));
        pallet_torus0::StakedBy::<Test>::set(0, 2, Some(20));

        ConsensusMembers::<Test>::set(1, Some(Default::default()));

        member.update_weights(BoundedVec::truncate_from(vec![(0, 0), (1, 10)]));

        let input = ConsensusMemberInput::<Test>::from_agent(0, member.clone(), 15);
        assert_eq!(input.total_stake, I96F32::from_num(30));
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

        let stake = pallet_torus0::MinimumAllowedStake::<Test>::get();

        for id in [validator, new, miner, delegating_registered] {
            pallet_torus0::Agents::<Test>::set(
                id,
                Some(Agent {
                    key: id,
                    name: Default::default(),
                    url: Default::default(),
                    metadata: Default::default(),
                    weight_factor: Default::default(),
                }),
            );
        }

        for id in [miner, delegating_registered, delegating_unregistered] {
            ConsensusMembers::<Test>::set(id, Some(Default::default()));
        }

        let mut member = ConsensusMember::<Test>::default();
        member.update_weights(BoundedVec::truncate_from(vec![(miner, 10)]));
        ConsensusMembers::<Test>::set(validator, Some(member));
        pallet_torus0::StakedBy::<Test>::set(validator, staker, Some(stake * 3));

        pallet_torus0::StakedBy::<Test>::set(delegating_registered, staker, Some(stake));

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
                weights: vec![(miner, 1.to_fixed())],
                stakes: vec![(staker, (stake * 3).to_fixed())],
                total_stake: (stake * 3).to_fixed(),
                normalized_stake: 0.75f64.to_fixed(),
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
                total_stake: 0.to_fixed(),
                normalized_stake: 0.to_fixed(),
                delegating_to: None,
                registered: true,
            }
        );

        assert_eq!(
            members[&delegating_registered],
            ConsensusMemberInput {
                agent_id: delegating_registered,
                validator_permit: true,
                weights: vec![(miner, 1.to_fixed())],
                stakes: vec![(staker, stake.to_fixed())],
                total_stake: stake.to_fixed(),
                normalized_stake: 0.25f64.to_fixed(),
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
                total_stake: 0.to_fixed(),
                normalized_stake: 0.to_fixed(),
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
                total_stake: 0.to_fixed(),
                normalized_stake: 0.to_fixed(),
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
                total_stake: 0.to_fixed(),
                normalized_stake: 0.to_fixed(),
                delegating_to: None,
                registered: true,
            }
        );
    });
}
