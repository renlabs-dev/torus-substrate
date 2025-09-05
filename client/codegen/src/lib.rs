use clap::Parser;
use codec::Decode;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::{Path, PathBuf};
use subxt_codegen::{CodegenBuilder, Metadata};
use subxt_utils_fetchmetadata::{MetadataVersion, Url};
use syn::parse_quote;

use crate::codegen::generate_wrappers_for_network;

mod codegen;
mod ir;
mod parser;

#[derive(Parser)]
#[command(name = "generate-wrappers")]
#[command(about = "Generate storage wrapper functions from subxt interfaces")]
struct Args {
    #[arg(short, long)]
    output: PathBuf,

    /// Force overwrite existing output file
    #[arg(short, long)]
    force: bool,
}

#[derive(clap::ValueEnum, Clone)]
enum InterfaceSource {
    Mainnet,
    Testnet,
    Devnet,
}

impl InterfaceSource {
    fn to_str(&self) -> &'static str {
        match self {
            InterfaceSource::Mainnet => "mainnet",
            InterfaceSource::Testnet => "testnet",
            InterfaceSource::Devnet => "devnet",
        }
    }

    fn to_chain_type(&self) -> &'static str {
        match self {
            InterfaceSource::Mainnet => "crate::chain::MainNet",
            InterfaceSource::Testnet => "crate::chain::TestNet",
            InterfaceSource::Devnet => "crate::chain::DevNet",
        }
    }
}

pub async fn generate_interfaces(
    output: &Path,
    devnet_url: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mainnet_content_tokens = generate_subxt_code_for_url("wss://api.torus.network").await?;
    let testnet_content_tokens =
        generate_subxt_code_for_url("wss://api.testnet.torus.network").await?;
    let devnet_content_tokens = if let Some(url) = &devnet_url {
        generate_subxt_code_for_url(url).await?
    } else {
        quote! {}
    };

    let mainnet_content = mainnet_content_tokens.to_string();
    let testnet_content = testnet_content_tokens.to_string();
    let devnet_content = devnet_content_tokens.to_string();

    let mainnet_pallets = parser::parse_api_file(&mainnet_content)?;
    let testnet_pallets = parser::parse_api_file(&testnet_content)?;
    let devnet_pallets = if devnet_url.is_some() {
        parser::parse_api_file(&devnet_content)?
    } else {
        vec![]
    };

    let wrappers_tokens =
        generate_wrappers_for_network(&mainnet_pallets, &testnet_pallets, &devnet_pallets);

    // Parse the TokenStream into a syn::File and format with prettyplease
    let wrappers_file: syn::File = parse_quote! {
        #[cfg(feature = "mainnet")]
        pub mod mainnet {
            #mainnet_content_tokens
        }

        #[cfg(feature = "testnet")]
        pub mod testnet {
            #testnet_content_tokens
        }

        #[cfg(feature = "devnet")]
        pub mod devnet {
            #devnet_content_tokens
        }

        pub mod wrappers {
            #wrappers_tokens
        }
    };

    let wrappers_code = prettyplease::unparse(&wrappers_file).to_string();

    // Create output directory if it doesn't exist
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(output, wrappers_code)?;

    Ok(())
}

async fn generate_subxt_code_for_url(
    url: impl AsRef<str>,
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let metadata_bytes =
        subxt_utils_fetchmetadata::from_url(Url::parse(url.as_ref())?, MetadataVersion::default())
            .await?;

    let mut metadata = Metadata::decode(&mut &*metadata_bytes)?;

    scale_typegen::utils::ensure_unique_type_paths(metadata.types_mut())?;

    let codegen = CodegenBuilder::default();

    let code = codegen.generate(metadata)?;

    Ok(code)
}
