use crate::utils::parser::StoragePattern;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_str, Type};

/// Generates ergonomic wrapper functions for Substrate storage items
pub struct WrapperGenerator;

impl WrapperGenerator {
    /// Generate all wrapper functions for the given storage patterns
    pub fn generate_wrappers(patterns: &[StoragePattern]) -> TokenStream {
        Self::generate_wrappers_for_network(patterns, "mainnet")
    }

    /// Generate all wrapper functions for the given storage patterns and network
    pub fn generate_wrappers_for_network(
        patterns: &[StoragePattern],
        network: &str,
    ) -> TokenStream {
        let functions: Vec<TokenStream> = patterns
            .iter()
            .flat_map(|pattern| Self::generate_pattern_wrappers(pattern, network))
            .collect();

        let api_import = match network {
            "mainnet" => quote! {
                use crate::interfaces::mainnet::api;
                use crate::interfaces::mainnet::api::runtime_types;
            },
            "testnet" => quote! {
                use crate::interfaces::testnet::api;
                use crate::interfaces::testnet::api::runtime_types;
            },
            _ => quote! {
                use crate::interfaces::mainnet::api;
                use crate::interfaces::mainnet::api::runtime_types;
            },
        };

        quote! {
            //! Generated storage wrapper functions
            //!
            //! This module provides ergonomic access to Substrate storage items.
            //! Functions are automatically generated from the subxt API metadata.

            #![allow(dead_code)]

            use std::collections::HashMap;
            use subxt::{OnlineClient, PolkadotConfig};
            use codec::Decode;
            #api_import

            #(#functions)*
        }
    }

    /// Generate wrapper functions for a single storage pattern
    fn generate_pattern_wrappers(pattern: &StoragePattern, _network: &str) -> Vec<TokenStream> {
        match pattern {
            StoragePattern::Value { .. } => {
                vec![Self::generate_value_wrapper(pattern)]
            }
            StoragePattern::Map { .. } => {
                vec![
                    Self::generate_map_getter(pattern),
                    Self::generate_map_query(pattern),
                ]
            }
            StoragePattern::DoubleMap { .. } => {
                vec![
                    Self::generate_double_map_getter(pattern),
                    Self::generate_double_map_query(pattern),
                ]
            }
            StoragePattern::NMap { .. } => {
                vec![
                    Self::generate_nmap_getter(pattern),
                    Self::generate_nmap_query(pattern),
                ]
            }
        }
    }

    /// Generate wrapper for Storage Value
    fn generate_value_wrapper(pattern: &StoragePattern) -> TokenStream {
        if let StoragePattern::Value {
            name,
            pallet,
            return_type,
        } = pattern
        {
            let func_name = format_ident!("get_{}_{}", pallet, name);
            let pallet_ident = format_ident!("{}", pallet);
            let storage_ident = format_ident!("{}", name);
            let return_type_tokens = Self::parse_type_string(return_type);

            quote! {
                /// Get storage value
                pub async fn #func_name(
                    client: &OnlineClient<PolkadotConfig>,
                ) -> Result<Option<#return_type_tokens>, Box<dyn std::error::Error>> {
                    let storage = client.storage().at_latest().await?;
                    let result = storage.fetch(&api::storage().#pallet_ident().#storage_ident()).await?;
                    Ok(result)
                }
            }
        } else {
            panic!("Expected StoragePattern::Value");
        }
    }

    /// Generate getter function for Storage Map
    fn generate_map_getter(pattern: &StoragePattern) -> TokenStream {
        if let StoragePattern::Map {
            name,
            pallet,
            key_type,
            return_type,
        } = pattern
        {
            let func_name = format_ident!(
                "get_{}_{}_by_{}",
                pallet,
                name,
                Self::type_to_param_name(key_type)
            );
            let pallet_ident = format_ident!("{}", pallet);
            let storage_ident = format_ident!("{}", name);
            let key_param = format_ident!("{}", Self::type_to_param_name(key_type));
            let key_type_tokens = Self::parse_type_string(key_type);
            let return_type_tokens = Self::parse_type_string(return_type);

            quote! {
                /// Get storage map value by key
                pub async fn #func_name(
                    client: &OnlineClient<PolkadotConfig>,
                    #key_param: #key_type_tokens,
                ) -> Result<Option<#return_type_tokens>, Box<dyn std::error::Error>> {
                    let storage = client.storage().at_latest().await?;
                    let result = storage.fetch(&api::storage().#pallet_ident().#storage_ident(#key_param)).await?;
                    Ok(result)
                }
            }
        } else {
            panic!("Expected StoragePattern::Map");
        }
    }

    /// Generate query function for Storage Map
    fn generate_map_query(pattern: &StoragePattern) -> TokenStream {
        if let StoragePattern::Map {
            name,
            pallet,
            key_type,
            return_type,
        } = pattern
        {
            let func_name = format_ident!("query_map_{}_{}", pallet, name);
            let pallet_ident = format_ident!("{}", pallet);
            let storage_iter_ident = format_ident!("{}_iter", name);
            let key_type_tokens = Self::parse_type_string(key_type);
            let return_type_tokens = Self::parse_type_string(return_type);

            // Use String keys for types that don't implement Hash/Eq traits
            let key_type_for_hashmap =
                if key_type.contains("AccountId32") || key_type.contains("U256") {
                    quote! { String }
                } else {
                    key_type_tokens.clone()
                };

            let key_conversion = if key_type.contains("AccountId32") || key_type.contains("U256") {
                quote! { format!("{:?}", key) }
            } else {
                quote! { key }
            };

            quote! {
                /// Query all entries in storage map
                pub async fn #func_name(
                    client: &OnlineClient<PolkadotConfig>,
                ) -> Result<HashMap<#key_type_for_hashmap, #return_type_tokens>, Box<dyn std::error::Error>> {
                    let storage = client.storage().at_latest().await?;
                    let mut result = HashMap::new();
                    let mut iter = storage.iter(api::storage().#pallet_ident().#storage_iter_ident()).await?;

                    while let Some(Ok(kv)) = iter.next().await {
                        if let Ok(key) = #key_type_tokens::decode(&mut &kv.key_bytes[32..]) {
                            result.insert(#key_conversion, kv.value);
                        }
                    }
                    Ok(result)
                }
            }
        } else {
            panic!("Expected StoragePattern::Map");
        }
    }

    /// Generate getter function for Storage Double Map
    fn generate_double_map_getter(pattern: &StoragePattern) -> TokenStream {
        if let StoragePattern::DoubleMap {
            name,
            pallet,
            key1_type,
            key2_type,
            return_type,
        } = pattern
        {
            let func_name = format_ident!(
                "get_{}_{}_by_{}_by_{}",
                pallet,
                name,
                Self::type_to_param_name(key1_type),
                Self::type_to_param_name(key2_type)
            );
            let pallet_ident = format_ident!("{}", pallet);
            let storage_ident = format_ident!("{}", name);
            let key1_param = format_ident!("{}_1", Self::type_to_param_name(key1_type));
            let key2_param = format_ident!("{}_2", Self::type_to_param_name(key2_type));
            let key1_type_tokens = Self::parse_type_string(key1_type);
            let key2_type_tokens = Self::parse_type_string(key2_type);
            let return_type_tokens = Self::parse_type_string(return_type);

            quote! {
                /// Get storage double map value by keys
                pub async fn #func_name(
                    client: &OnlineClient<PolkadotConfig>,
                    #key1_param: #key1_type_tokens,
                    #key2_param: #key2_type_tokens,
                ) -> Result<Option<#return_type_tokens>, Box<dyn std::error::Error>> {
                    let storage = client.storage().at_latest().await?;
                    let result = storage.fetch(&api::storage().#pallet_ident().#storage_ident(#key1_param, #key2_param)).await?;
                    Ok(result)
                }
            }
        } else {
            panic!("Expected StoragePattern::DoubleMap");
        }
    }

    /// Generate query function for Storage Double Map
    fn generate_double_map_query(pattern: &StoragePattern) -> TokenStream {
        if let StoragePattern::DoubleMap {
            name,
            pallet,
            key1_type,
            key2_type,
            return_type,
        } = pattern
        {
            let func_name = format_ident!("query_map_{}_{}", pallet, name);
            let pallet_ident = format_ident!("{}", pallet);
            let storage_iter_ident = format_ident!("{}_iter", name);
            let key1_type_tokens = Self::parse_type_string(key1_type);
            let key2_type_tokens = Self::parse_type_string(key2_type);
            let return_type_tokens = Self::parse_type_string(return_type);

            // Use String keys for types that don't implement Hash/Eq traits
            let key1_type_for_hashmap =
                if key1_type.contains("AccountId32") || key1_type.contains("U256") {
                    quote! { String }
                } else {
                    key1_type_tokens.clone()
                };

            let key2_type_for_hashmap =
                if key2_type.contains("AccountId32") || key2_type.contains("U256") {
                    quote! { String }
                } else {
                    key2_type_tokens.clone()
                };

            let key1_conversion = if key1_type.contains("AccountId32") || key1_type.contains("U256")
            {
                quote! { format!("{:?}", key1) }
            } else {
                quote! { key1 }
            };

            let key2_conversion = if key2_type.contains("AccountId32") || key2_type.contains("U256")
            {
                quote! { format!("{:?}", key2) }
            } else {
                quote! { key2 }
            };

            quote! {
                /// Query all entries in storage double map
                pub async fn #func_name(
                    client: &OnlineClient<PolkadotConfig>,
                ) -> Result<HashMap<#key1_type_for_hashmap, HashMap<#key2_type_for_hashmap, #return_type_tokens>>, Box<dyn std::error::Error>> {
                    let storage = client.storage().at_latest().await?;
                    let mut result = HashMap::new();
                    let mut iter = storage.iter(api::storage().#pallet_ident().#storage_iter_ident()).await?;

                    while let Some(Ok(kv)) = iter.next().await {
                        if let Ok((key1, key2)) = <(#key1_type_tokens, #key2_type_tokens)>::decode(&mut &kv.key_bytes[32..]) {
                            result.entry(#key1_conversion).or_insert_with(HashMap::new).insert(#key2_conversion, kv.value);
                        }
                    }
                    Ok(result)
                }
            }
        } else {
            panic!("Expected StoragePattern::DoubleMap");
        }
    }

    /// Generate getter function for Storage N Map
    fn generate_nmap_getter(pattern: &StoragePattern) -> TokenStream {
        if let StoragePattern::NMap {
            name,
            pallet,
            key_types,
            return_type,
        } = pattern
        {
            let key_names: Vec<String> = key_types
                .iter()
                .enumerate()
                .map(|(i, t)| format!("{}_{}", Self::type_to_param_name(t), i + 1))
                .collect();

            let func_name = format_ident!("get_{}_{}_by_{}", pallet, name, key_names.join("_by_"));
            let pallet_ident = format_ident!("{}", pallet);
            let storage_ident = format_ident!("{}", name);

            let key_params: Vec<Ident> = key_names.iter().map(|n| format_ident!("{}", n)).collect();
            let key_type_tokens: Vec<TokenStream> = key_types
                .iter()
                .map(|t| Self::parse_type_string(t))
                .collect();
            let return_type_tokens = Self::parse_type_string(return_type);

            quote! {
                /// Get storage n-map value by keys
                pub async fn #func_name(
                    client: &OnlineClient<PolkadotConfig>,
                    #(#key_params: #key_type_tokens),*
                ) -> Result<Option<#return_type_tokens>, Box<dyn std::error::Error>> {
                    let storage = client.storage().at_latest().await?;
                    let result = storage.fetch(&api::storage().#pallet_ident().#storage_ident(#(#key_params),*)).await?;
                    Ok(result)
                }
            }
        } else {
            panic!("Expected StoragePattern::NMap");
        }
    }

    /// Generate query function for Storage N Map
    fn generate_nmap_query(pattern: &StoragePattern) -> TokenStream {
        if let StoragePattern::NMap {
            name,
            pallet,
            key_types,
            return_type,
        } = pattern
        {
            let func_name = format_ident!("query_map_{}_{}", pallet, name);
            let pallet_ident = format_ident!("{}", pallet);
            let storage_iter_ident = format_ident!("{}_iter", name);

            let key_type_tokens: Vec<TokenStream> = key_types
                .iter()
                .map(|t| Self::parse_type_string(t))
                .collect();
            let return_type_tokens = Self::parse_type_string(return_type);

            // Generate tuple type for keys
            let keys_tuple = if key_types.len() == 1 {
                quote! { #(#key_type_tokens),* }
            } else {
                quote! { (#(#key_type_tokens),*) }
            };

            quote! {
                /// Query all entries in storage n-map
                pub async fn #func_name(
                    client: &OnlineClient<PolkadotConfig>,
                ) -> Result<Vec<(#keys_tuple, #return_type_tokens)>, Box<dyn std::error::Error>> {
                    let storage = client.storage().at_latest().await?;
                    let mut result = Vec::new();
                    let mut iter = storage.iter(api::storage().#pallet_ident().#storage_iter_ident()).await?;

                    while let Some(Ok(kv)) = iter.next().await {
                        if let Ok(keys) = <#keys_tuple>::decode(&mut &kv.key_bytes[32..]) {
                            result.push((keys, kv.value));
                        }
                    }
                    Ok(result)
                }
            }
        } else {
            panic!("Expected StoragePattern::NMap");
        }
    }

    /// Convert a type string to parameter name (PascalCase -> snake_case)
    /// This method is public to allow testing from separate test files
    pub fn type_to_param_name(type_str: &str) -> String {
        let cleaned_type = type_str.trim().replace(' ', "");

        // Handle complex type constructs
        let simplified_type = Self::simplify_type_for_param_name(&cleaned_type);

        // Handle complex type paths - extract the last part
        let type_name = if simplified_type.contains("::") {
            simplified_type
                .split("::")
                .last()
                .unwrap_or(&simplified_type)
        } else {
            &simplified_type
        };

        // Convert PascalCase to snake_case automatically
        let mut result = String::new();
        let chars: Vec<char> = type_name.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if ch.is_uppercase() {
                // Add underscore before uppercase letters, except at the start
                if i > 0 {
                    let prev_char = chars[i - 1];
                    // Add underscore if previous char was lowercase or
                    // if this is start of a word after consecutive capitals
                    if prev_char.is_lowercase()
                        || (i + 1 < chars.len()
                            && chars[i + 1].is_lowercase()
                            && prev_char.is_uppercase())
                    {
                        result.push('_');
                    }
                }
                result.push(ch.to_lowercase().next().unwrap_or(ch));
            } else if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
            }
            // Skip other characters like brackets, semicolons, etc.
        }

        // Ensure we have a valid identifier
        if result.is_empty() {
            result = "key".to_string();
        }

        result
    }

    /// Simplify complex type names for parameter generation
    fn simplify_type_for_param_name(type_str: &str) -> String {
        let simplified = type_str.to_string();

        // Handle array types like [u8; 32] -> u8_array
        if simplified.starts_with('[') && simplified.contains(';') {
            let inner = simplified.trim_start_matches('[').trim_end_matches(']');
            if let Some(semicolon_pos) = inner.find(';') {
                let element_type = inner[..semicolon_pos].trim();
                let simplified_element = if element_type.contains("::") {
                    element_type.split("::").last().unwrap_or(element_type)
                } else {
                    element_type
                };
                return format!("{}_array", simplified_element);
            }
        }

        // Handle Vec<T> -> vec_of_t
        if simplified.starts_with("Vec<") && simplified.ends_with('>') {
            let inner = &simplified[4..simplified.len() - 1];
            let simplified_inner = if inner.contains("::") {
                inner.split("::").last().unwrap_or(inner)
            } else {
                inner
            };
            return format!("vec_of_{}", simplified_inner.to_lowercase());
        }

        // Handle Option<T> -> optional_t
        if simplified.starts_with("Option<") && simplified.ends_with('>') {
            let inner = &simplified[7..simplified.len() - 1];
            let simplified_inner = if inner.contains("::") {
                inner.split("::").last().unwrap_or(inner)
            } else {
                inner
            };
            return format!("optional_{}", simplified_inner.to_lowercase());
        }

        simplified
    }

    /// Parse a type string into TokenStream
    fn parse_type_string(type_str: &str) -> TokenStream {
        let cleaned_type = type_str.trim();

        if cleaned_type.is_empty() {
            return Self::unknown_type_token();
        }

        parse_str::<Type>(cleaned_type)
            .map(|parsed_type| quote! { #parsed_type })
            .unwrap_or_else(|_| Self::unknown_type_token())
    }

    /// Generate fallback token for unknown types
    fn unknown_type_token() -> TokenStream {
        let type_ident = format_ident!("UnknownType");
        quote! { #type_ident }
    }
}
