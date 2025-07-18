use std::error::Error;

use syn::{Item, ItemImpl, ItemMod, Type, TypePath};

use crate::{
    ir::PalletPattern,
    parser::{calls::parse_calls_module, storage::parse_storage_module},
};

mod calls;
mod storage;

const IGNORED_MODULES: [&'static str; 8] = [
    "runtime_types",
    "constants",
    "storage",
    "apis",
    "events",
    "calls",
    "runtime_apis",
    "root_mod",
];

pub fn parse_api_file(content: &str) -> Result<Vec<PalletPattern>, Box<dyn Error>> {
    let syntax_tree = syn::parse_file(content)?;

    let api_mod = syntax_tree
        .items
        .iter()
        .find_map(|item| match item {
            Item::Mod(item_mod) => {
                if item_mod.ident.to_string() == "api" {
                    return Some(item_mod);
                }
                return None;
            }
            _ => return None,
        })
        .unwrap();

    let Some((_, items)) = &api_mod.content else {
        return Err("api module not found".into());
    };

    let pallets: Result<Vec<PalletPattern>, Box<dyn Error>> = items
        .iter()
        .filter_map(|item| match item {
            Item::Mod(item_mod) => {
                let mod_name = item_mod.ident.to_string();
                if IGNORED_MODULES.contains(&mod_name.as_str()) {
                    return None;
                }

                Some(parse_pallet_module(item_mod))
            }
            _ => None,
        })
        .collect();

    let pallets = pallets?;

    if pallets.is_empty() {
        return Err("api module not found".into());
    }

    Ok(pallets)
}

fn parse_pallet_module(pallet_mod: &ItemMod) -> Result<PalletPattern, Box<dyn Error>> {
    let pallet_name = &pallet_mod.ident;

    let mut storages = Vec::new();
    let mut calls = Vec::new();

    if let Some((_, items)) = &pallet_mod.content {
        // Find the storage module
        for item in items {
            if let Item::Mod(item_mod) = item {
                if item_mod.ident == "storage" {
                    storages.extend(parse_storage_module(item_mod, &pallet_name.to_string())?);
                }

                if item_mod.ident == "calls" {
                    calls.extend(parse_calls_module(item_mod, &pallet_name)?);
                }
            }
        }
    }

    Ok(PalletPattern {
        name: pallet_name.clone(),
        storages,
        calls,
    })
}

fn is_api_impl(impl_item: &ItemImpl, name: &'static str) -> bool {
    if let Type::Path(TypePath { path, .. }) = &*impl_item.self_ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == name;
        }
    }
    false
}

fn type_to_string(ty: &Type) -> String {
    quote::quote! { #ty }.to_string()
}
