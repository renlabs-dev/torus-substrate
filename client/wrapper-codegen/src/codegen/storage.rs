use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use stringcase::pascal_case;
use syn::{Ident, Type, parse_str};

use crate::{
    InterfaceSource,
    ir::{PalletPattern, StoragePattern},
};

pub(super) fn generate_pallet_storage(
    network: &InterfaceSource,
    pallet: &PalletPattern,
) -> Option<TokenStream> {
    let client_name = format_ident!("{}", pascal_case(&format!("{}_client", pallet.name)));
    let struct_name = format_ident!("{}", pascal_case(&format!("{}_storage", pallet.name)));
    let network_chain: Type = syn::parse_str(network.to_chain_type()).unwrap();

    let functions: Vec<TokenStream> = pallet
        .storages
        .iter()
        .flat_map(|pattern| generate_pattern_wrappers(network, pattern))
        .collect();

    if functions.is_empty() {
        return None;
    }

    Some(quote! {
        impl #client_name<#network_chain> {
            pub fn storage(&self) -> #struct_name<#network_chain> {
                #struct_name {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData
                }
            }

            pub fn storage_at(&self, block_hash: H256) -> #struct_name<#network_chain> {
                #struct_name {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData
                }
            }
        }

        impl #struct_name<#network_chain> {
            #(#functions)*
        }
    })
}

/// Generate helper functions for key decoding
fn generate_helper_functions() -> TokenStream {
    quote! {
        /// Helper function for decoding single map keys
        fn decode_map_key<K: Decode>(key_bytes: &[u8]) -> Result<K, codec::Error> {
            if key_bytes.len() < 32 {
                return Err(codec::Error::from("Key bytes too short"));
            }
            K::decode(&mut &key_bytes[32..])
        }

        /// Helper function for decoding double map keys
        fn decode_double_map_keys<K1: Decode, K2: Decode>(key_bytes: &[u8]) -> Result<(K1, K2), codec::Error> {
            if key_bytes.len() < 32 {
                return Err(codec::Error::from("Key bytes too short"));
            }
            <(K1, K2)>::decode(&mut &key_bytes[32..])
        }

        /// Helper function for decoding n-map keys
        fn decode_nmap_keys<K: Decode>(key_bytes: &[u8]) -> Result<K, codec::Error> {
            if key_bytes.len() < 32 {
                return Err(codec::Error::from("Key bytes too short"));
            }
            K::decode(&mut &key_bytes[32..])
        }
    }
}

/// Generate wrapper functions for a single storage pattern
fn generate_pattern_wrappers(
    network: &InterfaceSource,
    pattern: &StoragePattern,
) -> Vec<TokenStream> {
    match pattern {
        StoragePattern::Value { .. } => {
            vec![generate_value_wrapper(network, pattern)]
        }
        StoragePattern::Map { .. } => {
            vec![generate_map_wrapper(network, pattern)]
        }
        StoragePattern::DoubleMap { .. } => {
            vec![generate_double_map_wrapper(network, pattern)]
        }
        StoragePattern::NMap { .. } => {
            vec![
                generate_nmap_getter(network, pattern),
                generate_nmap_query(network, pattern),
                // generate_nmap_query_raw(pattern),
            ]
        }
    }
}

/// Generate wrapper for Storage Value
fn generate_value_wrapper(network: &InterfaceSource, pattern: &StoragePattern) -> TokenStream {
    if let StoragePattern::Value {
        name,
        pallet,
        return_type,
    } = pattern
    {
        let network_ident = format_ident!("{}", network.to_str());
        let pallet_ident = format_ident!("{}", pallet);
        let storage_ident = format_ident!("{}", name);
        let return_type_tokens = parse_type_string(return_type);

        quote! {
            pub async fn #storage_ident(&self) -> Result<Option<#return_type_tokens>, crate::error::StorageError> {
                let call = crate::interfaces::#network_ident::api::storage()
                    .#pallet_ident()
                    .#storage_ident();

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                Ok(api.fetch(&call).await?)
            }
        }
    } else {
        panic!("Expected StoragePattern::Value");
    }
}

fn generate_map_wrapper(network: &InterfaceSource, pattern: &StoragePattern) -> TokenStream {
    if let StoragePattern::Map {
        name,
        pallet,
        key_type,
        return_type,
    } = pattern
    {
        let network_ident = format_ident!("{}", network.to_str());
        let pallet_ident = format_ident!("{}", pallet);
        let storage_ident_get = format_ident!("{}_get", name);
        let storage_ident_iter = format_ident!("{}_iter", name);
        let storage_ident = format_ident!("{}", name);
        let key_tt = parse_type_string(key_type);
        let value_tt = parse_type_string(return_type);

        quote! {
            pub async fn #storage_ident_get(&self, key: &#key_tt) -> anyhow::Result<Option<#value_tt>> {
                let call = crate::interfaces::#network_ident::api::storage()
                    .#pallet_ident()
                    .#storage_ident(key);

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                Ok(api.fetch(&call).await?)
            }

            pub async fn #storage_ident_iter(
                &self,
            ) -> Result<impl Stream<Item = Result<(#key_tt, #value_tt), crate::error::StorageError>>, crate::error::StorageError> {
                let call = crate::interfaces::#network_ident::api::storage()
                    .#pallet_ident()
                    .#storage_ident_iter();

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key =
                            <#key_tt as Decode>::decode(&mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]))?;
                        Ok((key, res.value))
                    })
                    .into_stream();

                Ok(stream)
            }
        }
    } else {
        panic!("Expected StoragePattern::Map");
    }
}

fn generate_double_map_wrapper(network: &InterfaceSource, pattern: &StoragePattern) -> TokenStream {
    if let StoragePattern::DoubleMap {
        name,
        pallet,
        key1_type,
        key2_type,
        return_type,
    } = pattern
    {
        let network_ident = format_ident!("{}", network.to_str());
        let pallet_ident = format_ident!("{}", pallet);
        let storage_ident_get = format_ident!("{}_get", name);
        let storage_ident_iter = format_ident!("{}_iter", name);
        let storage_ident_iter1 = format_ident!("{}_iter1", name);
        let storage_ident = format_ident!("{}", name);
        let key1_tt = parse_type_string(key1_type);
        let key2_tt = parse_type_string(key2_type);
        let value_tt = parse_type_string(return_type);

        quote! {
            pub async fn #storage_ident_get(&self, key1: &#key1_tt, key2: &#key2_tt) -> Result<Option<#value_tt>, crate::error::StorageError> {
                let call = crate::interfaces::#network_ident::api::storage()
                    .#pallet_ident()
                    .#storage_ident(key1, key2);

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                Ok(api.fetch(&call).await?)
            }

            pub async fn #storage_ident_iter(
                &self,
            ) -> Result<impl Stream<Item = Result<((#key1_tt, #key2_tt), #value_tt), crate::error::StorageError>>, crate::error::StorageError> {
                let call = crate::interfaces::#network_ident::api::storage()
                    .#pallet_ident()
                    .#storage_ident_iter();

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair =
                            <(#key1_tt, #key2_tt) as Decode>::decode(&mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]))?;
                        Ok((pair, res.value))
                    })
                    .into_stream();

                Ok(stream)
            }

            pub async fn #storage_ident_iter1(
                &self,
                key1: &#key1_tt
            ) -> Result<impl Stream<Item = Result<((#key1_tt, #key2_tt), #value_tt), crate::error::StorageError>>, crate::error::StorageError> {
                let call = crate::interfaces::#network_ident::api::storage()
                    .#pallet_ident()
                    .#storage_ident_iter1(key1);

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair =
                            <(#key1_tt, #key2_tt) as Decode>::decode(&mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]))?;
                        Ok((pair, res.value))
                    })
                    .into_stream();

                Ok(stream)
            }
        }
    } else {
        panic!("Expected StoragePattern::DoubleMap");
    }
}

/// Generate getter function for Storage N Map
fn generate_nmap_getter(network: &InterfaceSource, pattern: &StoragePattern) -> TokenStream {
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
            .map(|(i, t)| format!("{}_{}", type_to_param_name(t), i + 1))
            .collect();

        let func_name = format_ident!("{}_get", name);
        let pallet_ident = format_ident!("{}", pallet);
        let storage_ident = format_ident!("{}", name);

        let key_params: Vec<Ident> = key_names.iter().map(|n| format_ident!("{}", n)).collect();
        let key_type_tokens: Vec<TokenStream> =
            key_types.iter().map(|t| parse_type_string(t)).collect();
        let return_type_tokens = parse_type_string(return_type);
        let network_ident = format_ident!("{}", network.to_str());

        quote! {
            /// Get storage n-map value by keys
            pub async fn #func_name(
                &self,
                #(#key_params: #key_type_tokens),*
            ) -> Result<Option<#return_type_tokens>, Box<dyn std::error::Error>> {
                let call = crate::interfaces::#network_ident::api::storage().#pallet_ident().#storage_ident(#(#key_params),*);
                let storage = self.client.storage().at_latest().await?;
                Ok(storage.fetch(&call).await?)
            }
        }
    } else {
        panic!("Expected StoragePattern::NMap");
    }
}

/// Generate query function for Storage N Map
fn generate_nmap_query(network: &InterfaceSource, pattern: &StoragePattern) -> TokenStream {
    if let StoragePattern::NMap {
        name,
        pallet,
        key_types,
        return_type,
    } = pattern
    {
        let pallet_ident = format_ident!("{}", pallet);
        let storage_iter_ident = format_ident!("{}_iter", name);

        let key_type_tokens: Vec<TokenStream> =
            key_types.iter().map(|t| parse_type_string(t)).collect();
        let return_type_tokens = parse_type_string(return_type);
        let network_ident = format_ident!("{}", network.to_str());

        // Generate tuple type for keys
        let keys_tuple = if key_types.len() == 1 {
            quote! { #(#key_type_tokens),* }
        } else {
            quote! { (#(#key_type_tokens),*) }
        };

        let mut prefix_iters = vec![];
        if key_types.len() > 1 {
            for i in 1..key_types.len() {
                let storage_iter_ident = format_ident!("{}_iter{}", name, i);

                let (key_ident_tokens, key_type_tokens) = key_types
                    .iter()
                    .take(i)
                    .enumerate()
                    .map(|(idx, t)| (format_ident!("key{}", idx), parse_type_string(t)))
                    .collect::<(Vec<_>, Vec<_>)>();

                let keys_tuple = if key_type_tokens.len() == 1 {
                    quote! { #(#key_type_tokens),* }
                } else {
                    quote! { (#(#key_type_tokens),*) }
                };

                prefix_iters.push(quote! {
                    pub async fn #storage_iter_ident(
                        &self,
                        #(#key_ident_tokens: #key_type_tokens),*
                    ) -> Result<impl Stream<Item = Result<(#keys_tuple, #return_type_tokens), crate::error::StorageError>>, crate::error::StorageError> {
                        let call = crate::interfaces::#network_ident::api::storage()
                            .#pallet_ident()
                            .#storage_iter_ident(#(#key_ident_tokens),*);

                        let api = match &self.block {
                            Some(block_hash) => self.client.storage().at(*block_hash),
                            None => self.client.storage().at_latest().await?,
                        };

                        let stream = api
                            .iter(call)
                            .await?
                            .map(|res| {
                                let res = res?;
                                let tuple =
                                    <#keys_tuple as Decode>::decode(&mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]))?;
                                Ok((tuple, res.value))
                            })
                            .into_stream();

                        Ok(stream)
                    }
                });
            }
        }

        quote! {
            /// Query all entries in storage n-map
            pub async fn #storage_iter_ident(
                &self,
            ) -> Result<impl Stream<Item = Result<(#keys_tuple, #return_type_tokens), crate::error::StorageError>>, crate::error::StorageError> {
                let call = crate::interfaces::#network_ident::api::storage()
                    .#pallet_ident()
                    .#storage_iter_ident();

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let tuple =
                            <#keys_tuple as Decode>::decode(&mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]))?;
                        Ok((tuple, res.value))
                    })
                    .into_stream();

                Ok(stream)
            }

            #(#prefix_iters)*
        }
    } else {
        panic!("Expected StoragePattern::NMap");
    }
}

/// Generate query function for Storage N Map (raw variant)
fn generate_nmap_query_raw(pattern: &StoragePattern) -> TokenStream {
    if let StoragePattern::NMap {
        name,
        pallet,
        key_types,
        return_type,
    } = pattern
    {
        let func_name = format_ident!("query_all_{}_{}_raw", pallet, name);
        let pallet_ident = format_ident!("{}", pallet);
        let storage_iter_ident = format_ident!("{}_iter", name);

        let key_type_tokens: Vec<TokenStream> =
            key_types.iter().map(|t| parse_type_string(t)).collect();
        let return_type_tokens = parse_type_string(return_type);

        // Generate tuple type for keys
        let keys_tuple = if key_types.len() == 1 {
            quote! { #(#key_type_tokens),* }
        } else {
            quote! { (#(#key_type_tokens),*) }
        };

        quote! {
            /// Query all entries in storage n-map (raw types preserved)
            pub async fn #func_name(
                client: &OnlineClient<PolkadotConfig>,
            ) -> Result<Vec<(#keys_tuple, #return_type_tokens)>, Box<dyn std::error::Error>> {
                let storage = client.storage().at_latest().await?;
                let mut result = Vec::new();
                let mut iter = storage.iter(api::storage().#pallet_ident().#storage_iter_ident()).await?;

                while let Some(Ok(kv)) = iter.next().await {
                    match decode_nmap_keys::<#keys_tuple>(&kv.key_bytes) {
                        Ok(keys) => {
                            result.push((keys, kv.value));
                        }
                        Err(_) => continue, // Skip malformed entries
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
    let simplified_type = simplify_type_for_param_name(&cleaned_type);

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
        return unknown_type_token();
    }

    parse_str::<Type>(cleaned_type)
        .map(|parsed_type| quote! { #parsed_type })
        .unwrap_or_else(|_| unknown_type_token())
}

/// Generate fallback token for unknown types
fn unknown_type_token() -> TokenStream {
    let type_ident = format_ident!("UnknownType");
    quote! { #type_ident }
}
