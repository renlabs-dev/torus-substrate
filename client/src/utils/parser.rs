use std::collections::HashMap;
use syn::{parse_file, ImplItem, Item, ItemImpl, ItemMod, Type, TypePath};

/// Error types for storage parsing
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Syn parsing error: {0}")]
    SynError(#[from] syn::Error),
    #[error("API module not found in the provided file")]
    ApiModuleNotFound,
    #[error("Storage module not found in pallet: {pallet}")]
    StorageModuleNotFound { pallet: String },
    #[error("StorageApi implementation not found in pallet: {pallet}")]
    StorageApiNotFound { pallet: String },
    #[error("Invalid method signature: {method}")]
    InvalidMethodSignature { method: String },
    #[error("Type information not found for storage: {storage}")]
    TypeInfoNotFound { storage: String },
}

/// Represents different types of Substrate storage patterns
#[derive(Debug, Clone, PartialEq)]
pub enum StoragePattern {
    /// Storage Value - no input parameters, single output value
    Value {
        name: String,
        pallet: String,
        return_type: String,
    },
    /// Storage Map - single key input, maps to single value
    Map {
        name: String,
        pallet: String,
        key_type: String,
        return_type: String,
    },
    /// Storage Double Map - two key inputs, maps to single value
    DoubleMap {
        name: String,
        pallet: String,
        key1_type: String,
        key2_type: String,
        return_type: String,
    },
    /// Storage N Map - multiple key inputs (N > 2), maps to single value
    NMap {
        name: String,
        pallet: String,
        key_types: Vec<String>,
        return_type: String,
    },
}

impl StoragePattern {
    /// Get the storage name
    pub fn name(&self) -> &str {
        match self {
            StoragePattern::Value { name, .. }
            | StoragePattern::Map { name, .. }
            | StoragePattern::DoubleMap { name, .. }
            | StoragePattern::NMap { name, .. } => name,
        }
    }

    /// Get the pallet name
    pub fn pallet(&self) -> &str {
        match self {
            StoragePattern::Value { pallet, .. }
            | StoragePattern::Map { pallet, .. }
            | StoragePattern::DoubleMap { pallet, .. }
            | StoragePattern::NMap { pallet, .. } => pallet,
        }
    }

    /// Get the return type
    pub fn return_type(&self) -> &str {
        match self {
            StoragePattern::Value { return_type, .. }
            | StoragePattern::Map { return_type, .. }
            | StoragePattern::DoubleMap { return_type, .. }
            | StoragePattern::NMap { return_type, .. } => return_type,
        }
    }
}

/// Contains information about a storage method found in the API
#[derive(Clone)]
struct StorageMethodInfo {
    name: String,
    param_count: usize,
    is_iter: bool,
}

/// Type information extracted from storage type modules
#[derive(Debug, Clone)]
struct StorageTypeInfo {
    return_type: String,
    param_types: Vec<String>,
}

/// Main parser for extracting storage patterns from subxt-generated API files
pub struct StorageParser;

impl StorageParser {
    /// Parse an api.rs file and extract all storage patterns
    pub fn parse_api_file(content: &str) -> Result<Vec<StoragePattern>, ParseError> {
        let syntax_tree = parse_file(content)?;
        let mut patterns = Vec::new();

        // Find the main API module
        for item in syntax_tree.items {
            if let Item::Mod(api_mod) = item {
                if api_mod.ident == "api" {
                    patterns.extend(Self::parse_api_module(&api_mod)?);
                    break;
                }
            }
        }

        if patterns.is_empty() {
            return Err(ParseError::ApiModuleNotFound);
        }

        Ok(patterns)
    }

    /// Parse the main API module to find pallet modules
    fn parse_api_module(api_mod: &ItemMod) -> Result<Vec<StoragePattern>, ParseError> {
        let mut patterns = Vec::new();

        if let Some((_, items)) = &api_mod.content {
            for item in items {
                if let Item::Mod(pallet_mod) = item {
                    // Skip special modules that aren't pallets
                    let pallet_name = pallet_mod.ident.to_string();
                    if [
                        "runtime_types",
                        "constants",
                        "storage",
                        "apis",
                        "events",
                        "calls",
                    ]
                    .contains(&pallet_name.as_str())
                    {
                        continue;
                    }

                    if let Ok(pallet_patterns) = Self::parse_pallet_module(pallet_mod) {
                        patterns.extend(pallet_patterns);
                    }
                }
            }
        }

        Ok(patterns)
    }

    /// Parse a single pallet module to find storage patterns
    fn parse_pallet_module(pallet_mod: &ItemMod) -> Result<Vec<StoragePattern>, ParseError> {
        let pallet_name = pallet_mod.ident.to_string();
        let mut patterns = Vec::new();

        if let Some((_, items)) = &pallet_mod.content {
            // Find the storage module
            for item in items {
                if let Item::Mod(storage_mod) = item {
                    if storage_mod.ident == "storage" {
                        patterns.extend(Self::parse_storage_module(storage_mod, &pallet_name)?);
                        break;
                    }
                }
            }
        }

        Ok(patterns)
    }

    /// Parse a storage module to extract storage patterns
    fn parse_storage_module(
        storage_mod: &ItemMod,
        pallet_name: &str,
    ) -> Result<Vec<StoragePattern>, ParseError> {
        let mut patterns = Vec::new();

        if let Some((_, items)) = &storage_mod.content {
            // Extract type information first
            let type_info = Self::extract_type_info(items)?;

            // Find StorageApi implementation
            for item in items {
                if let Item::Impl(impl_item) = item {
                    if Self::is_storage_api_impl(impl_item) {
                        let methods = Self::extract_storage_methods(impl_item)?;
                        patterns.extend(Self::analyze_storage_methods(
                            methods,
                            &type_info,
                            pallet_name,
                        )?);
                    }
                }
            }
        }

        Ok(patterns)
    }

    /// Check if an impl block is for StorageApi
    fn is_storage_api_impl(impl_item: &ItemImpl) -> bool {
        if let Type::Path(TypePath { path, .. }) = &*impl_item.self_ty {
            if let Some(segment) = path.segments.last() {
                return segment.ident == "StorageApi";
            }
        }
        false
    }

    /// Extract storage methods from StorageApi implementation
    fn extract_storage_methods(impl_item: &ItemImpl) -> Result<Vec<StorageMethodInfo>, ParseError> {
        let mut methods = Vec::new();

        for item in &impl_item.items {
            if let ImplItem::Fn(method) = item {
                let method_name = method.sig.ident.to_string();
                let param_count = method.sig.inputs.len().saturating_sub(1); // Subtract 1 for &self
                let is_iter = method_name.ends_with("_iter")
                    || method_name.ends_with("_iter1")
                    || method_name.ends_with("_iter2");

                methods.push(StorageMethodInfo {
                    name: method_name,
                    param_count,
                    is_iter,
                });
            }
        }

        Ok(methods)
    }

    /// Analyze storage methods to determine patterns
    fn analyze_storage_methods(
        methods: Vec<StorageMethodInfo>,
        type_info: &HashMap<String, StorageTypeInfo>,
        pallet_name: &str,
    ) -> Result<Vec<StoragePattern>, ParseError> {
        let mut patterns = Vec::new();
        let mut method_groups: HashMap<String, Vec<&StorageMethodInfo>> = HashMap::new();

        // Group methods by base name (without _iter suffix and variants)
        for method in &methods {
            let base_name = if method.is_iter {
                // Handle _iter, _iter1, _iter2, etc.
                method.name.strip_suffix("_iter").unwrap_or(
                    method
                        .name
                        .strip_suffix("_iter1")
                        .unwrap_or(method.name.strip_suffix("_iter2").unwrap_or(&method.name)),
                )
            } else {
                &method.name
            };
            method_groups
                .entry(base_name.to_string())
                .or_default()
                .push(method);
        }

        // Analyze each group to determine storage pattern
        for (base_name, group) in method_groups {
            let pattern =
                Self::determine_storage_pattern(&base_name, group, type_info, pallet_name)?;
            if let Some(p) = pattern {
                patterns.push(p);
            }
        }

        Ok(patterns)
    }

    /// Determine the storage pattern from a group of methods
    fn determine_storage_pattern(
        base_name: &str,
        methods: Vec<&StorageMethodInfo>,
        type_info: &HashMap<String, StorageTypeInfo>,
        pallet_name: &str,
    ) -> Result<Option<StoragePattern>, ParseError> {
        let has_iter = methods.iter().any(|m| m.is_iter);
        let non_iter_methods: Vec<_> = methods.iter().filter(|m| !m.is_iter).collect();

        if non_iter_methods.is_empty() {
            // Skip if we don't have any non-iter methods
            return Ok(None);
        }

        // For multiple non-iter methods, pick the one with the most parameters
        // (this handles cases where there might be overloaded methods)
        let main_method = non_iter_methods
            .iter()
            .max_by_key(|m| m.param_count)
            .unwrap();
        let param_count = main_method.param_count;

        // Get type information for this storage item - if not found, skip
        let types = match type_info.get(base_name) {
            Some(t) => t,
            None => return Ok(None), // Skip if no type info found
        };

        let pattern = match (param_count, has_iter) {
            (0, false) => {
                // Storage Value - no parameters, no iter method
                StoragePattern::Value {
                    name: base_name.to_string(),
                    pallet: pallet_name.to_string(),
                    return_type: types.return_type.clone(),
                }
            }
            (1, true) => {
                // Storage Map - one parameter, has iter method
                StoragePattern::Map {
                    name: base_name.to_string(),
                    pallet: pallet_name.to_string(),
                    key_type: types.param_types.get(0).cloned().unwrap_or_default(),
                    return_type: types.return_type.clone(),
                }
            }
            (2, true) => {
                // Storage Double Map - two parameters, has iter method
                StoragePattern::DoubleMap {
                    name: base_name.to_string(),
                    pallet: pallet_name.to_string(),
                    key1_type: types.param_types.get(0).cloned().unwrap_or_default(),
                    key2_type: types.param_types.get(1).cloned().unwrap_or_default(),
                    return_type: types.return_type.clone(),
                }
            }
            (n, true) if n > 2 => {
                // Storage N Map - multiple parameters (N > 2), has iter method
                StoragePattern::NMap {
                    name: base_name.to_string(),
                    pallet: pallet_name.to_string(),
                    key_types: types.param_types.clone(),
                    return_type: types.return_type.clone(),
                }
            }
            _ => {
                // Unsupported pattern, skip
                return Ok(None);
            }
        };

        Ok(Some(pattern))
    }

    /// Extract type information from the types module
    fn extract_type_info(items: &[Item]) -> Result<HashMap<String, StorageTypeInfo>, ParseError> {
        let mut type_info = HashMap::new();

        // Find the types module
        for item in items {
            if let Item::Mod(types_mod) = item {
                if types_mod.ident == "types" {
                    if let Some((_, type_items)) = &types_mod.content {
                        for type_item in type_items {
                            if let Item::Mod(storage_type_mod) = type_item {
                                let storage_name = storage_type_mod.ident.to_string();
                                let info = Self::parse_storage_type_mod(storage_type_mod)?;
                                type_info.insert(storage_name, info);
                            }
                        }
                    }
                }
            }
        }

        Ok(type_info)
    }

    /// Parse a single storage type module to extract type information
    fn parse_storage_type_mod(storage_type_mod: &ItemMod) -> Result<StorageTypeInfo, ParseError> {
        let mut return_type = String::new();
        let mut param_types = Vec::new();

        if let Some((_, items)) = &storage_type_mod.content {
            for item in items {
                if let Item::Type(type_alias) = item {
                    let type_name = type_alias.ident.to_string();
                    let type_str = Self::type_to_string(&type_alias.ty);

                    if type_name.starts_with("Param") {
                        // This is a parameter type (Param0, Param1, etc.)
                        let param_index = type_name
                            .strip_prefix("Param")
                            .and_then(|s| s.parse::<usize>().ok())
                            .unwrap_or(0);

                        // Ensure the param_types vector is large enough
                        while param_types.len() <= param_index {
                            param_types.push(String::new());
                        }
                        param_types[param_index] = type_str;
                    } else if !type_name.starts_with("__") {
                        // This is likely the return type (not a special subxt type)
                        return_type = type_str;
                    }
                }
            }
        }

        // Remove empty entries from param_types
        param_types.retain(|t| !t.is_empty());

        Ok(StorageTypeInfo {
            return_type,
            param_types,
        })
    }

    /// Convert a syn Type to a string representation
    fn type_to_string(ty: &Type) -> String {
        quote::quote! { #ty }.to_string()
    }
}
