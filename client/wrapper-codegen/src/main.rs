use clap::Parser;
use std::path::PathBuf;
use syn::{parse_quote, parse_str};

use crate::{codegen::generate_wrappers_for_network, parser::parse_api_file};

mod codegen;
mod error;
mod ir;
mod parser;
mod utils;

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
}

impl InterfaceSource {
    fn to_str(&self) -> &'static str {
        match self {
            InterfaceSource::Mainnet => "mainnet",
            InterfaceSource::Testnet => "testnet",
        }
    }

    fn to_chain_type(&self) -> &'static str {
        match self {
            InterfaceSource::Mainnet => "crate::chain::MainNet",
            InterfaceSource::Testnet => "crate::chain::TestNet",
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mainnet_content = read_interface("src/interfaces/mainnet.rs")?;
    let testnet_content = read_interface("src/interfaces/testnet.rs")?;

    let mainnet_pallets = parser::parse_api_file(&mainnet_content)?;
    let testnet_pallets = parser::parse_api_file(&testnet_content)?;

    let wrappers_tokens = generate_wrappers_for_network(&mainnet_pallets, &testnet_pallets);

    // Parse the TokenStream into a syn::File and format with prettyplease
    let wrappers_file: syn::File = parse_quote! {
        #wrappers_tokens
    };

    let wrappers_code = prettyplease::unparse(&wrappers_file);

    // Create output directory if it doesn't exist
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&args.output, wrappers_code)?;
    println!("Generated wrappers: {:?}", args.output);

    Ok(())
}

fn read_interface(interface_path: &'static str) -> Result<String, std::io::Error> {
    let full_path = std::path::Path::new(interface_path)
        .canonicalize()
        .unwrap_or_else(|_| {
            std::env::current_dir()
                .unwrap_or_default()
                .join(interface_path)
        });

    if !std::path::Path::new(interface_path).exists() {
        eprintln!("Error: Interface file not found: {}", interface_path);
        eprintln!("Full path searched: {:?}", full_path);
        eprintln!("Current working directory: {:?}", std::env::current_dir()?);
        std::process::exit(1);
    }

    std::fs::read_to_string(interface_path)
}
