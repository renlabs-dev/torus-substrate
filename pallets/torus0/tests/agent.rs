use pallet_governance_api::GovernanceApi;
use pallet_torus0::{agent::Agent, AgentUpdateCooldown, Agents, Burn, Error};
use pallet_torus0_api::NamespacePath;
use polkadot_sdk::{frame_support::assert_err, sp_core::Get, sp_runtime::Percent};
use test_utils::{
    assert_ok, clear_cooldown, get_balance, get_origin,
    pallet_emission0::{PendingEmission, WeightControlDelegation},
    pallet_governance::{self, Allocators, DaoTreasuryAddress, TreasuryEmissionFee},
    step_block, Governance, Test,
};

#[test]
fn register_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        PendingEmission::<Test>::set(0);
        TreasuryEmissionFee::<Test>::set(Percent::zero());
        let balance = get_balance(DaoTreasuryAddress::<Test>::get());

        let agent_id = 0;
        let allocator_id = 1;

        let name = b"alice".to_vec();
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

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
            allocator_id
        ));
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

        assert!(pallet_torus0::Namespaces::<Test>::contains_key(
            pallet_torus0::namespace::NamespaceOwnership::Account(agent_id),
            "agent.alice".parse::<NamespacePath>().unwrap()
        ));

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

        assert_eq!(
            get_balance(Governance::dao_treasury_address()),
            balance + Burn::<Test>::get()
        );
    });
}

#[test]
fn register_without_being_whitelisted() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = b"agent".to_vec();
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

        assert_ok!(pallet_torus0::agent::register::<Test>(
            agent,
            agent,
            name.clone(),
            url.clone(),
            metadata.clone(),
        ));

        assert_eq!(WeightControlDelegation::<Test>::get(agent), None);

        assert!(pallet_torus0::Agents::<Test>::get(agent).is_some());
    });
}

#[test]
fn register_without_enough_balance() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = b"agent".to_vec();
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

        Burn::<Test>::set(100);

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
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

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
            pallet_torus0::Error::<Test>::InvalidNamespacePath
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
            pallet_torus0::Error::<Test>::InvalidNamespacePath
        );

        assert_err!(
            pallet_torus0::agent::register::<Test>(
                agent,
                agent,
                vec![249u8, 9u8, 42u8],
                url.clone(),
                metadata.clone(),
            ),
            pallet_torus0::Error::<Test>::InvalidNamespacePath
        );
    });
}

#[test]
fn register_fail_url_validation() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = b"agent".to_vec();
        let metadata = b"idk://agent".to_vec();

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
        let name = b"agent".to_vec();
        let url = b"idk://agent".to_vec();

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
fn register_more_than_allowed_registrations_per_block() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = b"agent".to_vec();
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

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
        let name = b"agent".to_vec();
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

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
        let name = b"alice".to_vec();
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

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

        assert!(pallet_torus0::Namespaces::<Test>::contains_key(
            pallet_torus0::namespace::NamespaceOwnership::Account(agent),
            "agent.alice".parse::<NamespacePath>().unwrap()
        ));

        assert_ok!(pallet_torus0::Pallet::<Test>::unregister_agent(get_origin(
            agent
        )));

        assert!(!pallet_torus0::Namespaces::<Test>::contains_key(
            pallet_torus0::namespace::NamespaceOwnership::Account(agent),
            "agent.alice".parse::<NamespacePath>().unwrap()
        ));

        assert!(pallet_torus0::Agents::<Test>::get(agent).is_none());
    });
}

#[test]
fn unregister_twice() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = b"agent".to_vec();
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

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
        let name = b"agent".to_vec();
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

        assert_err!(
            pallet_torus0::agent::update::<Test>(agent, b"".to_vec(), None, None, None,),
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

        let new_url = b"new-idk://agent".to_vec();
        let new_metadata = b"new-idk://agent".to_vec();

        let constraints = pallet_torus0::FeeConstraints::<Test>::get();
        let staking_fee = constraints.min_staking_fee;
        let weight_control_fee = constraints.min_weight_control_fee;

        assert_ok!(pallet_torus0::Pallet::<Test>::update_agent(
            get_origin(agent),
            new_url.clone(),
            Some(new_metadata.clone()),
            Some(staking_fee),
            Some(weight_control_fee),
        ));

        let agent = Agents::<Test>::get(agent).expect("it should exists");

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
        let name = b"agent".to_vec();
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

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

        let new_url = b"new-idk://agent".to_vec();
        let new_metadata = b"new-idk://agent".to_vec();

        let constraints = pallet_torus0::FeeConstraints::<Test>::get();
        let staking_fee = constraints.min_staking_fee;
        let weight_control_fee = constraints.min_weight_control_fee;

        assert_err!(
            pallet_torus0::agent::update::<Test>(
                agent,
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
        let name = b"agent".to_vec();
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

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

        let new_url = b"new-idk://agent".to_vec();
        let new_metadata = b"new-idk://agent".to_vec();

        let constraints = pallet_torus0::FeeConstraints::<Test>::get();
        let staking_fee = constraints.min_staking_fee;
        let weight_control_fee = constraints.min_weight_control_fee;

        assert_err!(
            pallet_torus0::agent::update::<Test>(
                agent,
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
fn fails_updating_whitout_waiting_cooldown() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = b"agent".to_vec();
        let url = b"idk://agent".to_vec();
        let metadata = b"idk://agent".to_vec();

        assert_err!(
            pallet_torus0::agent::update::<Test>(agent, b"".to_vec(), None, None, None,),
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

        let new_url = b"new-idk://agent".to_vec();
        let new_metadata = b"new-idk://agent".to_vec();

        let constraints = pallet_torus0::FeeConstraints::<Test>::get();
        let staking_fee = constraints.min_staking_fee;
        let weight_control_fee = constraints.min_weight_control_fee;

        assert_err!(
            pallet_torus0::Pallet::<Test>::update_agent(
                get_origin(agent),
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
            new_url.clone(),
            Some(new_metadata.clone()),
            Some(staking_fee),
            Some(weight_control_fee),
        ));

        assert_err!(
            pallet_torus0::Pallet::<Test>::update_agent(
                get_origin(agent),
                new_url.clone(),
                Some(new_metadata.clone()),
                Some(staking_fee),
                Some(weight_control_fee),
            ),
            crate::Error::<Test>::AgentUpdateOnCooldown
        );
    });
}

#[test]
fn agent_freezing() {
    test_utils::new_test_ext().execute_with(|| {
        pallet_governance::AgentsFrozen::<Test>::set(true);
        assert_err!(
            pallet_torus0::Pallet::<Test>::register_agent(
                get_origin(0),
                0,
                b"agent name".to_vec(),
                b"agent url".to_vec(),
                vec![],
            ),
            pallet_torus0::Error::<Test>::AgentsFrozen
        );
    });
}
