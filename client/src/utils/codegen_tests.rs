#[cfg(test)]
mod tests {
    use super::super::codegen::*;
    use crate::utils::parser::StoragePattern;

    #[test]
    fn test_type_to_param_name_conversions() {
        assert_eq!(
            WrapperGenerator::type_to_param_name("AccountId32"),
            "account_id32"
        );
        assert_eq!(
            WrapperGenerator::type_to_param_name("StreamId"),
            "stream_id"
        );
        assert_eq!(
            WrapperGenerator::type_to_param_name("PermissionId"),
            "permission_id"
        );
        assert_eq!(
            WrapperGenerator::type_to_param_name("ProposalId"),
            "proposal_id"
        );
        assert_eq!(
            WrapperGenerator::type_to_param_name("BlockNumber"),
            "block_number"
        );
        assert_eq!(
            WrapperGenerator::type_to_param_name("SomeCustomType"),
            "some_custom_type"
        );
    }

    #[test]
    fn test_generate_wrappers_multiple_patterns() {
        let patterns = vec![
            StoragePattern::Value {
                name: "total_stake".to_string(),
                pallet: "torus0".to_string(),
                return_type: "u128".to_string(),
            },
            StoragePattern::Map {
                name: "agents".to_string(),
                pallet: "torus0".to_string(),
                key_type: "AccountId32".to_string(),
                return_type: "Agent".to_string(),
            },
        ];

        let result = WrapperGenerator::generate_wrappers(&patterns);
        let code = result.to_string();

        // Should contain imports and both functions
        assert!(code.contains("use std :: collections :: HashMap"));
        assert!(code.contains("use subxt :: { OnlineClient , PolkadotConfig }"));
        assert!(code.contains("get_total_stake"));
        assert!(code.contains("get_agents_by_account_id32"));
        assert!(code.contains("query_map_agents"));
    }

    #[test]
    fn test_snake_case_conversion() {
        // Test various PascalCase to snake_case conversions
        assert_eq!(
            WrapperGenerator::type_to_param_name("SimpleType"),
            "simple_type"
        );
        assert_eq!(
            WrapperGenerator::type_to_param_name("XMLHttpRequest"),
            "xml_http_request"
        );
        assert_eq!(WrapperGenerator::type_to_param_name("IOError"), "io_error");
        assert_eq!(WrapperGenerator::type_to_param_name("ID"), "id");
        assert_eq!(
            WrapperGenerator::type_to_param_name("lowercase"),
            "lowercase"
        );
        assert_eq!(
            WrapperGenerator::type_to_param_name("AccountId32"),
            "account_id32"
        );
        assert_eq!(WrapperGenerator::type_to_param_name("Balance"), "balance");
        assert_eq!(WrapperGenerator::type_to_param_name("Hash"), "hash");
    }

    #[test]
    fn test_automatic_type_conversion_edge_cases() {
        // Test edge cases for automatic type conversion
        assert_eq!(WrapperGenerator::type_to_param_name("A"), "a");
        assert_eq!(WrapperGenerator::type_to_param_name("AB"), "ab");
        assert_eq!(WrapperGenerator::type_to_param_name("ABC"), "abc");
        assert_eq!(WrapperGenerator::type_to_param_name("AbC"), "ab_c");
        assert_eq!(WrapperGenerator::type_to_param_name("AbCD"), "ab_cd");
        assert_eq!(WrapperGenerator::type_to_param_name("abcd"), "abcd");
        assert_eq!(
            WrapperGenerator::type_to_param_name("HTTPResponseCode"),
            "http_response_code"
        );
        assert_eq!(WrapperGenerator::type_to_param_name("URLPath"), "url_path");
    }

    #[test]
    fn test_generate_wrappers_with_complex_types() {
        let patterns = vec![
            StoragePattern::DoubleMap {
                name: "staked_by".to_string(),
                pallet: "torus0".to_string(),
                key1_type: "AccountId32".to_string(),
                key2_type: "PermissionId".to_string(),
                return_type: "Balance".to_string(),
            },
            StoragePattern::NMap {
                name: "multi_key_storage".to_string(),
                pallet: "permission0".to_string(),
                key_types: vec![
                    "AccountId32".to_string(),
                    "StreamId".to_string(),
                    "BlockNumber".to_string(),
                ],
                return_type: "SomeComplexType".to_string(),
            },
        ];

        let result = WrapperGenerator::generate_wrappers(&patterns);
        let code = result.to_string();

        // Test that the function names use the automatic conversion
        assert!(code.contains("get_staked_by_by_account_id32_by_permission_id"));
        assert!(code.contains("get_multi_key_storage_by_account_id32_by_stream_id_by_block_number"));
        assert!(code.contains("query_map_staked_by"));
        assert!(code.contains("query_map_multi_key_storage"));
    }
}
