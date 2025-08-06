use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use stringcase::pascal_case;
use syn::{Ident, Type};

use crate::{
    InterfaceSource,
    ir::{CallPattern, PalletPattern},
};

const UNSIGNED_CALLS: [&str; 1] = ["faucet"];

pub(super) fn generate_pallet_calls(
    network: &InterfaceSource,
    pallet: &PalletPattern,
) -> Option<TokenStream> {
    let client_name = format_ident!("{}", pascal_case(&format!("{}_client", pallet.name)));
    let struct_name = format_ident!("{}", pascal_case(&format!("{}_calls", pallet.name)));
    let network_chain: Type = syn::parse_str(network.to_chain_type()).unwrap();

    let functions: Vec<TokenStream> = pallet
        .calls
        .iter()
        .map(|pattern| generate_pattern_wrappers(network, pattern))
        .collect();

    if functions.is_empty() {
        return None;
    }

    Some(quote! {
        impl #client_name<#network_chain> {
            pub fn calls(&self) -> #struct_name<#network_chain> {
                #struct_name {
                    client: self.client.clone(),
                    _pd: PhantomData
                }
            }
        }

        impl #struct_name<#network_chain> {
            #(#functions)*
        }
    })
}

fn generate_pattern_wrappers(network: &InterfaceSource, pattern: &CallPattern) -> TokenStream {
    let fn_name = &pattern.name;
    let (param_idents, param_types) = &pattern
        .params
        .iter()
        .cloned()
        .collect::<(Vec<Ident>, Vec<Type>)>();

    let network = format_ident!("{}", network.to_str());
    let pallet = &pattern.pallet;

    let wait_fn_name = format_ident!("{}_wait", fn_name);

    if !UNSIGNED_CALLS.contains(&fn_name.to_string().as_str()) {
        quote! {
            pub async fn #fn_name(&self, #(#param_idents: crate::interfaces::#network::api::#pallet::calls::#param_types,)* signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::#network::api::tx().#pallet().#fn_name(#(#param_idents),*);
                Ok(self.client.tx().sign_and_submit_default(&call, &signer).await?)
            }

            pub async fn #wait_fn_name(&self, #(#param_idents: crate::interfaces::#network::api::#pallet::calls::#param_types,)* signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::#network::api::tx().#pallet().#fn_name(#(#param_idents),*);

                self.client.tx().sign_and_submit_then_watch_default(&call, &signer).await?.wait_for_finalized_success().await?;

                Ok(H256::zero())
            }
        }
    } else {
        quote! {
            pub async fn #fn_name(&self, #(#param_idents: crate::interfaces::#network::api::#pallet::calls::#param_types,)*) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::#network::api::tx().#pallet().#fn_name(#(#param_idents),*);
                Ok(self.client.tx().create_unsigned(&call)?.submit().await?)
            }

            pub async fn #wait_fn_name(&self, #(#param_idents: crate::interfaces::#network::api::#pallet::calls::#param_types,)*) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::#network::api::tx().#pallet().#fn_name(#(#param_idents),*);

                self.client.tx().create_unsigned(&call)?.submit_and_watch().await?.wait_for_finalized_success().await?;

                Ok(H256::zero())
            }
        }
    }
}
