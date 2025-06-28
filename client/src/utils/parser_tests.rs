#[cfg(test)]
mod tests {
    use crate::utils::parser::*;

    #[test]
    fn test_storage_pattern_accessors() {
        let pattern = StoragePattern::Value {
            name: "TotalStake".to_string(),
            pallet: "torus0".to_string(),
            return_type: "u128".to_string(),
        };

        assert_eq!(pattern.name(), "TotalStake");
        assert_eq!(pattern.pallet(), "torus0");
        assert_eq!(pattern.return_type(), "u128");
    }

    #[test]
    fn test_parse_real_mainnet_metadata() {
        // Test against real generated metadata
        let mainnet_api = include_str!("../interfaces/mainnet.rs");

        let patterns = StorageParser::parse_api_file(mainnet_api).unwrap();

        // We should find patterns from torus0 pallet
        let torus0_patterns: Vec<_> = patterns.iter().filter(|p| p.pallet() == "torus0").collect();

        assert!(
            !torus0_patterns.is_empty(),
            "Should find torus0 storage patterns"
        );

        // Look for specific patterns we know exist
        let burn_pattern = patterns
            .iter()
            .find(|p| p.name() == "burn" && p.pallet() == "torus0");
        assert!(burn_pattern.is_some(), "Should find burn storage value");

        let agents_pattern = patterns
            .iter()
            .find(|p| p.name() == "agents" && p.pallet() == "torus0");
        assert!(agents_pattern.is_some(), "Should find agents storage map");

        let staked_by_pattern = patterns
            .iter()
            .find(|p| p.name() == "staked_by" && p.pallet() == "torus0");
        assert!(
            staked_by_pattern.is_some(),
            "Should find staked_by storage double map"
        );

        // Verify the patterns are correct types
        if let Some(StoragePattern::Value { return_type, .. }) = burn_pattern {
            assert!(return_type.contains("u128") || return_type.contains("Burn"));
        }

        if let Some(StoragePattern::Map {
            key_type,
            return_type,
            ..
        }) = agents_pattern
        {
            assert!(key_type.contains("AccountId32"));
            assert!(return_type.contains("Agent"));
        }

        if let Some(StoragePattern::DoubleMap {
            key1_type,
            key2_type,
            return_type,
            ..
        }) = staked_by_pattern
        {
            assert!(key1_type.contains("AccountId32"));
            assert!(key2_type.contains("AccountId32"));
            assert!(return_type.contains("u128") || return_type.contains("StakedBy"));
        }
    }

    #[test]
    fn test_parse_simple_storage_value() {
        let api_code = r#"
            pub mod api {
                pub mod torus0 {
                    pub mod storage {
                        pub mod types {
                            pub mod total_stake {
                                pub type TotalStake = u128;
                            }
                        }
                        pub struct StorageApi;
                        impl StorageApi {
                            pub fn total_stake(&self) -> StaticAddress<(), types::total_stake::TotalStake, Yes, Yes, ()> {
                                unimplemented!()
                            }
                        }
                    }
                }
            }
        "#;

        let patterns = StorageParser::parse_api_file(api_code).unwrap();
        assert_eq!(patterns.len(), 1);

        match &patterns[0] {
            StoragePattern::Value {
                name,
                pallet,
                return_type,
            } => {
                assert_eq!(name, "total_stake");
                assert_eq!(pallet, "torus0");
                assert_eq!(return_type, "u128");
            }
            _ => panic!("Expected storage value pattern"),
        }
    }

    #[test]
    fn test_parse_storage_map() {
        let api_code = r#"
            pub mod api {
                pub mod torus0 {
                    pub mod storage {
                        pub mod types {
                            pub mod agents {
                                pub type Agents = Agent;
                                pub type Param0 = AccountId32;
                            }
                        }
                        pub struct StorageApi;
                        impl StorageApi {
                            pub fn agents(&self, _0: impl Borrow<types::agents::Param0>) 
                                -> StaticAddress<StaticStorageKey<types::agents::Param0>, types::agents::Agents, Yes, (), ()> {
                                unimplemented!()
                            }
                            pub fn agents_iter(&self) 
                                -> StaticAddress<(), types::agents::Agents, (), (), Yes> {
                                unimplemented!()
                            }
                        }
                    }
                }
            }
        "#;

        let patterns = StorageParser::parse_api_file(api_code).unwrap();
        assert_eq!(patterns.len(), 1);

        match &patterns[0] {
            StoragePattern::Map {
                name,
                pallet,
                key_type,
                return_type,
            } => {
                assert_eq!(name, "agents");
                assert_eq!(pallet, "torus0");
                assert_eq!(key_type, "AccountId32");
                assert_eq!(return_type, "Agent");
            }
            _ => panic!("Expected storage map pattern"),
        }
    }

    #[test]
    fn test_parse_storage_double_map() {
        let api_code = r#"
            pub mod api {
                pub mod torus0 {
                    pub mod storage {
                        pub mod types {
                            pub mod staked_by {
                                pub type StakedBy = u128;
                                pub type Param0 = AccountId32;
                                pub type Param1 = AccountId32;
                            }
                        }
                        pub struct StorageApi;
                        impl StorageApi {
                            pub fn staked_by(
                                &self, 
                                _0: impl Borrow<types::staked_by::Param0>,
                                _1: impl Borrow<types::staked_by::Param1>
                            ) -> StaticAddress<StaticStorageKey<(types::staked_by::Param0, types::staked_by::Param1)>, types::staked_by::StakedBy, Yes, (), ()> {
                                unimplemented!()
                            }
                            pub fn staked_by_iter(&self) 
                                -> StaticAddress<(), types::staked_by::StakedBy, (), (), Yes> {
                                unimplemented!()
                            }
                        }
                    }
                }
            }
        "#;

        let patterns = StorageParser::parse_api_file(api_code).unwrap();
        assert_eq!(patterns.len(), 1);

        match &patterns[0] {
            StoragePattern::DoubleMap {
                name,
                pallet,
                key1_type,
                key2_type,
                return_type,
            } => {
                assert_eq!(name, "staked_by");
                assert_eq!(pallet, "torus0");
                assert_eq!(key1_type, "AccountId32");
                assert_eq!(key2_type, "AccountId32");
                assert_eq!(return_type, "u128");
            }
            _ => panic!("Expected storage double map pattern"),
        }
    }

    #[test]
    fn test_parse_storage_n_map() {
        let api_code = r#"
            pub mod api {
                pub mod permission0 {
                    pub mod storage {
                        pub mod types {
                            pub mod accumulated_stream_amounts {
                                pub type AccumulatedStreamAmounts = u128;
                                pub type Param0 = AccountId32;
                                pub type Param1 = StreamId;
                                pub type Param2 = PermissionId;
                            }
                        }
                        pub struct StorageApi;
                        impl StorageApi {
                            pub fn accumulated_stream_amounts(
                                &self, 
                                _0: impl Borrow<types::accumulated_stream_amounts::Param0>,
                                _1: impl Borrow<types::accumulated_stream_amounts::Param1>,
                                _2: impl Borrow<types::accumulated_stream_amounts::Param2>
                            ) -> StaticAddress<StaticStorageKey<(types::accumulated_stream_amounts::Param0, types::accumulated_stream_amounts::Param1, types::accumulated_stream_amounts::Param2)>, types::accumulated_stream_amounts::AccumulatedStreamAmounts, Yes, (), ()> {
                                unimplemented!()
                            }
                            pub fn accumulated_stream_amounts_iter(&self) 
                                -> StaticAddress<(), types::accumulated_stream_amounts::AccumulatedStreamAmounts, (), (), Yes> {
                                unimplemented!()
                            }
                        }
                    }
                }
            }
        "#;

        let patterns = StorageParser::parse_api_file(api_code).unwrap();
        assert_eq!(patterns.len(), 1);

        match &patterns[0] {
            StoragePattern::NMap {
                name,
                pallet,
                key_types,
                return_type,
            } => {
                assert_eq!(name, "accumulated_stream_amounts");
                assert_eq!(pallet, "permission0");
                assert_eq!(
                    key_types,
                    &vec![
                        "AccountId32".to_string(),
                        "StreamId".to_string(),
                        "PermissionId".to_string()
                    ]
                );
                assert_eq!(return_type, "u128");
            }
            _ => panic!("Expected storage N map pattern"),
        }
    }

    #[test]
    fn test_parse_multiple_patterns() {
        let api_code = r#"
            pub mod api {
                pub mod torus0 {
                    pub mod storage {
                        pub mod types {
                            pub mod total_stake {
                                pub type TotalStake = u128;
                            }
                            pub mod agents {
                                pub type Agents = Agent;
                                pub type Param0 = AccountId32;
                            }
                        }
                        pub struct StorageApi;
                        impl StorageApi {
                            pub fn total_stake(&self) -> StaticAddress<(), types::total_stake::TotalStake, Yes, Yes, ()> {
                                unimplemented!()
                            }
                            pub fn agents(&self, _0: impl Borrow<types::agents::Param0>) 
                                -> StaticAddress<StaticStorageKey<types::agents::Param0>, types::agents::Agents, Yes, (), ()> {
                                unimplemented!()
                            }
                            pub fn agents_iter(&self) 
                                -> StaticAddress<(), types::agents::Agents, (), (), Yes> {
                                unimplemented!()
                            }
                        }
                    }
                }
            }
        "#;

        let patterns = StorageParser::parse_api_file(api_code).unwrap();
        assert_eq!(patterns.len(), 2);

        // Check that we got both a Value and a Map pattern
        let value_patterns: Vec<_> = patterns
            .iter()
            .filter(|p| matches!(p, StoragePattern::Value { .. }))
            .collect();
        let map_patterns: Vec<_> = patterns
            .iter()
            .filter(|p| matches!(p, StoragePattern::Map { .. }))
            .collect();

        assert_eq!(value_patterns.len(), 1);
        assert_eq!(map_patterns.len(), 1);
    }

    #[test]
    fn test_parse_error_no_api_module() {
        let api_code = r#"
            pub mod not_api {
                pub mod torus0 {
                    // ...
                }
            }
        "#;

        let result = StorageParser::parse_api_file(api_code);
        assert!(matches!(result, Err(ParseError::ApiModuleNotFound)));
    }

    #[test]
    fn test_parse_error_invalid_syntax() {
        let api_code = r#"
            pub mod api {
                this is not valid rust syntax!
            }
        "#;

        let result = StorageParser::parse_api_file(api_code);
        assert!(matches!(result, Err(ParseError::SynError(_))));
    }
}
