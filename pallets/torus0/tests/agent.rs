use pallet_torus0::{
    agent::Agent, AgentUpdateCooldown, Agents, Burn, Error, ImmunityPeriod, MaxAllowedAgents,
};
use polkadot_sdk::{frame_support::assert_err, sp_core::Get, sp_runtime::Percent};
use test_utils::{
    assert_ok, clear_cooldown, get_origin,
    pallet_emission0::{ConsensusMembers, WeightControlDelegation},
    pallet_governance::{self, Allocators},
    step_block, Test,
};

#[test]
fn register_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let agent_id = 0;
        let allocator_id = 1;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        // Register allocator
        Agents::<Test>::set(
            allocator_id,
            Some(Agent {
                key: allocator_id,
                name: Default::default(),
                url: Default::default(),
                metadata: Default::default(),
                weight_penalty_factor: Default::default(),
                registration_block: Default::default(),
                fees: Default::default(),
                last_update_block: Default::default(),
            }),
        );
        Allocators::<Test>::set(allocator_id, Some(()));

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent_id
        ));

        assert_ok!(pallet_torus0::agent::register::<Test>(
            agent_id,
            agent_id,
            name.clone(),
            url.clone(),
            metadata.clone(),
        ));

        let agent = Agents::<Test>::get(agent_id).expect("it should exists");

        assert_eq!(agent.name.to_vec(), name);
        assert_eq!(agent.url.to_vec(), url);
        assert_eq!(agent.metadata.to_vec(), metadata);

        assert_eq!(
            WeightControlDelegation::<Test>::get(agent.key),
            Some(allocator_id)
        );

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent_id,
                agent_id,
                name.clone(),
                url.clone(),
                metadata.clone(),
            ),
            Error::<Test>::AgentAlreadyRegistered
        );
    });
}

#[test]
fn register_without_being_whitelisted() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                name.clone(),
                url.clone(),
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::AgentKeyNotWhitelisted
        );

        assert!(pallet_torus0::Agents::<Test>::get(agent).is_none());
    });
}

#[test]
fn register_without_enough_balance() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        Burn::<Test>::set(100);

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                name.clone(),
                url.clone(),
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::NotEnoughBalanceToRegisterAgent
        );

        assert!(pallet_torus0::Agents::<Test>::get(agent).is_none());
    });
}

#[test]
fn register_fail_name_validation() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                "".as_bytes().to_vec(),
                url.clone(),
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::AgentNameTooShort
        );

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                " ".repeat(pallet_torus0::MaxNameLength::<Test>::get() as usize + 1)
                    .as_bytes()
                    .to_vec(),
                url.clone(),
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::AgentNameTooLong
        );

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                vec![249u8, 9u8, 42u8],
                url.clone(),
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::InvalidAgentName
        );
    });
}

#[test]
fn register_fail_url_validation() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                name.clone(),
                "".as_bytes().to_vec(),
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::AgentUrlTooShort
        );

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                name.clone(),
                "X".repeat(pallet_torus0::MaxAgentUrlLength::<Test>::get() as usize + 1)
                    .as_bytes()
                    .to_vec(),
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::AgentUrlTooLong
        );

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                name.clone(),
                vec![249u8, 9u8, 42u8],
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::InvalidAgentUrl
        );
    });
}

#[test]
fn register_fail_metadata_validation() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                name.clone(),
                url.clone(),
                "".as_bytes().to_vec(),
            ),
            pallet_torus0::Error::<Test>::AgentMetadataTooShort
        );

        let max_metadata_length =
            <<Test as pallet_torus0::Config>::MaxAgentMetadataLengthConstraint as Get<u32>>::get()
                as usize;

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                name.clone(),
                url.clone(),
                " ".repeat(max_metadata_length + 1).as_bytes().to_vec(),
            ),
            pallet_torus0::Error::<Test>::AgentMetadataTooLong
        );

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                name.clone(),
                url.clone(),
                vec![249u8, 9u8, 42u8],
            ),
            pallet_torus0::Error::<Test>::InvalidAgentMetadata
        );
    });
}

#[test]
fn register_more_than_max_allowed_agents() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        pallet_torus0::MaxAllowedAgents::<Test>::set(0);

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                name.clone(),
                url.clone(),
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::MaxAllowedAgents,
        );

        assert!(pallet_torus0::Agents::<Test>::get(agent).is_none());
    });
}

#[test]
fn register_more_than_allowed_registrations_per_block() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        pallet_torus0::MaxRegistrationsPerBlock::<Test>::set(0);

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                name.clone(),
                url.clone(),
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::TooManyAgentRegistrationsThisBlock,
        );

        assert!(pallet_torus0::Agents::<Test>::get(agent).is_none());
    });
}

#[test]
fn register_more_than_registrations_per_interval() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        pallet_torus0::BurnConfig::<Test>::mutate(|config| {
            config.max_registrations_per_interval = 0;
        });

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                name.clone(),
                url.clone(),
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::TooManyAgentRegistrationsThisInterval,
        );

        assert!(pallet_torus0::Agents::<Test>::get(agent).is_none());
    });
}

#[test]
fn unregister_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_ok!(pallet_torus0::Pallet::<Test>::register_agent(
            get_origin(agent),
            agent,
            name.clone(),
            url.clone(),
            metadata.clone(),
        ));

        assert_ok!(pallet_torus0::Pallet::<Test>::unregister_agent(get_origin(
            agent
        )));

        assert!(pallet_torus0::Agents::<Test>::get(agent).is_none());
    });
}

#[test]
fn unregister_twice() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_ok!(pallet_torus0::Pallet::<Test>::register_agent(
            get_origin(agent),
            agent,
            name.clone(),
            url.clone(),
            metadata.clone(),
        ));

        assert_ok!(pallet_torus0::Pallet::<Test>::unregister_agent(get_origin(
            agent
        )));
        assert_err!(
            pallet_torus0::agent::unregister::<Test>(agent),
            Error::<Test>::AgentDoesNotExist
        );

        assert!(pallet_torus0::Agents::<Test>::get(agent).is_none());
    });
}

#[test]
fn update_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        clear_cooldown();

        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_err!(
            pallet_torus0::agent::update::<Test>(
                agent,
                b"".to_vec(),
                b"".to_vec(),
                None,
                None,
                None,
            ),
            Error::<Test>::AgentDoesNotExist
        );

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_ok!(pallet_torus0::Pallet::<Test>::register_agent(
            get_origin(agent),
            agent,
            name,
            url,
            metadata,
        ));

        let new_name = "new-agent".as_bytes().to_vec();
        let new_url = "new-idk://agent".as_bytes().to_vec();
        let new_metadata = "new-idk://agent".as_bytes().to_vec();

        let constraints = pallet_torus0::FeeConstraints::<Test>::get();
        let staking_fee = constraints.min_staking_fee;
        let weight_control_fee = constraints.min_weight_control_fee;

        assert_ok!(pallet_torus0::Pallet::<Test>::update_agent(
            get_origin(agent),
            new_name.clone(),
            new_url.clone(),
            Some(new_metadata.clone()),
            Some(staking_fee),
            Some(weight_control_fee),
        ));

        let agent = Agents::<Test>::get(agent).expect("it should exists");

        assert_eq!(agent.name.to_vec(), new_name);
        assert_eq!(agent.url.to_vec(), new_url);
        assert_eq!(agent.metadata.to_vec(), new_metadata);
        assert_eq!(agent.fees.staking_fee, staking_fee);
        assert_eq!(agent.fees.weight_control_fee, weight_control_fee);
    });
}

#[test]
fn update_with_zero_staking_fee() {
    test_utils::new_test_ext().execute_with(|| {
        clear_cooldown();

        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_ok!(pallet_torus0::agent::register::<Test>(
            agent,
            agent,
            name.clone(),
            url.clone(),
            metadata.clone(),
        ));

        let new_name = "new-agent".as_bytes().to_vec();
        let new_url = "new-idk://agent".as_bytes().to_vec();
        let new_metadata = "new-idk://agent".as_bytes().to_vec();

        let constraints = pallet_torus0::FeeConstraints::<Test>::get();
        let staking_fee = constraints.min_staking_fee;
        let weight_control_fee = constraints.min_weight_control_fee;

        assert_err!(
            pallet_torus0::agent::update::<Test>(
                agent,
                new_name.clone(),
                new_url.clone(),
                Some(new_metadata.clone()),
                Some(Percent::zero()),
                Some(weight_control_fee),
            ),
            Error::<Test>::InvalidStakingFee,
        );

        let agent = Agents::<Test>::get(agent).expect("it should exists");

        assert_eq!(agent.name.to_vec(), name);
        assert_eq!(agent.url.to_vec(), url);
        assert_eq!(agent.metadata.to_vec(), metadata);
        assert_eq!(agent.fees.staking_fee, staking_fee);
        assert_eq!(agent.fees.weight_control_fee, weight_control_fee);
    });
}

#[test]
fn update_with_zero_weight_control_fee() {
    test_utils::new_test_ext().execute_with(|| {
        clear_cooldown();

        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_ok!(pallet_torus0::agent::register::<Test>(
            agent,
            agent,
            name.clone(),
            url.clone(),
            metadata.clone(),
        ));

        let new_name = "new-agent".as_bytes().to_vec();
        let new_url = "new-idk://agent".as_bytes().to_vec();
        let new_metadata = "new-idk://agent".as_bytes().to_vec();

        let constraints = pallet_torus0::FeeConstraints::<Test>::get();
        let staking_fee = constraints.min_staking_fee;
        let weight_control_fee = constraints.min_weight_control_fee;

        assert_err!(
            pallet_torus0::agent::update::<Test>(
                agent,
                new_name.clone(),
                new_url.clone(),
                Some(new_metadata.clone()),
                Some(staking_fee),
                Some(Percent::zero()),
            ),
            Error::<Test>::InvalidWeightControlFee,
        );

        let agent = Agents::<Test>::get(agent).expect("it should exists");

        assert_eq!(agent.name.to_vec(), name);
        assert_eq!(agent.url.to_vec(), url);
        assert_eq!(agent.metadata.to_vec(), metadata);
        assert_eq!(agent.fees.staking_fee, staking_fee);
        assert_eq!(agent.fees.weight_control_fee, weight_control_fee);
    });
}

#[test]
fn prunes_excess_agents() {
    test_utils::new_test_ext().execute_with(|| {
        let immunity_period = 10;
        ImmunityPeriod::<Test>::set(immunity_period as u16);

        let current_block = immunity_period * 2;
        step_block(current_block);

        let max_allowed_agents = 3u32;
        MaxAllowedAgents::<Test>::set(max_allowed_agents as u16);

        const IMMUNE_KEY: u32 = 1;
        const ZEROED_SCORE_KEY: u32 = 2;
        const MAX_OLD_VALIDATOR_KEY: u32 = 3;
        const MAX_VALIDATOR_KEY: u32 = 4;
        const MIN_VALIDATOR_KEY: u32 = 5;
        for key in 0..6 {
            let registration_block = if key == IMMUNE_KEY {
                current_block
            } else {
                key as u64
            };

            Agents::<Test>::set(
                key,
                Some(Agent {
                    key,
                    name: Default::default(),
                    url: Default::default(),
                    metadata: Default::default(),
                    weight_penalty_factor: Default::default(),
                    registration_block,
                    fees: Default::default(),
                    last_update_block: registration_block,
                }),
            );

            if ![IMMUNE_KEY, ZEROED_SCORE_KEY].contains(&key) {
                ConsensusMembers::<Test>::mutate(key, |member| {
                    let member = member.get_or_insert_with(Default::default);
                    member.last_incentives = 40;
                    match key {
                        MIN_VALIDATOR_KEY => member.last_dividends = 100,
                        MAX_OLD_VALIDATOR_KEY | MAX_VALIDATOR_KEY => member.last_dividends = 200,
                        _ => {}
                    }
                })
            }
        }

        let agent_id = 6;
        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent_id
        ));

        assert_ok!(pallet_torus0::agent::register::<Test>(
            agent_id,
            agent_id,
            b"agent".to_vec(),
            b"idk://agent".to_vec(),
            b"idk://agent".to_vec(),
        ));

        let expected_keys = [IMMUNE_KEY, MAX_VALIDATOR_KEY, agent_id];
        assert_eq!(Agents::<Test>::iter().count(), expected_keys.len());

        for expected_key in expected_keys {
            assert!(Agents::<Test>::contains_key(expected_key));
        }

        let _ = Agents::<Test>::clear(u32::MAX, None);
        MaxAllowedAgents::<Test>::set(1);

        Agents::<Test>::set(
            0,
            Some(Agent {
                key: 0,
                name: Default::default(),
                url: Default::default(),
                metadata: Default::default(),
                weight_penalty_factor: Default::default(),
                registration_block: current_block,
                fees: Default::default(),
                last_update_block: current_block,
            }),
        );

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent_id,
                agent_id,
                b"agent".to_vec(),
                b"idk://agent".to_vec(),
                b"idk://agent".to_vec(),
            ),
            Error::<Test>::MaxAllowedAgents
        );
    });
}

#[test]
fn fails_updating_whitout_waiting_cooldown() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_err!(
            pallet_torus0::agent::update::<Test>(
                agent,
                b"".to_vec(),
                b"".to_vec(),
                None,
                None,
                None,
            ),
            Error::<Test>::AgentDoesNotExist
        );

        assert_ok!(pallet_governance::whitelist::add_to_whitelist::<Test>(
            agent
        ));

        assert_ok!(pallet_torus0::Pallet::<Test>::register_agent(
            get_origin(agent),
            agent,
            name,
            url,
            metadata,
        ));

        let new_name = "new-agent".as_bytes().to_vec();
        let new_url = "new-idk://agent".as_bytes().to_vec();
        let new_metadata = "new-idk://agent".as_bytes().to_vec();

        let constraints = pallet_torus0::FeeConstraints::<Test>::get();
        let staking_fee = constraints.min_staking_fee;
        let weight_control_fee = constraints.min_weight_control_fee;

        assert_err!(
            pallet_torus0::Pallet::<Test>::update_agent(
                get_origin(agent),
                new_name.clone(),
                new_url.clone(),
                Some(new_metadata.clone()),
                Some(staking_fee),
                Some(weight_control_fee),
            ),
            crate::Error::<Test>::AgentUpdateOnCooldown
        );

        step_block(AgentUpdateCooldown::<Test>::get());

        assert_ok!(pallet_torus0::Pallet::<Test>::update_agent(
            get_origin(agent),
            new_name.clone(),
            new_url.clone(),
            Some(new_metadata.clone()),
            Some(staking_fee),
            Some(weight_control_fee),
        ));

        assert_err!(
            pallet_torus0::Pallet::<Test>::update_agent(
                get_origin(agent),
                new_name.clone(),
                new_url.clone(),
                Some(new_metadata.clone()),
                Some(staking_fee),
                Some(weight_control_fee),
            ),
            crate::Error::<Test>::AgentUpdateOnCooldown
        );
    });
}
