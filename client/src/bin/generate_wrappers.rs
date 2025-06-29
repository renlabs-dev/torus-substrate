use clap::Parser;
use std::path::PathBuf;
use syn::parse_quote;
use torus_client::utils::{codegen::WrapperGenerator, parser::StorageParser};

#[derive(Parser)]
#[command(name = "generate-wrappers")]
#[command(about = "Generate storage wrapper functions from subxt interfaces")]
struct Args {
    /// Source interface to parse (mainnet or testnet)
    #[arg(short, long, value_enum)]
    source: InterfaceSource,

    /// Output file path
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let interface_path = match args.source {
        InterfaceSource::Mainnet => "src/interfaces/mainnet.rs",
        InterfaceSource::Testnet => "src/interfaces/testnet.rs",
    };

    // Get the full path for better error reporting
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

    if args.output.exists() && !args.force {
        eprintln!("Error: Output file already exists: {:?}", args.output);
        eprintln!("Use --force to overwrite");
        std::process::exit(1);
    }

    println!("Parsing interface: {}", interface_path);
    let content = std::fs::read_to_string(interface_path)?;
    let patterns = StorageParser::parse_api_file(&content)?;

    println!("Found {} storage patterns", patterns.len());

    // Generate the wrappers
    let wrappers_tokens =
        WrapperGenerator::generate_wrappers_for_network(&patterns, args.source.to_str());

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
