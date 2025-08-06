use std::error::Error;

use syn::{
    GenericArgument, Ident, ImplItem, Item, ItemImpl, ItemMod, PathArguments, ReturnType, Type,
    TypeArray, TypePath, TypeReference, TypeSlice, TypeTuple,
};

use crate::{ir::CallPattern, parser::is_api_impl};

pub(super) fn parse_calls_module(
    calls_mod: &ItemMod,
    pallet_name: &Ident,
) -> Result<Vec<CallPattern>, Box<dyn Error>> {
    let mut calls = Vec::new();

    if let Some((_, items)) = &calls_mod.content {
        for item in items {
            if let Item::Impl(impl_item) = item {
                if is_api_impl(impl_item, "TransactionApi") {
                    extract_calls(impl_item, &mut calls, pallet_name)?;
                }
            }
        }
    }

    Ok(calls)
}

fn extract_calls(
    impl_item: &ItemImpl,
    calls: &mut Vec<CallPattern>,
    pallet: &Ident,
) -> Result<(), Box<dyn Error>> {
    for item in &impl_item.items {
        if let ImplItem::Fn(method) = item {
            let params = method
                .sig
                .inputs
                .iter()
                .filter_map(|arg| match arg {
                    syn::FnArg::Receiver(_) => None,
                    syn::FnArg::Typed(pat_type) => match &*pat_type.pat {
                        syn::Pat::Ident(pat_ident) => {
                            Some((pat_ident.ident.clone(), *pat_type.ty.clone()))
                        }
                        _ => None,
                    },
                })
                .collect::<Vec<_>>();

            let return_type = match &method.sig.output {
                ReturnType::Default => syn::Type::Tuple(TypeTuple {
                    elems: Default::default(),
                    paren_token: Default::default(),
                }),
                ReturnType::Type(_, ty) => {
                    extract_all_generics(ty).first().cloned().cloned().unwrap()
                }
            };

            calls.push(CallPattern {
                name: method.sig.ident.clone(),
                params,
                ret: return_type,
                pallet: pallet.clone(),
            });
        }
    }

    Ok(())
}

fn extract_all_generics(ty: &Type) -> Vec<&Type> {
    let mut generics = Vec::new();

    match ty {
        Type::Path(TypePath { path, .. }) => {
            for segment in &path.segments {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let GenericArgument::Type(inner_ty) = arg {
                            generics.push(inner_ty);
                            // Recursively extract generics from nested types
                            generics.extend(extract_all_generics(inner_ty));
                        }
                    }
                }
            }
        }
        Type::Reference(TypeReference { elem, .. }) => {
            generics.extend(extract_all_generics(elem));
        }
        Type::Slice(TypeSlice { elem, .. }) => {
            generics.extend(extract_all_generics(elem));
        }
        Type::Array(TypeArray { elem, .. }) => {
            generics.extend(extract_all_generics(elem));
        }
        Type::Tuple(tuple) => {
            for elem in &tuple.elems {
                generics.extend(extract_all_generics(elem));
            }
        }
        _ => {}
    }

    generics
}

// fn extract_type_info(items: &[Item]) -> Result<HashMap<String, TypeInfo>, ParseError> {
//     let mut type_info = HashMap::new();

//     // Find the types module
//     for item in items {
//         if let Item::Mod(types_mod) = item {
//             if types_mod.ident == "types" {
//                 if let Some((_, type_items)) = &types_mod.content {
//                     for type_item in type_items {
//                         if let Item::Mod(storage_type_mod) = type_item {
//                             let storage_name = storage_type_mod.ident.to_string();
//                             let info = parse_calls_type_mod(storage_type_mod)?;
//                             type_info.insert(storage_name, info);
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     Ok(type_info)
// }
