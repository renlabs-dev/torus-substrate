#![allow(clippy::indexing_slicing)]

use pallet_torus0::namespace::{NamespaceOwnership, NamespacePricingConfig};
use pallet_torus0_api::NamespacePath;
use polkadot_sdk::{
    frame_support::{assert_err, assert_ok, traits::Currency},
    pallet_balances,
    sp_runtime::{BoundedVec, Percent},
};
use test_utils::{as_tors, get_origin, new_test_ext, pallet_governance, Balances, Test};

/// Helper function to register an agent with a specific name and balance
fn register_agent_with(account_id: u32, name: &str, balance: u128) {
    // Register the agent with the specified name
    pallet_torus0::Agents::<Test>::set(
        account_id,
        Some(pallet_torus0::agent::Agent {
            key: account_id,
            name: name.as_bytes().to_vec().try_into().unwrap(),
            url: Default::default(),
            metadata: Default::default(),
            weight_penalty_factor: Default::default(),
            registration_block: <polkadot_sdk::frame_system::Pallet<Test>>::block_number(),
            fees: Default::default(),
            last_update_block: Default::default(),
        }),
    );

    // Set the balance for the account
    let _ = Balances::deposit_creating(&account_id, balance);
}

fn set_namespace_config() {
    pallet_torus0::NamespacePricingConfig::<Test>::set(
        pallet_torus0::namespace::NamespacePricingConfig {
            base_fee: as_tors(5),
            deposit_per_byte: as_tors(5),

            count_midpoint: 20,
            fee_steepness: Percent::from_percent(20),
            max_fee_multiplier: 3,
        },
    )
}

#[test]
fn namespace_fee_at_midpoint() {
    new_test_ext().execute_with(|| {
        let config = NamespacePricingConfig::<Test> {
            deposit_per_byte: 100,
            base_fee: as_tors(1000),
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(25),
            max_fee_multiplier: 2,
        };

        let fee = config.namespace_fee(10).unwrap();
        assert_eq!(fee, as_tors(2000)); // 1000 + 1000 * 1
    });
}

#[test]
fn namespace_fee_below_midpoint() {
    new_test_ext().execute_with(|| {
        let config = NamespacePricingConfig::<Test> {
            deposit_per_byte: 100,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(10),
            max_fee_multiplier: 100,
        };

        let fee_5 = config.namespace_fee(5).unwrap();
        assert!(fee_5 < config.fee_midpoint());

        let fee_0 = config.namespace_fee(0).unwrap();
        assert!(fee_0 < fee_5);
    });
}

#[test]
fn namespace_fee_above_midpoint() {
    new_test_ext().execute_with(|| {
        let config = NamespacePricingConfig::<Test> {
            deposit_per_byte: 100,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(10),
            max_fee_multiplier: 2,
        };

        let fee_15 = config.namespace_fee(15).unwrap();
        assert!(fee_15 > config.fee_midpoint());

        let fee_20 = config.namespace_fee(20).unwrap();
        assert!(fee_20 > fee_15);
        assert!(fee_20 < config.fee_ceiling());
    });
}

#[test]
fn namespace_fee_steepness_effect() {
    new_test_ext().execute_with(|| {
        let config_low_steep = NamespacePricingConfig::<Test> {
            deposit_per_byte: 100,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(5),
            max_fee_multiplier: 100,
        };

        let config_high_steep = NamespacePricingConfig::<Test> {
            deposit_per_byte: 100,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(20),
            max_fee_multiplier: 100,
        };

        // Below midpoint: higher steepness = higher fee
        let fee_low_5 = config_low_steep.namespace_fee(5).unwrap();
        let fee_high_5 = config_high_steep.namespace_fee(5).unwrap();
        assert!(
            fee_high_5 < fee_low_5,
            "agent.alice.higher steepness should result in lower fee below midpoint"
        );

        // Above midpoint: higher steepness = lower fee
        let fee_low_15 = config_low_steep.namespace_fee(15).unwrap();
        let fee_high_15 = config_high_steep.namespace_fee(15).unwrap();
        assert!(
            fee_high_15 > fee_low_15,
            "agent.alice.higher steepness should result in higher fee above midpoint"
        );
    });
}

#[test]
fn namespace_fee_zero_steepness() {
    new_test_ext().execute_with(|| {
        let config = NamespacePricingConfig::<Test> {
            deposit_per_byte: 100,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(0),
            max_fee_multiplier: 2,
        };

        let fee_0 = config.namespace_fee(0).unwrap();
        let fee_10 = config.namespace_fee(10).unwrap();
        let fee_20 = config.namespace_fee(20).unwrap();

        // steepness is 0, so always expect midpoint
        let expected = 1000 + 1000;
        assert_eq!(fee_0, expected);
        assert_eq!(fee_10, expected);
        assert_eq!(fee_20, expected);
    });
}

#[test]
fn namespace_fee_high_steepness() {
    new_test_ext().execute_with(|| {
        let config = NamespacePricingConfig::<Test> {
            deposit_per_byte: 100,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(100),
            max_fee_multiplier: 100,
        };

        let fee_9 = config.namespace_fee(9).unwrap();
        let fee_10 = config.namespace_fee(10).unwrap();
        let fee_11 = config.namespace_fee(11).unwrap();

        assert!(fee_9 < fee_10);
        assert!(fee_10 < fee_11);

        let diff_before = fee_10.saturating_sub(fee_9);
        let diff_after = fee_11.saturating_sub(fee_10);
        assert!(diff_before > 100, "agent.alice.steep curve is not steep");
        assert!(diff_after > 100, "agent.alice.steep curve is not steep");
    });
}

#[test]
fn namespace_fee_monotonic_increase() {
    new_test_ext().execute_with(|| {
        let config = NamespacePricingConfig::<Test> {
            deposit_per_byte: 100,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(10),
            max_fee_multiplier: 100,
        };

        let mut prev_fee = config.namespace_fee(0).unwrap();
        for count in 1..50 {
            let current_fee = config.namespace_fee(count).unwrap();
            assert!(
                current_fee >= prev_fee,
                "agent.alice.fee should increase or stay same as count increases: {current_fee} > {prev_fee} at count {count}"
            );
            prev_fee = current_fee;
        }
    });
}

#[test]
fn namespace_fee_edge_cases() {
    new_test_ext().execute_with(|| {
        let config = NamespacePricingConfig::<Test> {
            deposit_per_byte: 100,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(10),
            max_fee_multiplier: 10,
        };

        let fee_1000 = config.namespace_fee(1000).unwrap();
        assert_eq!(fee_1000, config.fee_ceiling());

        let fee_max = config.namespace_fee(u32::MAX).unwrap();
        assert_eq!(fee_max, config.fee_ceiling());
    });
}

#[test]
fn namespace_deposit_basic() {
    new_test_ext().execute_with(|| {
        let config = NamespacePricingConfig::<Test> {
            deposit_per_byte: 100,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(10),
            max_fee_multiplier: 100,
        };

        // The simple agent path
        let path_agent = "agent.alice".parse().unwrap();
        assert_eq!(config.namespace_deposit(&path_agent), 1100);

        // Single character
        let path_a = "agent.alice.a".parse().unwrap();
        assert_eq!(config.namespace_deposit(&path_a), 1300);

        // 5 characters
        let path_hello = "agent.alice.hello".parse().unwrap();
        assert_eq!(config.namespace_deposit(&path_hello), 1700);

        // With dots (24 characters total)
        let path_long = "agent.alice.very.long.namespace.path".parse().unwrap();
        assert_eq!(config.namespace_deposit(&path_long), 3600);
    });
}

#[test]
fn namespace_deposit_with_separators() {
    new_test_ext().execute_with(|| {
        let config = NamespacePricingConfig::<Test> {
            deposit_per_byte: 50,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(10),
            max_fee_multiplier: 100,
        };

        // "agent.alice.a.b.c" = 5 bytes
        let path_dots = "agent.alice.a.b.c".parse().unwrap();
        assert_eq!(config.namespace_deposit(&path_dots), 850);

        // "agent.alice.abc" = 3 bytes
        let path_no_dots = "agent.alice.abc".parse().unwrap();
        assert_eq!(config.namespace_deposit(&path_no_dots), 750);
    });
}

#[test]
fn namespace_deposit_different_rates() {
    new_test_ext().execute_with(|| {
        let config_low = NamespacePricingConfig::<Test> {
            deposit_per_byte: 10,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(10),
            max_fee_multiplier: 100,
        };

        let config_high = NamespacePricingConfig::<Test> {
            deposit_per_byte: 1000,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(10),
            max_fee_multiplier: 100,
        };

        let path = "agent.alice.test.namespace".parse().unwrap(); // 14 bytes
        assert_eq!(config_low.namespace_deposit(&path), 260);
        assert_eq!(config_high.namespace_deposit(&path), 26000);
    });
}

#[test]
fn namespace_deposit_zero_rate() {
    new_test_ext().execute_with(|| {
        let config = NamespacePricingConfig::<Test> {
            deposit_per_byte: 0,
            base_fee: 1000,
            count_midpoint: 10,
            fee_steepness: Percent::from_percent(10),
            max_fee_multiplier: 100,
        };

        let path = "agent.alice.free.namespace".parse().unwrap();
        assert_eq!(config.namespace_deposit(&path), 0);
    });
}

#[test]
fn namespace_fee_precision_handling() {
    new_test_ext().execute_with(|| {
        let config = NamespacePricingConfig::<Test> {
            deposit_per_byte: 100,
            base_fee: as_tors(1),
            count_midpoint: 100,
            fee_steepness: Percent::from_percent(1),
            max_fee_multiplier: 5,
        };

        let fee_99 = config.namespace_fee(99).unwrap();
        let fee_100 = config.namespace_fee(100).unwrap();
        let fee_101 = config.namespace_fee(101).unwrap();
        assert!(fee_99 < fee_100);
        assert!(fee_100 < fee_101);

        let fee_0 = config.namespace_fee(0).unwrap();
        let fee_1 = config.namespace_fee(1).unwrap();
        let fee_2 = config.namespace_fee(2).unwrap();

        assert!(fee_0 > as_tors(1));
        assert!(fee_1 > fee_0);
        assert!(fee_2 > fee_1);
    });
}

#[test]
fn find_missing_paths_all_new() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        test_utils::register_empty_agent(owner);

        let path = "agent.alice.v1.compute.gpu.h100".parse().unwrap();
        let missing = pallet_torus0::namespace::find_missing_paths::<Test>(
            &NamespaceOwnership::Account(owner),
            &path,
        );

        assert_eq!(missing.len(), 5);
        assert_eq!(missing[0].as_bytes(), b"agent.alice.v1.compute.gpu.h100");
        assert_eq!(missing[1].as_bytes(), b"agent.alice.v1.compute.gpu");
        assert_eq!(missing[2].as_bytes(), b"agent.alice.v1.compute");
        assert_eq!(missing[3].as_bytes(), b"agent.alice.v1");
        assert_eq!(missing[4].as_bytes(), b"agent.alice");
    });
}

#[test]
fn find_missing_paths_partial_exists() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        test_utils::register_empty_agent(owner);

        // Helper closure to insert namespace metadata
        let insert_namespace = |path_str: &str| {
            let path: NamespacePath = path_str.parse().unwrap();
            pallet_torus0::Namespaces::<Test>::insert(
                NamespaceOwnership::Account(owner),
                path,
                pallet_torus0::namespace::NamespaceMetadata {
                    created_at: 0,
                    deposit: 100,
                },
            );
        };

        for segment in ["agent.alice", "agent.alice.v1", "agent.alice.v1.compute"] {
            insert_namespace(segment);
        }

        let path = "agent.alice.v1.compute.gpu.h100".parse().unwrap();
        let missing = pallet_torus0::namespace::find_missing_paths::<Test>(
            &NamespaceOwnership::Account(owner),
            &path,
        );

        assert_eq!(missing.len(), 2);
        assert_eq!(missing[0].as_bytes(), b"agent.alice.v1.compute.gpu.h100");
        assert_eq!(missing[1].as_bytes(), b"agent.alice.v1.compute.gpu");
    });
}

#[test]
fn find_missing_paths_all_exists() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        test_utils::register_empty_agent(owner);

        for path in [
            "agent.alice",
            "agent.alice.v1",
            "agent.alice.v1.compute",
            "agent.alice.v1.compute.gpu",
            "agent.alice.v1.compute.gpu.h100",
        ] {
            let path: NamespacePath = path.parse().unwrap();

            pallet_torus0::Namespaces::<Test>::insert(
                NamespaceOwnership::Account(owner),
                path,
                pallet_torus0::namespace::NamespaceMetadata {
                    created_at: 0,
                    deposit: 100,
                },
            );
        }

        let path = "agent.alice.v1.compute.gpu.h100".parse().unwrap();
        let missing = pallet_torus0::namespace::find_missing_paths::<Test>(
            &NamespaceOwnership::Account(owner),
            &path,
        );

        assert_eq!(missing.len(), 0);
    });
}

#[test]
fn find_missing_paths_single_segment() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        test_utils::register_empty_agent(owner);

        let path = "agent.alice".parse().unwrap();
        let missing = pallet_torus0::namespace::find_missing_paths::<Test>(
            &NamespaceOwnership::Account(owner),
            &path,
        );

        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].as_bytes(), b"agent.alice");
    });
}

#[test]
fn find_missing_paths_middle_exists() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        test_utils::register_empty_agent(owner);

        for path in [
            "agent.alice",
            "agent.alice.v1",
            "agent.alice.v1.compute.gpu",
        ] {
            let path: NamespacePath = path.parse().unwrap();
            pallet_torus0::Namespaces::<Test>::insert(
                NamespaceOwnership::Account(owner),
                path,
                pallet_torus0::namespace::NamespaceMetadata {
                    created_at: 0,
                    deposit: 100,
                },
            );
        }

        let path = "agent.alice.v1.compute.gpu.h100".parse().unwrap();
        let missing = pallet_torus0::namespace::find_missing_paths::<Test>(
            &NamespaceOwnership::Account(owner),
            &path,
        );

        assert_eq!(missing.len(), 3);
        assert_eq!(missing[0].as_bytes(), b"agent.alice.v1.compute.gpu.h100");
        assert_eq!(missing[1].as_bytes(), b"agent.alice.v1.compute.gpu");
        assert_eq!(missing[2].as_bytes(), b"agent.alice.v1.compute");
    });
}

#[test]
fn find_missing_paths_different_branches() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        test_utils::register_empty_agent(owner);

        for path in [
            "agent.alice",
            "agent.alice.v1",
            "agent.alice.v1.storage",
            "agent.alice.v1.storage.disk",
        ] {
            let path: NamespacePath = path.parse().unwrap();

            pallet_torus0::Namespaces::<Test>::insert(
                NamespaceOwnership::Account(owner),
                path,
                pallet_torus0::namespace::NamespaceMetadata {
                    created_at: 0,
                    deposit: 100,
                },
            );
        }

        let path = "agent.alice.v1.compute.gpu.h100".parse().unwrap();
        let missing = pallet_torus0::namespace::find_missing_paths::<Test>(
            &NamespaceOwnership::Account(owner),
            &path,
        );

        assert_eq!(missing.len(), 3);
        assert_eq!(missing[0].as_bytes(), b"agent.alice.v1.compute.gpu.h100");
        assert_eq!(missing[1].as_bytes(), b"agent.alice.v1.compute.gpu");
        assert_eq!(missing[2].as_bytes(), b"agent.alice.v1.compute");
    });
}

#[test]
fn calculate_cost_no_existing_namespaces() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        let owner = 1;
        test_utils::register_empty_agent(owner);

        let paths: &[NamespacePath] = &[
            "agent.alice.v1".parse().unwrap(),
            "agent.alice.v1.compute".parse().unwrap(),
            "agent.alice.v1.compute.gpu".parse().unwrap(),
        ];

        let (fee, deposit) = pallet_torus0::namespace::calculate_cost::<Test>(
            &NamespaceOwnership::Account(owner),
            paths,
        )
        .unwrap();

        // Using default config: base_fee = 5 TORS, deposit_per_byte = 5 TORS
        // Fee calculation: each subsequent namespace has higher fee due to increasing count
        // Count 0: fee_0, Count 1: fee_1, Count 2: fee_2
        assert!(fee > as_tors(15)); // 3 namespaces * min 5 TORS base fee

        assert_eq!(deposit, as_tors(310));
    });
}

#[test]
fn calculate_cost_with_existing_namespaces() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        let owner = 1;
        test_utils::register_empty_agent(owner);

        // Set namespace count to simulate existing namespaces
        pallet_torus0::NamespaceCount::<Test>::insert(NamespaceOwnership::Account(owner), 10);

        let paths: &[NamespacePath] = &[
            "agent.alice.agent".parse().unwrap(),
            "agent.alice.super.names".parse().unwrap(),
        ];

        let (fee, deposit) = pallet_torus0::namespace::calculate_cost::<Test>(
            &NamespaceOwnership::Account(owner),
            paths,
        )
        .unwrap();

        // Fee should be higher due to existing count of 10
        // Count 10: at midpoint (20), Count 11: above midpoint
        let config = pallet_torus0::NamespacePricingConfig::<Test>::get();
        let fee_10 = config.namespace_fee(10).unwrap();
        let fee_11 = config.namespace_fee(11).unwrap();
        assert_eq!(fee, fee_10 + fee_11);

        // Deposit: "agent.alice.agent" = 17 bytes, "agent.alice" = 11 bytes
        assert_eq!(deposit, as_tors(200));
    });
}

#[test]
fn calculate_cost_progressive_fee_increase() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        let owner = 1;
        test_utils::register_empty_agent(owner);

        // Create a long path to test progressive fee increase
        let paths: &[NamespacePath] = &[
            "agent.alice.a".parse().unwrap(),
            "agent.alice.a.b".parse().unwrap(),
            "agent.alice.a.b.c".parse().unwrap(),
            "agent.alice.a.b.c.d".parse().unwrap(),
            "agent.alice.a.b.c.d.e".parse().unwrap(),
        ];

        let (fee, _deposit) = pallet_torus0::namespace::calculate_cost::<Test>(
            &NamespaceOwnership::Account(owner),
            paths,
        )
        .unwrap();

        // Verify progressive increase: calculate individual fees
        let config = pallet_torus0::NamespacePricingConfig::<Test>::get();
        let mut expected_fee = 0u128;
        for i in 0..5 {
            expected_fee += config.namespace_fee(i as u32).unwrap();
        }
        assert_eq!(fee, expected_fee);
    });
}

#[test]
fn calculate_cost_empty_paths() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        let owner = 1;
        test_utils::register_empty_agent(owner);

        let (fee, deposit) = pallet_torus0::namespace::calculate_cost::<Test>(
            &NamespaceOwnership::Account(owner),
            &[],
        )
        .unwrap();

        assert_eq!(fee, 0);
        assert_eq!(deposit, 0);
    });
}

#[test]
fn calculate_cost_different_pricing_config() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        let owner = 1;
        test_utils::register_empty_agent(owner);

        // Update pricing config
        pallet_torus0::NamespacePricingConfig::<Test>::put(NamespacePricingConfig {
            base_fee: as_tors(10),
            deposit_per_byte: as_tors(2),
            count_midpoint: 5,
            fee_steepness: Percent::from_percent(50),
            max_fee_multiplier: 10,
        });

        let paths: &[NamespacePath] = &[
            "agent.alice.test".parse().unwrap(),
            "agent.alice.test.namespace".parse().unwrap(),
        ];

        let (fee, deposit) = pallet_torus0::namespace::calculate_cost::<Test>(
            &NamespaceOwnership::Account(owner),
            paths,
        )
        .unwrap();

        // With higher base fee and steeper curve
        assert!(fee > as_tors(20)); // 2 namespaces * min 10 TORS base fee

        assert_eq!(deposit, as_tors(84));
    });
}

#[test]
fn calculate_cost_long_path_names() {
    new_test_ext().execute_with(|| {
        set_namespace_config();

        let owner = 1;
        test_utils::register_empty_agent(owner);

        let paths: &[NamespacePath] = &[
            "agent.alice.very-long-namespace-name".parse().unwrap(),
            "agent.alice.very-long-namespace-name.with-another-segment"
                .parse()
                .unwrap(),
        ];

        let (_fee, deposit) = pallet_torus0::namespace::calculate_cost::<Test>(
            &NamespaceOwnership::Account(owner),
            paths,
        )
        .unwrap();

        // Deposit calculation based on bytes
        assert_eq!(deposit, as_tors(465));
    });
}

#[test]
fn calculate_cost_at_fee_ceiling() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        let owner = 1;
        test_utils::register_empty_agent(owner);

        pallet_torus0::NamespaceCount::<Test>::insert(NamespaceOwnership::Account(owner), 1000);

        let paths: &[NamespacePath] = &["agent.alice.test".parse().unwrap()];

        let (fee, _deposit) = pallet_torus0::namespace::calculate_cost::<Test>(
            &NamespaceOwnership::Account(owner),
            paths,
        )
        .unwrap();

        // fee should be at ceiling because of high amount of registered entries
        let config = pallet_torus0::NamespacePricingConfig::<Test>::get();
        assert_eq!(fee, config.fee_ceiling());
    });
}

#[test]
fn namespace_freezing() {
    new_test_ext().execute_with(|| {
        pallet_governance::NamespacesFrozen::<Test>::set(true);
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_namespace(
                get_origin(0),
                BoundedVec::truncate_from(b"agent.alice.new.namespace".to_vec())
            ),
            pallet_torus0::Error::<Test>::NamespacesFrozen
        );
    });
}

#[test]
fn namespace_must_start_with_agent_prefix() {
    new_test_ext().execute_with(|| {
        // Register alice as an agent with proper name and balance
        register_agent_with(0, "alice", as_tors(1000));

        // Helper closure to test invalid namespace creation
        let assert_invalid_namespace = |path: &[u8]| {
            assert_err!(
                pallet_torus0::Pallet::<Test>::create_namespace(
                    get_origin(0),
                    BoundedVec::truncate_from(path.to_vec())
                ),
                pallet_torus0::Error::<Test>::InvalidNamespacePath
            );
        };

        // Test various invalid prefixes
        assert_invalid_namespace(b"foobar.alice");
        assert_invalid_namespace(b"alice.agent");
        assert_invalid_namespace(b"agents.alice");
        assert_invalid_namespace(b"Agent.alice"); // wrong case
        assert_invalid_namespace(b"agentX.alice"); // extra character
        assert_invalid_namespace(b""); // empty path
        assert_invalid_namespace(b"alice"); // no prefix at all

        // Verify that a valid namespace starting with "agent." works
        assert_ok!(pallet_torus0::Pallet::<Test>::create_namespace(
            get_origin(0),
            BoundedVec::truncate_from(b"agent.alice".to_vec())
        ));
    });
}

#[test]
fn create_namespace_simple() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        register_agent_with(0, "alice", as_tors(1000));

        assert_ok!(pallet_torus0::Pallet::<Test>::create_namespace(
            get_origin(0),
            BoundedVec::truncate_from(b"agent.alice".to_vec())
        ));

        // Verify namespace was created
        assert!(pallet_torus0::Namespaces::<Test>::contains_key(
            pallet_torus0::namespace::NamespaceOwnership::Account(0),
            pallet_torus0_api::NamespacePath::new_agent(b"agent.alice").unwrap()
        ));
    });
}

#[test]
fn create_namespace_with_hierarchy() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        register_agent_with(0, "alice", as_tors(1000));

        assert_ok!(pallet_torus0::Pallet::<Test>::create_namespace(
            get_origin(0),
            BoundedVec::truncate_from(b"agent.alice.v1.compute.gpu".to_vec())
        ));

        // Helper closure to check if namespace exists
        let ownership = pallet_torus0::namespace::NamespaceOwnership::Account(0);
        let assert_namespace_exists = |path: &str| {
            assert!(pallet_torus0::Namespaces::<Test>::contains_key(
                ownership.clone(),
                path.parse::<pallet_torus0_api::NamespacePath>().unwrap()
            ));
        };

        // Verify all hierarchical namespaces were created
        assert_namespace_exists("agent.alice");
        assert_namespace_exists("agent.alice.v1");
        assert_namespace_exists("agent.alice.v1.compute");
        assert_namespace_exists("agent.alice.v1.compute.gpu");
    });
}

#[test]
fn create_namespace_already_exists() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        register_agent_with(0, "alice", as_tors(1000));

        // Helper closure for namespace creation
        let create_namespace = |path: &[u8]| {
            pallet_torus0::Pallet::<Test>::create_namespace(
                get_origin(0),
                BoundedVec::truncate_from(path.to_vec()),
            )
        };

        // Create namespace first time
        assert_ok!(create_namespace(b"agent.alice"));

        // Try to create same namespace again
        assert_err!(
            create_namespace(b"agent.alice"),
            pallet_torus0::Error::<Test>::NamespaceAlreadyExists
        );
    });
}

#[test]
fn create_namespace_not_an_agent() {
    new_test_ext().execute_with(|| {
        set_namespace_config();

        // Try to create namespace without being registered as agent
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_namespace(
                get_origin(0),
                BoundedVec::truncate_from(b"agent.alice".to_vec())
            ),
            pallet_torus0::Error::<Test>::AgentDoesNotExist
        );
    });
}

#[test]
fn create_namespace_wrong_agent_name() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        register_agent_with(0, "alice", as_tors(1000));

        // Try to create namespace with different agent name
        assert_err!(
            pallet_torus0::Pallet::<Test>::create_namespace(
                get_origin(0),
                BoundedVec::truncate_from(b"agent.bob".to_vec())
            ),
            pallet_torus0::Error::<Test>::InvalidNamespacePath
        );
    });
}

#[test]
fn create_namespace_insufficient_balance() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        register_agent_with(0, "alice", as_tors(1)); // Very low balance

        assert_err!(
            pallet_torus0::Pallet::<Test>::create_namespace(
                get_origin(0),
                BoundedVec::truncate_from(b"agent.alice".to_vec())
            ),
            pallet_balances::Error::<Test>::InsufficientBalance
        );
    });
}

#[test]
fn delete_namespace_not_exists() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        register_agent_with(0, "alice", as_tors(1000));

        assert_err!(
            pallet_torus0::Pallet::<Test>::delete_namespace(
                get_origin(0),
                BoundedVec::truncate_from(b"agent.alice.nonexistent".to_vec())
            ),
            pallet_torus0::Error::<Test>::NamespaceNotFound
        );
    });
}

#[test]
fn delete_namespace_root_agent_entry() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        register_agent_with(0, "alice", as_tors(1000));

        assert_ok!(pallet_torus0::Pallet::<Test>::create_namespace(
            get_origin(0),
            BoundedVec::truncate_from(b"agent.alice.compute".to_vec())
        ));

        assert!(pallet_torus0::Namespaces::<Test>::contains_key(
            pallet_torus0::namespace::NamespaceOwnership::Account(0),
            "agent.alice.compute".parse::<NamespacePath>().unwrap()
        ));

        assert_err!(
            pallet_torus0::Pallet::<Test>::delete_namespace(
                get_origin(0),
                BoundedVec::truncate_from(b"agent.alice".to_vec())
            ),
            pallet_torus0::Error::<Test>::InvalidNamespacePath
        );
    });
}

#[test]
fn delete_namespace_being_delegated() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        register_agent_with(0, "alice", as_tors(1000));

        assert_ok!(pallet_torus0::Pallet::<Test>::create_namespace(
            get_origin(0),
            BoundedVec::truncate_from(b"agent.alice.compute".to_vec())
        ));

        let mut paths = polkadot_sdk::sp_runtime::BoundedBTreeSet::new();
        paths
            .try_insert(b"agent.alice.compute".to_vec().try_into().unwrap())
            .unwrap();

        assert_ok!(test_utils::Permission0::grant_namespace_permission(
            get_origin(0),
            1,
            paths,
            test_utils::pallet_permission0::PermissionDuration::Indefinite,
            test_utils::pallet_permission0::RevocationTerms::RevocableByGrantor,
        ));

        assert_err!(
            pallet_torus0::Pallet::<Test>::delete_namespace(
                get_origin(0),
                BoundedVec::truncate_from(b"agent.alice.compute".to_vec())
            ),
            pallet_torus0::Error::<Test>::NamespaceBeingDelegated
        );
    });
}

#[test]
fn delete_namespace_deposit_refund() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        let initial_balance = as_tors(1000);
        register_agent_with(0, "alice", initial_balance);

        assert_ok!(pallet_torus0::Pallet::<Test>::create_namespace(
            get_origin(0),
            BoundedVec::truncate_from(b"agent.alice.compute".to_vec())
        ));

        let balance_after_creation = Balances::free_balance(0);
        let reserved_after_creation = Balances::reserved_balance(0);

        assert!(balance_after_creation < initial_balance);
        assert!(reserved_after_creation > 0);

        // Sum deposits from all created namespaces
        let ownership = pallet_torus0::namespace::NamespaceOwnership::Account(0);
        let mut total_deposit_from_storage = 0u128;
        for (_, metadata) in pallet_torus0::Namespaces::<Test>::iter_prefix(&ownership) {
            total_deposit_from_storage += metadata.deposit;
        }
        assert_eq!(reserved_after_creation, total_deposit_from_storage);

        // Get deposit amount for the namespace being deleted
        let compute_path = "agent.alice.compute".parse::<NamespacePath>().unwrap();
        let compute_deposit = pallet_torus0::Namespaces::<Test>::get(&ownership, &compute_path)
            .map(|m| m.deposit)
            .unwrap_or(0);

        assert_ok!(pallet_torus0::Pallet::<Test>::delete_namespace(
            get_origin(0),
            BoundedVec::truncate_from(b"agent.alice.compute".to_vec())
        ));

        let balance_after_deletion = Balances::free_balance(0);
        let reserved_after_deletion = Balances::reserved_balance(0);

        // Verify the compute namespace deposit was refunded
        assert_eq!(
            reserved_after_deletion,
            reserved_after_creation - compute_deposit
        );
        assert_eq!(
            balance_after_deletion,
            balance_after_creation + compute_deposit
        );

        // Verify remaining deposits match what's in storage
        let mut remaining_deposit = 0u128;
        for (_, metadata) in pallet_torus0::Namespaces::<Test>::iter_prefix(&ownership) {
            remaining_deposit += metadata.deposit;
        }
        assert_eq!(reserved_after_deletion, remaining_deposit);
    });
}

#[test]
fn delete_namespace_hierarchy_deposit_refund() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        let initial_balance = as_tors(1000);
        register_agent_with(0, "alice", initial_balance);

        assert_ok!(pallet_torus0::Pallet::<Test>::create_namespace(
            get_origin(0),
            BoundedVec::truncate_from(b"agent.alice.v1.compute.gpu".to_vec())
        ));

        let reserved_after_creation = Balances::reserved_balance(0);
        let balance_after_creation = Balances::free_balance(0);

        let ownership = pallet_torus0::namespace::NamespaceOwnership::Account(0);
        let mut total_deposit_from_storage = 0u128;
        for (_, metadata) in pallet_torus0::Namespaces::<Test>::iter_prefix(&ownership) {
            total_deposit_from_storage += metadata.deposit;
        }
        assert_eq!(reserved_after_creation, total_deposit_from_storage);

        let paths_to_delete = [
            "agent.alice.v1",
            "agent.alice.v1.compute",
            "agent.alice.v1.compute.gpu",
        ];
        let mut deposits_to_refund = 0u128;
        for path_str in &paths_to_delete {
            let path = path_str.parse::<NamespacePath>().unwrap();
            if let Some(metadata) = pallet_torus0::Namespaces::<Test>::get(&ownership, &path) {
                deposits_to_refund += metadata.deposit;
            }
        }

        assert_ok!(pallet_torus0::Pallet::<Test>::delete_namespace(
            get_origin(0),
            BoundedVec::truncate_from(b"agent.alice.v1".to_vec())
        ));

        let reserved_after_deletion = Balances::reserved_balance(0);
        let balance_after_deletion = Balances::free_balance(0);

        assert_eq!(
            reserved_after_deletion,
            reserved_after_creation - deposits_to_refund
        );
        assert_eq!(
            balance_after_deletion,
            balance_after_creation + deposits_to_refund
        );

        for path in paths_to_delete {
            assert!(!pallet_torus0::Namespaces::<Test>::contains_key(
                ownership.clone(),
                path.parse::<NamespacePath>().unwrap()
            ));
        }

        let mut remaining_deposit = 0u128;
        for (_, metadata) in pallet_torus0::Namespaces::<Test>::iter_prefix(&ownership) {
            remaining_deposit += metadata.deposit;
        }
        assert_eq!(reserved_after_deletion, remaining_deposit);
    });
}

#[test]
fn delete_namespace_not_owner() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        register_agent_with(0, "alice", as_tors(1000));
        register_agent_with(1, "bob", as_tors(1000));

        assert_ok!(pallet_torus0::Pallet::<Test>::create_namespace(
            get_origin(0),
            BoundedVec::truncate_from(b"agent.alice.compute".to_vec())
        ));

        assert_err!(
            pallet_torus0::Pallet::<Test>::delete_namespace(
                get_origin(1),
                BoundedVec::truncate_from(b"agent.alice.compute".to_vec())
            ),
            pallet_torus0::Error::<Test>::NamespaceNotFound
        );
    });
}

#[test]
fn delete_namespace_partial_hierarchy() {
    new_test_ext().execute_with(|| {
        set_namespace_config();
        register_agent_with(0, "alice", as_tors(1000));

        for path in [
            "agent.alice.v1.compute.gpu",
            "agent.alice.v1.storage.disk",
            "agent.alice.v2.network",
        ] {
            assert_ok!(pallet_torus0::Pallet::<Test>::create_namespace(
                get_origin(0),
                BoundedVec::truncate_from(path.as_bytes().to_vec())
            ));
        }

        let initial_count = pallet_torus0::NamespaceCount::<Test>::get(
            pallet_torus0::namespace::NamespaceOwnership::Account(0),
        );
        assert_eq!(initial_count, 8); // agent.alice + 7 other namespaces

        assert_ok!(pallet_torus0::Pallet::<Test>::delete_namespace(
            get_origin(0),
            BoundedVec::truncate_from(b"agent.alice.v1.compute".to_vec())
        ));

        let count_after = pallet_torus0::NamespaceCount::<Test>::get(
            pallet_torus0::namespace::NamespaceOwnership::Account(0),
        );
        assert_eq!(count_after, 6);

        for path in ["agent.alice.v1.compute", "agent.alice.v1.compute.gpu"] {
            assert!(!pallet_torus0::Namespaces::<Test>::contains_key(
                pallet_torus0::namespace::NamespaceOwnership::Account(0),
                path.parse::<NamespacePath>().unwrap()
            ));
        }

        for path in [
            "agent.alice.v1",
            "agent.alice.v1.storage",
            "agent.alice.v1.storage.disk",
            "agent.alice.v2",
            "agent.alice.v2.network",
        ] {
            assert!(pallet_torus0::Namespaces::<Test>::contains_key(
                pallet_torus0::namespace::NamespaceOwnership::Account(0),
                path.parse::<NamespacePath>().unwrap()
            ));
        }
    });
}
