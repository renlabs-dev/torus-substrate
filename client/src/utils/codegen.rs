use crate::utils::parser::StoragePattern;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

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
            "mainnet" => quote! { use torus_client::interfaces::mainnet::api; },
            "testnet" => quote! { use torus_client::interfaces::testnet::api; },
            _ => quote! { use torus_client::interfaces::mainnet::api; },
        };

        quote! {
            //! Generated storage wrapper functions
            //!
            //! This module provides ergonomic access to Substrate storage items.
            //! Functions are automatically generated from the subxt API metadata.

            use std::collections::HashMap;
            use subxt::{OnlineClient, PolkadotConfig};
            use subxt::ext::subxt_core::utils::AccountId32;
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
            let func_name = format_ident!("get_{}", name);
            let pallet_ident = format_ident!("{}", pallet);
            let storage_ident = format_ident!("{}", name);
            let return_type_tokens = Self::parse_type_string(return_type);

            quote! {
                /// Get storage value
                pub async fn #func_name(
                    client: &OnlineClient<PolkadotConfig>,
                ) -> Result<#return_type_tokens, Box<dyn std::error::Error>> {
                    let storage = client.storage().at_latest().await?;
                    let result = storage.fetch(&api::storage().#pallet_ident().#storage_ident()).await?;
                    Ok(result.unwrap_or_default())
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
            let func_name = format_ident!("get_{}_by_{}", name, Self::type_to_param_name(key_type));
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
            let func_name = format_ident!("query_map_{}", name);
            let pallet_ident = format_ident!("{}", pallet);
            let storage_iter_ident = format_ident!("{}_iter", name);
            let key_type_tokens = Self::parse_type_string(key_type);
            let return_type_tokens = Self::parse_type_string(return_type);

            quote! {
                /// Query all entries in storage map
                pub async fn #func_name(
                    client: &OnlineClient<PolkadotConfig>,
                ) -> Result<HashMap<#key_type_tokens, #return_type_tokens>, Box<dyn std::error::Error>> {
                    let storage = client.storage().at_latest().await?;
                    let mut result = HashMap::new();
                    let mut iter = storage.iter(api::storage().#pallet_ident().#storage_iter_ident()).await?;

                    while let Some(Ok(kv)) = iter.next().await {
                        if let Ok(key) = #key_type_tokens::decode(&mut &kv.key_bytes[32..]) {
                            result.insert(key, kv.value);
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
                "get_{}_by_{}_by_{}",
                name,
                Self::type_to_param_name(key1_type),
                Self::type_to_param_name(key2_type)
            );
            let pallet_ident = format_ident!("{}", pallet);
            let storage_ident = format_ident!("{}", name);
            let key1_param = format_ident!("{}", Self::type_to_param_name(key1_type));
            let key2_param = format_ident!("{}", Self::type_to_param_name(key2_type));
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
            let func_name = format_ident!("query_map_{}", name);
            let pallet_ident = format_ident!("{}", pallet);
            let storage_iter_ident = format_ident!("{}_iter", name);
            let key1_type_tokens = Self::parse_type_string(key1_type);
            let key2_type_tokens = Self::parse_type_string(key2_type);
            let return_type_tokens = Self::parse_type_string(return_type);

            quote! {
                /// Query all entries in storage double map
                pub async fn #func_name(
                    client: &OnlineClient<PolkadotConfig>,
                ) -> Result<HashMap<#key1_type_tokens, HashMap<#key2_type_tokens, #return_type_tokens>>, Box<dyn std::error::Error>> {
                    let storage = client.storage().at_latest().await?;
                    let mut result = HashMap::new();
                    let mut iter = storage.iter(api::storage().#pallet_ident().#storage_iter_ident()).await?;

                    while let Some(Ok(kv)) = iter.next().await {
                        if let Ok((key1, key2)) = <(#key1_type_tokens, #key2_type_tokens)>::decode(&mut &kv.key_bytes[32..]) {
                            result.entry(key1).or_insert_with(HashMap::new).insert(key2, kv.value);
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
                .map(|t| Self::type_to_param_name(t))
                .collect();

            let func_name = format_ident!("get_{}_by_{}", name, key_names.join("_by_"));
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
            let func_name = format_ident!("query_map_{}", name);
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
        // Convert PascalCase to snake_case automatically
        let mut result = String::new();
        let chars: Vec<char> = type_str.chars().collect();

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
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Parse a type string into TokenStream
    fn parse_type_string(type_str: &str) -> TokenStream {
        let type_ident = format_ident!("{}", type_str.replace(' ', ""));
        quote! { #type_ident }
    }
}
