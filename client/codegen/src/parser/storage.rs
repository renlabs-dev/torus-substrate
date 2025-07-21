use std::{collections::HashMap, error::Error};

use syn::{ImplItem, Item, ItemImpl, ItemMod};

use super::is_api_impl;
use crate::{ir::StoragePattern, parser::type_to_string};

pub(super) fn parse_storage_module(
    storage_mod: &ItemMod,
    pallet_name: &str,
) -> Result<Vec<StoragePattern>, Box<dyn Error>> {
    let mut patterns = Vec::new();

    if let Some((_, items)) = &storage_mod.content {
        // Extract type information first
        let type_info = extract_type_info(items)?;

        // Find StorageApi implementation
        for item in items {
            if let Item::Impl(impl_item) = item {
                if is_api_impl(impl_item, "StorageApi") {
                    let methods = extract_storage_methods(impl_item)?;
                    patterns.extend(analyze_storage_methods(methods, &type_info, pallet_name)?);
                }
            }
        }
    }

    Ok(patterns)
}

fn extract_storage_methods(impl_item: &ItemImpl) -> Result<Vec<MethodInfo>, Box<dyn Error>> {
    let mut methods = Vec::new();

    for item in &impl_item.items {
        if let ImplItem::Fn(method) = item {
            let method_name = method.sig.ident.to_string();
            let param_count = method.sig.inputs.len().saturating_sub(1); // Subtract 1 for &self
            let is_iter = method_name.contains("_iter");

            methods.push(MethodInfo {
                name: method_name,
                param_count,
                is_iter,
            });
        }
    }

    Ok(methods)
}

fn analyze_storage_methods(
    methods: Vec<MethodInfo>,
    type_info: &HashMap<String, TypeInfo>,
    pallet_name: &str,
) -> Result<Vec<StoragePattern>, Box<dyn Error>> {
    let mut patterns = Vec::new();
    let mut method_groups: HashMap<String, Vec<&MethodInfo>> = HashMap::new();

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
        let pattern = determine_storage_pattern(&base_name, group, type_info, pallet_name)?;
        if let Some(p) = pattern {
            patterns.push(p);
        }
    }

    Ok(patterns)
}

fn determine_storage_pattern(
    base_name: &str,
    methods: Vec<&MethodInfo>,
    type_info: &HashMap<String, TypeInfo>,
    pallet_name: &str,
) -> Result<Option<StoragePattern>, Box<dyn Error>> {
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
                key_type: types.param_types.first().cloned().unwrap_or_default(),
                return_type: types.return_type.clone(),
            }
        }
        (2, true) => {
            // Storage Double Map - two parameters, has iter method
            StoragePattern::DoubleMap {
                name: base_name.to_string(),
                pallet: pallet_name.to_string(),
                key1_type: types.param_types.first().cloned().unwrap_or_default(),
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

#[derive(Clone)]
struct MethodInfo {
    name: String,
    param_count: usize,
    is_iter: bool,
}

#[derive(Debug, Clone)]
struct TypeInfo {
    return_type: String,
    param_types: Vec<String>,
}

fn extract_type_info(items: &[Item]) -> Result<HashMap<String, TypeInfo>, Box<dyn Error>> {
    let mut type_info = HashMap::new();

    // Find the types module
    for item in items {
        if let Item::Mod(types_mod) = item {
            if types_mod.ident == "types" {
                if let Some((_, type_items)) = &types_mod.content {
                    for type_item in type_items {
                        if let Item::Mod(storage_type_mod) = type_item {
                            let storage_name = storage_type_mod.ident.to_string();
                            let info = parse_storage_type_mod(storage_type_mod)?;
                            type_info.insert(storage_name, info);
                        }
                    }
                }
            }
        }
    }

    Ok(type_info)
}

fn parse_storage_type_mod(storage_type_mod: &ItemMod) -> Result<TypeInfo, Box<dyn Error>> {
    let mut return_type = String::new();
    let mut param_types = Vec::new();

    if let Some((_, items)) = &storage_type_mod.content {
        for item in items {
            if let Item::Type(type_alias) = item {
                let type_name = type_alias.ident.to_string();
                let type_str = type_to_string(&type_alias.ty);

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

    Ok(TypeInfo {
        return_type,
        param_types,
    })
}
