use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    InterfaceSource,
    codegen::{calls::generate_pallet_calls, storage::generate_pallet_storage},
    ir::PalletPattern,
};

mod calls;
mod storage;

pub fn generate_wrappers_for_network(
    network: &InterfaceSource,
    pallets: &[PalletPattern],
) -> TokenStream {
    let mut sorted_patterns = pallets.to_vec();
    sorted_patterns.sort_by(|a, b| a.name.cmp(&b.name));

    let pallets: Vec<TokenStream> = sorted_patterns
        .iter()
        .map(|pattern| generate_pallet_mod(network, pattern))
        .collect();

    // Generate helper functions for key decoding
    // let helper_functions = generate_helper_functions();

    let api_import = match network {
        InterfaceSource::Mainnet => quote! {
            use crate::interfaces::mainnet::api;
            use crate::interfaces::mainnet::api::runtime_types;
        },
        InterfaceSource::Testnet => quote! {
            use crate::interfaces::testnet::api;
            use crate::interfaces::testnet::api::runtime_types;
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

        #(#pallets)*
    }
}

fn generate_pallet_mod(network: &InterfaceSource, pallet: &PalletPattern) -> TokenStream {
    let network_name = network.to_str();
    let pallet_name = format_ident!("{}", pallet.name);

    let storage = generate_pallet_storage(network, pallet);
    let calls = generate_pallet_calls(network, pallet);

    quote! {

        #[cfg(feature = #network_name)]
        pub mod #pallet_name {

            #storage

            #calls

        }
    }
}
