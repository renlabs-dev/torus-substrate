#![allow(clippy::indexing_slicing)]

use pallet_torus0::{namespace::NamespacePricingConfig, Error, NamespaceCount, Namespaces};
use pallet_torus0_api::NamespacePath;
use polkadot_sdk::{
    frame_support::{assert_err, assert_ok, traits::Currency},
    sp_runtime::Percent,
};
use test_utils::{as_tors, get_balance, get_origin, new_test_ext, Balances, Test};

fn setup_agent_with_balance(agent: u32, name: &str) {
    // Register agent with whitelist
    test_utils::pallet_governance::Whitelist::<Test>::insert(agent, ());

    // Create agent with specific name
    pallet_torus0::Agents::<Test>::insert(
        agent,
        pallet_torus0::agent::Agent {
            key: agent,
            name: name.as_bytes().to_vec().try_into().unwrap(),
            url: Default::default(),
            metadata: Default::default(),
            weight_penalty_factor: Default::default(),
            registration_block: polkadot_sdk::frame_system::Pallet::<Test>::block_number(),
            fees: Default::default(),
            last_update_block: Default::default(),
        },
    );

    let _ = Balances::deposit_creating(&agent, as_tors(10000));
}

#[test]
fn create_namespace_simple() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "alice");

        let initial_balance = get_balance(owner);

        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(owner),
            b"agent.alice".to_vec().try_into().unwrap()
        ));

        // Verify namespace was created
        let path: NamespacePath = "agent.alice".parse().unwrap();
        let namespace_owner = pallet_torus0::namespace::NamespaceOwner::Agent(owner);
        assert!(Namespaces::<Test>::contains_key(&namespace_owner, &path));

        // Verify metadata stored
        let metadata = Namespaces::<Test>::get(&namespace_owner, &path).unwrap();
        assert_eq!(metadata.created_at, 0); // Block 0
        assert_eq!(metadata.deposit, as_tors(55)); // 11 bytes * 5 TORS

        // Verify count incremented (should be 2: "agent" and "agent.alice")
        assert_eq!(NamespaceCount::<Test>::get(&owner), 2);

        // Verify balance deduction (fee + deposit)
        let final_balance = get_balance(owner);
        assert!(final_balance < initial_balance);
    });
}

#[test]
fn create_namespace_with_hierarchy() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "alice");

        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(owner),
            b"agent.alice.memory.twitter".to_vec().try_into().unwrap()
        ));

        // Verify all parent paths were created
        let paths = [
            "agent",
            "agent.alice",
            "agent.alice.memory",
            "agent.alice.memory.twitter",
        ];

        let namespace_owner = pallet_torus0::namespace::NamespaceOwner::Agent(owner);
        for path in paths {
            let namespace_path: NamespacePath = path.parse().unwrap();
            assert!(
                Namespaces::<Test>::contains_key(&namespace_owner, &namespace_path),
                "Path {} should exist",
                path
            );
        }

        // Verify namespace count
        assert_eq!(NamespaceCount::<Test>::get(&owner), 4);
    });
}

#[test]
fn create_namespace_partial_hierarchy_exists() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "alice");

        // Create parent namespaces first
        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(owner),
            b"agent.alice".to_vec().try_into().unwrap()
        ));

        let initial_count = NamespaceCount::<Test>::get(&owner);
        assert_eq!(initial_count, 2); // "agent" and "agent.alice"

        // Create deeper namespace
        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(owner),
            b"agent.alice.memory.twitter".to_vec().try_into().unwrap()
        ));

        // Verify only missing paths were created
        assert_eq!(NamespaceCount::<Test>::get(&owner), 4); // +2 new ones

        let namespace_owner = pallet_torus0::namespace::NamespaceOwner::Agent(owner);
        let new_paths = ["agent.alice.memory", "agent.alice.memory.twitter"];
        for path in new_paths {
            let namespace_path: NamespacePath = path.parse().unwrap();
            assert!(Namespaces::<Test>::contains_key(
                &namespace_owner,
                &namespace_path
            ));
        }
    });
}

#[test]
fn create_namespace_already_exists() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "alice");

        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(owner),
            b"agent.alice".to_vec().try_into().unwrap()
        ));

        // Try to create same namespace again
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                b"agent.alice".to_vec().try_into().unwrap()
            ),
            Error::<Test>::NamespaceAlreadyExists
        );
    });
}

#[test]
fn create_namespace_not_an_agent() {
    new_test_ext().execute_with(|| {
        let non_agent = 999;

        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(non_agent),
                b"agent.test".to_vec().try_into().unwrap()
            ),
            Error::<Test>::AgentDoesNotExist
        );
    });
}

#[test]
fn create_namespace_invalid_path() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "alice");

        // Empty path
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                b"".to_vec().try_into().unwrap()
            ),
            Error::<Test>::InvalidNamespacePath
        );

        // Path starting with dot
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                b".agent".to_vec().try_into().unwrap()
            ),
            Error::<Test>::InvalidNamespacePath
        );

        // Path ending with dot
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                b"agent.".to_vec().try_into().unwrap()
            ),
            Error::<Test>::InvalidNamespacePath
        );

        // Double dots
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                b"agent..alice".to_vec().try_into().unwrap()
            ),
            Error::<Test>::InvalidNamespacePath
        );

        // Invalid characters
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                b"agent.alice!".to_vec().try_into().unwrap()
            ),
            Error::<Test>::InvalidNamespacePath
        );

        // Spaces
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                b"agent alice".to_vec().try_into().unwrap()
            ),
            Error::<Test>::InvalidNamespacePath
        );
    });
}

#[test]
fn create_namespace_wrong_agent_name() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "alice");

        // Try to create namespace for different agent name
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                b"agent.bob.memory".to_vec().try_into().unwrap()
            ),
            Error::<Test>::NamespaceNotOwnedByAgent
        );

        // Try without agent prefix
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                b"memory".to_vec().try_into().unwrap()
            ),
            Error::<Test>::InvalidNamespacePath
        );

        // Try with wrong prefix
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                b"system.alice".to_vec().try_into().unwrap()
            ),
            Error::<Test>::InvalidNamespacePath
        );
    });
}

#[test]
fn create_namespace_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        // Register agent manually with name
        test_utils::pallet_governance::Whitelist::<Test>::insert(owner, ());
        pallet_torus0::Agents::<Test>::insert(
            owner,
            pallet_torus0::agent::Agent {
                key: owner,
                name: b"alice".to_vec().try_into().unwrap(),
                url: Default::default(),
                metadata: Default::default(),
                weight_penalty_factor: Default::default(),
                registration_block: polkadot_sdk::frame_system::Pallet::<Test>::block_number(),
                fees: Default::default(),
                last_update_block: Default::default(),
            },
        );
        // Give minimal balance
        let _ = Balances::deposit_creating(&owner, 100);

        // This should be less than required for namespace creation

        // Should fail due to insufficient funds
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                b"agent.alice".to_vec().try_into().unwrap()
            ),
            polkadot_sdk::pallet_balances::Error::<Test>::InsufficientBalance
        );
    });
}

#[test]
fn create_namespace_fee_increases_with_count() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "alice");

        // First create the base namespace
        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(owner),
            b"agent.alice".to_vec().try_into().unwrap()
        ));

        // Track balance changes for each namespace creation
        let mut costs = Vec::new();

        for i in 0..5 {
            let initial_balance = get_balance(owner);

            let namespace = format!("agent.alice.namespace{}", i);
            assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                namespace.as_bytes().to_vec().try_into().unwrap()
            ));

            let final_balance = get_balance(owner);
            let cost = initial_balance - final_balance;
            costs.push(cost);
        }

        // Verify costs are increasing (allowing for small variations due to fee calculation)
        for i in 1..costs.len() {
            assert!(
                costs[i] >= costs[i - 1],
                "Cost should not decrease: {} >= {} at iteration {}",
                costs[i],
                costs[i - 1],
                i
            );
        }

        // Count should be 7: "agent", "agent.alice", and 5 namespace{i}
        assert_eq!(NamespaceCount::<Test>::get(&owner), 7);
    });
}

#[test]
fn create_namespace_with_special_characters() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "alice-1");

        // Test all allowed special characters in namespace segments
        let valid_paths = [
            "agent.alice-1.service_v2",
            "agent.alice-1.model+vision",
            "agent.alice-1.key=value",
            "agent.alice-1.tag1+tag2",
            "agent.alice-1.service_v2-beta",
            "agent.alice-1.model=gpt4+vision",
        ];

        for path in valid_paths {
            assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                path.as_bytes().to_vec().try_into().unwrap()
            ));
        }
    });
}

#[test]
fn create_namespace_deposit_calculation() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "test");

        // Update pricing config to test deposit calculation
        pallet_torus0::NamespacePricingConfig::<Test>::put(NamespacePricingConfig {
            base_fee: as_tors(10),
            deposit_per_byte: as_tors(2), // 2 TORS per byte
            count_midpoint: 20,
            fee_steepness: Percent::from_percent(20),
            max_fee_multiplier: 5,
        });

        let initial_balance = get_balance(owner);

        // Create namespace with known length
        let path = b"agent.test.api"; // 14 bytes
        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(owner),
            path.to_vec().try_into().unwrap()
        ));

        // Verify deposit amount for the specific path
        let namespace_path: NamespacePath = "agent.test.api".parse().unwrap();
        let namespace_owner = pallet_torus0::namespace::NamespaceOwner::Agent(owner);
        let metadata = Namespaces::<Test>::get(&namespace_owner, &namespace_path).unwrap();
        assert_eq!(metadata.deposit, as_tors(28)); // 14 bytes * 2 TORS

        // Verify total cost includes deposits for all created paths
        let final_balance = get_balance(owner);
        let total_cost = initial_balance - final_balance;
        // Total deposit: "agent" (5*2) + "agent.test" (10*2) + "agent.test.api" (14*2) = 58 TORS
        // Plus fee for 3 namespaces
        assert!(total_cost >= as_tors(88)); // Min 30 TORS fee (3*10) + 58 TORS deposit
    });
}

#[test]
fn create_namespace_event_emitted() {
    new_test_ext().execute_with(|| {
        // Initialize the block
        polkadot_sdk::frame_system::Pallet::<Test>::set_block_number(1);

        let owner = 1;
        setup_agent_with_balance(owner, "alice");

        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(owner),
            b"agent.alice.memory".to_vec().try_into().unwrap()
        ));

        // Check events
        let events = polkadot_sdk::frame_system::Pallet::<Test>::events();
        let namespace_events: Vec<_> = events
            .iter()
            .filter_map(|record| match &record.event {
                test_utils::RuntimeEvent::Torus0(pallet_torus0::Event::NamespaceCreated {
                    owner: event_owner,
                    path,
                    deposit,
                }) => Some((event_owner.clone(), path.clone(), *deposit)),
                _ => None,
            })
            .collect();

        assert!(
            !namespace_events.is_empty(),
            "No NamespaceCreated events found"
        );

        // Only one event is emitted for the entire operation
        assert_eq!(namespace_events.len(), 1);
        let (event_owner, path, deposit) = &namespace_events[0];
        assert_eq!(*event_owner, owner);
        assert_eq!(path.as_bytes(), b"agent.alice.memory");
        // Deposit for "agent" (5 bytes) + "agent.alice" (11 bytes) + "agent.alice.memory" (18 bytes) = 34 * 5 = 170 TORS
        assert_eq!(*deposit, as_tors(170));
    });
}

#[test]
fn create_namespace_max_depth() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "test");

        // Create namespace at maximum depth (10 segments)
        let max_depth_path = "agent.test.a.b.c.d.e.f.g.h";
        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(owner),
            max_depth_path.as_bytes().to_vec().try_into().unwrap()
        ));

        // Verify all 10 namespaces were created
        assert_eq!(NamespaceCount::<Test>::get(&owner), 10);

        // Try to exceed maximum depth (11 segments)
        let too_deep_path = "agent.test.a.b.c.d.e.f.g.h.i";
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                too_deep_path.as_bytes().to_vec().try_into().unwrap()
            ),
            Error::<Test>::InvalidNamespacePath
        );
    });
}

#[test]
fn create_namespace_concurrent_from_different_agents() {
    new_test_ext().execute_with(|| {
        let agent1 = 1;
        let agent2 = 2;
        setup_agent_with_balance(agent1, "alice");
        setup_agent_with_balance(agent2, "bob");

        // Both agents create their own namespaces
        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(agent1),
            b"agent.alice.api.v1".to_vec().try_into().unwrap()
        ));

        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(agent2),
            b"agent.bob.api.v1".to_vec().try_into().unwrap()
        ));

        // Verify each agent owns their own namespace
        let alice_path: NamespacePath = "agent.alice.api.v1".parse().unwrap();
        let bob_path: NamespacePath = "agent.bob.api.v1".parse().unwrap();
        let alice_owner = pallet_torus0::namespace::NamespaceOwner::Agent(agent1);
        let bob_owner = pallet_torus0::namespace::NamespaceOwner::Agent(agent2);

        assert!(Namespaces::<Test>::contains_key(&alice_owner, &alice_path));
        assert!(Namespaces::<Test>::contains_key(&bob_owner, &bob_path));

        // Verify counts are independent
        assert_eq!(NamespaceCount::<Test>::get(&agent1), 4); // agent + agent.alice + agent.alice.api + agent.alice.api.v1
        assert_eq!(NamespaceCount::<Test>::get(&agent2), 4); // agent + agent.bob + agent.bob.api + agent.bob.api.v1
    });
}

#[test]
fn create_namespace_treasury_receives_fees() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "alice");

        let treasury = test_utils::pallet_governance::DaoTreasuryAddress::<Test>::get();
        let initial_treasury_balance = get_balance(treasury);

        assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
            get_origin(owner),
            b"agent.alice".to_vec().try_into().unwrap()
        ));

        let final_treasury_balance = get_balance(treasury);
        assert!(final_treasury_balance > initial_treasury_balance);

        // Treasury should receive the fee (not the deposit)
        let fee_received = final_treasury_balance - initial_treasury_balance;
        assert!(fee_received >= as_tors(10)); // At least base fee * 2 (for "agent" and "agent.alice")
    });
}

#[test]
fn create_namespace_stress_many_segments() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        setup_agent_with_balance(owner, "alice");

        // Create a complex nested structure
        let paths = [
            "agent.alice.api",
            "agent.alice.api.v1",
            "agent.alice.api.v1.users",
            "agent.alice.api.v1.users.profile",
            "agent.alice.api.v2",
            "agent.alice.api.v2.users",
            "agent.alice.api.v2.posts",
            "agent.alice.data",
            "agent.alice.data.public",
            "agent.alice.data.private",
        ];

        let namespace_owner = pallet_torus0::namespace::NamespaceOwner::Agent(owner);

        for path in paths {
            assert_ok!(pallet_torus0::Pallet::<Test>::create_agent_namespace(
                get_origin(owner),
                path.as_bytes().to_vec().try_into().unwrap()
            ));

            let namespace_path: NamespacePath = path.parse().unwrap();
            assert!(Namespaces::<Test>::contains_key(
                &namespace_owner,
                &namespace_path
            ));
        }

        // Should have created: agent, agent.alice, and all the paths in the array
        // Some paths reuse existing parents, so count may be less than total paths
        assert!(NamespaceCount::<Test>::get(&owner) >= 12);
    });
}
