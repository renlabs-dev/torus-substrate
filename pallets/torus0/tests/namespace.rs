#![allow(clippy::indexing_slicing)]

use pallet_torus0::namespace::{NamespaceOwnership, NamespacePricingConfig};
use pallet_torus0_api::NamespacePath;
use polkadot_sdk::{
    frame_support::assert_err,
    sp_runtime::{BoundedVec, Percent},
};
use test_utils::{as_tors, get_origin, new_test_ext, pallet_governance, Test};

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
                "agent.alice.fee should increase or stay same as count increases: {} > {} at count {}",
                current_fee,
                prev_fee,
                count
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
        assert_eq!(config.namespace_deposit(&path_a), 100);

        // 5 characters
        let path_hello = "agent.alice.hello".parse().unwrap();
        assert_eq!(config.namespace_deposit(&path_hello), 500);

        // With dots (24 characters total)
        let path_long = "agent.alice.very.long.namespace.path".parse().unwrap();
        assert_eq!(config.namespace_deposit(&path_long), 2400);
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
        assert_eq!(config.namespace_deposit(&path_dots), 250);

        // "agent.alice.abc" = 3 bytes
        let path_no_dots = "agent.alice.abc".parse().unwrap();
        assert_eq!(config.namespace_deposit(&path_no_dots), 150);
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
        assert_eq!(config_low.namespace_deposit(&path), 140);
        assert_eq!(config_high.namespace_deposit(&path), 14000);
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

        for segment in ["agent.alice", "agent.alice.v1", "agent.alice.v1.compute"] {
            let path: NamespacePath = segment.parse().unwrap();

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

        // Deposit: "agent.alice.v1" = 2 bytes, "agent.alice.v1.compute" = 10 bytes, "agent.alice.v1.compute.gpu" = 14 bytes
        // Total = 26 bytes * 5 TORS = 130 TORS
        assert_eq!(deposit, as_tors(130));
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

        // Deposit: "agent.alice.agent" = 5 bytes, "agent.alice" = 11 bytes
        // Total = 16 bytes * 5 TORS = 80 TORS
        assert_eq!(deposit, as_tors(80));
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

        // Deposit: "agent.alice.test" = 4 bytes, "agent.alice.test.namespace" = 14 bytes
        // Total = 18 bytes * 2 TORS = 36 TORS
        assert_eq!(deposit, as_tors(36));
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
        // "agent.alice.very-long-namespace-name" = 24 bytes
        // "agent.alice.very-long-namespace-name.with-another-segment" = 45 bytes
        // Total = 69 bytes * 5 TORS = 345 TORS
        assert_eq!(deposit, as_tors(345));
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
