use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use stringcase::pascal_case;

use crate::{
    InterfaceSource,
    codegen::{calls::generate_pallet_calls, storage::generate_pallet_storage},
    ir::PalletPattern,
};

mod calls;
mod storage;

pub fn generate_wrappers_for_network(
    mainnet_pallets: &[PalletPattern],
    testnet_pallets: &[PalletPattern],
) -> TokenStream {
    let mut chained_pallet_names = mainnet_pallets
        .iter()
        .chain(testnet_pallets)
        .map(|p| p.name.to_string())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<_>>();
    chained_pallet_names.sort();

    let clients = generate_client_structs(&chained_pallet_names);

    let mainnet_pallets: Vec<TokenStream> = mainnet_pallets
        .iter()
        .map(|pattern| generate_pallet_mod(&InterfaceSource::Mainnet, pattern))
        .collect();

    let testnet_pallets: Vec<TokenStream> = testnet_pallets
        .iter()
        .map(|pattern| generate_pallet_mod(&InterfaceSource::Testnet, pattern))
        .collect();

    quote! {
        //! Generated storage wrapper functions
        //!
        //! This module provides ergonomic access to Substrate storage items.
        //! Functions are automatically generated from the subxt API metadata.

        #![allow(dead_code)]

        // use std::collections::HashMap;
        use subxt::{OnlineClient, PolkadotConfig, utils::H256};
        use codec::Decode;
        use futures::{Stream, StreamExt, TryStreamExt};
        use std::marker::PhantomData;
        // #api_import

        #clients

        #[cfg(feature = "mainnet")]
        pub mod mainnet {
            use super::*;
            use crate::interfaces::mainnet::api::runtime_types;

            #(#mainnet_pallets)*
        }

        #[cfg(feature = "testnet")]
        pub mod testnet {
            use super::*;
            use crate::interfaces::testnet::api::runtime_types;

            #(#testnet_pallets)*
        }
    }
}

fn generate_client_structs(pallet_names: &[String]) -> TokenStream {
    let (struct_idents, storage_struct_idents, calls_struct_idents, method_idents) = pallet_names
        .iter()
        .map(|name| {
            (
                format_ident!("{}", pascal_case(&format!("{name}_client"))),
                format_ident!("{}", pascal_case(&format!("{name}_storage"))),
                format_ident!("{}", pascal_case(&format!("{name}_calls"))),
                format_ident!("{}", name),
            )
        })
        .collect::<(Vec<_>, Vec<_>, Vec<_>, Vec<_>)>();

    quote! {
        #(
            #[derive(Clone)]
            pub struct #struct_idents<C: crate::chain::Chain> {
                pub(crate) client: OnlineClient<PolkadotConfig>,
                pub(crate) _pd: PhantomData<C>
            }

            impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
                pub fn #method_idents(&self) -> #struct_idents<C> {
                    #struct_idents {
                        client: self.client.clone(),
                        _pd: PhantomData
                    }
                }
            }

            #[derive(Clone)]
            pub struct #storage_struct_idents<C: crate::chain::Chain> {
                pub(crate) client: OnlineClient<PolkadotConfig>,
                pub(crate) block: Option<H256>,
                pub(crate) _pd: PhantomData<C>,
            }

            #[derive(Clone)]
            pub struct #calls_struct_idents<C: crate::chain::Chain> {
                pub(crate) client: OnlineClient<PolkadotConfig>,
                pub(crate) _pd: PhantomData<C>,
            }
        )*
    }
}

fn generate_pallet_mod(network: &InterfaceSource, pallet: &PalletPattern) -> TokenStream {
    let pallet_name = format_ident!("{}", pallet.name);

    let storage = generate_pallet_storage(network, pallet);
    let calls = generate_pallet_calls(network, pallet);

    if storage.is_none() && calls.is_none() {
        return quote! {};
    }

    quote! {
        pub mod #pallet_name {
            use super::*;

            #storage

            #calls

        }
    }
}
